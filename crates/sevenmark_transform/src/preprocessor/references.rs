use super::define_if::process_defines_and_ifs_with_protected_keys;
use super::{
    DEFAULT_NAMESPACE, DocumentReference, MediaReference, collect_metadata, normalized_plain_text,
    parse_namespace,
};
use crate::wiki::DocumentNamespace;
use sevenmark_ast::IncludeElement;
use sevenmark_ast::{Element, Traversable};
use sevenmark_utils::extract_plain_text;
use std::collections::{HashMap, HashSet};
use tracing::warn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IncludeCacheKey {
    target: DocumentReference,
    params: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct IncludeCacheValue {
    ast: Vec<Element>,
    media: HashSet<MediaReference>,
}

pub(super) fn collect_includes(elements: &[Element], includes: &mut HashSet<DocumentReference>) {
    for element in elements {
        collect_includes_recursive(element, includes);
    }
}

fn collect_includes_recursive(element: &Element, includes: &mut HashSet<DocumentReference>) {
    if let Element::Include(include_elem) = element
        && let Some(title) = normalized_plain_text(&include_elem.children)
    {
        let namespace_str = include_elem
            .parameters
            .get("namespace")
            .and_then(|param| normalized_plain_text(&param.value))
            .unwrap_or_else(|| DEFAULT_NAMESPACE.to_string());
        let namespace = parse_namespace(&namespace_str);
        includes.insert(DocumentReference { namespace, title });
    }

    element.traverse_children_ref(&mut |child| {
        collect_includes_recursive(child, includes);
    });
}

/// Collect all document references from AST
pub(super) fn collect_references(
    elements: &[Element],
    references: &mut HashSet<DocumentReference>,
) {
    for element in elements {
        collect_references_recursive(element, references);
    }
}

fn collect_references_recursive(element: &Element, references: &mut HashSet<DocumentReference>) {
    match element {
        Element::Category(cat_elem) => {
            if let Some(name) = normalized_plain_text(&cat_elem.children) {
                references.insert(DocumentReference {
                    namespace: DocumentNamespace::Category,
                    title: name,
                });
            }
        }
        Element::Media(media_elem) => {
            if let Some(file_param) = media_elem.parameters.get("file")
                && let Some(title) = normalized_plain_text(&file_param.value)
            {
                references.insert(DocumentReference {
                    namespace: DocumentNamespace::File,
                    title,
                });
            }

            if let Some(doc_param) = media_elem.parameters.get("document")
                && let Some(title) = normalized_plain_text(&doc_param.value)
            {
                references.insert(DocumentReference {
                    namespace: DocumentNamespace::Document,
                    title,
                });
            }

            if let Some(cat_param) = media_elem.parameters.get("category")
                && let Some(title) = normalized_plain_text(&cat_param.value)
            {
                references.insert(DocumentReference {
                    namespace: DocumentNamespace::Category,
                    title,
                });
            }

            if let Some(user_param) = media_elem.parameters.get("user")
                && let Some(title) = normalized_plain_text(&user_param.value)
            {
                references.insert(DocumentReference {
                    namespace: DocumentNamespace::User,
                    title,
                });
            }
        }
        _ => {}
    }

    element.traverse_children_ref(&mut |child| {
        collect_references_recursive(child, references);
    });
}

pub(super) fn substitute_includes(
    elements: &mut [Element],
    docs_map: &HashMap<DocumentReference, Vec<Element>>,
    all_media: &mut HashSet<MediaReference>,
) {
    let mut include_cache = HashMap::new();
    for element in elements {
        substitute_includes_recursive(element, docs_map, all_media, &mut include_cache);
    }
}

fn build_include_cache_key(
    include_elem: &IncludeElement,
    target: DocumentReference,
) -> IncludeCacheKey {
    let params = include_elem
        .parameters
        .iter()
        .filter(|(k, _)| k.as_str() != "namespace")
        .map(|(k, v)| (k.clone(), extract_plain_text(&v.value)))
        .collect();

    IncludeCacheKey { target, params }
}

fn build_include_param_context(
    include_elem: &IncludeElement,
) -> (HashSet<String>, HashMap<String, String>) {
    let mut protected_keys = HashSet::new();
    let mut params_map = HashMap::new();

    for (key, parameter) in &include_elem.parameters {
        if key == "namespace" {
            continue;
        }

        protected_keys.insert(key.clone());
        params_map.insert(key.clone(), extract_plain_text(&parameter.value));
    }

    (protected_keys, params_map)
}

fn substitute_includes_recursive(
    element: &mut Element,
    docs_map: &HashMap<DocumentReference, Vec<Element>>,
    all_media: &mut HashSet<MediaReference>,
    include_cache: &mut HashMap<IncludeCacheKey, IncludeCacheValue>,
) {
    if let Element::Include(include_elem) = element
        && let Some(title) = normalized_plain_text(&include_elem.children)
    {
        let namespace_str = include_elem
            .parameters
            .get("namespace")
            .and_then(|param| normalized_plain_text(&param.value))
            .unwrap_or_else(|| DEFAULT_NAMESPACE.to_string());
        let namespace = parse_namespace(&namespace_str);
        let doc_key = DocumentReference {
            namespace: namespace.clone(),
            title: title.clone(),
        };

        let cache_key = build_include_cache_key(include_elem, doc_key.clone());
        if let Some(cached) = include_cache.get(&cache_key) {
            all_media.extend(cached.media.iter().cloned());
            include_elem.children = cached.ast.clone();
            return;
        }

        if let Some(base_ast) = docs_map.get(&doc_key) {
            // Clone the document AST
            let mut included_ast = base_ast.clone();

            let (protected_keys, mut params_map) = build_include_param_context(include_elem);

            // Process defines and ifs (include parameters have priority)
            process_defines_and_ifs_with_protected_keys(
                &mut included_ast,
                &mut params_map,
                Some(&protected_keys),
            );

            // Collect media from included document
            let mut categories = HashSet::new();
            let mut redirect = None;
            let mut ignored_sections = Vec::new();
            let mut ignored_user_mentions = HashSet::new();
            let mut included_media = HashSet::new();
            collect_metadata(
                &included_ast,
                &mut categories,
                &mut redirect,
                &mut included_media,
                &mut ignored_sections,
                &mut ignored_user_mentions,
                false,
            );
            all_media.extend(included_media.iter().cloned());

            include_cache.insert(
                cache_key,
                IncludeCacheValue {
                    ast: included_ast.clone(),
                    media: included_media,
                },
            );

            // Replace include content
            include_elem.children = included_ast;
            return;
        }

        warn!(namespace = ?namespace, title = %title, "Include target not found");
    }

    // Traverse children
    element.traverse_children(&mut |child| {
        substitute_includes_recursive(child, docs_map, all_media, include_cache);
    });
}
