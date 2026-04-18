use crate::parser::ParserInput;
use memchr::memchr2;
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::literal;

pub struct RawBlockParseResult {
    pub value: String,
    pub close_start: usize,
    pub close_end: usize,
}

/// Parse raw block body until matching triple-brace depth is closed.
///
/// - Initial depth is 1 (the opening `{{{` already consumed by caller).
/// - Every `{{{` increments depth.
/// - Every `}}}` decrements depth.
/// - Triple braces escaped by an odd number of preceding backslashes are content.
/// - When depth returns to 0, that `}}}` is treated as the block closer.
pub fn parse_raw_until_balanced_triple_brace(
    parser_input: &mut ParserInput,
) -> Result<RawBlockParseResult> {
    let remaining: &str = parser_input.peek_slice(parser_input.eof_offset());
    let bytes = remaining.as_bytes();

    let mut i = 0usize;
    let mut depth = 1usize;
    let mut close_byte_idx: Option<usize> = None;

    while i < bytes.len() {
        let Some(rel_idx) = memchr2(b'{', b'}', &bytes[i..]) else {
            break;
        };
        i += rel_idx;

        if i + 2 >= bytes.len() {
            break;
        }

        let b = bytes[i];
        let is_escaped = has_odd_preceding_backslashes(bytes, i);

        match b {
            b'{' if !is_escaped && bytes[i + 1] == b'{' && bytes[i + 2] == b'{' => {
                depth += 1;
                i += 3;
            }
            b'}' if !is_escaped && bytes[i + 1] == b'}' && bytes[i + 2] == b'}' => {
                if depth == 1 {
                    close_byte_idx = Some(i);
                    break;
                }
                depth -= 1;
                i += 3;
            }
            _ => {
                i += 1;
            }
        }
    }

    let Some(close_idx) = close_byte_idx else {
        return Err(winnow::error::ContextError::new());
    };

    // `close_idx` comes from scanning ASCII delimiters (`{{{` / `}}}`) in UTF-8 bytes,
    // so it is a valid byte boundary for slicing `&str`.
    let value: &str = parser_input.next_slice(close_idx);

    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let close_end = parser_input.previous_token_end();

    Ok(RawBlockParseResult {
        value: value.to_string(),
        close_start,
        close_end,
    })
}

fn has_odd_preceding_backslashes(bytes: &[u8], offset: usize) -> bool {
    let mut count = 0usize;
    let mut i = offset;
    while i > 0 && bytes[i - 1] == b'\\' {
        count += 1;
        i -= 1;
    }
    count % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ParseContext;
    use crate::parser::InputSource;

    #[test]
    fn parse_balanced_triple_brace_with_utf8_content() {
        let value = "한글🙂{{{중첩}}}끝";
        let input = [value, "}}}"].concat();
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(&input),
            state: context,
        };

        let result = parse_raw_until_balanced_triple_brace(&mut parser_input)
            .expect("raw block parse should succeed");

        assert_eq!(result.value, value);
        assert_eq!(result.close_start, value.len());
        assert_eq!(result.close_end, input.len());
        assert!(parser_input.input.is_empty());
    }

    #[test]
    fn parse_escaped_closing_triple_brace_as_content() {
        let value = "literal \\}}} still content";
        let input = [value, "}}}"].concat();
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(&input),
            state: context,
        };

        let result = parse_raw_until_balanced_triple_brace(&mut parser_input)
            .expect("raw block parse should skip escaped close delimiter");

        assert_eq!(result.value, value);
        assert_eq!(result.close_start, value.len());
        assert_eq!(result.close_end, input.len());
        assert!(parser_input.input.is_empty());
    }

    #[test]
    fn parse_escaped_opening_triple_brace_as_content() {
        let value = "literal \\{{{ still content";
        let input = [value, "}}}"].concat();
        let context = ParseContext::new();

        let mut parser_input = ParserInput {
            input: InputSource::new(&input),
            state: context,
        };

        let result = parse_raw_until_balanced_triple_brace(&mut parser_input)
            .expect("raw block parse should skip escaped open delimiter");

        assert_eq!(result.value, value);
        assert_eq!(result.close_start, value.len());
        assert_eq!(result.close_end, input.len());
        assert!(parser_input.input.is_empty());
    }
}
