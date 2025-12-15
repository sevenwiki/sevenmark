//! Render context for stateful rendering

use chrono::{DateTime, Utc};
use sevenmark_parser::ast::SevenMarkElement;

/// Collected footnote during rendering
#[derive(Debug, Clone)]
pub struct CollectedFootnote {
    pub index: usize,
    pub content: Vec<SevenMarkElement>,
}

/// Render context holding state during rendering
pub struct RenderContext {
    /// Current time for TimeNow and Age macros
    pub now: DateTime<Utc>,
    /// Collected footnotes
    footnotes: Vec<CollectedFootnote>,
    /// Footnote counter
    footnote_counter: usize,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderContext {
    /// Create a new render context with current time
    pub fn new() -> Self {
        Self {
            now: Utc::now(),
            footnotes: Vec::new(),
            footnote_counter: 0,
        }
    }

    /// Create a render context with a specific time (for testing)
    pub fn with_time(now: DateTime<Utc>) -> Self {
        Self {
            now,
            footnotes: Vec::new(),
            footnote_counter: 0,
        }
    }

    /// Add a footnote and return its index
    pub fn add_footnote(&mut self, content: Vec<SevenMarkElement>) -> usize {
        self.footnote_counter += 1;
        let index = self.footnote_counter;
        self.footnotes.push(CollectedFootnote { index, content });
        index
    }

    /// Get collected footnotes
    pub fn footnotes(&self) -> &[CollectedFootnote] {
        &self.footnotes
    }

    /// Check if there are any footnotes
    pub fn has_footnotes(&self) -> bool {
        !self.footnotes.is_empty()
    }
}
