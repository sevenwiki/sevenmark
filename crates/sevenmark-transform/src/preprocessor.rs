use crate::expression_evaluator::evaluate_condition;
use crate::utils::extract_plain_text;
use crate::wiki::{DocumentNamespace, fetch_documents_batch};
use anyhow::Result;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use sevenmark_parser::ast::{
    ListContentItem, Location, MentionType, SevenMarkElement, TableCellItem, TableRowItem,
    TextElement, Traversable,
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

/// Section range information for frontend consumption
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize)]
pub struct SectionInfo {
    /// Section index (same as Header's section_index)
    pub section_index: usize,
    /// Header level (1-6)
    pub level: usize,
    /// Section start byte offset (header start position)
    pub start: usize,
    /// Section end byte offset (next same/higher level header or document end)
    pub end: usize,
}

/// Redirect reference with namespace and title
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct RedirectReference {
    pub namespace: DocumentNamespace,
    pub title: String,
}

/// Document reference with namespace and title
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct DocumentReference {
    pub namespace: DocumentNamespace,
    pub title: String,
}

/// Final result after include resolution
#[derive(Debug, Clone, Serialize)]
pub struct PreProcessedDocument {
    pub media: HashSet<MediaReference>,
    pub categories: HashSet<String>,
    pub redirect: Option<RedirectReference>,
    pub references: HashSet<DocumentReference>,
    /// User mention UUIDs collected from the document
    pub user_mentions: HashSet<String>,
    pub ast: Vec<SevenMarkElement>,
    pub sections: Vec<SectionInfo>,
}

/// Processes document with 1-depth include resolution
pub async fn preprocess_sevenmark(
    mut ast: Vec<SevenMarkElement>,
    db: &DatabaseConnection,
) -> Result<PreProcessedDocument> {
    // Process defines and ifs in document order (single pass)
    let mut variables = HashMap::new();
    process_defines_and_ifs(&mut ast, &mut variables);

    // Collect metadata from main document
    let mut categories = HashSet::new();
    let mut redirect = None;
    let mut all_media = HashSet::new();
    let mut sections = Vec::new();
    let mut user_mentions = HashSet::new();

    collect_metadata(
        &ast,
        &mut categories,
        &mut redirect,
        &mut all_media,
        &mut sections,
        &mut user_mentions,
        true,
    );

    // Collect unique includes for fetching (only Include elements need content fetching)
    // This also serves as references from the main document (before substitution overwrites content)
    let mut includes_to_fetch = HashSet::new();
    collect_includes(&ast, &mut includes_to_fetch);

    if !includes_to_fetch.is_empty() {
        // Prepare batch fetch requests (convert DocumentReference to tuple for API)
        let requests: Vec<_> = includes_to_fetch
            .iter()
            .map(|r| (r.namespace.clone(), r.title.clone()))
            .collect();

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
    // Start with includes_to_fetch (main document's direct includes, collected before substitution)
    // Then add references from included content (2+ depth includes, categories, media from included docs)
    let mut all_references = includes_to_fetch;
    collect_references(&ast, &mut all_references);

    Ok(PreProcessedDocument {
        ast,
        media: all_media,
        categories,
        redirect,
        references: all_references,
        user_mentions,
        sections,
    })
}

/// Define과 If를 문서 순서대로 처리 (single pass, in-place)
/// - Define을 만나면 변수 등록
/// - Variable을 만나면 치환
/// - If를 만나면 조건 평가 후 전개/제거
fn process_defines_and_ifs(
    elements: &mut Vec<SevenMarkElement>,
    variables: &mut HashMap<String, String>,
) {
    let mut i = 0;
    while i < elements.len() {
        // 1. DefineElement: 변수 등록
        if let SevenMarkElement::DefineElement(def) = &elements[i] {
            for (key, param) in &def.parameters {
                let value = extract_plain_text(&param.value);
                if !value.is_empty() {
                    variables.insert(key.clone(), value);
                }
            }
            i += 1;
            continue;
        }

        // 2. Variable: 치환
        if let SevenMarkElement::Variable(var) = &elements[i] {
            if let Some(value) = variables.get(&var.content) {
                elements[i] = SevenMarkElement::Text(TextElement {
                    location: Location::synthesized(),
                    content: value.clone(),
                });
            }
            i += 1;
            continue;
        }

        // 3. IfElement: 조건 평가 후 전개/제거
        if let SevenMarkElement::IfElement(if_elem) = &elements[i] {
            if evaluate_condition(&if_elem.condition, variables) {
                // 조건 true: 내용으로 대체 후 같은 위치부터 재처리
                let content = if_elem.content.clone();
                elements.splice(i..i + 1, content);
            } else {
                // 조건 false: 제거
                elements.remove(i);
            }
            continue;
        }

        // 4. TableElement: 테이블 내부 조건부 처리
        if let SevenMarkElement::TableElement(table) = &mut elements[i] {
            process_table_conditionals(&mut table.content, variables);
            i += 1;
            continue;
        }

        // 5. ListElement: 리스트 내부 조건부 처리
        if let SevenMarkElement::ListElement(list) = &mut elements[i] {
            process_list_conditionals(&mut list.content, variables);
            i += 1;
            continue;
        }

        // 6. 기타 요소: 자식 재귀 처리
        elements[i].for_each_content_vec(&mut |vec| {
            process_defines_and_ifs(vec, variables);
        });

        i += 1;
    }
}

fn collect_metadata(
    elements: &[SevenMarkElement],
    categories: &mut HashSet<String>,
    redirect: &mut Option<RedirectReference>,
    media: &mut HashSet<MediaReference>,
    sections: &mut Vec<SectionInfo>,
    user_mentions: &mut HashSet<String>,
    collect_categories_redirect: bool,
) {
    let mut section_stack: Vec<SectionInfo> = Vec::new();
    let mut max_end: usize = 0;

    for element in elements {
        collect_metadata_recursive(
            element,
            categories,
            redirect,
            media,
            sections,
            user_mentions,
            &mut section_stack,
            &mut max_end,
            collect_categories_redirect,
        );
    }

    // Remaining headers in stack end at document end
    for mut section in section_stack {
        section.end = max_end;
        sections.push(section);
    }
}

fn collect_metadata_recursive(
    element: &SevenMarkElement,
    categories: &mut HashSet<String>,
    redirect: &mut Option<RedirectReference>,
    media: &mut HashSet<MediaReference>,
    sections: &mut Vec<SectionInfo>,
    user_mentions: &mut HashSet<String>,
    section_stack: &mut Vec<SectionInfo>,
    max_end: &mut usize,
    collect_categories_redirect: bool,
) {
    // Track max location.end for document length
    if let Some(loc) = element.location()
        && loc.end > *max_end
    {
        *max_end = loc.end;
    }

    match element {
        SevenMarkElement::Header(header) => {
            let start = header.location.start;
            let level = header.level;

            // Pop headers with level >= current (same or lower priority)
            while let Some(mut section) = section_stack.pop() {
                if section.level >= level {
                    section.end = start;
                    sections.push(section);
                } else {
                    section_stack.push(section);
                    break;
                }
            }

            section_stack.push(SectionInfo {
                section_index: header.section_index,
                level,
                start,
                end: 0,
            });
        }
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
            let title = extract_plain_text(&redir.content);
            if !title.is_empty() && redirect.is_none() {
                // Read namespace from parameters (same as Include)
                let namespace_str = redir
                    .parameters
                    .get("namespace")
                    .map(|param| extract_plain_text(&param.value))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "Document".to_string());
                let namespace = parse_namespace(&namespace_str);
                *redirect = Some(RedirectReference { namespace, title });
            }
        }
        SevenMarkElement::Mention(mention) if mention.mention_type == MentionType::User => {
            user_mentions.insert(mention.uuid.clone());
        }
        _ => {}
    }

    element.traverse_children_ref(&mut |child| {
        collect_metadata_recursive(
            child,
            categories,
            redirect,
            media,
            sections,
            user_mentions,
            section_stack,
            max_end,
            collect_categories_redirect,
        );
    });
}

fn collect_includes(elements: &[SevenMarkElement], includes: &mut HashSet<DocumentReference>) {
    for element in elements {
        collect_includes_recursive(element, includes);
    }
}

fn collect_includes_recursive(
    element: &SevenMarkElement,
    includes: &mut HashSet<DocumentReference>,
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
            includes.insert(DocumentReference { namespace, title });
        }
    }

    element.traverse_children_ref(&mut |child| {
        collect_includes_recursive(child, includes);
    });
}

/// Collect all document references from AST
/// This should be called after substitute_includes() to capture references from included documents
fn collect_references(elements: &[SevenMarkElement], references: &mut HashSet<DocumentReference>) {
    for element in elements {
        collect_references_recursive(element, references);
    }
}

fn collect_references_recursive(
    element: &SevenMarkElement,
    references: &mut HashSet<DocumentReference>,
) {
    match element {
        // {{{#category}}} 요소
        SevenMarkElement::Category(cat) => {
            let name = extract_plain_text(&cat.content);
            if !name.is_empty() {
                references.insert(DocumentReference {
                    namespace: DocumentNamespace::Category,
                    title: name,
                });
            }
        }
        // MediaElement의 file/document/category 파라미터
        SevenMarkElement::MediaElement(m) => {
            if let Some(file_param) = m.parameters.get("file") {
                let title = extract_plain_text(&file_param.value);
                if !title.is_empty() {
                    references.insert(DocumentReference {
                        namespace: DocumentNamespace::File,
                        title,
                    });
                }
            }
            if let Some(doc_param) = m.parameters.get("document") {
                let title = extract_plain_text(&doc_param.value);
                if !title.is_empty() {
                    references.insert(DocumentReference {
                        namespace: DocumentNamespace::Document,
                        title,
                    });
                }
            }
            if let Some(cat_param) = m.parameters.get("category") {
                let title = extract_plain_text(&cat_param.value);
                if !title.is_empty() {
                    references.insert(DocumentReference {
                        namespace: DocumentNamespace::Category,
                        title,
                    });
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

                // Process defines and ifs (include parameters have priority)
                process_defines_and_ifs(&mut included_ast, &mut params_map);

                // Collect media from included document (sections and user_mentions ignored for includes)
                let mut categories = HashSet::new();
                let mut redirect = None;
                let mut ignored_sections = Vec::new();
                let mut ignored_user_mentions = HashSet::new();
                collect_metadata(
                    &included_ast,
                    &mut categories,
                    &mut redirect,
                    all_media,
                    &mut ignored_sections,
                    &mut ignored_user_mentions,
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

/// TableElement 내부의 행/셀 레벨 조건부 처리
fn process_table_conditionals(
    rows: &mut Vec<TableRowItem>,
    variables: &mut HashMap<String, String>,
) {
    let mut i = 0;
    while i < rows.len() {
        match &mut rows[i] {
            TableRowItem::Row(row) => {
                // 행 내부의 셀 레벨 조건부 처리
                process_table_cell_conditionals(&mut row.content, variables);
                i += 1;
            }
            TableRowItem::Conditional {
                condition,
                rows: cond_rows,
                ..
            } => {
                if evaluate_condition(condition, variables) {
                    // 조건이 true: rows를 펼침 (처리는 펼친 후 루프에서)
                    let expanded: Vec<TableRowItem> =
                        cond_rows.drain(..).map(TableRowItem::Row).collect();
                    rows.splice(i..i + 1, expanded);
                    // i 유지 → 다음 반복에서 Row로 처리됨
                } else {
                    // 조건이 false: 제거
                    rows.remove(i);
                }
            }
        }
    }
}

/// 테이블 셀 레벨 조건부 처리
fn process_table_cell_conditionals(
    cells: &mut Vec<TableCellItem>,
    variables: &mut HashMap<String, String>,
) {
    let mut i = 0;
    while i < cells.len() {
        match &mut cells[i] {
            TableCellItem::Cell(cell) => {
                // 셀 내부 처리 (define/if 포함)
                process_defines_and_ifs(&mut cell.content, variables);
                i += 1;
            }
            TableCellItem::Conditional {
                condition,
                cells: cond_cells,
                ..
            } => {
                if evaluate_condition(condition, variables) {
                    // 조건이 true: cells를 펼침 (처리는 펼친 후 루프에서)
                    let expanded: Vec<TableCellItem> =
                        cond_cells.drain(..).map(TableCellItem::Cell).collect();
                    cells.splice(i..i + 1, expanded);
                    // i 유지 → 다음 반복에서 Cell로 처리됨
                } else {
                    // 조건이 false: 제거
                    cells.remove(i);
                }
            }
        }
    }
}

/// ListElement 내부의 아이템 레벨 조건부 처리
fn process_list_conditionals(
    items: &mut Vec<ListContentItem>,
    variables: &mut HashMap<String, String>,
) {
    let mut i = 0;
    while i < items.len() {
        match &mut items[i] {
            ListContentItem::Item(item) => {
                // 아이템 내부 처리 (define/if 포함)
                process_defines_and_ifs(&mut item.content, variables);
                i += 1;
            }
            ListContentItem::Conditional {
                condition,
                items: cond_items,
                ..
            } => {
                if evaluate_condition(condition, variables) {
                    // 조건이 true: items를 펼침 (처리는 펼친 후 루프에서)
                    let expanded: Vec<ListContentItem> =
                        cond_items.drain(..).map(ListContentItem::Item).collect();
                    items.splice(i..i + 1, expanded);
                    // i 유지 → 다음 반복에서 Item으로 처리됨
                } else {
                    // 조건이 false: 제거
                    items.remove(i);
                }
            }
        }
    }
}
