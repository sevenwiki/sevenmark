use std::fmt;
use std::ops::Deref;

use winnow::stream::{
    AsBStr, AsBytes, Compare, CompareResult, FindSlice, LocatingSlice, Location, Needed, Offset,
    SliceLen, Stream, StreamIsPartial, UpdateSlice,
};

#[derive(Debug, Clone)]
pub struct SourceSegment {
    pub logical_start: usize,
    pub original_start: usize,
    pub len: usize,
}

impl SourceSegment {
    fn logical_end(&self) -> usize {
        self.logical_start + self.len
    }

    fn original_end(&self) -> usize {
        self.original_start + self.len
    }
}

#[derive(Debug, Clone)]
enum SourceMap {
    Identity {
        base: usize,
    },
    Segmented {
        segments: Vec<SourceSegment>,
        empty_original_offset: usize,
    },
}

impl SourceMap {
    fn map_start(&self, offset: usize) -> usize {
        match self {
            SourceMap::Identity { base } => base + offset,
            SourceMap::Segmented {
                segments,
                empty_original_offset,
            } => {
                if segments.is_empty() {
                    return *empty_original_offset;
                }

                for segment in segments {
                    if offset >= segment.logical_start && offset < segment.logical_end() {
                        return segment.original_start + (offset - segment.logical_start);
                    }
                }

                segments
                    .last()
                    .map(SourceSegment::original_end)
                    .unwrap_or(*empty_original_offset)
            }
        }
    }

    fn map_end(&self, offset: usize) -> usize {
        match self {
            SourceMap::Identity { base } => base + offset,
            SourceMap::Segmented {
                segments,
                empty_original_offset,
            } => {
                if segments.is_empty() {
                    return *empty_original_offset;
                }

                for segment in segments.iter().rev() {
                    if offset > segment.logical_start && offset <= segment.logical_end() {
                        return segment.original_start + (offset - segment.logical_start);
                    }
                }

                segments
                    .first()
                    .map(|segment| segment.original_start)
                    .unwrap_or(*empty_original_offset)
            }
        }
    }
}

#[derive(Clone)]
pub struct InputSource<'i> {
    original: &'i str,
    logical: LocatingSlice<&'i str>,
    source_map: SourceMap,
}

impl<'i> InputSource<'i> {
    pub fn new(input: &'i str) -> Self {
        Self::new_at(input, 0)
    }

    pub fn new_at(input: &'i str, base: usize) -> Self {
        Self {
            original: input,
            logical: LocatingSlice::new(input),
            source_map: SourceMap::Identity { base },
        }
    }

    pub fn new_segmented(
        input: &'i str,
        segments: Vec<SourceSegment>,
        empty_original_offset: usize,
    ) -> Self {
        Self {
            original: input,
            logical: LocatingSlice::new(input),
            source_map: SourceMap::Segmented {
                segments,
                empty_original_offset,
            },
        }
    }

    pub fn is_at_line_start(&self) -> bool {
        let offset = self.logical.current_token_start();
        offset == 0 || self.original.as_bytes().get(offset - 1) == Some(&b'\n')
    }
}

impl fmt::Debug for InputSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logical.fmt(f)
    }
}

impl fmt::Display for InputSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logical.fmt(f)
    }
}

impl Deref for InputSource<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.logical.as_ref()
    }
}

impl SliceLen for InputSource<'_> {
    fn slice_len(&self) -> usize {
        self.logical.slice_len()
    }
}

impl<'i> Stream for InputSource<'i> {
    type Token = <LocatingSlice<&'i str> as Stream>::Token;
    type Slice = <LocatingSlice<&'i str> as Stream>::Slice;
    type IterOffsets = <LocatingSlice<&'i str> as Stream>::IterOffsets;
    type Checkpoint = <LocatingSlice<&'i str> as Stream>::Checkpoint;

    fn iter_offsets(&self) -> Self::IterOffsets {
        self.logical.iter_offsets()
    }

    fn eof_offset(&self) -> usize {
        self.logical.eof_offset()
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        self.logical.next_token()
    }

    fn peek_token(&self) -> Option<Self::Token> {
        self.logical.peek_token()
    }

    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        self.logical.offset_for(predicate)
    }

    fn offset_at(&self, tokens: usize) -> Result<usize, Needed> {
        self.logical.offset_at(tokens)
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        self.logical.next_slice(offset)
    }

    unsafe fn next_slice_unchecked(&mut self, offset: usize) -> Self::Slice {
        unsafe { self.logical.next_slice_unchecked(offset) }
    }

    fn peek_slice(&self, offset: usize) -> Self::Slice {
        self.logical.peek_slice(offset)
    }

    unsafe fn peek_slice_unchecked(&self, offset: usize) -> Self::Slice {
        unsafe { self.logical.peek_slice_unchecked(offset) }
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.logical.checkpoint()
    }

    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.logical.reset(checkpoint);
    }

    fn trace(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.logical.trace(f)
    }
}

impl Location for InputSource<'_> {
    fn previous_token_end(&self) -> usize {
        self.source_map.map_end(self.logical.previous_token_end())
    }

    fn current_token_start(&self) -> usize {
        self.source_map
            .map_start(self.logical.current_token_start())
    }
}

impl<'i> StreamIsPartial for InputSource<'i> {
    type PartialState = <LocatingSlice<&'i str> as StreamIsPartial>::PartialState;

    fn complete(&mut self) -> Self::PartialState {
        self.logical.complete()
    }

    fn restore_partial(&mut self, state: Self::PartialState) {
        self.logical.restore_partial(state);
    }

    fn is_partial_supported() -> bool {
        <LocatingSlice<&'i str> as StreamIsPartial>::is_partial_supported()
    }

    fn is_partial(&self) -> bool {
        self.logical.is_partial()
    }
}

impl<'i> Offset<<InputSource<'i> as Stream>::Checkpoint> for InputSource<'i> {
    fn offset_from(&self, other: &<InputSource<'i> as Stream>::Checkpoint) -> usize {
        self.logical.offset_from(other)
    }
}

impl AsBytes for InputSource<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.logical.as_bytes()
    }
}

impl AsBStr for InputSource<'_> {
    fn as_bstr(&self) -> &[u8] {
        self.logical.as_bstr()
    }
}

impl<T> Compare<T> for InputSource<'_>
where
    for<'a> &'a str: Compare<T>,
{
    fn compare(&self, other: T) -> CompareResult {
        self.logical.as_ref().compare(other)
    }
}

impl<T> FindSlice<T> for InputSource<'_>
where
    for<'a> &'a str: FindSlice<T>,
{
    fn find_slice(&self, substr: T) -> Option<std::ops::Range<usize>> {
        self.logical.as_ref().find_slice(substr)
    }
}

impl<'i> UpdateSlice for InputSource<'i> {
    fn update_slice(mut self, inner: Self::Slice) -> Self {
        self.logical = self.logical.update_slice(inner);
        self
    }
}
