pub fn escape_line_only_closer(value: &str, closer: &str) -> String {
    let mut out = String::with_capacity(value.len());

    for line_with_nl in value.split_inclusive('\n') {
        let (line, has_newline) = if let Some(stripped) = line_with_nl.strip_suffix('\n') {
            (stripped, true)
        } else {
            (line_with_nl, false)
        };

        let line_for_match = line.strip_suffix('\r').unwrap_or(line);
        let leading_ws = line_for_match
            .as_bytes()
            .iter()
            .take_while(|&&b| matches!(b, b' ' | b'\t'))
            .count();
        let rest = &line_for_match[leading_ws..];
        let rest_trimmed = rest.trim_end_matches([' ', '\t']);

        if rest_trimmed == closer {
            out.push_str(&line[..leading_ws]);
            out.push('\\');
            out.push_str(closer);
            out.push_str(&line[leading_ws + closer.len()..]);
        } else {
            out.push_str(line);
        }

        if has_newline {
            out.push('\n');
        }
    }

    out
}
