use serde::Serialize;

use super::{Location, Parameters, SevenMarkElement};

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

/// 멘션 타입
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum MentionType {
    /// 토론/문서 멘션 (<#uuid>)
    Discussion,
    /// 사용자 멘션 (<@uuid>)
    User,
}

/// 멘션 요소
#[derive(Debug, Clone, Serialize)]
pub struct MentionElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub mention_type: MentionType,
    pub uuid: String,
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
#[derive(Debug, Clone, Serialize, Default)]
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

/// 파일 resolve 결과 (DB에서 실제 URL을 가져옴)
#[derive(Debug, Clone, Serialize, Default)]
pub struct ResolvedFile {
    pub url: String,
    pub is_valid: bool,
}

/// 문서/카테고리 resolve 결과 (title만 저장, URL은 렌더러에서 조립)
#[derive(Debug, Clone, Serialize, Default)]
pub struct ResolvedDoc {
    pub title: String,
    pub is_valid: bool,
}

/// MediaElement resolve 결과
/// file, document, category, url 각각 독립적으로 처리
/// href 우선순위: url > document > category
#[derive(Debug, Clone, Serialize, Default)]
pub struct ResolvedMediaInfo {
    /// #file 참조 결과 (이미지 표시용, DB에서 실제 URL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ResolvedFile>,
    /// #document 참조 결과 (title만, URL은 렌더러에서 조립)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<ResolvedDoc>,
    /// #category 참조 결과 (title만, URL은 렌더러에서 조립)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ResolvedDoc>,
    /// #url 외부 링크
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
    pub content: String,
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
    /// Footnote index (1-based, assigned during parsing)
    pub footnote_index: usize,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 헤더
#[derive(Debug, Clone, Serialize)]
pub struct Header {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub level: usize,
    pub is_folded: bool,
    pub section_index: usize,
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
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}
