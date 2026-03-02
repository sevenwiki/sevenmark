use crate::parser::ParserInput;
use winnow::Result;
use winnow::stream::{Location as StreamLocation, Stream};

pub struct RawBlockParseResult {
    pub value: String,
    pub close_start: usize,
    pub close_end: usize,
}

fn is_space_or_tab(b: u8) -> bool {
    matches!(b, b' ' | b'\t')
}

// Returns (line_start, closer_start, line_end_without_newline).
// Closer is accepted only when the line is: ^[ \t]*<closer>[ \t]*\r?$
fn find_line_start_closer(s: &str, closer: &str) -> Option<(usize, usize, usize)> {
    let bytes = s.as_bytes();
    let closer_bytes = closer.as_bytes();
    let closer_len = closer_bytes.len();
    let mut line_start = 0usize;

    while line_start <= bytes.len() {
        let line_end = bytes[line_start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|rel| line_start + rel)
            .unwrap_or(bytes.len());

        let mut j = line_start;
        while j < line_end && is_space_or_tab(bytes[j]) {
            j += 1;
        }

        if j + closer_len <= line_end && &bytes[j..j + closer_len] == closer_bytes {
            let mut k = j + closer_len;
            while k < line_end && is_space_or_tab(bytes[k]) {
                k += 1;
            }
            if k == line_end || (k + 1 == line_end && bytes[k] == b'\r') {
                return Some((line_start, j, line_end));
            }
        }

        if line_end == bytes.len() {
            break;
        }
        line_start = line_end + 1;
    }

    None
}

fn unescape_literal_closer_lines(raw: &str, closer: &str) -> String {
    let mut out = String::with_capacity(raw.len());

    for line_with_nl in raw.split_inclusive('\n') {
        let (line, has_newline) = if let Some(stripped) = line_with_nl.strip_suffix('\n') {
            (stripped, true)
        } else {
            (line_with_nl, false)
        };

        let line_for_match = line.strip_suffix('\r').unwrap_or(line);
        let leading_ws = line_for_match
            .as_bytes()
            .iter()
            .take_while(|&&b| is_space_or_tab(b))
            .count();
        let rest = &line_for_match[leading_ws..];
        let rest_trimmed = rest.trim_end_matches([' ', '\t']);

        // A line that is exactly "\}}}" (with optional leading/trailing spaces)
        // should become literal "}}}" in raw content.
        if rest_trimmed.strip_prefix('\\').is_some_and(|x| x == closer) {
            out.push_str(&line[..leading_ws]);
            out.push_str(closer);
            out.push_str(&line[leading_ws + 1 + closer.len()..]);
        } else {
            out.push_str(line);
        }

        if has_newline {
            out.push('\n');
        }
    }

    out
}

pub fn parse_raw_until_line_closer(
    parser_input: &mut ParserInput,
    closer: &str,
) -> Result<RawBlockParseResult> {
    let remaining: &str = parser_input.peek_slice(parser_input.eof_offset());
    let Some((line_start, closer_start, line_end)) = find_line_start_closer(remaining, closer)
    else {
        return Err(winnow::error::ContextError::new());
    };

    let value = unescape_literal_closer_lines(&remaining[..line_start], closer);

    // Consume body bytes up to the closer line.
    if line_start > 0 {
        let _ = parser_input.next_slice(line_start);
    }

    // Skip indentation before closer and capture exact closer span.
    let leading_ws_len = closer_start - line_start;
    if leading_ws_len > 0 {
        let _ = parser_input.next_slice(leading_ws_len);
    }

    let close_start = parser_input.current_token_start();
    let _ = parser_input.next_slice(closer.len());
    let close_end = parser_input.current_token_start();

    // Consume trailing spaces on the closer line, but not the trailing '\n'.
    let trailing_ws_len = line_end.saturating_sub(closer_start + closer.len());
    if trailing_ws_len > 0 {
        let _ = parser_input.next_slice(trailing_ws_len);
    }

    Ok(RawBlockParseResult {
        value,
        close_start,
        close_end,
    })
}
