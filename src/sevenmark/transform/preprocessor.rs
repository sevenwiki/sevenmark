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

/// 처리 완료된 문서 (재귀 처리 결과)
#[derive(Debug, Clone, Serialize)]
pub struct ProcessedDocument {
    /// 모든 media 파일 목록 (중첩된 include 포함)
    pub media: HashSet<String>,
    /// 모든 category 목록
    pub categories: HashSet<String>,
    /// Redirect 대상 (있으면)
    pub redirect: Option<String>,
    /// 최종 AST (모든 include가 치환됨)
    pub ast: Vec<SevenMarkElement>,
}

#[derive(Debug, Clone)]
pub struct PreprocessInfo {
    pub includes: HashMap<String, IncludeInfo>, // key: "namespace:title"
    pub categories: HashSet<String>,
    pub redirect: Option<String>,
    pub media: HashSet<String>,
}

/// 재귀 처리 중간 결과
#[derive(Debug, Clone)]
struct ResolvedDocument {
    ast: Vec<SevenMarkElement>,
    media: HashSet<String>,
    categories: HashSet<String>, // depth == 0일 때만 채워짐
    redirect: Option<String>,    // depth == 0일 때만 채워짐
}

impl ResolvedDocument {
    /// CollectedInfo로부터 ResolvedDocument 생성 (depth에 따라 categories/redirect 포함 여부 결정)
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
pub async fn preprocess_sevenmark(
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
async fn resolve_document(
    content: &str,
    parent_params: &HashMap<String, String>,
    depth: usize,
    max_depth: usize,
    visited: &mut HashSet<String>,
    wiki_client: &WikiClient,
) -> Result<ResolvedDocument> {
    debug!("[Depth {}] Starting document resolution", depth);

    // 1. 문서 파싱
    let mut ast = parse_document(content);

    // 2. Define 수집 + Variable 치환 (단일 순회, forward-only, parent_params 우선)
    let mut all_params = parent_params.clone();
    substitute_variables_forward_only(&mut ast, &mut all_params);
    debug!("[Depth {}] Processed variables (forward-only)", depth);

    // 3. Include, Media 수집 (depth == 0이면 Category, Redirect도 수집)
    let mut info = CollectedInfo::default();
    let is_top_level = depth == 0;
    collect_info(&mut ast, &mut info, is_top_level);

    if is_top_level {
        debug!(
            "[Depth {}] Collected {} includes, {} media, {} categories",
            depth,
            info.includes.len(),
            info.media.len(),
            info.categories.len()
        );
    } else {
        debug!(
            "[Depth {}] Collected {} includes, {} media",
            depth,
            info.includes.len(),
            info.media.len()
        );
    }

    // 5. Include가 없거나 depth 한계 도달 시 종료
    if info.includes.is_empty() {
        debug!("[Depth {}] No includes found, returning", depth);
        return Ok(ResolvedDocument::from_collected_info(ast, info, depth));
    }

    if depth >= max_depth {
        debug!(
            "[Depth {}] Maximum depth reached, includes will not be resolved",
            depth
        );
        return Ok(ResolvedDocument::from_collected_info(ast, info, depth));
    }

    // 6. 새로운 include 필터링 (순환 참조 방지) + 파라미터별 중복 제거
    let new_includes = filter_new_includes(std::mem::take(&mut info.includes), visited, depth);

    if new_includes.is_empty() {
        debug!(
            "[Depth {}] All includes already visited (circular reference)",
            depth
        );
        return Ok(ResolvedDocument::from_collected_info(ast, info, depth));
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
    let fetched_docs = wiki_client.fetch_documents_batch(requests).await?;
    debug!("[Depth {}] Fetched {} documents", depth, fetched_docs.len());

    // 8. 각 Include를 재귀적으로 resolve
    let mut resolved_includes: HashMap<String, ResolvedDocument> = HashMap::new();

    // 응답 순서와 무관하게 매칭하기 위해 fetched_docs를 HashMap으로 변환
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

        // 응답에서 해당 문서 찾기
        let Some(doc) = docs_map.get(&doc_key) else {
            warn!("[Warning] Include target not found, skipping: {}", doc_key);
            continue;
        };

        // visited에 추가 (순환 참조 방지용)
        visited.insert(doc_key.clone());

        // Parameters를 HashMap으로 변환
        let params_map: HashMap<String, String> = include_info
            .parameters
            .iter()
            .map(|(k, v)| (k.clone(), extract_plain_text(&v.value)))
            .collect();

        // 재귀 호출
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

        // 결과 저장 (파라미터 포함한 해시 key로)
        let hash_key = make_include_key(&include_info.title, &include_info.parameters);
        resolved_includes.insert(hash_key, resolved);
        debug!("[Depth {}] Resolved include: {}", depth, doc_key);

        // 재귀 종료 후 visited에서 제거 (다른 분기에서 재사용 가능하도록)
        visited.remove(&doc_key);
    }

    // 9. AST에서 Include 요소를 resolved AST로 치환
    substitute_includes(&mut ast, &resolved_includes);

    // 10. 모든 media를 누적 (categories/redirect는 depth 0에서만)
    let mut all_media = info.media;

    for resolved in resolved_includes.values() {
        all_media.extend(resolved.media.clone());
    }

    debug!("[Depth {}] Document resolution complete", depth);

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

/// Include 정보
#[derive(Debug, Clone, Serialize)]
pub struct IncludeInfo {
    pub title: String,
    pub namespace: DocumentNamespace,
    pub parameters: Parameters,
}

/// 수집된 정보
#[derive(Debug, Clone, Default)]
struct CollectedInfo {
    includes: Vec<IncludeInfo>,
    media: HashSet<String>,
    categories: HashSet<String>,
    redirect: Option<String>,
}

/// 순환 참조 필터링 + 파라미터별 중복 제거
fn filter_new_includes(
    includes: Vec<IncludeInfo>,
    visited: &HashSet<String>,
    depth: usize,
) -> Vec<IncludeInfo> {
    let mut new_includes_map: HashMap<String, IncludeInfo> = HashMap::new();

    for inc in includes {
        // 순환 참조는 namespace:title로만 체크
        let doc_key = format!("{}:{}", namespace_to_string(&inc.namespace), &inc.title);
        if visited.contains(&doc_key) {
            warn!("[Depth {}] Circular reference detected: {}", depth, doc_key);
        } else {
            // 중복 제거는 파라미터 포함한 해시로 (같은 문서 + 같은 파라미터만 중복 제거)
            let hash_key = make_include_key(&inc.title, &inc.parameters);
            new_includes_map.insert(hash_key, inc);
        }
    }

    new_includes_map.into_values().collect()
}

/// AST에서 Define 수집 + Variable 치환 (단일 순회, forward-only)
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
    // 1. Define 먼저 처리 (선언) - parent_params가 우선순위 높음
    if let SevenMarkElement::DefineElement(def) = element {
        for (key, param) in &def.parameters {
            let value = extract_plain_text(&param.value);
            if !value.is_empty() {
                // parent_params에 이미 있으면 덮어쓰지 않음 (parent 우선)
                params.entry(key.clone()).or_insert(value);
            }
        }
    }

    // 2. Variable 처리 (사용)
    if let SevenMarkElement::Variable(var) = element {
        if let Some(value) = params.get(&var.content) {
            *element = SevenMarkElement::Text(TextElement {
                location: Location::synthesized(),
                content: value.clone(),
            });
            return; // 치환했으니 자식 순회 불필요
        }
    }

    // 3. 자식 순회 (순서대로)
    let mut children = Vec::new();
    element.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &mut children {
        substitute_variables_forward_only_recursive(child, params);
    }
}

/// AST에서 Include, Media 수집 (is_top_level이면 Category, Redirect도 수집)
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
                // namespace 추출
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

    // 자식 순회
    let mut children = Vec::new();
    element.traverse_children(&mut |child| {
        children.push(child.clone());
    });

    for child in &mut children {
        collect_info_recursive(child, info, collect_categories_redirect);
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
    match element {
        SevenMarkElement::Include(inc) => {
            let title = extract_plain_text(&inc.content);
            if !title.is_empty() {
                // 파라미터 포함한 해시 key로 조회
                let hash_key = make_include_key(&title, &inc.parameters);

                if let Some(resolved) = resolved_includes.get(&hash_key) {
                    // Include의 content를 resolved AST로 교체
                    inc.content = resolved.ast.clone();
                    debug!("Substituted include: {} (hash: {})", title, &hash_key);
                    // 치환했으면 이 Include의 content는 이미 resolved된 AST이므로
                    // 더 이상 traverse하지 않음 (circular reference 방지)
                    return;
                }
            }
            // 치환하지 못한 Include의 content는 traverse (nested includes 처리)
            for child in &mut inc.content {
                substitute_includes_recursive(child, resolved_includes);
            }
        }
        _ => {
            // 다른 element는 traverse_children 사용
            element.traverse_children(&mut |child| {
                substitute_includes_recursive(child, resolved_includes);
            });
        }
    }
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

/// Include의 고유 key 생성 (title + parameters의 blake3 해시)
fn make_include_key(title: &str, params: &Parameters) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(title.as_bytes());

    // BTreeMap은 이미 키로 정렬되어 있음
    for (k, v) in params {
        hasher.update(k.as_bytes());
        hasher.update(extract_plain_text(&v.value).as_bytes());
    }

    hasher.finalize().to_hex().to_string()
}
