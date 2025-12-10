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
    NewLine,
    Age(AgeElement),
    Variable(VariableElement),

    // Markdown text styles
    BoldItalic(TextStyle),
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
