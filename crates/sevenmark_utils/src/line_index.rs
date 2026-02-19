//! Line index for byte offset → LSP Position conversion
//!
//! Provides O(log n) line lookup via binary search on precomputed line starts,
//! plus O(k) UTF-16 character offset calculation within the line.

use line_span::LineSpans;
use sevenmark_ast::Span;

/// Precomputed line start offsets for fast byte-offset-to-position conversion.
///
/// Construction is O(n) where n is the text length.
/// Each position lookup is O(log L + k) where L is the number of lines
/// and k is the byte length from the line start to the target offset.
pub struct LineIndex {
    /// Byte offset of each line's first character.
    /// Always non-empty: at minimum contains `[0]` for single-line text.
    line_starts: Vec<usize>,
}

impl LineIndex {
    /// Builds a line index from the given text in O(n).
    pub fn new(text: &str) -> Self {
        let line_starts: Vec<usize> = text.line_spans().map(|span| span.range().start).collect();

        // Empty text still has one logical line starting at offset 0
        if line_starts.is_empty() {
            return Self {
                line_starts: vec![0],
            };
        }

        Self { line_starts }
    }

    /// Converts a byte offset to an LSP-compatible (line, character) pair.
    ///
    /// - `line`: 0-based line number
    /// - `character`: 0-based UTF-16 code unit offset from line start
    ///
    /// If `offset` exceeds the text length, it is clamped to `text.len()`.
    pub fn byte_offset_to_position(&self, text: &str, offset: usize) -> (u32, u32) {
        let offset = offset.min(text.len());

        // Binary search: find the last line whose start <= offset
        // partition_point returns the first index where line_starts[i] > offset
        let line = self.line_starts.partition_point(|&start| start <= offset);
        // line is now 1 past the target, so subtract 1 (partition_point returns > 0
        // because line_starts[0] == 0 <= any offset >= 0)
        let line = line.saturating_sub(1);

        let line_start = self.line_starts[line];
        let character = utf16_len(&text[line_start..offset]);

        (line as u32, character)
    }

    /// Converts a parser `Span` (byte offsets) to an LSP Range pair.
    ///
    /// Returns `((start_line, start_char), (end_line, end_char))`.
    pub fn span_to_range(&self, text: &str, span: &Span) -> ((u32, u32), (u32, u32)) {
        let start = self.byte_offset_to_position(text, span.start);
        let end = self.byte_offset_to_position(text, span.end);
        (start, end)
    }

    /// Converts an LSP Position (line, character in UTF-16 code units) to a byte offset.
    ///
    /// Returns `text.len()` if the position is beyond the end of the text.
    pub fn position_to_byte_offset(&self, text: &str, line: u32, character: u32) -> usize {
        let line = line as usize;
        if line >= self.line_starts.len() {
            return text.len();
        }

        let line_start = self.line_starts[line];
        // Line ends at either the next line's start or text end
        let line_end = self
            .line_starts
            .get(line + 1)
            .copied()
            .unwrap_or(text.len());
        let line_text = &text[line_start..line_end];

        // Walk the line counting UTF-16 code units until we reach `character`
        let mut utf16_count = 0u32;
        let mut byte_offset = 0;
        for ch in line_text.chars() {
            if utf16_count >= character {
                break;
            }
            utf16_count += ch.len_utf16() as u32;
            byte_offset += ch.len_utf8();
        }

        line_start + byte_offset
    }
}

/// Counts the number of UTF-16 code units in a UTF-8 string slice.
///
/// Each BMP character (U+0000..U+FFFF) contributes 1 unit,
/// each supplementary character (U+10000..) contributes 2 units (surrogate pair).
#[inline]
fn utf16_len(s: &str) -> u32 {
    s.chars().map(|c| c.len_utf16() as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_ascii() {
        let text = "hello world";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0));
        assert_eq!(idx.byte_offset_to_position(text, 5), (0, 5));
        assert_eq!(idx.byte_offset_to_position(text, 11), (0, 11));
    }

    #[test]
    fn test_multi_line_ascii() {
        let text = "aaa\nbbb\nccc";
        let idx = LineIndex::new(text);
        // Line 0: "aaa" (bytes 0..3)
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0));
        assert_eq!(idx.byte_offset_to_position(text, 2), (0, 2));
        // Line 1: "bbb" (bytes 4..7)
        assert_eq!(idx.byte_offset_to_position(text, 4), (1, 0));
        assert_eq!(idx.byte_offset_to_position(text, 6), (1, 2));
        // Line 2: "ccc" (bytes 8..11)
        assert_eq!(idx.byte_offset_to_position(text, 8), (2, 0));
        assert_eq!(idx.byte_offset_to_position(text, 11), (2, 3));
    }

    #[test]
    fn test_newline_boundary() {
        // The '\n' at byte 3 belongs to line 0
        let text = "ab\ncd\nef";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 2), (0, 2)); // '\n'
        assert_eq!(idx.byte_offset_to_position(text, 3), (1, 0)); // 'c'
        assert_eq!(idx.byte_offset_to_position(text, 5), (1, 2)); // '\n'
        assert_eq!(idx.byte_offset_to_position(text, 6), (2, 0)); // 'e'
    }

    #[test]
    fn test_korean_utf8() {
        // "한글" = 6 bytes UTF-8, 2 UTF-16 code units (BMP chars)
        let text = "한글";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0)); // '한' start
        assert_eq!(idx.byte_offset_to_position(text, 3), (0, 1)); // '글' start
        assert_eq!(idx.byte_offset_to_position(text, 6), (0, 2)); // end
    }

    #[test]
    fn test_emoji_surrogate_pair() {
        // "a🚀b" = 1 + 4 + 1 = 6 bytes
        // UTF-16: 'a'=1, '🚀'=2 (surrogate pair), 'b'=1 → total 4
        let text = "a🚀b";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0)); // 'a'
        assert_eq!(idx.byte_offset_to_position(text, 1), (0, 1)); // '🚀' start
        assert_eq!(idx.byte_offset_to_position(text, 5), (0, 3)); // 'b'
        assert_eq!(idx.byte_offset_to_position(text, 6), (0, 4)); // end
    }

    #[test]
    fn test_mixed_multiline_korean() {
        // "한\n글" = 3 + 1 + 3 = 7 bytes
        let text = "한\n글";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0)); // '한'
        assert_eq!(idx.byte_offset_to_position(text, 3), (0, 1)); // '\n' (still line 0)
        assert_eq!(idx.byte_offset_to_position(text, 4), (1, 0)); // '글'
        assert_eq!(idx.byte_offset_to_position(text, 7), (1, 1)); // end
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0));
    }

    #[test]
    fn test_crlf() {
        let text = "aa\r\nbb";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0)); // 'a'
        assert_eq!(idx.byte_offset_to_position(text, 2), (0, 2)); // '\r'
        assert_eq!(idx.byte_offset_to_position(text, 4), (1, 0)); // 'b'
    }

    #[test]
    fn test_offset_clamped() {
        let text = "abc";
        let idx = LineIndex::new(text);
        // Offset beyond text length gets clamped
        assert_eq!(idx.byte_offset_to_position(text, 100), (0, 3));
    }

    #[test]
    fn test_span_to_range() {
        let text = "aaa\nbbb\nccc";
        let idx = LineIndex::new(text);
        let span = Span::new(4, 7); // "bbb" on line 1
        let ((sl, sc), (el, ec)) = idx.span_to_range(text, &span);
        assert_eq!((sl, sc), (1, 0));
        assert_eq!((el, ec), (1, 3));
    }

    #[test]
    fn test_span_across_lines() {
        let text = "aaa\nbbb\nccc";
        let idx = LineIndex::new(text);
        let span = Span::new(2, 9); // from "a" on line 0 to "c" on line 2
        let ((sl, sc), (el, ec)) = idx.span_to_range(text, &span);
        assert_eq!((sl, sc), (0, 2));
        assert_eq!((el, ec), (2, 1));
    }

    #[test]
    fn test_trailing_newline() {
        let text = "abc\n";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 3), (0, 3)); // '\n'
        assert_eq!(idx.byte_offset_to_position(text, 4), (0, 4)); // end (past newline)
    }

    #[test]
    fn test_multiple_empty_lines() {
        let text = "a\n\n\nb";
        let idx = LineIndex::new(text);
        assert_eq!(idx.byte_offset_to_position(text, 0), (0, 0)); // 'a'
        assert_eq!(idx.byte_offset_to_position(text, 2), (1, 0)); // empty line 1
        assert_eq!(idx.byte_offset_to_position(text, 3), (2, 0)); // empty line 2
        assert_eq!(idx.byte_offset_to_position(text, 4), (3, 0)); // 'b'
    }

    // === position_to_byte_offset tests ===

    #[test]
    fn test_pos_to_byte_ascii() {
        let text = "aaa\nbbb\nccc";
        let idx = LineIndex::new(text);
        assert_eq!(idx.position_to_byte_offset(text, 0, 0), 0);
        assert_eq!(idx.position_to_byte_offset(text, 0, 2), 2);
        assert_eq!(idx.position_to_byte_offset(text, 1, 0), 4);
        assert_eq!(idx.position_to_byte_offset(text, 1, 2), 6);
        assert_eq!(idx.position_to_byte_offset(text, 2, 0), 8);
        assert_eq!(idx.position_to_byte_offset(text, 2, 3), 11);
    }

    #[test]
    fn test_pos_to_byte_korean() {
        // "한글" = 6 bytes, 2 UTF-16 code units
        let text = "한글";
        let idx = LineIndex::new(text);
        assert_eq!(idx.position_to_byte_offset(text, 0, 0), 0);
        assert_eq!(idx.position_to_byte_offset(text, 0, 1), 3); // after '한'
        assert_eq!(idx.position_to_byte_offset(text, 0, 2), 6); // end
    }

    #[test]
    fn test_pos_to_byte_emoji() {
        // "a🚀b": UTF-16 positions: a=0, 🚀=1..2 (surrogate), b=3
        let text = "a🚀b";
        let idx = LineIndex::new(text);
        assert_eq!(idx.position_to_byte_offset(text, 0, 0), 0); // 'a'
        assert_eq!(idx.position_to_byte_offset(text, 0, 1), 1); // '🚀' start
        assert_eq!(idx.position_to_byte_offset(text, 0, 3), 5); // 'b'
        assert_eq!(idx.position_to_byte_offset(text, 0, 4), 6); // end
    }

    #[test]
    fn test_pos_to_byte_roundtrip() {
        let text = "Hello, 세계!\n🚀 rocket\nend";
        let idx = LineIndex::new(text);

        // Test roundtrip for several byte offsets
        for byte_offset in [0, 1, 7, 10, 14, 15, 19, 23, 26] {
            if byte_offset <= text.len() {
                let (line, character) = idx.byte_offset_to_position(text, byte_offset);
                let recovered = idx.position_to_byte_offset(text, line, character);
                assert_eq!(
                    recovered, byte_offset,
                    "roundtrip failed for byte_offset={byte_offset}"
                );
            }
        }
    }

    #[test]
    fn test_pos_to_byte_beyond_end() {
        let text = "abc";
        let idx = LineIndex::new(text);
        // Line beyond end → text.len()
        assert_eq!(idx.position_to_byte_offset(text, 5, 0), text.len());
        // Character beyond line end → clamps to line end
        assert_eq!(idx.position_to_byte_offset(text, 0, 100), 3);
    }
}
