use crate::SevenMarkElement;
use crate::sevenmark::transform::postprocessor::{ProcessedDocument, postprocess_sevenmark};
use crate::sevenmark::transform::preprocessor::preprocess_sevenmark;
use anyhow::Result;
use sea_orm::DatabaseConnection;

/// Processes SevenMark AST through preprocessing and postprocessing pipeline
///
/// This function orchestrates the full document processing:
/// 1. Preprocessing: Variable substitution, include resolution, media collection
/// 2. Postprocessing: Media reference resolution (file URLs, document/category links)
pub async fn process_sevenmark(
    ast: Vec<SevenMarkElement>,
    db: &DatabaseConnection,
) -> Result<ProcessedDocument> {
    // Step 1: Preprocess - resolve includes and collect media references
    let preprocessed = preprocess_sevenmark(ast, db).await?;

    // Step 2: Postprocess - resolve media references to URLs
    let processed = postprocess_sevenmark(preprocessed, db).await?;

    Ok(processed)
}
