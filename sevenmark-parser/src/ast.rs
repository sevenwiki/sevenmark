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
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
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
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: String,
}

/// 이스케이프 요소
#[derive(Debug, Clone, Serialize)]
pub struct EscapeElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: String,
}

/// age
#[derive(Debug, Clone, Serialize)]
pub struct AgeElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: String,
}

/// variable
#[derive(Debug, Clone, Serialize)]
pub struct VariableElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: String,
}

/// 주석 요소
#[derive(Debug, Clone, Serialize)]
pub struct CommentElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: String,
}

/// 에러 요소 (파싱 실패한 내용)
#[derive(Debug, Clone, Serialize)]
pub struct ErrorElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
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
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 스타일이 적용된 요소 {{{#style="..." content}}}
#[derive(Debug, Clone, Serialize)]
pub struct StyledElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DefineElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
}

/// 미디어 요소 [[#file="..." #url="..." display_text]]
#[derive(Debug, Clone, Serialize)]
pub struct MediaElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_info: Option<ResolvedMediaInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedMediaInfo {
    pub resolved_url: String,
    pub is_valid: bool,
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
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
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
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
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
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: (FoldInnerElement, FoldInnerElement),
}

/// 인용 블록
#[derive(Debug, Clone, Serialize)]
pub struct BlockQuoteElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 루비 텍스트
#[derive(Debug, Clone, Serialize)]
pub struct RubyElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 코드 블록
#[derive(Debug, Clone, Serialize)]
pub struct CodeElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeXElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub is_block: bool,
    pub content: String,
}

/// 각주
#[derive(Debug, Clone, Serialize)]
pub struct FootnoteElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 헤더
#[derive(Debug, Clone, Serialize)]
pub struct Header {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub level: usize,
    pub is_folded: bool,
    pub content: Vec<SevenMarkElement>,
}

/// 텍스트 스타일 (Bold, Italic 등)
#[derive(Debug, Clone, Serialize)]
pub struct TextStyle {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 포함 요소
#[derive(Debug, Clone, Serialize)]
pub struct IncludeElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 카테고리 요소
#[derive(Debug, Clone, Serialize)]
pub struct CategoryElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 리다이렉트 요소
#[derive(Debug, Clone, Serialize)]
pub struct RedirectElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 비교 연산자
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ComparisonOperator {
    Equal,        // ==
    NotEqual,     // !=
    GreaterThan,  // >
    LessThan,     // <
    GreaterEqual, // >=
    LessEqual,    // <=
}

/// 조건식 Expression AST
#[derive(Debug, Clone, Serialize)]
pub enum Expression {
    /// 논리 OR 연산
    Or(Box<Expression>, Box<Expression>),
    /// 논리 AND 연산
    And(Box<Expression>, Box<Expression>),
    /// 논리 NOT 연산
    Not(Box<Expression>),

    /// 비교 연산
    Comparison {
        left: Box<Expression>,
        operator: ComparisonOperator,
        right: Box<Expression>,
    },

    /// 함수 호출: int([var(x)]), len([var(str)])
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },

    /// 조건식 전용 리터럴
    StringLiteral(String),
    NumberLiteral(i64),
    BoolLiteral(bool),
    Null,

    /// 기존 SevenMarkElement 그대로 포함 (변환 없음)
    Element(Box<SevenMarkElement>),

    /// 괄호 그룹
    Group(Box<Expression>),
}

/// If 조건문 요소
#[derive(Debug, Clone, Serialize)]
pub struct IfElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub condition: Expression,
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

    // Conditional
    IfElement(IfElement),
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
    /// 각 자식 요소에 대해 visitor 호출
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where
        F: FnMut(&mut SevenMarkElement);

    /// 각 content Vec에 대해 f 호출 (Vec 구조 변경이 필요할 때 사용)
    fn for_each_content_vec<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Vec<SevenMarkElement>);
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

            // IfElement - content와 condition 내 Element들 순회
            SevenMarkElement::IfElement(if_elem) => {
                for child in &mut if_elem.content {
                    visitor(child);
                }
                // condition 내 Expression::Element들도 순회
                traverse_expression(&mut if_elem.condition, visitor);
            }
        }
    }

    fn for_each_content_vec<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Vec<SevenMarkElement>),
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
            | SevenMarkElement::HLine
            | SevenMarkElement::DefineElement(_) => {}

            // content Vec 하나만 있는 요소들
            SevenMarkElement::LiteralElement(e) => f(&mut e.content),
            SevenMarkElement::Header(e) => f(&mut e.content),
            SevenMarkElement::Category(e) => f(&mut e.content),
            SevenMarkElement::Redirect(e) => f(&mut e.content),
            SevenMarkElement::FootnoteElement(e) => f(&mut e.content),
            SevenMarkElement::StyledElement(e) => f(&mut e.content),
            SevenMarkElement::BlockQuoteElement(e) => f(&mut e.content),
            SevenMarkElement::RubyElement(e) => f(&mut e.content),
            SevenMarkElement::CodeElement(e) => f(&mut e.content),
            SevenMarkElement::Include(e) => f(&mut e.content),
            SevenMarkElement::MediaElement(e) => f(&mut e.content),
            SevenMarkElement::IfElement(e) => f(&mut e.content),

            // TextStyle 계열
            SevenMarkElement::BoldItalic(e)
            | SevenMarkElement::Bold(e)
            | SevenMarkElement::Italic(e)
            | SevenMarkElement::Strikethrough(e)
            | SevenMarkElement::Underline(e)
            | SevenMarkElement::Superscript(e)
            | SevenMarkElement::Subscript(e) => f(&mut e.content),

            // 특수 중첩 구조들
            SevenMarkElement::TableElement(table) => {
                for row in &mut table.content {
                    for cell in &mut row.inner_content {
                        f(&mut cell.content);
                    }
                }
            }
            SevenMarkElement::ListElement(list) => {
                for item in &mut list.content {
                    f(&mut item.content);
                }
            }
            SevenMarkElement::FoldElement(fold) => {
                f(&mut fold.content.0.content);
                f(&mut fold.content.1.content);
            }
        }
    }
}

/// Expression 내의 SevenMarkElement들을 순회하는 헬퍼 함수
fn traverse_expression<F>(expr: &mut Expression, visitor: &mut F)
where
    F: FnMut(&mut SevenMarkElement),
{
    match expr {
        Expression::Or(left, right) | Expression::And(left, right) => {
            traverse_expression(left, visitor);
            traverse_expression(right, visitor);
        }
        Expression::Not(inner) | Expression::Group(inner) => {
            traverse_expression(inner, visitor);
        }
        Expression::Comparison { left, right, .. } => {
            traverse_expression(left, visitor);
            traverse_expression(right, visitor);
        }
        Expression::FunctionCall { arguments, .. } => {
            for arg in arguments {
                traverse_expression(arg, visitor);
            }
        }
        Expression::Element(elem) => {
            visitor(elem);
        }
        Expression::StringLiteral(_)
        | Expression::NumberLiteral(_)
        | Expression::BoolLiteral(_)
        | Expression::Null => {
            // 자식 없음
        }
    }
}
