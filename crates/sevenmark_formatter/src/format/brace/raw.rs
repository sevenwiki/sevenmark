use sevenmark_ast::Element;

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

pub fn needs_line_break_before_brace_close(children: &[Element]) -> bool {
    let last_semantic = children
        .iter()
        .rev()
        .find(|el| !is_ignorable_trailing_text(el));

    matches!(
        last_semantic,
        Some(Element::Code(_) | Element::TeX(_) | Element::Css(_))
    )
}

fn is_ignorable_trailing_text(el: &Element) -> bool {
    match el {
        Element::Text(t) => t.value.chars().all(|c| matches!(c, ' ' | '\t' | '\r')),
        Element::SoftBreak(_) => true,
        _ => false,
    }
}
