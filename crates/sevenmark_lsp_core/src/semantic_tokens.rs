use ls_types::{SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensLegend};
use sevenmark_ast::{
    ConditionalListItems, ConditionalTableCells, ConditionalTableRows, Element, Expression,
    FoldInnerElement, ListContentItem, ListItemElement, Parameter, TableCellElement, TableCellItem,
    TableRowElement, TableRowItem,
};

use crate::document::DocumentState;

#[repr(u32)]
#[derive(Copy, Clone)]
enum TokenIdx {
    Text = 0,
    Comment = 1,
    Escape = 2,
    Error = 3,
    Literal = 4,
    Define = 5,
    Styled = 6,
    Table = 7,
    List = 8,
    Fold = 9,
    BlockQuote = 10,
    Ruby = 11,
    Footnote = 12,
    Code = 13,
    TeX = 14,
    Include = 15,
    Category = 16,
    Redirect = 17,
    Media = 18,
    ExternalMedia = 19,
    Null = 20,
    FootnoteRef = 21,
    TimeNow = 22,
    Age = 23,
    Variable = 24,
    Mention = 25,
    Bold = 26,
    Italic = 27,
    Strikethrough = 28,
    Underline = 29,
    Superscript = 30,
    Subscript = 31,
    SoftBreak = 32,
    HardBreak = 33,
    HLine = 34,
    Header = 35,
    FoldedHeader = 36,
    If = 37,
    Parameter = 38,
    TableRow = 39,
    TableCell = 40,
    ConditionalTableRows = 41,
    ConditionalTableCells = 42,
    ListItem = 43,
    ConditionalListItems = 44,
    FoldInner = 45,
    ExprOr = 46,
    ExprAnd = 47,
    ExprNot = 48,
    ExprComparison = 49,
    ExprFunctionCall = 50,
    ExprStringLiteral = 51,
    ExprNumberLiteral = 52,
    ExprBoolLiteral = 53,
    ExprNull = 54,
    ExprGroup = 55,
    LogicalOperator = 56,
    ComparisonOperator = 57,
}

impl TokenIdx {
    const fn as_u32(self) -> u32 {
        self as u32
    }
}

// Maps SevenMark AST spans to LSP standard semantic token types.
// Keep indices stable because token emitters use numeric indices.
pub const TOKEN_TYPES: &[SemanticTokenType] = &[
    // ── Element variants (0–36) ──
    SemanticTokenType::STRING,   // 0  Text
    SemanticTokenType::COMMENT,  // 1  Comment
    SemanticTokenType::STRING,   // 2  Escape
    SemanticTokenType::MODIFIER, // 3  Error
    SemanticTokenType::STRING,   // 4  Literal
    SemanticTokenType::KEYWORD,  // 5  Define
    SemanticTokenType::KEYWORD,  // 6  Styled
    SemanticTokenType::KEYWORD,  // 7  Table
    SemanticTokenType::KEYWORD,  // 8  List
    SemanticTokenType::KEYWORD,  // 9  Fold
    SemanticTokenType::KEYWORD,  // 10 BlockQuote
    SemanticTokenType::KEYWORD,  // 11 Ruby
    SemanticTokenType::KEYWORD,  // 12 Footnote
    SemanticTokenType::KEYWORD,  // 13 Code
    SemanticTokenType::STRING,   // 14 TeX
    SemanticTokenType::KEYWORD,  // 15 Include
    SemanticTokenType::KEYWORD,  // 16 Category
    SemanticTokenType::KEYWORD,  // 17 Redirect
    SemanticTokenType::STRING,   // 18 Media
    SemanticTokenType::STRING,   // 19 ExternalMedia
    SemanticTokenType::KEYWORD,  // 20 Null
    SemanticTokenType::VARIABLE, // 21 FootnoteRef
    SemanticTokenType::FUNCTION, // 22 TimeNow
    SemanticTokenType::FUNCTION, // 23 Age
    SemanticTokenType::VARIABLE, // 24 Variable
    SemanticTokenType::VARIABLE, // 25 Mention
    SemanticTokenType::MODIFIER, // 26 Bold
    SemanticTokenType::MODIFIER, // 27 Italic
    SemanticTokenType::MODIFIER, // 28 Strikethrough
    SemanticTokenType::MODIFIER, // 29 Underline
    SemanticTokenType::MODIFIER, // 30 Superscript
    SemanticTokenType::MODIFIER, // 31 Subscript
    SemanticTokenType::OPERATOR, // 32 SoftBreak
    SemanticTokenType::OPERATOR, // 33 HardBreak
    SemanticTokenType::OPERATOR, // 34 HLine
    SemanticTokenType::KEYWORD,  // 35 Header
    SemanticTokenType::KEYWORD,  // 36 FoldedHeader
    SemanticTokenType::KEYWORD,  // 37 If
    // ── Structural sub-elements (38–45) ──
    SemanticTokenType::PARAMETER, // 38 Parameter
    SemanticTokenType::OPERATOR,  // 39 TableRow
    SemanticTokenType::OPERATOR,  // 40 TableCell
    SemanticTokenType::KEYWORD,   // 41 ConditionalTableRows
    SemanticTokenType::KEYWORD,   // 42 ConditionalTableCells
    SemanticTokenType::OPERATOR,  // 43 ListItem
    SemanticTokenType::KEYWORD,   // 44 ConditionalListItems
    SemanticTokenType::OPERATOR,  // 45 FoldInner
    // ── Expression nodes (46–55) ──
    SemanticTokenType::OPERATOR, // 46 ExprOr
    SemanticTokenType::OPERATOR, // 47 ExprAnd
    SemanticTokenType::OPERATOR, // 48 ExprNot
    SemanticTokenType::OPERATOR, // 49 ExprComparison
    SemanticTokenType::FUNCTION, // 50 ExprFunctionCall
    SemanticTokenType::STRING,   // 51 ExprStringLiteral
    SemanticTokenType::NUMBER,   // 52 ExprNumberLiteral
    SemanticTokenType::KEYWORD,  // 53 ExprBoolLiteral
    SemanticTokenType::KEYWORD,  // 54 ExprNull
    SemanticTokenType::OPERATOR, // 55 ExprGroup
    // ── Operators (56–57) ──
    SemanticTokenType::OPERATOR, // 56 LogicalOperator
    SemanticTokenType::OPERATOR, // 57 ComparisonOperator
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
            // Multi-line: cover from start char to end of content on the first line.
            // position_to_byte_offset(line+1, 0) gives the start of the next line;
            // strip trailing \n and \r to get the byte just past the last content char.
            let next_line_byte = state
                .line_index
                .position_to_byte_offset(&state.text, line + 1, 0);
            let mut content_end = next_line_byte;
            if content_end > 0 && state.text.as_bytes().get(content_end - 1) == Some(&b'\n') {
                content_end -= 1;
            }
            if content_end > 0 && state.text.as_bytes().get(content_end - 1) == Some(&b'\r') {
                content_end -= 1;
            }
            let (_, line_end_char) = state
                .line_index
                .byte_offset_to_position(&state.text, content_end.min(end));
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
    // 1. Token for this element.
    //    Skip Text — it's the default color, and emitting it would override
    //    parent tokens (bold, header, etc.) due to overlapping.
    if !matches!(element, Element::Text(_)) {
        let token_type = element_token_type(element);

        // For delimited elements, emit separate tokens for opening/closing delimiters
        // so inner content can have its own colors.
        match element {
            Element::Literal(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Define(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Styled(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Table(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::List(e) => emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw),
            Element::Fold(e) => emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw),
            Element::BlockQuote(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Ruby(e) => emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw),
            Element::Footnote(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Code(e) => emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw),
            Element::TeX(e) => emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw),
            Element::Include(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Category(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Redirect(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::Media(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::ExternalMedia(e) => {
                emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw)
            }
            Element::If(e) => emit_delimiter_tokens(&e.open_span, &e.close_span, token_type, raw),
            // Non-delimited elements: emit full span
            Element::Text(_)
            | Element::Comment(_)
            | Element::Escape(_)
            | Element::Error(_)
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
            | Element::Header(_) => {
                let span = element.span();
                raw.push((span.start, span.end, token_type));
            }
        }
    }

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
    raw.push((
        param.span.start,
        param.span.end,
        TokenIdx::Parameter.as_u32(),
    ));
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
    emit_delimiter_tokens(
        &row.open_span,
        &row.close_span,
        TokenIdx::TableRow.as_u32(),
        raw,
    );
    walk_parameters(&row.parameters, raw);
    for cell_item in &row.children {
        match cell_item {
            TableCellItem::Cell(cell) => walk_table_cell(cell, raw),
            TableCellItem::Conditional(cond) => walk_conditional_table_cells(cond, raw),
        }
    }
}

fn walk_table_cell(cell: &TableCellElement, raw: &mut Vec<(usize, usize, u32)>) {
    emit_delimiter_tokens(
        &cell.open_span,
        &cell.close_span,
        TokenIdx::TableCell.as_u32(),
        raw,
    );
    walk_parameters(&cell.parameters, raw);
    walk_elements(&cell.x, raw);
    walk_elements(&cell.y, raw);
    walk_elements(&cell.children, raw);
}

fn walk_conditional_table_rows(cond: &ConditionalTableRows, raw: &mut Vec<(usize, usize, u32)>) {
    emit_delimiter_tokens(
        &cond.open_span,
        &cond.close_span,
        TokenIdx::ConditionalTableRows.as_u32(),
        raw,
    );
    walk_expression(&cond.condition, raw);
    for row in &cond.rows {
        walk_table_row(row, raw);
    }
}

fn walk_conditional_table_cells(cond: &ConditionalTableCells, raw: &mut Vec<(usize, usize, u32)>) {
    emit_delimiter_tokens(
        &cond.open_span,
        &cond.close_span,
        TokenIdx::ConditionalTableCells.as_u32(),
        raw,
    );
    walk_expression(&cond.condition, raw);
    for cell in &cond.cells {
        walk_table_cell(cell, raw);
    }
}

// ── List sub-structure walking ──────────────────────────────────────────

fn walk_list_item(li: &ListItemElement, raw: &mut Vec<(usize, usize, u32)>) {
    emit_delimiter_tokens(
        &li.open_span,
        &li.close_span,
        TokenIdx::ListItem.as_u32(),
        raw,
    );
    walk_parameters(&li.parameters, raw);
    walk_elements(&li.children, raw);
}

fn walk_conditional_list_items(cond: &ConditionalListItems, raw: &mut Vec<(usize, usize, u32)>) {
    emit_delimiter_tokens(
        &cond.open_span,
        &cond.close_span,
        TokenIdx::ConditionalListItems.as_u32(),
        raw,
    );
    walk_expression(&cond.condition, raw);
    for li in &cond.items {
        walk_list_item(li, raw);
    }
}

// ── Fold sub-structure walking ──────────────────────────────────────────

fn walk_fold_inner(inner: &FoldInnerElement, raw: &mut Vec<(usize, usize, u32)>) {
    emit_delimiter_tokens(
        &inner.open_span,
        &inner.close_span,
        TokenIdx::FoldInner.as_u32(),
        raw,
    );
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
            raw.push((span.start, span.end, TokenIdx::ExprOr.as_u32()));
            raw.push((
                operator.span.start,
                operator.span.end,
                TokenIdx::LogicalOperator.as_u32(),
            ));
            walk_expression(left, raw);
            walk_expression(right, raw);
        }
        Expression::And {
            span,
            operator,
            left,
            right,
        } => {
            raw.push((span.start, span.end, TokenIdx::ExprAnd.as_u32()));
            raw.push((
                operator.span.start,
                operator.span.end,
                TokenIdx::LogicalOperator.as_u32(),
            ));
            walk_expression(left, raw);
            walk_expression(right, raw);
        }
        Expression::Not {
            span,
            operator,
            inner,
        } => {
            raw.push((span.start, span.end, TokenIdx::ExprNot.as_u32()));
            raw.push((
                operator.span.start,
                operator.span.end,
                TokenIdx::LogicalOperator.as_u32(),
            ));
            walk_expression(inner, raw);
        }
        Expression::Comparison {
            span,
            left,
            operator,
            right,
        } => {
            raw.push((span.start, span.end, TokenIdx::ExprComparison.as_u32()));
            raw.push((
                operator.span.start,
                operator.span.end,
                TokenIdx::ComparisonOperator.as_u32(),
            ));
            walk_expression(left, raw);
            walk_expression(right, raw);
        }
        Expression::FunctionCall {
            span, arguments, ..
        } => {
            raw.push((span.start, span.end, TokenIdx::ExprFunctionCall.as_u32()));
            for arg in arguments {
                walk_expression(arg, raw);
            }
        }
        Expression::StringLiteral { span, value } => {
            raw.push((span.start, span.end, TokenIdx::ExprStringLiteral.as_u32()));
            walk_elements(value, raw);
        }
        Expression::NumberLiteral { span, .. } => {
            raw.push((span.start, span.end, TokenIdx::ExprNumberLiteral.as_u32()));
        }
        Expression::BoolLiteral { span, .. } => {
            raw.push((span.start, span.end, TokenIdx::ExprBoolLiteral.as_u32()));
        }
        Expression::Null { span } => {
            raw.push((span.start, span.end, TokenIdx::ExprNull.as_u32()));
        }
        Expression::Group { span, inner } => {
            raw.push((span.start, span.end, TokenIdx::ExprGroup.as_u32()));
            walk_expression(inner, raw);
        }
        Expression::Element(e) => {
            walk_element(e, raw);
        }
    }
}

// ── Delimiter token helper ─────────────────────────────────────────────

/// Emits separate tokens for opening and closing delimiters of a delimited element.
fn emit_delimiter_tokens(
    open_span: &sevenmark_ast::Span,
    close_span: &sevenmark_ast::Span,
    token_type: u32,
    raw: &mut Vec<(usize, usize, u32)>,
) {
    raw.push((open_span.start, open_span.end, token_type));
    if close_span.start >= open_span.end && close_span.end > close_span.start {
        raw.push((close_span.start, close_span.end, token_type));
    }
}

// ── Element → token type index ──────────────────────────────────────────

fn element_token_type(element: &Element) -> u32 {
    match element {
        Element::Text(_) => TokenIdx::Text.as_u32(),
        Element::Comment(_) => TokenIdx::Comment.as_u32(),
        Element::Escape(_) => TokenIdx::Escape.as_u32(),
        Element::Error(_) => TokenIdx::Error.as_u32(),
        Element::Literal(_) => TokenIdx::Literal.as_u32(),
        Element::Define(_) => TokenIdx::Define.as_u32(),
        Element::Styled(_) => TokenIdx::Styled.as_u32(),
        Element::Table(_) => TokenIdx::Table.as_u32(),
        Element::List(_) => TokenIdx::List.as_u32(),
        Element::Fold(_) => TokenIdx::Fold.as_u32(),
        Element::BlockQuote(_) => TokenIdx::BlockQuote.as_u32(),
        Element::Ruby(_) => TokenIdx::Ruby.as_u32(),
        Element::Footnote(_) => TokenIdx::Footnote.as_u32(),
        Element::Code(_) => TokenIdx::Code.as_u32(),
        Element::TeX(_) => TokenIdx::TeX.as_u32(),
        Element::Include(_) => TokenIdx::Include.as_u32(),
        Element::Category(_) => TokenIdx::Category.as_u32(),
        Element::Redirect(_) => TokenIdx::Redirect.as_u32(),
        Element::Media(_) => TokenIdx::Media.as_u32(),
        Element::ExternalMedia(_) => TokenIdx::ExternalMedia.as_u32(),
        Element::Null(_) => TokenIdx::Null.as_u32(),
        Element::FootnoteRef(_) => TokenIdx::FootnoteRef.as_u32(),
        Element::TimeNow(_) => TokenIdx::TimeNow.as_u32(),
        Element::Age(_) => TokenIdx::Age.as_u32(),
        Element::Variable(_) => TokenIdx::Variable.as_u32(),
        Element::Mention(_) => TokenIdx::Mention.as_u32(),
        Element::Bold(_) => TokenIdx::Bold.as_u32(),
        Element::Italic(_) => TokenIdx::Italic.as_u32(),
        Element::Strikethrough(_) => TokenIdx::Strikethrough.as_u32(),
        Element::Underline(_) => TokenIdx::Underline.as_u32(),
        Element::Superscript(_) => TokenIdx::Superscript.as_u32(),
        Element::Subscript(_) => TokenIdx::Subscript.as_u32(),
        Element::SoftBreak(_) => TokenIdx::SoftBreak.as_u32(),
        Element::HardBreak(_) => TokenIdx::HardBreak.as_u32(),
        Element::HLine(_) => TokenIdx::HLine.as_u32(),
        Element::Header(h) if h.is_folded => TokenIdx::FoldedHeader.as_u32(),
        Element::Header(_) => TokenIdx::Header.as_u32(),
        Element::If(_) => TokenIdx::If.as_u32(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(text: &str) -> DocumentState {
        DocumentState::new(text.to_string())
    }

    #[test]
    fn plain_text_produces_no_tokens() {
        let state = make_state("hello");
        let tokens = collect_semantic_tokens(&state);
        // Text elements are skipped — they use the default color
        assert!(tokens.is_empty(), "plain text should produce no tokens");
    }

    #[test]
    fn bold_produces_bold_token_only() {
        let state = make_state("**bold**");
        let tokens = collect_semantic_tokens(&state);
        // bold, text children are skipped
        assert!(
            tokens
                .iter()
                .any(|t| t.token_type == TokenIdx::Bold.as_u32()),
            "expected bold token"
        );
        assert!(
            !tokens
                .iter()
                .any(|t| t.token_type == TokenIdx::Text.as_u32()),
            "text tokens should not be emitted"
        );
    }

    #[test]
    fn define_produces_define_and_parameter_tokens() {
        let state = make_state("{{{#define #x=\"v\"}}}");
        let tokens = collect_semantic_tokens(&state);
        // define + parameter tokens should both exist
        assert!(
            tokens
                .iter()
                .any(|t| t.token_type == TokenIdx::Define.as_u32()),
            "expected define token"
        );
        assert!(
            tokens
                .iter()
                .any(|t| t.token_type == TokenIdx::Parameter.as_u32()),
            "expected parameter token"
        );
    }

    #[test]
    fn header_span_covers_full_line() {
        // Without trailing newline
        let state = make_state("## Hello");
        let tokens = collect_semantic_tokens(&state);
        let h = tokens
            .iter()
            .find(|t| t.token_type == TokenIdx::Header.as_u32())
            .unwrap();
        eprintln!(
            "no newline: delta_start={}, length={}",
            h.delta_start, h.length
        );
        assert_eq!(h.length, 8, "'## Hello' should be 8 chars");

        // With trailing newline (real file scenario)
        let state2 = make_state("## Hello\nsome text");
        let tokens2 = collect_semantic_tokens(&state2);
        let h2 = tokens2
            .iter()
            .find(|t| t.token_type == TokenIdx::Header.as_u32())
            .unwrap();
        eprintln!(
            "with newline: delta_start={}, length={}",
            h2.delta_start, h2.length
        );
        assert_eq!(
            h2.length, 8,
            "'## Hello' with newline should still be 8 chars"
        );
    }

    #[test]
    fn table_with_conditional_rows() {
        let src = "{{{#table\n[[ [[a]] [[b]] ]]\n{{{#if true ::\n[[ [[c]] ]]\n}}}\n}}}";
        let state = make_state(src);
        let tokens = collect_semantic_tokens(&state);

        // Print detailed token info
        for t in &tokens {
            eprintln!(
                "  type={:<2} delta_line={} delta_start={} length={}",
                t.token_type, t.delta_line, t.delta_start, t.length
            );
        }

        let types: Vec<u32> = tokens.iter().map(|t| t.token_type).collect();
        // table delimiter tokens should appear exactly twice (open + close)
        let table_count = types
            .iter()
            .filter(|&&t| t == TokenIdx::Table.as_u32())
            .count();
        assert_eq!(
            table_count, 2,
            "expected 2 table tokens (open+close), got {table_count}"
        );
        assert!(
            types.contains(&TokenIdx::TableRow.as_u32()),
            "expected tableRow token"
        );
        assert!(
            types.contains(&TokenIdx::TableCell.as_u32()),
            "expected tableCell token"
        );
        assert!(
            types.contains(&TokenIdx::ConditionalTableRows.as_u32()),
            "expected conditionalTableRows token"
        );
    }

    #[test]
    fn if_produces_if_and_expression_tokens() {
        let state = make_state("{{{#if true :: content}}}");
        let tokens = collect_semantic_tokens(&state);
        assert!(
            tokens.iter().any(|t| t.token_type == TokenIdx::If.as_u32()),
            "expected if token"
        );
        // Should have at least 2 tokens (if, expression; content text is skipped)
        assert!(
            tokens.len() >= 2,
            "expected at least 2 tokens, got {}",
            tokens.len()
        );
    }

    #[test]
    fn folded_header_produces_folded_header_token() {
        let state = make_state("##! Folded");
        let tokens = collect_semantic_tokens(&state);
        assert!(
            tokens
                .iter()
                .any(|t| t.token_type == TokenIdx::FoldedHeader.as_u32()),
            "expected foldedHeader token for '##!'"
        );
        assert!(
            !tokens
                .iter()
                .any(|t| t.token_type == TokenIdx::Header.as_u32()),
            "folded header should not produce regular header token"
        );
    }

    #[test]
    fn adjacent_delimiters_emit_both_tokens() {
        let mut raw = Vec::new();
        let open = sevenmark_ast::Span::new(0, 3);
        let close = sevenmark_ast::Span::new(3, 6);
        emit_delimiter_tokens(&open, &close, TokenIdx::Table.as_u32(), &mut raw);

        assert_eq!(raw.len(), 2, "expected open+close delimiter tokens");
        assert_eq!(raw[0], (0, 3, TokenIdx::Table.as_u32()));
        assert_eq!(raw[1], (3, 6, TokenIdx::Table.as_u32()));
    }
}
