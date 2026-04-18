use crate::context::ParseContext;
use winnow::stream::Stateful;

mod block;
mod brace;
mod bracket;
mod comment;
pub mod document;
pub mod element;
pub mod escape;
mod r#macro;
pub mod markdown;
mod mention;
mod parameter;

mod expr;
mod input_source;
pub mod text;
pub mod token;
mod utils;

pub use block::block_document_parser;
pub use input_source::{InputSource, SourceSegment};

pub type ParserInput<'input> = Stateful<InputSource<'input>, ParseContext>;
