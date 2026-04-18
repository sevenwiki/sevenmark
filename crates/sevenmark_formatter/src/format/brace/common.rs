use sevenmark_ast::Element;

pub fn needs_close_separator_for_raw_value(value: &str) -> bool {
    value.ends_with('}')
}

pub fn needs_close_separator_for_elements(elements: &[Element]) -> bool {
    elements.last().is_some_and(formatted_element_ends_with_right_brace)
}

fn formatted_element_ends_with_right_brace(el: &Element) -> bool {
    match el {
        Element::Text(e) => e.value.ends_with('}'),
        Element::Error(e) => e.value.ends_with('}'),
        Element::Comment(e) => e.value.ends_with('}'),
        Element::BlockQuote(_)
        | Element::Styled(_)
        | Element::Table(_)
        | Element::List(_)
        | Element::Fold(_)
        | Element::Ruby(_)
        | Element::Footnote(_)
        | Element::Code(_)
        | Element::TeX(_)
        | Element::Css(_)
        | Element::Include(_)
        | Element::Category(_)
        | Element::Redirect(_)
        | Element::If(_)
        | Element::Literal(_)
        | Element::Define(_) => true,
        _ => false,
    }
}
