use super::{
    DEFAULT_NAMESPACE, MediaReference, RedirectReference, SectionInfo, normalized_plain_text,
    parse_namespace,
};
use crate::wiki::DocumentNamespace;
use sevenmark_ast::{Element, MentionType, Traversable};
use std::collections::HashSet;

struct MetadataCollector<'a> {
    categories: &'a mut HashSet<String>,
    redirect: &'a mut Option<RedirectReference>,
    media: &'a mut HashSet<MediaReference>,
    sections: &'a mut Vec<SectionInfo>,
    user_mentions: &'a mut HashSet<String>,
    section_stack: Vec<SectionInfo>,
    max_end: usize,
    collect_categories_redirect: bool,
}

pub(super) fn collect_metadata(
    elements: &[Element],
    categories: &mut HashSet<String>,
    redirect: &mut Option<RedirectReference>,
    media: &mut HashSet<MediaReference>,
    sections: &mut Vec<SectionInfo>,
    user_mentions: &mut HashSet<String>,
    collect_categories_redirect: bool,
) {
    let mut collector = MetadataCollector {
        categories,
        redirect,
        media,
        sections,
        user_mentions,
        section_stack: Vec::new(),
        max_end: 0,
        collect_categories_redirect,
    };

    for element in elements {
        collect_metadata_recursive(element, &mut collector);
    }

    // Remaining headers in stack end at document end
    for mut section in collector.section_stack {
        section.end = collector.max_end;
        collector.sections.push(section);
    }
}

fn collect_metadata_recursive(element: &Element, c: &mut MetadataCollector) {
    // Track max span.end for document length
    let span = element.span();
    if span.end > c.max_end {
        c.max_end = span.end;
    }

    match element {
        Element::Header(header) => {
            let start = span.start;
            let level = header.level;

            // Pop headers with level >= current (same or lower priority)
            while let Some(mut section) = c.section_stack.pop() {
                if section.level >= level {
                    section.end = start;
                    c.sections.push(section);
                } else {
                    c.section_stack.push(section);
                    break;
                }
            }

            c.section_stack.push(SectionInfo {
                section_index: header.section_index,
                level,
                start,
                end: 0,
            });
        }
        Element::Media(media_elem) => {
            if let Some(file_param) = media_elem.parameters.get("file")
                && let Some(title) = normalized_plain_text(&file_param.value)
            {
                c.media.insert(MediaReference {
                    namespace: DocumentNamespace::File,
                    title,
                });
            }

            if let Some(doc_param) = media_elem.parameters.get("document")
                && let Some(title) = normalized_plain_text(&doc_param.value)
            {
                c.media.insert(MediaReference {
                    namespace: DocumentNamespace::Document,
                    title,
                });
            }

            if let Some(cat_param) = media_elem.parameters.get("category")
                && let Some(title) = normalized_plain_text(&cat_param.value)
            {
                c.media.insert(MediaReference {
                    namespace: DocumentNamespace::Category,
                    title,
                });
            }

            if let Some(user_param) = media_elem.parameters.get("user")
                && let Some(title) = normalized_plain_text(&user_param.value)
            {
                c.media.insert(MediaReference {
                    namespace: DocumentNamespace::User,
                    title,
                });
            }
        }
        Element::Category(cat_elem) if c.collect_categories_redirect => {
            if let Some(name) = normalized_plain_text(&cat_elem.children) {
                c.categories.insert(name);
            }
        }
        Element::Redirect(redirect_elem) if c.collect_categories_redirect => {
            if let Some(title) = normalized_plain_text(&redirect_elem.children)
                && c.redirect.is_none()
            {
                let namespace_str = redirect_elem
                    .parameters
                    .get("namespace")
                    .and_then(|param| normalized_plain_text(&param.value))
                    .unwrap_or_else(|| DEFAULT_NAMESPACE.to_string());
                let namespace = parse_namespace(&namespace_str);
                *c.redirect = Some(RedirectReference { namespace, title });
            }
        }
        Element::Mention(mention_elem) if mention_elem.kind == MentionType::User => {
            c.user_mentions.insert(mention_elem.id.clone());
        }
        _ => {}
    }

    element.traverse_children_ref(&mut |child| {
        collect_metadata_recursive(child, c);
    });
}
