//! Document-level rendering

use maud::{Markup, html};

use super::{brace, element, markdown};
use crate::classes;
use crate::context::RenderContext;
use crate::section::{Section, SectionTree, build_section_tree};
use sevenmark_parser::ast::SevenMarkElement;

/// Render a document to semantic HTML
///
/// # Arguments
/// * `ast` - The parsed AST elements
/// * `edit_url` - Base URL for edit links (e.g., "/edit/문서제목")
pub fn render_document(ast: &[SevenMarkElement], edit_url: &str) -> String {
    let tree = build_section_tree(ast);
    let mut ctx = RenderContext::new();
    let content = render_section_tree(&tree, edit_url, &mut ctx);
    let footnotes = brace::footnote::render_list(&ctx);

    let markup = html! {
        (content)
        @if !ctx.footnotes.is_empty() {
            (footnotes)
        }
    };

    markup.into_string()
}

/// Render a section tree
fn render_section_tree(tree: &SectionTree<'_>, edit_url: &str, ctx: &mut RenderContext) -> Markup {
    html! {
        @for el in &tree.preamble {
            (element::render_element(el, ctx))
        }
        @for section in &tree.sections {
            (render_section(section, edit_url, ctx))
        }
    }
}

/// Render a single section with nested structure
fn render_section(section: &Section<'_>, edit_url: &str, ctx: &mut RenderContext) -> Markup {
    let header_markup =
        markdown::header::render_with_path(section.header, &section.section_path, edit_url, ctx);

    html! {
        section class=(classes::SECTION) data-section=(section.header.section_index) {
            (header_markup)
            div class=(classes::SECTION_CONTENT) {
                @for el in &section.content {
                    (element::render_element(el, ctx))
                }
                @for child in &section.children {
                    (render_section(child, edit_url, ctx))
                }
            }
        }
    }
}
