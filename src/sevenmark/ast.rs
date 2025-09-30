use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 소스 코드 위치 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl Location {
    /// Creates a synthesized location for elements generated during preprocessing
    pub fn synthesized() -> Self {
        Self { start: 0, end: 0 }
    }
}

/// Individual parameter with location tracking
#[derive(Debug, Clone, Serialize)]
pub struct Parameter {
    #[serde(skip_serializing)]
    pub location: Location,
    pub key: String,
    pub value: Vec<SevenMarkElement>,
}

/// 파라미터 맵: key-value 쌍으로 각 value는 Parameter 구조체 (location 포함)  
/// BTreeMap을 사용하여 키 순서를 일관되게 유지
pub type Parameters = BTreeMap<String, Parameter>;

/// 텍스트 요소
#[derive(Debug, Clone, Serialize)]
pub struct TextElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: String,
}

/// 이스케이프 요소
#[derive(Debug, Clone, Serialize)]
pub struct EscapeElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: String,
}

/// age
#[derive(Debug, Clone, Serialize)]
pub struct AgeElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: String,
}

/// variable
#[derive(Debug, Clone, Serialize)]
pub struct VariableElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: String,
}

/// 주석 요소
#[derive(Debug, Clone, Serialize)]
pub struct CommentElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: String,
}

/// 에러 요소 (파싱 실패한 내용)
#[derive(Debug, Clone, Serialize)]
pub struct ErrorElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: String,
}

/// 공통 스타일 속성들
#[derive(Debug, Clone, Serialize)]
pub struct CommonStyleAttributes {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub size: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub color: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bg_color: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub opacity: Vec<SevenMarkElement>,
}

/// 리터럴 요소 {{{ content }}}
#[derive(Debug, Clone, Serialize)]
pub struct LiteralElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 스타일이 적용된 요소 {{{#style="..." content}}}
#[derive(Debug, Clone, Serialize)]
pub struct StyledElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DefineElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
}

/// 미디어 요소 [[#file="..." #url="..." display_text]]
#[derive(Debug, Clone, Serialize)]
pub struct MediaElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 테이블 셀
#[derive(Debug, Clone, Serialize)]
pub struct TableInnerElement2 {
    pub parameters: Parameters,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub x: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub y: Vec<SevenMarkElement>,
    pub content: Vec<SevenMarkElement>,
}

/// 테이블 행
#[derive(Debug, Clone, Serialize)]
pub struct TableInnerElement1 {
    pub parameters: Parameters,
    pub inner_content: Vec<TableInnerElement2>,
}

/// 테이블 요소
#[derive(Debug, Clone, Serialize)]
pub struct TableElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<TableInnerElement1>,
}

/// 리스트 아이템
#[derive(Debug, Clone, Serialize)]
pub struct ListInnerElement1 {
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 리스트 요소
#[derive(Debug, Clone, Serialize)]
pub struct ListElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub kind: String,
    pub parameters: Parameters,
    pub content: Vec<ListInnerElement1>,
}

/// 폴드 내부 요소
#[derive(Debug, Clone, Serialize)]
pub struct FoldInnerElement {
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 폴드 요소
#[derive(Debug, Clone, Serialize)]
pub struct FoldElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: (FoldInnerElement, FoldInnerElement),
}

/// 인용 블록
#[derive(Debug, Clone, Serialize)]
pub struct BlockQuoteElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 루비 텍스트
#[derive(Debug, Clone, Serialize)]
pub struct RubyElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 코드 블록
#[derive(Debug, Clone, Serialize)]
pub struct CodeElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeXElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub is_block: bool,
    pub content: String,
}

/// 각주
#[derive(Debug, Clone, Serialize)]
pub struct FootnoteElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 헤더
#[derive(Debug, Clone, Serialize)]
pub struct Header {
    #[serde(skip_serializing)]
    pub location: Location,
    pub level: usize,
    pub is_folded: bool,
    pub content: Vec<SevenMarkElement>,
}

/// 텍스트 스타일 (Bold, Italic 등)
#[derive(Debug, Clone, Serialize)]
pub struct TextStyle {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 포함 요소
#[derive(Debug, Clone, Serialize)]
pub struct IncludeElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 카테고리 요소
#[derive(Debug, Clone, Serialize)]
pub struct CategoryElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 리다이렉트 요소
#[derive(Debug, Clone, Serialize)]
pub struct RedirectElement {
    #[serde(skip_serializing)]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

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
}

impl Default for CommonStyleAttributes {
    fn default() -> Self {
        Self {
            style: Vec::new(),
            size: Vec::new(),
            color: Vec::new(),
            bg_color: Vec::new(),
            opacity: Vec::new(),
        }
    }
}

/// Trait for automatically traversing AST elements
pub trait Traversable {
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where
        F: FnMut(&mut SevenMarkElement);
}

impl Traversable for SevenMarkElement {
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where
        F: FnMut(&mut SevenMarkElement),
    {
        match self {
            // 자식이 없는 요소들
            SevenMarkElement::Text(_)
            | SevenMarkElement::Comment(_)
            | SevenMarkElement::Escape(_)
            | SevenMarkElement::Error(_)
            | SevenMarkElement::Age(_)
            | SevenMarkElement::Variable(_)
            | SevenMarkElement::TeXElement(_)
            | SevenMarkElement::Null
            | SevenMarkElement::FootNote
            | SevenMarkElement::TimeNow
            | SevenMarkElement::NewLine
            | SevenMarkElement::HLine => {
                // 자식 없음
            }

            // content 필드 하나만 있는 요소들
            SevenMarkElement::LiteralElement(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::Header(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::Category(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::Redirect(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::FootnoteElement(e) => e.content.iter_mut().for_each(visitor),

            // content + parameters 둘 다 있는 요소들
            SevenMarkElement::StyledElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::BlockQuoteElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::RubyElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::CodeElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::Include(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::MediaElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }

            // parameters만 있는 요소
            SevenMarkElement::DefineElement(e) => {
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }

            // TextStyle 계열
            SevenMarkElement::BoldItalic(e)
            | SevenMarkElement::Bold(e)
            | SevenMarkElement::Italic(e)
            | SevenMarkElement::Strikethrough(e)
            | SevenMarkElement::Underline(e)
            | SevenMarkElement::Superscript(e)
            | SevenMarkElement::Subscript(e) => {
                e.content.iter_mut().for_each(visitor);
            }

            // 특수 중첩 구조들
            SevenMarkElement::TableElement(table) => {
                for row in &mut table.content {
                    for cell in &mut row.inner_content {
                        for child in &mut cell.content {
                            visitor(child);
                        }
                    }
                }
            }
            SevenMarkElement::ListElement(list) => {
                for item in &mut list.content {
                    for child in &mut item.content {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::FoldElement(fold) => {
                for child in &mut fold.content.0.content {
                    visitor(child);
                }
                for child in &mut fold.content.1.content {
                    visitor(child);
                }
            }
        }
    }
}
