/// Segment table entry: (stripped_start, original_start).
/// Maps byte positions in a stripped/reconstructed string to the original document.
/// Must be sorted by stripped_start.
pub type SegmentTable = Vec<(usize, usize)>;

/// Maps a stripped offset to its original document offset using a segment table.
pub fn remap_offset(offset: usize, segments: &SegmentTable) -> usize {
    debug_assert!(!segments.is_empty());
    let idx = segments.partition_point(|&(s, _)| s <= offset);
    let idx = if idx > 0 { idx - 1 } else { 0 };
    let (seg_start, orig_start) = segments[idx];
    orig_start + (offset - seg_start)
}