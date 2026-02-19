use sevenmark_ast::Element;
use tower_lsp_server::ls_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensLegend,
};

use crate::ast_walk::visit_elements;
use crate::document::DocumentState;
// Custom SevenMark token types.
// VS Code extension will map these to colors via `semanticTokenColorCustomizations`.
pub const TOKEN_TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::new("comment"),       // 0
    SemanticTokenType::new("variable"),      // 1  - [var(...)]
    SemanticTokenType::new("code"),          // 2  - {{{#code ...}}}
    SemanticTokenType::new("tex"),           // 3  - $...$, $$...$$
    SemanticTokenType::new("header"),        // 4
    SemanticTokenType::new("bold"),          // 5
    SemanticTokenType::new("italic"),        // 6
    SemanticTokenType::new("strikethrough"), // 7
    SemanticTokenType::new("underline"),     // 8
    SemanticTokenType::new("superscript"),   // 9
    SemanticTokenType::new("subscript"),     // 10
    SemanticTokenType::new("escape"),        // 11
    SemanticTokenType::new("error"),         // 12
    SemanticTokenType::new("macro"),         // 13 - [br], [null], [now], [fn], [age]
    SemanticTokenType::new("mention"),       // 14 - <@uuid>, <#uuid>
    SemanticTokenType::new("media"),         // 15 - [[...]]
    SemanticTokenType::new("externalMedia"), // 16 - [[#youtube ...]]
];

pub const TOKEN_MODIFIERS: &[SemanticTokenModifier] = &[];

pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: TOKEN_TYPES.to_vec(),
        token_modifiers: TOKEN_MODIFIERS.to_vec(),
    }
}

/// Collects semantic tokens from the AST.
///
/// Only leaf elements emit tokens. Container elements (if, define, table, list,
/// fold, styled, etc.) are skipped because their span covers children — the
/// children produce their own tokens.
pub fn collect_semantic_tokens(state: &DocumentState) -> Vec<SemanticToken> {
    let mut raw: Vec<(usize, usize, u32)> = Vec::new();

    visit_elements(&state.elements, &mut |element| {
        if let Some((start, end, ty)) = token_for_element(element) {
            raw.push((start, end, ty));
        }
    });

    raw.sort_by_key(|&(start, _, _)| start);

    // Delta-encode
    let mut tokens = Vec::with_capacity(raw.len());
    let mut prev_line: u32 = 0;
    let mut prev_char: u32 = 0;

    for (start, end, token_type) in raw {
        let (line, character) = state.line_index.byte_offset_to_position(&state.text, start);
        let (end_line, end_char) =
            state.line_index.byte_offset_to_position(&state.text, end);

        // For multi-line tokens, only report the first line's portion.
        let length = if line == end_line {
            end_char - character
        } else {
            let line_end_byte =
                state
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

/// Maps a leaf element to (span_start, span_end, token_type_index).
///
/// Container elements return None — they would overlap with children.
fn token_for_element(element: &Element) -> Option<(usize, usize, u32)> {
    let span = element.span();
    let (start, end) = (span.start, span.end);

    let ty = match element {
        // Leaf elements with unique token types
        Element::Comment(_) => 0,
        Element::Variable(_) => 1,
        Element::Code(_) => 2,
        Element::TeX(_) => 3,
        Element::Header(_) => 4,
        Element::Bold(_) => 5,
        Element::Italic(_) => 6,
        Element::Strikethrough(_) => 7,
        Element::Underline(_) => 8,
        Element::Superscript(_) => 9,
        Element::Subscript(_) => 10,
        Element::Escape(_) => 11,
        Element::Error(_) => 12,
        Element::Null(_)
        | Element::FootnoteRef(_)
        | Element::TimeNow(_)
        | Element::Age(_)
        | Element::HardBreak(_) => 13,
        Element::Mention(_) => 14,
        Element::Media(_) => 15,
        Element::ExternalMedia(_) => 16,

        // Container/structural elements — skip (children handle themselves)
        // Text, SoftBreak, HLine — no semantic meaning worth highlighting
        _ => return None,
    };

    Some((start, end, ty))
}