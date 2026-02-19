use sevenmark_ast::{
    ConditionalListItems, ConditionalTableCells, ConditionalTableRows, Element, Expression,
    FoldInnerElement, ListContentItem, ListItemElement, Parameter, TableCellElement, TableCellItem,
    TableRowElement, TableRowItem,
};
use tower_lsp_server::ls_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensLegend,
};

use crate::document::DocumentState;

// One custom token type per AST node that carries a span.
// VS Code extension maps these to colors via `semanticTokenColorCustomizations`.
pub const TOKEN_TYPES: &[SemanticTokenType] = &[
    // ── Element variants (0–36) ──
    SemanticTokenType::new("text"),          // 0
    SemanticTokenType::new("comment"),       // 1
    SemanticTokenType::new("escape"),        // 2
    SemanticTokenType::new("error"),         // 3
    SemanticTokenType::new("literal"),       // 4
    SemanticTokenType::new("define"),        // 5
    SemanticTokenType::new("styled"),        // 6
    SemanticTokenType::new("table"),         // 7
    SemanticTokenType::new("list"),          // 8
    SemanticTokenType::new("fold"),          // 9
    SemanticTokenType::new("blockQuote"),    // 10
    SemanticTokenType::new("ruby"),          // 11
    SemanticTokenType::new("footnote"),      // 12
    SemanticTokenType::new("code"),          // 13
    SemanticTokenType::new("tex"),           // 14
    SemanticTokenType::new("include"),       // 15
    SemanticTokenType::new("category"),      // 16
    SemanticTokenType::new("redirect"),      // 17
    SemanticTokenType::new("media"),         // 18
    SemanticTokenType::new("externalMedia"), // 19
    SemanticTokenType::new("null"),          // 20
    SemanticTokenType::new("footnoteRef"),   // 21
    SemanticTokenType::new("timeNow"),       // 22
    SemanticTokenType::new("age"),           // 23
    SemanticTokenType::new("variable"),      // 24
    SemanticTokenType::new("mention"),       // 25
    SemanticTokenType::new("bold"),          // 26
    SemanticTokenType::new("italic"),        // 27
    SemanticTokenType::new("strikethrough"), // 28
    SemanticTokenType::new("underline"),     // 29
    SemanticTokenType::new("superscript"),   // 30
    SemanticTokenType::new("subscript"),     // 31
    SemanticTokenType::new("softBreak"),     // 32
    SemanticTokenType::new("hardBreak"),     // 33
    SemanticTokenType::new("hLine"),         // 34
    SemanticTokenType::new("header"),        // 35
    SemanticTokenType::new("if"),            // 36
    // ── Structural sub-elements (37–43) ──
    SemanticTokenType::new("parameter"),             // 37
    SemanticTokenType::new("tableRow"),              // 38
    SemanticTokenType::new("tableCell"),             // 39
    SemanticTokenType::new("conditionalTableRows"),  // 40
    SemanticTokenType::new("conditionalTableCells"), // 41
    SemanticTokenType::new("listItem"),              // 42
    SemanticTokenType::new("conditionalListItems"),  // 43
    SemanticTokenType::new("foldInner"),             // 44
    // ── Expression nodes (45–54) ──
    SemanticTokenType::new("exprOr"),            // 45
    SemanticTokenType::new("exprAnd"),           // 46
    SemanticTokenType::new("exprNot"),           // 47
    SemanticTokenType::new("exprComparison"),    // 48
    SemanticTokenType::new("exprFunctionCall"),  // 49
    SemanticTokenType::new("exprStringLiteral"), // 50
    SemanticTokenType::new("exprNumberLiteral"), // 51
    SemanticTokenType::new("exprBoolLiteral"),   // 52
    SemanticTokenType::new("exprNull"),          // 53
    SemanticTokenType::new("exprGroup"),         // 54
    // ── Operators (55–56) ──
    SemanticTokenType::new("logicalOperator"),    // 55
    SemanticTokenType::new("comparisonOperator"), // 56
];

pub const TOKEN_MODIFIERS: &[SemanticTokenModifier] = &[];

pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: TOKEN_TYPES.to_vec(),
        token_modifiers: TOKEN_MODIFIERS.to_vec(),
    }
}

/// Collects semantic tokens from the entire AST — every node with a span emits a token.
pub fn collect_semantic_tokens(state: &DocumentState) -> Vec<SemanticToken> {
    let mut raw: Vec<(usize, usize, u32)> = Vec::new();
    walk_elements(&state.elements, &mut raw);
    raw.sort_by_key(|&(start, _, _)| start);

    // Delta-encode
    let mut tokens = Vec::with_capacity(raw.len());
    let mut prev_line: u32 = 0;
    let mut prev_char: u32 = 0;

    for (start, end, token_type) in raw {
        let (line, character) = state.line_index.byte_offset_to_position(&state.text, start);
        let (end_line, end_char) = state.line_index.byte_offset_to_position(&state.text, end);

        let length = if line == end_line {
            end_char - character
        } else {
            let line_end_byte = state
                .line_index
                .position_to_byte_offset(&state.text, line + 1, 0);
            let (_, line_end_char) = state
                .line_index
                .byte_offset_to_position(&state.text, line_end_byte.min(end));
            line_end_char.saturating_sub(character).max(1)
        };

        let delta_line = line - prev_line;
        let delta_start = if delta_line == 0 {
            character - prev_char
        } else {
            character
        };

        tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset: 0,
        });

        prev_line = line;
        prev_char = character;
    }

    tokens
}

// ── Full AST walker ─────────────────────────────────────────────────────

fn walk_elements(elements: &[Element], raw: &mut Vec<(usize, usize, u32)>) {
    for element in elements {
        walk_element(element, raw);
    }
}

fn walk_element(element: &Element, raw: &mut Vec<(usize, usize, u32)>) {
    // 1. Token for this element
    let span = element.span();
    raw.push((span.start, span.end, element_token_type(element)));

    // 2. Parameters
    walk_element_parameters(element, raw);

    // 3. Expression (If condition)
    if let Element::If(e) = element {
        walk_expression(&e.condition, raw);
    }

    // 4. Children — custom walk instead of traverse_children_ref,
    //    because we also need to enter Table/List/Fold sub-structures.
    match element {
        // Container elements with just children
        Element::Literal(e) => walk_elements(&e.children, raw),
        Element::Styled(e) => walk_elements(&e.children, raw),
        Element::BlockQuote(e) => walk_elements(&e.children, raw),
        Element::Ruby(e) => walk_elements(&e.children, raw),
        Element::Footnote(e) => walk_elements(&e.children, raw),
        Element::Include(e) => walk_elements(&e.children, raw),
        Element::Category(e) => walk_elements(&e.children, raw),
        Element::Redirect(e) => walk_elements(&e.children, raw),
        Element::Media(e) => walk_elements(&e.children, raw),
        Element::Bold(e)
        | Element::Italic(e)
        | Element::Strikethrough(e)
        | Element::Underline(e)
        | Element::Superscript(e)
        | Element::Subscript(e) => walk_elements(&e.children, raw),
        Element::Header(e) => walk_elements(&e.children, raw),
        Element::If(e) => walk_elements(&e.children, raw),

        // Table
        Element::Table(e) => {
            for row_item in &e.children {
                match row_item {
                    TableRowItem::Row(row) => walk_table_row(row, raw),
                    TableRowItem::Conditional(cond) => walk_conditional_table_rows(cond, raw),
                }
            }
        }

        // List
        Element::List(e) => {
            for item in &e.children {
                match item {
                    ListContentItem::Item(li) => walk_list_item(li, raw),
                    ListContentItem::Conditional(cond) => walk_conditional_list_items(cond, raw),
                }
            }
        }

        // Fold
        Element::Fold(e) => {
            walk_fold_inner(&e.summary, raw);
            walk_fold_inner(&e.details, raw);
        }

        // Leaf elements — no children to walk
        Element::Text(_)
        | Element::Comment(_)
        | Element::Escape(_)
        | Element::Error(_)
        | Element::Code(_)
        | Element::TeX(_)
        | Element::Define(_)
        | Element::ExternalMedia(_)
        | Element::Null(_)
        | Element::FootnoteRef(_)
        | Element::TimeNow(_)
        | Element::Age(_)
        | Element::Variable(_)
        | Element::Mention(_)
        | Element::SoftBreak(_)
        | Element::HardBreak(_)
        | Element::HLine(_) => {}
    }
}

// ── Parameter walking ───────────────────────────────────────────────────

fn walk_parameters(params: &sevenmark_ast::Parameters, raw: &mut Vec<(usize, usize, u32)>) {
    for param in params.values() {
        walk_parameter(param, raw);
    }
}

fn walk_parameter(param: &Parameter, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((param.span.start, param.span.end, 37)); // parameter
    walk_elements(&param.value, raw);
}

fn walk_element_parameters(element: &Element, raw: &mut Vec<(usize, usize, u32)>) {
    match element {
        Element::Define(e) => walk_parameters(&e.parameters, raw),
        Element::Styled(e) => walk_parameters(&e.parameters, raw),
        Element::BlockQuote(e) => walk_parameters(&e.parameters, raw),
        Element::Ruby(e) => walk_parameters(&e.parameters, raw),
        Element::Footnote(e) => walk_parameters(&e.parameters, raw),
        Element::Code(e) => walk_parameters(&e.parameters, raw),
        Element::Include(e) => walk_parameters(&e.parameters, raw),
        Element::Redirect(e) => walk_parameters(&e.parameters, raw),
        Element::Media(e) => walk_parameters(&e.parameters, raw),
        Element::ExternalMedia(e) => walk_parameters(&e.parameters, raw),
        Element::Table(e) => walk_parameters(&e.parameters, raw),
        Element::List(e) => walk_parameters(&e.parameters, raw),
        Element::Fold(e) => walk_parameters(&e.parameters, raw),
        // Elements without parameters
        Element::Text(_)
        | Element::Comment(_)
        | Element::Escape(_)
        | Element::Error(_)
        | Element::Literal(_)
        | Element::TeX(_)
        | Element::Null(_)
        | Element::FootnoteRef(_)
        | Element::TimeNow(_)
        | Element::Age(_)
        | Element::Variable(_)
        | Element::Mention(_)
        | Element::Bold(_)
        | Element::Italic(_)
        | Element::Strikethrough(_)
        | Element::Underline(_)
        | Element::Superscript(_)
        | Element::Subscript(_)
        | Element::SoftBreak(_)
        | Element::HardBreak(_)
        | Element::HLine(_)
        | Element::Header(_)
        | Element::Category(_)
        | Element::If(_) => {}
    }
}

// ── Table sub-structure walking ─────────────────────────────────────────

fn walk_table_row(row: &TableRowElement, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((row.span.start, row.span.end, 38)); // tableRow
    walk_parameters(&row.parameters, raw);
    for cell_item in &row.children {
        match cell_item {
            TableCellItem::Cell(cell) => walk_table_cell(cell, raw),
            TableCellItem::Conditional(cond) => walk_conditional_table_cells(cond, raw),
        }
    }
}

fn walk_table_cell(cell: &TableCellElement, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((cell.span.start, cell.span.end, 39)); // tableCell
    walk_parameters(&cell.parameters, raw);
    walk_elements(&cell.x, raw);
    walk_elements(&cell.y, raw);
    walk_elements(&cell.children, raw);
}

fn walk_conditional_table_rows(cond: &ConditionalTableRows, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((cond.span.start, cond.span.end, 40)); // conditionalTableRows
    walk_expression(&cond.condition, raw);
    for row in &cond.rows {
        walk_table_row(row, raw);
    }
}

fn walk_conditional_table_cells(cond: &ConditionalTableCells, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((cond.span.start, cond.span.end, 41)); // conditionalTableCells
    walk_expression(&cond.condition, raw);
    for cell in &cond.cells {
        walk_table_cell(cell, raw);
    }
}

// ── List sub-structure walking ──────────────────────────────────────────

fn walk_list_item(li: &ListItemElement, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((li.span.start, li.span.end, 42)); // listItem
    walk_parameters(&li.parameters, raw);
    walk_elements(&li.children, raw);
}

fn walk_conditional_list_items(cond: &ConditionalListItems, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((cond.span.start, cond.span.end, 43)); // conditionalListItems
    walk_expression(&cond.condition, raw);
    for li in &cond.items {
        walk_list_item(li, raw);
    }
}

// ── Fold sub-structure walking ──────────────────────────────────────────

fn walk_fold_inner(inner: &FoldInnerElement, raw: &mut Vec<(usize, usize, u32)>) {
    raw.push((inner.span.start, inner.span.end, 44)); // foldInner
    walk_parameters(&inner.parameters, raw);
    walk_elements(&inner.children, raw);
}

// ── Expression walking ──────────────────────────────────────────────────

fn walk_expression(expr: &Expression, raw: &mut Vec<(usize, usize, u32)>) {
    match expr {
        Expression::Or {
            span,
            operator,
            left,
            right,
        } => {
            raw.push((span.start, span.end, 45));
            raw.push((operator.span.start, operator.span.end, 55)); // logicalOperator
            walk_expression(left, raw);
            walk_expression(right, raw);
        }
        Expression::And {
            span,
            operator,
            left,
            right,
        } => {
            raw.push((span.start, span.end, 46));
            raw.push((operator.span.start, operator.span.end, 55));
            walk_expression(left, raw);
            walk_expression(right, raw);
        }
        Expression::Not {
            span,
            operator,
            inner,
        } => {
            raw.push((span.start, span.end, 47));
            raw.push((operator.span.start, operator.span.end, 55));
            walk_expression(inner, raw);
        }
        Expression::Comparison {
            span,
            left,
            operator,
            right,
        } => {
            raw.push((span.start, span.end, 48));
            raw.push((operator.span.start, operator.span.end, 56)); // comparisonOperator
            walk_expression(left, raw);
            walk_expression(right, raw);
        }
        Expression::FunctionCall {
            span, arguments, ..
        } => {
            raw.push((span.start, span.end, 49));
            for arg in arguments {
                walk_expression(arg, raw);
            }
        }
        Expression::StringLiteral { span, value } => {
            raw.push((span.start, span.end, 50));
            walk_elements(value, raw);
        }
        Expression::NumberLiteral { span, .. } => {
            raw.push((span.start, span.end, 51));
        }
        Expression::BoolLiteral { span, .. } => {
            raw.push((span.start, span.end, 52));
        }
        Expression::Null { span } => {
            raw.push((span.start, span.end, 53));
        }
        Expression::Group { span, inner } => {
            raw.push((span.start, span.end, 54));
            walk_expression(inner, raw);
        }
        Expression::Element(e) => {
            walk_element(e, raw);
        }
    }
}

// ── Element → token type index ──────────────────────────────────────────

fn element_token_type(element: &Element) -> u32 {
    match element {
        Element::Text(_) => 0,
        Element::Comment(_) => 1,
        Element::Escape(_) => 2,
        Element::Error(_) => 3,
        Element::Literal(_) => 4,
        Element::Define(_) => 5,
        Element::Styled(_) => 6,
        Element::Table(_) => 7,
        Element::List(_) => 8,
        Element::Fold(_) => 9,
        Element::BlockQuote(_) => 10,
        Element::Ruby(_) => 11,
        Element::Footnote(_) => 12,
        Element::Code(_) => 13,
        Element::TeX(_) => 14,
        Element::Include(_) => 15,
        Element::Category(_) => 16,
        Element::Redirect(_) => 17,
        Element::Media(_) => 18,
        Element::ExternalMedia(_) => 19,
        Element::Null(_) => 20,
        Element::FootnoteRef(_) => 21,
        Element::TimeNow(_) => 22,
        Element::Age(_) => 23,
        Element::Variable(_) => 24,
        Element::Mention(_) => 25,
        Element::Bold(_) => 26,
        Element::Italic(_) => 27,
        Element::Strikethrough(_) => 28,
        Element::Underline(_) => 29,
        Element::Superscript(_) => 30,
        Element::Subscript(_) => 31,
        Element::SoftBreak(_) => 32,
        Element::HardBreak(_) => 33,
        Element::HLine(_) => 34,
        Element::Header(_) => 35,
        Element::If(_) => 36,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(text: &str) -> DocumentState {
        DocumentState::new(text.to_string())
    }

    #[test]
    fn plain_text_produces_text_token() {
        let state = make_state("hello");
        let tokens = collect_semantic_tokens(&state);
        assert!(!tokens.is_empty());
        // text token type is 0
        assert!(
            tokens.iter().any(|t| t.token_type == 0),
            "expected text token (type 0)"
        );
    }

    #[test]
    fn bold_produces_bold_and_text_tokens() {
        let state = make_state("**bold**");
        let tokens = collect_semantic_tokens(&state);
        // bold = 26, text = 0
        assert!(
            tokens.iter().any(|t| t.token_type == 26),
            "expected bold token (type 26)"
        );
        assert!(
            tokens.iter().any(|t| t.token_type == 0),
            "expected text token (type 0)"
        );
    }

    #[test]
    fn define_produces_define_and_parameter_tokens() {
        let state = make_state("{{{#define #x=\"v\"}}}");
        let tokens = collect_semantic_tokens(&state);
        // define = 5, parameter = 37
        assert!(
            tokens.iter().any(|t| t.token_type == 5),
            "expected define token (type 5)"
        );
        assert!(
            tokens.iter().any(|t| t.token_type == 37),
            "expected parameter token (type 37)"
        );
    }

    #[test]
    fn if_produces_if_and_expression_tokens() {
        let state = make_state("{{{#if true :: content}}}");
        let tokens = collect_semantic_tokens(&state);
        // if = 36
        assert!(
            tokens.iter().any(|t| t.token_type == 36),
            "expected if token (type 36)"
        );
        // Should have at least 3 tokens (if, expression, content text)
        assert!(
            tokens.len() >= 3,
            "expected at least 3 tokens, got {}",
            tokens.len()
        );
    }
}
