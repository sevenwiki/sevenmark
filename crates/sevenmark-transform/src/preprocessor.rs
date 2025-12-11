use crate::expression_evaluator::evaluate_condition;
use crate::utils::extract_plain_text;
use crate::wiki::{DocumentNamespace, fetch_documents_batch};
use anyhow::Result;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use sevenmark_parser::ast::{
    ListContentItem, Location, SevenMarkElement, TableCellItem, TableRowItem, TextElement,
    Traversable,
};
use sevenmark_parser::core::parse_document;
use std::collections::{HashMap, HashSet};
use tracing::{debug, warn};

/// Media reference with namespace and title
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct MediaReference {
    pub namespace: DocumentNamespace,
    pub title: String,
}

/// Final result after include resolution
#[derive(Debug, Clone, Serialize)]
pub struct PreProcessedDocument {
    pub media: HashSet<MediaReference>,
    pub categories: HashSet<String>,
    pub redirect: Option<String>,
    pub references: HashSet<(DocumentNamespace, String)>,
    pub ast: Vec<SevenMarkElement>,
}

/// Processes document with 1-depth include resolution
pub async fn preprocess_sevenmark(
    mut ast: Vec<SevenMarkElement>,
    db: &DatabaseConnection,
) -> Result<PreProcessedDocument> {
    // Substitute variables in main document
    let mut main_params = HashMap::new();
    substitute_variables(&mut ast, &mut main_params);

    // Process if conditionals
    process_if_elements(&mut ast, &main_params);

    // Collect metadata from main document
    let mut categories = HashSet::new();
    let mut redirect = None;
    let mut all_media = HashSet::new();

    collect_metadata(&ast, &mut categories, &mut redirect, &mut all_media, true);

    // Collect unique includes for fetching (only Include elements need content fetching)
    let mut includes_to_fetch = HashSet::new();
    collect_includes(&ast, &mut includes_to_fetch);

    if !includes_to_fetch.is_empty() {
        // Prepare batch fetch requests
        let requests: Vec<_> = includes_to_fetch.into_iter().collect();

        debug!("Fetching {} unique documents", requests.len());

        // Fetch all documents
        let fetched_docs = fetch_documents_batch(db, requests).await?;

        // Parse fetched documents and store in map
        let mut docs_map: HashMap<String, Vec<SevenMarkElement>> = HashMap::new();

        for doc in fetched_docs {
            let doc_key = format!("{}:{}", namespace_to_string(&doc.namespace), doc.title);
            let parsed_ast = parse_document(&doc.current_revision.content);
            docs_map.insert(doc_key, parsed_ast);
        }

        // Substitute includes with their content
        substitute_includes(&mut ast, &docs_map, &mut all_media);
    }

    // Collect all references from final AST (after include substitution)
    // This captures references from both main document and included documents
    let mut all_references = HashSet::new();
    collect_references(&ast, &mut all_references);

    Ok(PreProcessedDocument {
        ast,
        media: all_media,
        categories,
        redirect,
        references: all_references,
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
                params.insert(key.clone(), value);
            }
        }
    }

    if let SevenMarkElement::Variable(var) = element
        && let Some(value) = params.get(&var.content)
    {
        *element = SevenMarkElement::Text(TextElement {
            location: Location::synthesized(),
            content: value.clone(),
        });
        return;
    }

    element.traverse_children(&mut |child| {
        substitute_variables_recursive(child, params);
    });
}

fn collect_metadata(
    elements: &[SevenMarkElement],
    categories: &mut HashSet<String>,
    redirect: &mut Option<String>,
    media: &mut HashSet<MediaReference>,
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
    media: &mut HashSet<MediaReference>,
    collect_categories_redirect: bool,
) {
    match element {
        SevenMarkElement::MediaElement(m) => {
            // Collect #file parameter
            if let Some(file_param) = m.parameters.get("file") {
                let title = extract_plain_text(&file_param.value);
                if !title.is_empty() {
                    media.insert(MediaReference {
                        namespace: DocumentNamespace::File,
                        title,
                    });
                }
            }
            // Collect #document parameter
            if let Some(doc_param) = m.parameters.get("document") {
                let title = extract_plain_text(&doc_param.value);
                if !title.is_empty() {
                    media.insert(MediaReference {
                        namespace: DocumentNamespace::Document,
                        title,
                    });
                }
            }
            // Collect #category parameter
            if let Some(cat_param) = m.parameters.get("category") {
                let title = extract_plain_text(&cat_param.value);
                if !title.is_empty() {
                    media.insert(MediaReference {
                        namespace: DocumentNamespace::Category,
                        title,
                    });
                }
            }
            // #url parameter is ignored (already a complete URL, no need to fetch)
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

    element.traverse_children_ref(&mut |child| {
        collect_metadata_recursive(
            child,
            categories,
            redirect,
            media,
            collect_categories_redirect,
        );
    });
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

    element.traverse_children_ref(&mut |child| {
        collect_includes_recursive(child, includes);
    });
}

/// Collect all document references from AST
/// This should be called after substitute_includes() to capture references from included documents
fn collect_references(
    elements: &[SevenMarkElement],
    references: &mut HashSet<(DocumentNamespace, String)>,
) {
    for element in elements {
        collect_references_recursive(element, references);
    }
}

fn collect_references_recursive(
    element: &SevenMarkElement,
    references: &mut HashSet<(DocumentNamespace, String)>,
) {
    match element {
        // {{{#include}}} 요소
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
                references.insert((namespace, title));
            }
        }
        // {{{#category}}} 요소
        SevenMarkElement::Category(cat) => {
            let name = extract_plain_text(&cat.content);
            if !name.is_empty() {
                references.insert((DocumentNamespace::Category, name));
            }
        }
        // MediaElement의 file/document/category 파라미터
        SevenMarkElement::MediaElement(m) => {
            if let Some(file_param) = m.parameters.get("file") {
                let title = extract_plain_text(&file_param.value);
                if !title.is_empty() {
                    references.insert((DocumentNamespace::File, title));
                }
            }
            if let Some(doc_param) = m.parameters.get("document") {
                let title = extract_plain_text(&doc_param.value);
                if !title.is_empty() {
                    references.insert((DocumentNamespace::Document, title));
                }
            }
            if let Some(cat_param) = m.parameters.get("category") {
                let title = extract_plain_text(&cat_param.value);
                if !title.is_empty() {
                    references.insert((DocumentNamespace::Category, title));
                }
            }
        }
        _ => {}
    }

    element.traverse_children_ref(&mut |child| {
        collect_references_recursive(child, references);
    });
}

fn substitute_includes(
    elements: &mut [SevenMarkElement],
    docs_map: &HashMap<String, Vec<SevenMarkElement>>,
    all_media: &mut HashSet<MediaReference>,
) {
    for element in elements {
        substitute_includes_recursive(element, docs_map, all_media);
    }
}

fn substitute_includes_recursive(
    element: &mut SevenMarkElement,
    docs_map: &HashMap<String, Vec<SevenMarkElement>>,
    all_media: &mut HashSet<MediaReference>,
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

/// If 요소를 조건 평가 후 처리 (조건 true: 내용 전개, false: 제거)
fn process_if_elements(elements: &mut Vec<SevenMarkElement>, variables: &HashMap<String, String>) {
    let mut i = 0;
    while i < elements.len() {
        // 먼저 자식 요소들의 If 처리
        elements[i].for_each_content_vec(&mut |vec| {
            process_if_elements(vec, variables);
        });

        // TableElement 내부의 조건부 처리
        if let SevenMarkElement::TableElement(table) = &mut elements[i] {
            process_table_conditionals(&mut table.content, variables);
        }

        // ListElement 내부의 조건부 처리
        if let SevenMarkElement::ListElement(list) = &mut elements[i] {
            process_list_conditionals(&mut list.content, variables);
        }

        // 현재 요소가 IfElement인 경우 처리
        if let SevenMarkElement::IfElement(if_elem) = &elements[i] {
            if evaluate_condition(&if_elem.condition, variables) {
                // 조건이 true: 내용으로 대체
                let content = if_elem.content.clone();
                elements.splice(i..i + 1, content);
                // splice 후 새로 삽입된 요소들도 재처리 필요 없음 (이미 자식 처리됨)
                // 다음 요소로 이동하지 않고 같은 인덱스 유지 (새 요소 확인)
            } else {
                // 조건이 false: 제거
                elements.remove(i);
            }
            // i를 증가시키지 않음 - splice/remove 후 다음 요소가 현재 위치에 있음
            continue;
        }
        i += 1;
    }
}

/// TableElement 내부의 행/셀 레벨 조건부 처리
fn process_table_conditionals(rows: &mut Vec<TableRowItem>, variables: &HashMap<String, String>) {
    let mut i = 0;
    while i < rows.len() {
        match &mut rows[i] {
            TableRowItem::Row(row) => {
                // 행 내부의 셀 레벨 조건부 처리
                process_table_cell_conditionals(&mut row.inner_content, variables);
                i += 1;
            }
            TableRowItem::Conditional {
                condition,
                rows: cond_rows,
                ..
            } => {
                if evaluate_condition(condition, variables) {
                    // 조건이 true: rows를 펼침
                    // 먼저 펼쳐질 rows 내부의 셀 조건부도 처리
                    for row in cond_rows.iter_mut() {
                        process_table_cell_conditionals(&mut row.inner_content, variables);
                    }
                    let expanded: Vec<TableRowItem> =
                        cond_rows.drain(..).map(TableRowItem::Row).collect();
                    rows.splice(i..i + 1, expanded);
                    // 다음 반복에서 새로 삽입된 요소 확인
                } else {
                    // 조건이 false: 제거
                    rows.remove(i);
                }
                // i를 증가시키지 않음
            }
        }
    }
}

/// 테이블 셀 레벨 조건부 처리
fn process_table_cell_conditionals(
    cells: &mut Vec<TableCellItem>,
    variables: &HashMap<String, String>,
) {
    let mut i = 0;
    while i < cells.len() {
        match &mut cells[i] {
            TableCellItem::Cell(cell) => {
                // 셀 내부의 일반 IfElement 처리
                process_if_elements(&mut cell.content, variables);
                i += 1;
            }
            TableCellItem::Conditional {
                condition,
                cells: cond_cells,
                ..
            } => {
                if evaluate_condition(condition, variables) {
                    // 조건이 true: cells를 펼침
                    // 먼저 펼쳐질 cells 내부의 IfElement도 처리
                    for cell in cond_cells.iter_mut() {
                        process_if_elements(&mut cell.content, variables);
                    }
                    let expanded: Vec<TableCellItem> =
                        cond_cells.drain(..).map(TableCellItem::Cell).collect();
                    cells.splice(i..i + 1, expanded);
                    // 다음 반복에서 새로 삽입된 요소 확인
                } else {
                    // 조건이 false: 제거
                    cells.remove(i);
                }
                // i를 증가시키지 않음
            }
        }
    }
}

/// ListElement 내부의 아이템 레벨 조건부 처리
fn process_list_conditionals(
    items: &mut Vec<ListContentItem>,
    variables: &HashMap<String, String>,
) {
    let mut i = 0;
    while i < items.len() {
        match &mut items[i] {
            ListContentItem::Item(item) => {
                // 아이템 내부의 일반 IfElement 처리
                process_if_elements(&mut item.content, variables);
                i += 1;
            }
            ListContentItem::Conditional {
                condition,
                items: cond_items,
                ..
            } => {
                if evaluate_condition(condition, variables) {
                    // 조건이 true: items를 펼침
                    // 먼저 펼쳐질 items 내부의 IfElement도 처리
                    for item in cond_items.iter_mut() {
                        process_if_elements(&mut item.content, variables);
                    }
                    let expanded: Vec<ListContentItem> =
                        cond_items.drain(..).map(ListContentItem::Item).collect();
                    items.splice(i..i + 1, expanded);
                    // 다음 반복에서 새로 삽입된 요소 확인
                } else {
                    // 조건이 false: 제거
                    items.remove(i);
                }
                // i를 증가시키지 않음
            }
        }
    }
}
