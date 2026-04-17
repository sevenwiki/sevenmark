pub(super) type CompletionContext<'a> = Option<(&'a str, usize)>;

/// Walks the prefix tracking `{{{#keyword` opens and `}}}` closes.
/// Returns `(innermost_keyword, bracket_depth_from_that_context)`.
///
/// Bracket depth is counted as unclosed `[[` since the innermost `{{{#` open.
pub(super) fn context_and_bracket_depth(prefix: &str) -> CompletionContext<'_> {
    // stack of (brace_pos, kw_end) - positions into `prefix`
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < prefix.len() {
        if prefix[i..].starts_with("{{{#") {
            let kw_start = i + 4;
            let kw_end = prefix[kw_start..]
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .map(|e| kw_start + e)
                .unwrap_or(prefix.len());
            if kw_end > kw_start {
                stack.push((i, kw_end));
            }
            i += 4;
        } else if prefix[i..].starts_with("}}}") {
            stack.pop();
            i += 3;
        } else {
            // Advance by one Unicode scalar, not one byte.
            i += prefix[i..].chars().next().map_or(1, |c| c.len_utf8());
        }
    }

    let &(brace_pos, kw_end) = stack.last()?;
    let keyword = &prefix[brace_pos + 4..kw_end];

    let after = &prefix[brace_pos..];
    let opens = after.matches("[[").count();
    let closes = after.matches("]]").count();
    let depth = opens.saturating_sub(closes);

    Some((keyword, depth))
}

/// Detects bracket element context (`[[#keyword`) and returns the keyword.
pub(super) fn detect_bracket_element(prefix: &str) -> Option<&str> {
    let bracket_pos = prefix.rfind("[[")?;
    let after = &prefix[bracket_pos + 2..];
    if after.contains("]]") {
        return None;
    }
    let after = after.strip_prefix('#')?;
    let end = after
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(after.len());
    if end == 0 {
        return None;
    }
    Some(&after[..end])
}

/// Detects brace element context (`{{{#keyword`) and returns the keyword.
pub(super) fn detect_brace_element(prefix: &str) -> Option<&str> {
    let brace_pos = prefix.rfind("{{{#")?;
    let after = &prefix[brace_pos + 4..];
    if after.contains("}}}") {
        return None;
    }
    let end = after
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(after.len());
    if end == 0 {
        return None;
    }
    Some(&after[..end])
}

/// True when the cursor is inside an unclosed generic `[[...]]` media element.
pub(super) fn in_unclosed_bracket(prefix: &str) -> bool {
    let Some(bracket_pos) = prefix.rfind("[[") else {
        return false;
    };
    !prefix[bracket_pos + 2..].contains("]]")
}

/// True when the cursor is inside an unclosed triple-brace block that can parse
/// as a styled element (`{{{ #style=... }}` or compact `{{{#style=... }}`).
pub(super) fn in_unclosed_styled_brace(prefix: &str) -> bool {
    let Some(brace_pos) = prefix.rfind("{{{") else {
        return false;
    };
    let after = &prefix[brace_pos + 3..];
    !after.contains("}}}")
}
