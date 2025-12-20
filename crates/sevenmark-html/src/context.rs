//! Rendering context for footnote tracking

use sevenmark_parser::ast::SevenMarkElement;

/// Footnote entry for collection
#[derive(Debug, Clone)]
pub struct FootnoteEntry {
    /// Footnote index (1-based, from parser)
    pub index: usize,
    /// Display text (from #display parameter or auto-generated number)
    pub display: String,
    /// Footnote content (to be rendered later)
    pub content: Vec<SevenMarkElement>,
}

/// Simple rendering context - only tracks footnotes
pub struct RenderContext {
    /// Collected footnotes (rendered at document end)
    pub footnotes: Vec<FootnoteEntry>,
    /// Track if we are inside a footnote (to prevent nested footnotes)
    pub in_footnote: bool,
}

impl RenderContext {
    /// Creates a new render context
    pub fn new() -> Self {
        Self {
            footnotes: Vec::new(),
            in_footnote: false,
        }
    }

    /// Adds a footnote entry
    pub fn add_footnote(
        &mut self,
        index: usize,
        display: Option<String>,
        content: Vec<SevenMarkElement>,
    ) -> String {
        let display = display.unwrap_or_else(|| index.to_string());

        self.footnotes.push(FootnoteEntry {
            index,
            display: display.clone(),
            content,
        });

        display
    }
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
}
