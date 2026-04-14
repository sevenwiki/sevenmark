use maud::{Markup, PreEscaped, html};
use sevenmark_ast::{Element, Parameters};

use super::{age, date, datetime, dday, pagecount, timenow};
use crate::classes;
use crate::context::RenderContext;
use crate::render::{mention, text, utils};
use crate::section::{Section, SectionTree};

pub fn build(tree: &SectionTree<'_>, ctx: &mut RenderContext) -> Markup {
    if tree.sections.is_empty() {
        return html! {};
    }

    html! {
        details class=(classes::TOC) open {
            summary class=(classes::TOC_SUMMARY) aria-label="Table of contents" {}
            nav class=(classes::TOC_BODY) aria-label="Table of contents" {
                (render_section_list(&tree.sections, ctx))
            }
        }
    }
}

pub fn render(ctx: &RenderContext) -> Markup {
    match ctx.toc_markup.as_deref() {
        Some(markup) if !markup.is_empty() => PreEscaped(markup.to_string()),
        _ => html! {},
    }
}

fn render_section_list(sections: &[Section<'_>], ctx: &mut RenderContext) -> Markup {
    html! {
        ul class=(classes::TOC_LIST) {
            @for section in sections {
                li class=(classes::TOC_ITEM) {
                    a
                        class=(classes::TOC_LINK)
                        href=(format!("#{}{}", classes::SECTION_ID_PREFIX, section.section_path))
                    {
                        span class=(classes::SECTION_PATH) { (section.section_path) "." }
                        span class=(classes::HEADER_CONTENT) {
                            (render_toc_label(&section.header_children, ctx))
                        }
                    }
                    @if !section.children.is_empty() {
                        (render_section_list(&section.children, ctx))
                    }
                }
            }
        }
    }
}

fn render_toc_label(children: &[Element], ctx: &mut RenderContext) -> Markup {
    html! {
        @for child in children {
            (render_toc_label_element(child, ctx))
        }
    }
}

fn render_toc_label_element(el: &Element, ctx: &mut RenderContext) -> Markup {
    match el {
        Element::Text(text_el) => text::text::render(&text_el.span, &text_el.value, ctx),
        Element::Escape(escape_el) => text::escape::render(&escape_el.span, &escape_el.value, ctx),
        Element::Bold(style_el) => html! {
            strong class=(classes::BOLD) { (render_toc_label(&style_el.children, ctx)) }
        },
        Element::Italic(style_el) => html! {
            em class=(classes::ITALIC) { (render_toc_label(&style_el.children, ctx)) }
        },
        Element::Strikethrough(style_el) => html! {
            del class=(classes::STRIKETHROUGH) { (render_toc_label(&style_el.children, ctx)) }
        },
        Element::Underline(style_el) => html! {
            u class=(classes::UNDERLINE) { (render_toc_label(&style_el.children, ctx)) }
        },
        Element::Superscript(style_el) => html! {
            sup class=(classes::SUPERSCRIPT) { (render_toc_label(&style_el.children, ctx)) }
        },
        Element::Subscript(style_el) => html! {
            sub class=(classes::SUBSCRIPT) { (render_toc_label(&style_el.children, ctx)) }
        },
        Element::Styled(styled_el) => render_styled_label(
            &styled_el.parameters,
            &styled_el.children,
            classes::STYLED,
            ctx,
        ),
        Element::Ruby(ruby_el) => render_ruby_label(&ruby_el.parameters, &ruby_el.children, ctx),
        // Media headings should keep their caption text but never render the image/link widget.
        Element::Media(media_el) => render_toc_label(&media_el.children, ctx),
        Element::Include(include_el) => render_toc_label(&include_el.children, ctx),
        Element::Variable(var_el) => text::variable::render(&var_el.span, &var_el.name, ctx),
        Element::TimeNow(_) => timenow::render(),
        Element::Date(_) => date::render(),
        Element::DateTime(_) => datetime::render(),
        Element::Dday(dday_el) => dday::render(&dday_el.date),
        Element::Age(age_el) => age::render(&age_el.date),
        Element::PageCount(pagecount_el) => pagecount::render(pagecount_el.namespace.as_deref()),
        Element::Mention(mention_el) => mention::mention::render(&mention_el.kind, &mention_el.id),
        Element::SoftBreak(_) | Element::HardBreak(_) => html! { " " },
        Element::Comment(_)
        | Element::Error(_)
        | Element::Literal(_)
        | Element::Define(_)
        | Element::Table(_)
        | Element::List(_)
        | Element::Fold(_)
        | Element::BlockQuote(_)
        | Element::Footnote(_)
        | Element::Code(_)
        | Element::TeX(_)
        | Element::Css(_)
        | Element::Category(_)
        | Element::Redirect(_)
        | Element::ExternalMedia(_)
        | Element::Null(_)
        | Element::FootnoteRef(_)
        | Element::Anchor(_)
        | Element::Toc(_)
        | Element::Clear(_)
        | Element::HLine(_)
        | Element::Header(_)
        | Element::If(_) => html! {},
    }
}

fn render_styled_label(
    parameters: &Parameters,
    children: &[Element],
    base_class: &str,
    ctx: &mut RenderContext,
) -> Markup {
    let merged_class = utils::merge_class(base_class, parameters);
    let lk = ctx.add_light_style(utils::build_style(parameters));
    let dk = ctx.add_dark_style(utils::build_dark_style(parameters));

    html! {
        span class=(merged_class) data-lk=[lk] data-dk=[dk] {
            (render_toc_label(children, ctx))
        }
    }
}

fn render_ruby_label(
    parameters: &Parameters,
    children: &[Element],
    ctx: &mut RenderContext,
) -> Markup {
    let class = utils::merge_class(classes::RUBY, parameters);
    let lk = ctx.add_light_style(utils::build_style(parameters));
    let dk = ctx.add_dark_style(utils::build_dark_style(parameters));
    let ruby_text = utils::get_param(parameters, "ruby").unwrap_or_default();

    html! {
        ruby class=(class) data-lk=[lk] data-dk=[dk] {
            (render_toc_label(children, ctx))
            rt { (ruby_text) }
        }
    }
}
