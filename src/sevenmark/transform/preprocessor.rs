use crate::SevenMarkElement;
use crate::sevenmark::core::parse_document;
use crate::sevenmark::transform::wiki::{DocumentNamespace, WikiClient};
use crate::sevenmark::{Location, Parameters, TextElement, Traversable};
use anyhow::{Context, Result};
use async_recursion::async_recursion;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tracing::{debug, warn};

const MAX_INCLUDE_DEPTH: usize = 16;

/// Final result after recursive include resolution
#[derive(Debug, Clone, Serialize)]
pub struct ProcessedDocument {
    pub media: HashSet<String>,
    pub categories: HashSet<String>,
    pub redirect: Option<String>,
    pub ast: Vec<SevenMarkElement>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncludeInfo {
    pub title: String,
    pub namespace: DocumentNamespace,
    pub parameters: Parameters,
}

#[derive(Debug, Clone)]
pub struct PreprocessInfo {
    pub includes: HashMap<String, IncludeInfo>,
    pub categories: HashSet<String>,
    pub redirect: Option<String>,
    pub media: HashSet<String>,
}

#[derive(Debug, Clone)]
struct ResolvedDocument {
    ast: Vec<SevenMarkElement>,
    media: HashSet<String>,
    categories: HashSet<String>,
    redirect: Option<String>,
}

impl ResolvedDocument {
    fn from_collected_info(ast: Vec<SevenMarkElement>, info: CollectedInfo, depth: usize) -> Self {
        Self {
            ast,
            media: info.media,
            categories: if depth == 0 {
                info.categories
            } else {
                HashSet::new()
            },
            redirect: if depth == 0 { info.redirect } else { None },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct CollectedInfo {
    includes: Vec<IncludeInfo>,
    media: HashSet<String>,
    categories: HashSet<String>,
    redirect: Option<String>,
}

/// Recursively resolves includes and collects metadata
pub async fn preprocess_sevenmark(
    namespace: DocumentNamespace,
    title: String,
    input: &str,
    wiki_client: &WikiClient,
) -> Result<ProcessedDocument> {
    let mut visited = HashSet::new();
    let parent_params = HashMap::new();

    let initial_key = format!("{}:{}", namespace_to_string(&namespace), title);
    visited.insert(initial_key.clone());

    let resolved = resolve_document(
        input,
        &parent_params,
        0,
        MAX_INCLUDE_DEPTH,
        &mut visited,
        wiki_client,
    )
    .await?;

    Ok(ProcessedDocument {
        ast: resolved.ast,
        media: resolved.media,
        categories: resolved.categories,
        redirect: resolved.redirect,
    })
}

#[async_recursion]
async fn resolve_document(
    content: &str,
    parent_params: &HashMap<String, String>,
    depth: usize,
    max_depth: usize,
    visited: &mut HashSet<String>,
    wiki_client: &WikiClient,
) -> Result<ResolvedDocument> {
    let mut ast = parse_document(content);

    let mut all_params = parent_params.clone();
    substitute_variables_forward_only(&mut ast, &mut all_params);

    let mut info = CollectedInfo::default();
    let is_top_level = depth == 0;
    collect_info(&mut ast, &mut info, is_top_level);

    if info.includes.is_empty() {
        return Ok(ResolvedDocument::from_collected_info(ast, info, depth));
    }

    if depth >= max_depth {
        warn!("Maximum include depth ({}) reached, stopping recursion", max_depth);
        return Ok(ResolvedDocument::from_collected_info(ast, info, depth));
    }

    let new_includes = filter_new_includes(std::mem::take(&mut info.includes), visited, depth);

    if new_includes.is_empty() {
        return Ok(ResolvedDocument::from_collected_info(ast, info, depth));
    }

    let requests: Vec<_> = new_includes
        .iter()
        .map(|inc| (inc.namespace.clone(), inc.title.clone()))
        .collect();

    debug!("Fetching {} includes at depth {}", requests.len(), depth);

    let fetched_docs = wiki_client.fetch_documents_batch(requests).await?;

    let mut resolved_includes: HashMap<String, ResolvedDocument> = HashMap::new();

    let docs_map: HashMap<String, _> = fetched_docs
        .into_iter()
        .map(|doc| {
            let key = format!("{}:{}", namespace_to_string(&doc.namespace), doc.title);
            (key, doc)
        })
        .collect();

    for include_info in new_includes.iter() {
        let doc_key = format!(
            "{}:{}",
            namespace_to_string(&include_info.namespace),
            &include_info.title
        );

        let Some(doc) = docs_map.get(&doc_key) else {
            warn!("[Warning] Include target not found, skipping: {}", doc_key);
            continue;
        };

        visited.insert(doc_key.clone());

        let params_map: HashMap<String, String> = include_info
            .parameters
            .iter()
            .map(|(k, v)| (k.clone(), extract_plain_text(&v.value)))
            .collect();

        let resolved = resolve_document(
            &doc.current_revision.content,
            &params_map,
            depth + 1,
            max_depth,
            visited,
            wiki_client,
        )
        .await
        .with_context(|| format!("Failed to resolve include: {}", doc_key))?;

        let hash_key = make_include_key(&include_info.title, &include_info.parameters);
        resolved_includes.insert(hash_key, resolved);

        visited.remove(&doc_key);
    }

    substitute_includes(&mut ast, &resolved_includes);

    let mut all_media = info.media;

    for resolved in resolved_includes.values() {
        all_media.extend(resolved.media.clone());
    }

    Ok(ResolvedDocument {
        ast,
        media: all_media,
        categories: if depth == 0 {
            info.categories
        } else {
            HashSet::new()
        },
        redirect: if depth == 0 { info.redirect } else { None },
    })
}

fn filter_new_includes(
    includes: Vec<IncludeInfo>,
    visited: &HashSet<String>,
    depth: usize,
) -> Vec<IncludeInfo> {
    let mut new_includes_map: HashMap<String, IncludeInfo> = HashMap::new();

    for inc in includes {
        let doc_key = format!("{}:{}", namespace_to_string(&inc.namespace), &inc.title);
        if visited.contains(&doc_key) {
            warn!("[Depth {}] Circular reference detected: {}", depth, doc_key);
        } else {
            let hash_key = make_include_key(&inc.title, &inc.parameters);
            new_includes_map.insert(hash_key, inc);
        }
    }

    new_includes_map.into_values().collect()
}

fn substitute_variables_forward_only(
    elements: &mut [SevenMarkElement],
    params: &mut HashMap<String, String>,
) {
    for element in elements {
        substitute_variables_forward_only_recursive(element, params);
    }
}

fn substitute_variables_forward_only_recursive(
    element: &mut SevenMarkElement,
    params: &mut HashMap<String, String>,
) {
    if let SevenMarkElement::DefineElement(def) = element {
        for (key, param) in &def.parameters {
            let value = extract_plain_text(&param.value);
            if !value.is_empty() {
                params.entry(key.clone()).or_insert(value);
            }
        }
    }

    if let SevenMarkElement::Variable(var) = element {
        if let Some(value) = params.get(&var.content) {
            *element = SevenMarkElement::Text(TextElement {
                location: Location::synthesized(),
                content: value.clone(),
            });
            return;
        }
    }

    let mut children = Vec::new();
    element.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &mut children {
        substitute_variables_forward_only_recursive(child, params);
    }
}

fn collect_info(
    elements: &mut [SevenMarkElement],
    info: &mut CollectedInfo,
    collect_categories_redirect: bool,
) {
    for element in elements {
        collect_info_recursive(element, info, collect_categories_redirect);
    }
}

fn collect_info_recursive(
    element: &mut SevenMarkElement,
    info: &mut CollectedInfo,
    collect_categories_redirect: bool,
) {
    match element {
        SevenMarkElement::Include(inc) => {
            let title = extract_plain_text(&inc.content);
            if !title.is_empty() {
                let namespace_str = inc
                    .parameters
                    .get("namespace")
                    .map(|param| extract_plain_text(&param.value))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "Document".to_string());
                let namespace = parse_namespace(&namespace_str);

                info.includes.push(IncludeInfo {
                    title,
                    namespace,
                    parameters: inc.parameters.clone(),
                });
            }
        }
        SevenMarkElement::MediaElement(media) => {
            if let Some(url_param) = media.parameters.get("url") {
                let url = extract_plain_text(&url_param.value);
                if !url.is_empty() {
                    info.media.insert(url);
                }
            }
            if let Some(file_param) = media.parameters.get("file") {
                let file = extract_plain_text(&file_param.value);
                if !file.is_empty() {
                    info.media.insert(file);
                }
            }
        }
        SevenMarkElement::Category(cat) if collect_categories_redirect => {
            let name = extract_plain_text(&cat.content);
            if !name.is_empty() {
                info.categories.insert(name);
            }
        }
        SevenMarkElement::Redirect(redir) if collect_categories_redirect => {
            let target = extract_plain_text(&redir.content);
            if !target.is_empty() {
                info.redirect = Some(target);
            }
        }
        _ => {}
    }

    let mut children = Vec::new();
    element.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &mut children {
        collect_info_recursive(child, info, collect_categories_redirect);
    }
}

fn substitute_includes(
    elements: &mut [SevenMarkElement],
    resolved_includes: &HashMap<String, ResolvedDocument>,
) {
    for element in elements {
        substitute_includes_recursive(element, resolved_includes);
    }
}

fn substitute_includes_recursive(
    element: &mut SevenMarkElement,
    resolved_includes: &HashMap<String, ResolvedDocument>,
) {
    match element {
        SevenMarkElement::Include(inc) => {
            let title = extract_plain_text(&inc.content);
            if !title.is_empty() {
                let hash_key = make_include_key(&title, &inc.parameters);

                if let Some(resolved) = resolved_includes.get(&hash_key) {
                    inc.content = resolved.ast.clone();
                    return;
                }
            }
            for child in &mut inc.content {
                substitute_includes_recursive(child, resolved_includes);
            }
        }
        _ => {
            element.traverse_children(&mut |child| {
                substitute_includes_recursive(child, resolved_includes);
            });
        }
    }
}

fn extract_plain_text(elements: &[SevenMarkElement]) -> String {
    elements
        .iter()
        .filter_map(|element| match element {
            SevenMarkElement::Text(text_element) => Some(text_element.content.as_str()),
            SevenMarkElement::Escape(escape_element) => Some(escape_element.content.as_str()),
            _ => None,
        })
        .collect::<String>()
}

fn parse_namespace(namespace: &str) -> DocumentNamespace {
    match namespace {
        "Document" => DocumentNamespace::Document,
        "User" => DocumentNamespace::User,
        "Template" => DocumentNamespace::Template,
        "File" => DocumentNamespace::File,
        "Category" => DocumentNamespace::Category,
        "Wiki" => DocumentNamespace::Wiki,
        _ => DocumentNamespace::Document,
    }
}

fn namespace_to_string(namespace: &DocumentNamespace) -> &'static str {
    match namespace {
        DocumentNamespace::Document => "Document",
        DocumentNamespace::User => "User",
        DocumentNamespace::Template => "Template",
        DocumentNamespace::File => "File",
        DocumentNamespace::Category => "Category",
        DocumentNamespace::Wiki => "Wiki",
    }
}

fn make_include_key(title: &str, params: &Parameters) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(title.as_bytes());

    for (k, v) in params {
        hasher.update(k.as_bytes());
        hasher.update(extract_plain_text(&v.value).as_bytes());
    }

    hasher.finalize().to_hex().to_string()
}
