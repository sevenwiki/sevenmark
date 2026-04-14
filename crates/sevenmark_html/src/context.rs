//! Rendering context for footnote tracking

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use sevenmark_ast::Element;
use sevenmark_utils::Utf16OffsetConverter;

use crate::config::RenderConfig;

/// Footnote entry for collection
#[derive(Debug, Clone)]
pub struct FootnoteEntry {
    /// Footnote index (1-based, from parser)
    pub index: usize,
    /// Display text (from #display parameter or auto-generated number)
    pub display: String,
    /// Optional name for named footnotes (used for anchor IDs)
    pub name: Option<String>,
    /// Footnote content (to be rendered later)
    pub content: Vec<Element>,
}

/// Rendering context shared across document and child renders.
pub struct RenderContext<'a> {
    /// Collected footnotes (rendered at document end)
    pub footnotes: Vec<FootnoteEntry>,
    /// Sequential numbering for unnamed footnotes rendered without a custom display
    pub next_unnamed_footnote_number: usize,
    /// Named footnote tracking: name -> footnote index (persists across flushes)
    pub named_footnotes: HashMap<String, usize>,
    /// Track if we are inside a footnote (to prevent nested footnotes)
    pub in_footnote: bool,
    /// Track depth of elements that suppress SoftBreak rendering
    /// (SoftBreak renders as <br> only when this is 0)
    pub suppress_soft_breaks_depth: usize,
    /// Render configuration
    pub config: &'a RenderConfig<'a>,
    /// UTF-16 offset converter for span data attributes
    pub converter: Option<&'a Utf16OffsetConverter>,
    /// Pre-rendered table-of-contents markup reused by [toc]
    pub toc_markup: Option<String>,
    /// Shared dark-style registry flushed once at document top level
    pub dark_styles: Rc<RefCell<BTreeMap<String, String>>>,
}

impl<'a> RenderContext<'a> {
    /// Creates a new render context with config
    pub fn new(config: &'a RenderConfig<'a>) -> Self {
        Self {
            footnotes: Vec::new(),
            next_unnamed_footnote_number: 1,
            named_footnotes: HashMap::new(),
            in_footnote: false,
            suppress_soft_breaks_depth: 0,
            config,
            converter: None,
            toc_markup: None,
            dark_styles: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    /// Creates a new render context with config and UTF-16 converter
    pub fn with_converter(
        config: &'a RenderConfig<'a>,
        converter: &'a Utf16OffsetConverter,
    ) -> Self {
        Self {
            footnotes: Vec::new(),
            next_unnamed_footnote_number: 1,
            named_footnotes: HashMap::new(),
            in_footnote: false,
            suppress_soft_breaks_depth: 0,
            config,
            converter: Some(converter),
            toc_markup: None,
            dark_styles: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    /// Create a fresh child context preserving config/converter.
    /// Useful when re-rendering nested content (e.g., footnote lists).
    pub fn child(&self) -> Self {
        Self {
            footnotes: Vec::new(),
            next_unnamed_footnote_number: 1,
            named_footnotes: HashMap::new(),
            in_footnote: false,
            suppress_soft_breaks_depth: 0,
            config: self.config,
            converter: self.converter,
            toc_markup: self.toc_markup.clone(),
            dark_styles: Rc::clone(&self.dark_styles),
        }
    }

    /// Store pre-rendered table-of-contents markup for [toc] macros.
    pub fn set_toc_markup(&mut self, toc_markup: Option<String>) {
        self.toc_markup = toc_markup;
    }

    /// Register a sanitized dark-style payload and return its `data-dk` hash.
    pub fn add_dark_style(&mut self, dark_style: Option<String>) -> Option<String> {
        let dark_style = dark_style?;
        let (dk, rule) = crate::render::utils::build_dark_style_rule(&dark_style);
        self.dark_styles
            .borrow_mut()
            .entry(dk.clone())
            .or_insert(rule);
        Some(dk)
    }

    /// Return the aggregated dark stylesheet, if any.
    pub fn dark_style_sheet(&self) -> Option<String> {
        let registry = self.dark_styles.borrow();
        if registry.is_empty() {
            None
        } else {
            Some(registry.values().cloned().collect::<Vec<_>>().join("\n"))
        }
    }

    /// Enter a context that suppresses SoftBreak rendering
    pub fn enter_suppress_soft_breaks(&mut self) {
        self.suppress_soft_breaks_depth += 1;
    }

    /// Exit a context that suppresses SoftBreak rendering
    pub fn exit_suppress_soft_breaks(&mut self) {
        self.suppress_soft_breaks_depth = self.suppress_soft_breaks_depth.saturating_sub(1);
    }

    /// Check if SoftBreak should be suppressed
    pub fn is_soft_break_suppressed(&self) -> bool {
        self.suppress_soft_breaks_depth > 0
    }

    /// Get UTF-16 start offset for span data attribute
    pub fn span_start(&self, span: &sevenmark_ast::Span) -> Option<u32> {
        self.converter.map(|c| c.convert(span.start))
    }

    /// Get UTF-16 end offset for span data attribute
    pub fn span_end(&self, span: &sevenmark_ast::Span) -> Option<u32> {
        self.converter.map(|c| c.convert(span.end))
    }

    /// Adds a footnote entry and returns reference to the display text
    pub fn add_footnote(
        &mut self,
        index: usize,
        display: Option<String>,
        content: Vec<Element>,
    ) -> &str {
        let number = self.next_unnamed_footnote_number;
        self.next_unnamed_footnote_number += 1;

        let display = display.unwrap_or_else(|| number.to_string());

        self.footnotes.push(FootnoteEntry {
            index,
            display,
            name: None,
            content,
        });

        // Return reference to the display we just pushed
        &self.footnotes.last().unwrap().display
    }

    /// Adds a named footnote. Returns (is_new, existing_index_if_duplicate).
    /// If the name already exists, returns the existing footnote's index without adding a new entry.
    pub fn add_named_footnote(
        &mut self,
        index: usize,
        name: String,
        content: Vec<Element>,
    ) -> Result<&str, usize> {
        if let Some(&existing_index) = self.named_footnotes.get(&name) {
            // Duplicate — return existing index for back-reference
            return Err(existing_index);
        }

        self.named_footnotes.insert(name.clone(), index);
        self.footnotes.push(FootnoteEntry {
            index,
            display: name.clone(),
            name: Some(name),
            content,
        });

        Ok(&self.footnotes.last().unwrap().display)
    }
}
