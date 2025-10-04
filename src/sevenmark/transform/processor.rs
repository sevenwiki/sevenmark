use crate::SevenMarkElement;
use crate::sevenmark::transform::postprocessor::{postprocess_sevenmark, ProcessedDocument};
use crate::sevenmark::transform::preprocessor::preprocess_sevenmark;
use crate::sevenmark::transform::wiki::WikiClient;
use anyhow::Result;

/// Processes SevenMark AST through preprocessing and postprocessing pipeline
///
/// This function orchestrates the full document processing:
/// 1. Preprocessing: Variable substitution, include resolution, media collection
/// 2. Postprocessing: Media reference resolution (file URLs, document/category links)
pub async fn process_sevenmark(
    ast: Vec<SevenMarkElement>,
    wiki_client: &WikiClient,
) -> Result<ProcessedDocument> {
    // Step 1: Preprocess - resolve includes and collect media references
    let preprocessed = preprocess_sevenmark(ast, wiki_client).await?;

    // Step 2: Postprocess - resolve media references to URLs
    let processed = postprocess_sevenmark(preprocessed, wiki_client).await?;

    Ok(processed)
}