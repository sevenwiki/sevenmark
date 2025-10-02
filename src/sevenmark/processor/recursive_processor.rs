use crate::SevenMarkElement;
use crate::sevenmark::core::parse_document;
use crate::sevenmark::processor::wiki::{DocumentNamespace, WikiClient};
use crate::sevenmark::{Location, TextElement, Traversable};
use anyhow::{Context, Result};
use async_recursion::async_recursion;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tracing::{debug, warn};

const MAX_INCLUDE_DEPTH: usize = 16;

/// 처리 완료된 문서 (재귀 처리 결과)
#[derive(Debug, Clone, Serialize)]
pub struct ProcessedDocument {
    /// 최종 AST (모든 include가 치환됨)
    pub ast: Vec<SevenMarkElement>,
    /// 모든 media 파일 목록 (중첩된 include 포함)
    pub media: HashSet<String>,
    /// 모든 category 목록
    pub categories: HashSet<String>,
    /// Redirect 대상 (있으면)
    pub redirect: Option<String>,
}

/// 재귀 처리 중간 결과
#[derive(Debug, Clone)]
struct ResolvedDocument {
    ast: Vec<SevenMarkElement>,
    media: HashSet<String>,
    categories: HashSet<String>,
    redirect: Option<String>,
}

/// 문서를 재귀적으로 처리 (진입점)
///
/// # Arguments
/// * `namespace` - 문서의 namespace
/// * `title` - 문서의 title
/// * `input` - 처리할 SevenMark 원본 텍스트
/// * `wiki_client` - Wiki 백엔드 클라이언트
///
/// # Returns
/// 모든 include가 치환되고, media/category/redirect가 수집된 최종 문서
pub async fn process_document_recursive(
    namespace: DocumentNamespace,
    title: String,
    input: &str,
    wiki_client: &WikiClient,
) -> Result<ProcessedDocument> {
    let mut visited = HashSet::new();
    let parent_params = HashMap::new();

    // 최초 문서를 visited에 추가 (순환 참조 방지)
    let initial_key = format!("{}:{}", namespace_to_string(&namespace), title);
    visited.insert(initial_key.clone());
    println!("🚀 Starting recursive processing for: {}", initial_key);

    let resolved = resolve_document_recursive(
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

/// 문서를 재귀적으로 해결 (핵심 재귀 함수)
///
/// # Arguments
/// * `content` - SevenMark 원본 텍스트
/// * `parent_params` - 상위 문서에서 전달된 parameters (include의 #param="value")
/// * `depth` - 현재 재귀 깊이
/// * `max_depth` - 최대 재귀 깊이
/// * `visited` - 순환 참조 방지용 Set ("namespace:title")
/// * `wiki_client` - Wiki 클라이언트
#[async_recursion]
async fn resolve_document_recursive(
    content: &str,
    parent_params: &HashMap<String, String>,
    depth: usize,
    max_depth: usize,
    visited: &mut HashSet<String>,
    wiki_client: &WikiClient,
) -> Result<ResolvedDocument> {
    debug!("[Depth {}] Starting document resolution", depth);
    println!("[Depth {}] 📄 Starting document resolution", depth);

    // 1. 문서 파싱
    let mut ast = parse_document(content);
    println!(
        "[Depth {}]   ✓ Parsed document ({} elements)",
        depth,
        ast.len()
    );

    // 2. Define 수집 (현재 문서 스코프)
    let local_defines = collect_defines(&mut ast);
    debug!(
        "[Depth {}] Collected {} defines",
        depth,
        local_defines.len()
    );
    println!(
        "[Depth {}]   ✓ Collected {} defines",
        depth,
        local_defines.len()
    );
    if !local_defines.is_empty() {
        for (key, value) in &local_defines {
            println!("[Depth {}]     - {} = {:?}", depth, key, value);
        }
    }

    // 3. Variable 치환 (parent_params 우선, 없으면 local_defines)
    let mut all_params = local_defines.clone();
    all_params.extend(parent_params.clone()); // parent가 우선순위 높음
    substitute_variables(&mut ast, &all_params);
    println!(
        "[Depth {}]   ✓ Substituted variables ({} total params)",
        depth,
        all_params.len()
    );

    // 4. Include, Media, Category, Redirect 수집
    let mut info = CollectedInfo::default();
    collect_info(&mut ast, &mut info);
    debug!(
        "[Depth {}] Collected {} includes, {} media, {} categories",
        depth,
        info.includes.len(),
        info.media.len(),
        info.categories.len()
    );
    println!(
        "[Depth {}]   ✓ Collected {} includes, {} media, {} categories",
        depth,
        info.includes.len(),
        info.media.len(),
        info.categories.len()
    );

    // 5. Include가 없거나 depth 한계 도달 시 종료
    if info.includes.is_empty() {
        debug!("[Depth {}] No includes found, returning", depth);
        return Ok(ResolvedDocument {
            ast,
            media: info.media,
            categories: info.categories,
            redirect: info.redirect,
        });
    }

    if depth >= max_depth {
        warn!(
            "[Depth {}] Maximum depth reached, includes will not be resolved",
            depth
        );
        return Ok(ResolvedDocument {
            ast,
            media: info.media,
            categories: info.categories,
            redirect: info.redirect,
        });
    }

    // 6. 새로운 include 필터링 (순환 참조 방지)
    let new_includes: Vec<_> = info
        .includes
        .into_iter()
        .filter(|inc| {
            let key = format!("{}:{}", namespace_to_string(&inc.namespace), &inc.title);
            if visited.contains(&key) {
                warn!("[Depth {}] Circular reference detected: {}", depth, key);
                false
            } else {
                true
            }
        })
        .collect();

    if new_includes.is_empty() {
        debug!(
            "[Depth {}] All includes already visited (circular reference)",
            depth
        );
        return Ok(ResolvedDocument {
            ast,
            media: info.media,
            categories: info.categories,
            redirect: info.redirect,
        });
    }

    // 7. Batch API로 모든 include 문서 가져오기
    let requests: Vec<_> = new_includes
        .iter()
        .map(|inc| (inc.namespace.clone(), inc.title.clone()))
        .collect();

    debug!(
        "[Depth {}] Fetching {} includes via batch API",
        depth,
        requests.len()
    );
    println!(
        "[Depth {}]   🌐 Fetching {} includes via batch API...",
        depth,
        requests.len()
    );
    for (ns, title) in &requests {
        println!(
            "[Depth {}]     - {}:{}",
            depth,
            namespace_to_string(ns),
            title
        );
    }
    let fetched_docs = wiki_client.fetch_documents_batch(requests).await?;
    debug!("[Depth {}] Fetched {} documents", depth, fetched_docs.len());
    println!(
        "[Depth {}]   ✓ Fetched {} documents",
        depth,
        fetched_docs.len()
    );

    // 8. 각 Include를 재귀적으로 resolve
    let mut resolved_includes: HashMap<String, ResolvedDocument> = HashMap::new();

    println!(
        "[Depth {}]   🔄 Resolving {} includes recursively...",
        depth,
        new_includes.len()
    );
    for (include_info, doc) in new_includes.iter().zip(fetched_docs.iter()) {
        let key = format!(
            "{}:{}",
            namespace_to_string(&include_info.namespace),
            &include_info.title
        );

        // visited에 추가
        visited.insert(key.clone());

        println!("[Depth {}]     ⤷ Resolving: {}", depth, key);
        // 재귀 호출
        let resolved = resolve_document_recursive(
            &doc.current_revision.content,
            &include_info.parameters,
            depth + 1,
            max_depth,
            visited,
            wiki_client,
        )
        .await
        .with_context(|| format!("Failed to resolve include: {}", key))?;

        // 결과 저장
        resolved_includes.insert(key.clone(), resolved);
        debug!("[Depth {}] Resolved include: {}", depth, key);
        println!("[Depth {}]     ✓ Resolved: {}", depth, key);
    }

    // 9. AST에서 Include 요소를 resolved AST로 치환
    substitute_includes(&mut ast, &resolved_includes);
    println!(
        "[Depth {}]   ✓ Substituted {} includes in AST",
        depth,
        resolved_includes.len()
    );

    // 10. 모든 media/category를 누적
    let mut all_media = info.media;
    let mut all_categories = info.categories;

    for resolved in resolved_includes.values() {
        all_media.extend(resolved.media.clone());
        all_categories.extend(resolved.categories.clone());
    }

    debug!("[Depth {}] Document resolution complete", depth);
    println!(
        "[Depth {}] ✅ Document resolution complete (total media: {}, categories: {})",
        depth,
        all_media.len(),
        all_categories.len()
    );

    Ok(ResolvedDocument {
        ast,
        media: all_media,
        categories: all_categories,
        redirect: info.redirect,
    })
}

/// Include 정보
#[derive(Debug, Clone)]
struct IncludeInfo {
    title: String,
    namespace: DocumentNamespace,
    parameters: HashMap<String, String>,
}

/// 수집된 정보
#[derive(Debug, Clone, Default)]
struct CollectedInfo {
    includes: Vec<IncludeInfo>,
    media: HashSet<String>,
    categories: HashSet<String>,
    redirect: Option<String>,
}

/// AST에서 Define 수집
fn collect_defines(elements: &mut [SevenMarkElement]) -> HashMap<String, String> {
    let mut defines = HashMap::new();

    for element in elements {
        collect_defines_recursive(element, &mut defines);
    }

    defines
}

fn collect_defines_recursive(
    element: &mut SevenMarkElement,
    defines: &mut HashMap<String, String>,
) {
    if let SevenMarkElement::DefineElement(def) = element {
        for (key, param) in &def.parameters {
            let value = extract_plain_text(&param.value);
            if !value.is_empty() {
                defines.insert(key.clone(), value);
            }
        }
    }

    // 자식 순회
    let mut children = Vec::new();
    element.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &mut children {
        collect_defines_recursive(child, defines);
    }
}

/// AST에서 Variable 치환
fn substitute_variables(elements: &mut [SevenMarkElement], params: &HashMap<String, String>) {
    for element in elements {
        substitute_variables_recursive(element, params);
    }
}

fn substitute_variables_recursive(
    element: &mut SevenMarkElement,
    params: &HashMap<String, String>,
) {
    if let SevenMarkElement::Variable(var) = element {
        if let Some(value) = params.get(&var.content) {
            *element = SevenMarkElement::Text(TextElement {
                location: Location::synthesized(),
                content: value.clone(),
            });
            return; // 치환했으니 자식 순회 불필요
        }
    }

    // 자식 순회 (mutable)
    element.traverse_children(&mut |child| {
        substitute_variables_recursive(child, params);
    });
}

/// AST에서 Include, Media, Category, Redirect 수집
fn collect_info(elements: &mut [SevenMarkElement], info: &mut CollectedInfo) {
    for element in elements {
        collect_info_recursive(element, info);
    }
}

fn collect_info_recursive(element: &mut SevenMarkElement, info: &mut CollectedInfo) {
    match element {
        SevenMarkElement::Include(inc) => {
            // Include가 이미 processed되었으면 스킵
            if inc.processed {
                return;
            }

            let title = extract_plain_text(&inc.content);
            if !title.is_empty() {
                let parameters: HashMap<String, String> = inc
                    .parameters
                    .iter()
                    .map(|(k, v)| (k.clone(), extract_plain_text(&v.value)))
                    .collect();

                let namespace_str = parameters
                    .get("namespace")
                    .map(|s| s.as_str())
                    .unwrap_or("Document");
                let namespace = parse_namespace(namespace_str);

                info.includes.push(IncludeInfo {
                    title,
                    namespace,
                    parameters,
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
        SevenMarkElement::Category(cat) => {
            let name = extract_plain_text(&cat.content);
            if !name.is_empty() {
                info.categories.insert(name);
            }
        }
        SevenMarkElement::Redirect(redir) => {
            let target = extract_plain_text(&redir.content);
            if !target.is_empty() {
                info.redirect = Some(target);
            }
        }
        _ => {}
    }

    // 자식 순회
    let mut children = Vec::new();
    element.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &mut children {
        collect_info_recursive(child, info);
    }
}

/// AST에서 Include 요소를 resolved AST로 치환
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
    if let SevenMarkElement::Include(inc) = element {
        // 이미 처리된 Include는 스킵
        if !inc.processed {
            let title = extract_plain_text(&inc.content);
            if !title.is_empty() {
                let namespace_str = inc
                    .parameters
                    .get("namespace")
                    .map(|param| extract_plain_text(&param.value))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "Document".to_string());

                let key = format!("{}:{}", namespace_str, title);

                if let Some(resolved) = resolved_includes.get(&key) {
                    // Include의 content를 resolved AST로 교체하고 processed 설정
                    inc.content = resolved.ast.clone();
                    inc.processed = true;
                    debug!("Substituted include: {}", key);
                }
            }
        }
    }

    // 자식 순회 (mutable)
    element.traverse_children(&mut |child| {
        substitute_includes_recursive(child, resolved_includes);
    });
}

/// Plain text 추출
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

/// namespace 문자열을 DocumentNamespace enum으로 변환
fn parse_namespace(namespace: &str) -> DocumentNamespace {
    match namespace {
        "Document" => DocumentNamespace::Document,
        "User" => DocumentNamespace::User,
        "Template" => DocumentNamespace::Template,
        "File" => DocumentNamespace::File,
        "Category" => DocumentNamespace::Category,
        "Wiki" => DocumentNamespace::Wiki,
        _ => DocumentNamespace::Document, // 기본값
    }
}

/// namespace enum을 문자열로 변환
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
