//! AST (Abstract Syntax Tree) definitions for SevenMark
//!
//! This module contains all AST element definitions organized into submodules:
//! - `location`: Location and Parameter types
//! - `expression`: Expression AST for conditionals
//! - `table`: Table-related structures
//! - `list`: List-related structures
//! - `elements`: Basic element structures
//! - `traversable`: Traversable trait and implementation

mod elements;
mod expression;
mod list;
mod location;
mod table;
mod traversable;

// Re-export all public types
pub use elements::*;
pub use expression::*;
pub use list::*;
pub use location::*;
pub use table::*;
pub use traversable::*;

use serde::Serialize;

/// 메인 SevenMark AST 요소들
#[derive(Debug, Clone, Serialize)]
pub enum SevenMarkElement {
    // Basic text elements
    Text(TextElement),
    Comment(CommentElement),
    Escape(EscapeElement),
    Error(ErrorElement),

    // Block elements
    LiteralElement(LiteralElement),
    DefineElement(DefineElement),
    StyledElement(StyledElement),
    TableElement(TableElement),
    ListElement(ListElement),
    FoldElement(FoldElement),
    BlockQuoteElement(BlockQuoteElement),
    RubyElement(RubyElement),
    FootnoteElement(FootnoteElement),
    CodeElement(CodeElement),
    TeXElement(TeXElement),

    // Wiki elements
    Include(IncludeElement),
    Category(CategoryElement),
    Redirect(RedirectElement),

    // Media
    MediaElement(MediaElement),

    // Macros
    Null,
    FootNote,
    TimeNow,
    Age(AgeElement),

    // Line breaks
    SoftBreak, // from \n - context-dependent rendering
    HardBreak, // from [br] - always renders as <br>
    Variable(VariableElement),
    Mention(MentionElement),

    // Markdown text styles
    Bold(TextStyle),
    Italic(TextStyle),
    Strikethrough(TextStyle),
    Underline(TextStyle),
    Superscript(TextStyle),
    Subscript(TextStyle),

    // Other markdown elements
    HLine,
    Header(Header),

    // Conditional
    IfElement(IfElement),
}

impl SevenMarkElement {
    /// Returns the location of the element, if available
    pub fn location(&self) -> Option<&Location> {
        match self {
            Self::Text(e) => Some(&e.location),
            Self::Comment(e) => Some(&e.location),
            Self::Escape(e) => Some(&e.location),
            Self::Error(e) => Some(&e.location),
            Self::LiteralElement(e) => Some(&e.location),
            Self::DefineElement(e) => Some(&e.location),
            Self::StyledElement(e) => Some(&e.location),
            Self::TableElement(e) => Some(&e.location),
            Self::ListElement(e) => Some(&e.location),
            Self::FoldElement(e) => Some(&e.location),
            Self::BlockQuoteElement(e) => Some(&e.location),
            Self::RubyElement(e) => Some(&e.location),
            Self::FootnoteElement(e) => Some(&e.location),
            Self::CodeElement(e) => Some(&e.location),
            Self::TeXElement(e) => Some(&e.location),
            Self::Include(e) => Some(&e.location),
            Self::Category(e) => Some(&e.location),
            Self::Redirect(e) => Some(&e.location),
            Self::MediaElement(e) => Some(&e.location),
            Self::Age(e) => Some(&e.location),
            Self::Variable(e) => Some(&e.location),
            Self::Mention(e) => Some(&e.location),
            Self::Bold(e) => Some(&e.location),
            Self::Italic(e) => Some(&e.location),
            Self::Strikethrough(e) => Some(&e.location),
            Self::Underline(e) => Some(&e.location),
            Self::Superscript(e) => Some(&e.location),
            Self::Subscript(e) => Some(&e.location),
            Self::Header(e) => Some(&e.location),
            Self::IfElement(e) => Some(&e.location),
            // Elements without location
            Self::Null
            | Self::FootNote
            | Self::TimeNow
            | Self::SoftBreak
            | Self::HardBreak
            | Self::HLine => None,
        }
    }
}
