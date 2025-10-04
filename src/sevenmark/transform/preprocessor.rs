use crate::SevenMarkElement;
use crate::sevenmark::core::parse_document;
use crate::sevenmark::transform::wiki::{DocumentNamespace, WikiClient};
use crate::sevenmark::{Location, TextElement, Traversable};
use anyhow::Result;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tracing::{debug, warn};

/// Final result after include resolution
#[derive(Debug, Clone, Serialize)]
pub struct ProcessedDocument {
    pub media: HashSet<String>,
    pub categories: HashSet<String>,
    pub redirect: Option<String>,
    pub ast: Vec<SevenMarkElement>,
}

/// Processes document with 1-depth include resolution
pub async fn preprocess_sevenmark(
    mut ast: Vec<SevenMarkElement>,
    wiki_client: &WikiClient,
) -> Result<ProcessedDocument> {
    // Substitute variables in main document
    let mut main_params = HashMap::new();
    substitute_variables(&mut ast, &mut main_params);

    // Collect metadata from main document
    let mut categories = HashSet::new();
    let mut redirect = None;
    let mut all_media = HashSet::new();

    collect_metadata(&ast, &mut categories, &mut redirect, &mut all_media, true);

    // Collect unique includes (namespace:title)
    let mut includes_to_fetch = HashSet::new();
    collect_includes(&ast, &mut includes_to_fetch);

    if includes_to_fetch.is_empty() {
        return Ok(ProcessedDocument {
            ast,
            media: all_media,
            categories,
            redirect,
        });
    }

    // Prepare batch fetch requests
    let requests: Vec<_> = includes_to_fetch.into_iter().collect();

    debug!("Fetching {} unique documents", requests.len());

    // Fetch all documents
    let fetched_docs = wiki_client.fetch_documents_batch(requests).await?;

    // Parse fetched documents and store in map
    let mut docs_map: HashMap<String, Vec<SevenMarkElement>> = HashMap::new();

    for doc in fetched_docs {
        let doc_key = format!("{}:{}", namespace_to_string(&doc.namespace), doc.title);
        let parsed_ast = parse_document(&doc.current_revision.content);
        docs_map.insert(doc_key, parsed_ast);
    }

    // Substitute includes with their content
    substitute_includes(&mut ast, &docs_map, &mut all_media);

    Ok(ProcessedDocument {
        ast,
        media: all_media,
        categories,
        redirect,
    })
}

fn substitute_variables(elements: &mut [SevenMarkElement], params: &mut HashMap<String, String>) {
    for element in elements {
        substitute_variables_recursive(element, params);
    }
}

fn substitute_variables_recursive(
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

    element.traverse_children(&mut |child| {
        substitute_variables_recursive(child, params);
    });
}

fn collect_metadata(
    elements: &[SevenMarkElement],
    categories: &mut HashSet<String>,
    redirect: &mut Option<String>,
    media: &mut HashSet<String>,
    collect_categories_redirect: bool,
) {
    for element in elements {
        collect_metadata_recursive(
            element,
            categories,
            redirect,
            media,
            collect_categories_redirect,
        );
    }
}

fn collect_metadata_recursive(
    element: &SevenMarkElement,
    categories: &mut HashSet<String>,
    redirect: &mut Option<String>,
    media: &mut HashSet<String>,
    collect_categories_redirect: bool,
) {
    match element {
        SevenMarkElement::MediaElement(m) => {
            if let Some(url_param) = m.parameters.get("url") {
                let url = extract_plain_text(&url_param.value);
                if !url.is_empty() {
                    media.insert(url);
                }
            }
            if let Some(file_param) = m.parameters.get("file") {
                let file = extract_plain_text(&file_param.value);
                if !file.is_empty() {
                    media.insert(file);
                }
            }
        }
        SevenMarkElement::Category(cat) if collect_categories_redirect => {
            let name = extract_plain_text(&cat.content);
            if !name.is_empty() {
                categories.insert(name);
            }
        }
        SevenMarkElement::Redirect(redir) if collect_categories_redirect => {
            let target = extract_plain_text(&redir.content);
            if !target.is_empty() && redirect.is_none() {
                *redirect = Some(target);
            }
        }
        _ => {}
    }

    let mut children = Vec::new();
    let mut element_clone = element.clone();
    element_clone.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &children {
        collect_metadata_recursive(
            child,
            categories,
            redirect,
            media,
            collect_categories_redirect,
        );
    }
}

fn collect_includes(
    elements: &[SevenMarkElement],
    includes: &mut HashSet<(DocumentNamespace, String)>,
) {
    for element in elements {
        collect_includes_recursive(element, includes);
    }
}

fn collect_includes_recursive(
    element: &SevenMarkElement,
    includes: &mut HashSet<(DocumentNamespace, String)>,
) {
    if let SevenMarkElement::Include(inc) = element {
        let title = extract_plain_text(&inc.content);
        if !title.is_empty() {
            let namespace_str = inc
                .parameters
                .get("namespace")
                .map(|param| extract_plain_text(&param.value))
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Document".to_string());
            let namespace = parse_namespace(&namespace_str);
            includes.insert((namespace, title));
        }
    }

    let mut children = Vec::new();
    let mut element_clone = element.clone();
    element_clone.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &children {
        collect_includes_recursive(child, includes);
    }
}

fn substitute_includes(
    elements: &mut [SevenMarkElement],
    docs_map: &HashMap<String, Vec<SevenMarkElement>>,
    all_media: &mut HashSet<String>,
) {
    for element in elements {
        substitute_includes_recursive(element, docs_map, all_media);
    }
}

fn substitute_includes_recursive(
    element: &mut SevenMarkElement,
    docs_map: &HashMap<String, Vec<SevenMarkElement>>,
    all_media: &mut HashSet<String>,
) {
    if let SevenMarkElement::Include(inc) = element {
        let title = extract_plain_text(&inc.content);
        if !title.is_empty() {
            let namespace_str = inc
                .parameters
                .get("namespace")
                .map(|param| extract_plain_text(&param.value))
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Document".to_string());
            let namespace = parse_namespace(&namespace_str);
            let doc_key = format!("{}:{}", namespace_to_string(&namespace), title);

            if let Some(base_ast) = docs_map.get(&doc_key) {
                // Clone the document AST
                let mut included_ast = base_ast.clone();

                // Create parameter map from include parameters (excluding namespace)
                let mut params_map: HashMap<String, String> = inc
                    .parameters
                    .iter()
                    .filter(|(k, _)| k.as_str() != "namespace")
                    .map(|(k, v)| (k.clone(), extract_plain_text(&v.value)))
                    .collect();

                // Substitute variables (include parameters have priority)
                substitute_variables(&mut included_ast, &mut params_map);

                // Collect media from included document
                let mut categories = HashSet::new();
                let mut redirect = None;
                collect_metadata(
                    &included_ast,
                    &mut categories,
                    &mut redirect,
                    all_media,
                    false,
                );

                // Replace include content
                inc.content = included_ast;
                return;
            } else {
                warn!("Include target not found: {}", doc_key);
            }
        }
    }

    // Traverse children
    element.traverse_children(&mut |child| {
        substitute_includes_recursive(child, docs_map, all_media);
    });
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
        "File" => DocumentNamespace::File,
        "Category" => DocumentNamespace::Category,
        _ => DocumentNamespace::Document,
    }
}

fn namespace_to_string(namespace: &DocumentNamespace) -> &'static str {
    match namespace {
        DocumentNamespace::Document => "Document",
        DocumentNamespace::File => "File",
        DocumentNamespace::Category => "Category",
    }
}
