//! sevenmark-html - Semantic HTML renderer for SevenMark AST
//!
//! Renders SevenMark AST to clean semantic HTML for SEO purposes.
//!
//! # Example
//!
//! ```rust
//! use sevenmark_parser::core::parse_document;
//! use sevenmark_html::render_document;
//!
//! let ast = parse_document("# Hello\n\nThis is **bold** text.");
//! let html = render_document(&ast, "/edit/title");
//! ```

pub mod classes;
mod context;
mod render;
mod section;

pub use render::{render_document, render_element, render_elements};
