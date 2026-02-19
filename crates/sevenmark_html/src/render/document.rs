//! Document-level rendering

use maud::{Markup, html};
use sevenmark_utils::Utf16OffsetConverter;

use super::{brace, element, markdown};
use crate::classes;
use crate::config::RenderConfig;
use crate::context::RenderContext;
use crate::section::{Section, SectionTree, build_section_tree};
use sevenmark_ast::Element;

/// Render a document to semantic HTML
///
/// # Arguments
/// * `ast` - The parsed AST elements
/// * `config` - Render configuration
pub fn render_document(ast: &[Element], config: &RenderConfig) -> String {
    let tree = build_section_tree(ast);
    let mut ctx = RenderContext::new(config);
    let content = render_section_tree(&tree, config, &mut ctx);

    let markup = html! {
        (content)
        @if !ctx.footnotes.is_empty() {
            (brace::footnote::render_list(&ctx))
        }
    };

    markup.into_string()
}

/// Render a document to semantic HTML with span data attributes
///
/// Each element will have `data-start` and `data-end` attributes with UTF-16 offsets.
/// This is useful for editor synchronization (e.g., highlighting preview elements based on cursor position).
///
/// # Arguments
/// * `ast` - The parsed AST elements
/// * `config` - Render configuration (include_spans should be true)
/// * `input` - Original input text for UTF-16 offset calculation
pub fn render_document_with_spans(ast: &[Element], config: &RenderConfig, input: &str) -> String {
    let tree = build_section_tree(ast);
    let converter = Utf16OffsetConverter::new(input);
    let mut ctx = RenderContext::with_converter(config, &converter);
    let content = render_section_tree(&tree, config, &mut ctx);

    let markup = html! {
        (content)
        @if !ctx.footnotes.is_empty() {
            (brace::footnote::render_list(&ctx))
        }
    };

    markup.into_string()
}

/// Render a section tree
fn render_section_tree(
    tree: &SectionTree<'_>,
    config: &RenderConfig,
    ctx: &mut RenderContext,
) -> Markup {
    html! {
        @for el in &tree.preamble {
            (element::render_element(el, ctx))
        }
        @for section in &tree.sections {
            (render_section(section, config, ctx))
        }
    }
}

/// Render a single section with nested structure
fn render_section(section: &Section<'_>, config: &RenderConfig, ctx: &mut RenderContext) -> Markup {
    let header_markup = markdown::header::render_with_path(
        section.header_level,
        section.header_section_index,
        section.header_children,
        &section.section_path,
        config,
        ctx,
    );

    let section_content = html! {
        div class=(classes::SECTION_CONTENT) {
            @for el in &section.content {
                (element::render_element(el, ctx))
            }
            @for child in &section.children {
                (render_section(child, config, ctx))
            }
        }
    };

    if section.header_is_folded {
        let class = format!("{} {}", classes::SECTION, classes::SECTION_FOLDED);
        html! {
            details class=(class) data-section=(section.header_section_index) {
                summary { (header_markup) }
                (section_content)
            }
        }
    } else {
        html! {
            details class=(classes::SECTION) data-section=(section.header_section_index) open {
                summary { (header_markup) }
                (section_content)
            }
        }
    }
}
