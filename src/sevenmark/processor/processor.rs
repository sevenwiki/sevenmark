use crate::sevenmark::core::parse_document;
use crate::sevenmark::processor::postprocessor::SevenMarkPostprocessor;
use crate::sevenmark::processor::preprocessor::{PreprocessInfo, PreVisitor, SevenMarkPreprocessor, IncludeInfo};
use crate::sevenmark::processor::wiki::{WikiClient, WikiResolver};
use crate::SevenMarkElement;
use anyhow::Result;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

const MAX_INCLUDE_DEPTH: usize = 16;
#[derive(Debug, Clone, Serialize)]
pub struct ProcessedDocument {
    /// 최종 AST (모든 include가 치환됨)
    pub ast: Vec<SevenMarkElement>,
    /// 모든 media 파일 목록
    pub media: HashSet<String>,
    /// 모든 category 목록
    pub categories: HashSet<String>,
    /// Redirect 대상 (있으면)
    pub redirect: Option<String>,
}

pub async fn process_document(
    input: &str,
    wiki_client: &WikiClient,
) -> Result<ProcessedDocument> {
    let mut ast = parse_document(input);
    let mut all_media = HashSet::new();
    let mut all_categories = HashSet::new();
    let mut all_redirect = None;
    let mut processed_includes = HashSet::new();

    for depth in 0..MAX_INCLUDE_DEPTH {
        println!("\n=== Loop iteration {} ===", depth);

        // 1. Preprocess - include, media 등 수집
        let info = SevenMarkPreprocessor::preprocess(&ast);
        println!("Preprocessor found {} includes", info.includes.len());
        for (key, include_info) in &info.includes {
            println!("  - Include: {} (namespace: {:?})", key, include_info.namespace);
        }

        // 2. Media, category, redirect 누적
        all_media.extend(info.media);
        all_categories.extend(info.categories);
        if info.redirect.is_some() {
            all_redirect = info.redirect;
        }

        // 3. 새로운 include 필터링 (순환 참조 방지)
        let new_includes: HashMap<String, IncludeInfo> = info
            .includes
            .into_iter()
            .filter(|(key, _)| !processed_includes.contains(key))
            .collect();

        println!("New includes (not processed yet): {}", new_includes.len());
        for key in new_includes.keys() {
            println!("  - New: {}", key);
        }

        if new_includes.is_empty() {
            println!("No new includes, breaking loop");
            break; // 더 이상 새로운 include 없음
        }

        // 4. PreprocessInfo 형태로 변환
        let new_info = PreprocessInfo {
            includes: new_includes.clone(),
            categories: HashSet::new(),
            redirect: None,
            media: HashSet::new(),
        };

        // 5. Wiki fetch
        println!("Fetching {} includes from wiki...", new_includes.len());
        let wiki_data = WikiResolver::resolve(&new_info, wiki_client).await?;
        println!("WikiData received with {} includes", wiki_data.includes.len());
        for key in wiki_data.includes.keys() {
            println!("  - WikiData has: {}", key);
        }

        // 6. Postprocess - include 치환 (수집은 다음 루프의 Preprocessor가 담당)
        println!("Applying postprocessor...");
        ast = SevenMarkPostprocessor::apply(ast, &wiki_data);
        println!("Postprocessor completed");

        // 7. 처리 완료 기록
        processed_includes.extend(new_includes.keys().cloned());
        println!("Total processed includes so far: {}", processed_includes.len());
    }

    Ok(ProcessedDocument {
        ast,
        media: all_media,
        categories: all_categories,
        redirect: all_redirect,
    })
}