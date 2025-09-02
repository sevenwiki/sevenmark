use super::escape::escape_parser;
use super::markdown::{
    markdown_bold_parser, markdown_header_parser, markdown_hline_parser, markdown_italic_parser,
    markdown_strikethrough_parser, markdown_subscript_parser, markdown_superscript_parser,
    markdown_underline_parser,
};
use super::text::text_parser;
use super::token::*;
use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::parser::brace::{
    brace_blockquote_parser, brace_category_parser, brace_code_parser, brace_fold_parser,
    brace_include_parser, brace_list_parser, brace_literal_parser, brace_redirect_parser,
    brace_style_parser, brace_table_parser, brace_tex_parser,
};
use crate::sevenmark::parser::bracket::bracket_media_parser;
use crate::sevenmark::parser::comment::{inline_comment_parser, multiline_comment_parser};
use crate::sevenmark::parser::r#macro::{
    macro_age_parser, macro_newline_parser, macro_now_parser, macro_null_parser,
};
use winnow::Result;
use winnow::combinator::alt;
use winnow::combinator::repeat;
use winnow::prelude::*;

pub fn element_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    let result = repeat(
        1..,
        alt((
            // Escape
            escape_parser,
            // Comment
            multiline_comment_parser,
            inline_comment_parser,
            // Brace
            alt((
                brace_include_parser,
                brace_category_parser,
                brace_redirect_parser,
                // brace w/parameters
                brace_table_parser,
                brace_list_parser,
                brace_fold_parser,
                brace_blockquote_parser,
                brace_code_parser,
                brace_tex_parser,
                brace_style_parser,
                brace_literal_parser,
            )),
            // Bracket
            bracket_media_parser,
            alt((
                markdown_header_parser,
                markdown_hline_parser,
                markdown_bold_parser,
                markdown_italic_parser,
                markdown_underline_parser,
                markdown_strikethrough_parser,
                markdown_superscript_parser,
                markdown_subscript_parser,
            )),
            alt((
                macro_now_parser,
                macro_newline_parser,
                macro_age_parser,
                macro_null_parser,
            )),
            // Text,
            text_parser,
            // Token,
            token_newline_parser,
            token_brace_open_parser,
            token_brace_close_parser,
            token_bracket_open_parser,
            token_bracket_close_parser,
            token_slash,
            token_asterisk_parser,
            token_underscore_parser,
            token_tilde_parser,
            token_caret_parser,
            token_comma_parser,
            token_backslash_parser,
        )),
    )
    .parse_next(parser_input);
    result
}
