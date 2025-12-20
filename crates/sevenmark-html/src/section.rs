//! Section tree building for hierarchical document structure
//!
//! Converts flat AST into a tree structure where headers with higher level
//! numbers are children of headers with lower level numbers.

use sevenmark_parser::ast::{Header, SevenMarkElement};

/// A section in the document tree
#[derive(Debug)]
pub struct Section<'a> {
    /// The header element
    pub header: &'a Header,
    /// Hierarchical path (e.g., "1", "1.1", "1.2.3")
    pub section_path: String,
    /// Content elements before first child section
    pub content: Vec<&'a SevenMarkElement>,
    /// Nested child sections
    pub children: Vec<Section<'a>>,
}

/// Document organized as a section tree
#[derive(Debug)]
pub struct SectionTree<'a> {
    /// Content before the first header
    pub preamble: Vec<&'a SevenMarkElement>,
    /// Top-level sections
    pub sections: Vec<Section<'a>>,
}

/// Check if an element is a Header and return it
fn as_header(el: &SevenMarkElement) -> Option<&Header> {
    match el {
        SevenMarkElement::Header(h) => Some(h),
        _ => None,
    }
}

/// Build a hierarchical section tree from a flat AST
///
/// Headers with higher level numbers are children of headers with lower level numbers.
/// For example, H2 (level=2) is a child of H1 (level=1).
pub fn build_section_tree(elements: &[SevenMarkElement]) -> SectionTree<'_> {
    let mut preamble = Vec::new();
    let mut sections = Vec::new();
    let mut i = 0;

    // Collect preamble (content before first header)
    while i < elements.len() && as_header(&elements[i]).is_none() {
        preamble.push(&elements[i]);
        i += 1;
    }

    // Build sections recursively
    let mut top_level_counter = 0;
    while i < elements.len() {
        top_level_counter += 1;
        let path = top_level_counter.to_string();
        if let Some((section, next_index)) = build_section(elements, i, 0, path) {
            sections.push(section);
            i = next_index;
        } else {
            break;
        }
    }

    SectionTree { preamble, sections }
}

/// Build a single section starting at index
///
/// Returns the section and the next index to process, or None if no section starts here.
fn build_section(
    elements: &[SevenMarkElement],
    start_index: usize,
    min_level: usize,
    section_path: String,
) -> Option<(Section<'_>, usize)> {
    if start_index >= elements.len() {
        return None;
    }

    let header = as_header(&elements[start_index])?;

    // If this header's level is <= minLevel, it belongs to a parent section
    if header.level <= min_level {
        return None;
    }

    let mut section = Section {
        header,
        section_path,
        content: Vec::new(),
        children: Vec::new(),
    };

    let mut i = start_index + 1;
    let mut child_counter = 0;

    // Collect content and children until we hit a header of same or lower level
    while i < elements.len() {
        if let Some(next_header) = as_header(&elements[i]) {
            // If next header has same or lower level number, this section ends
            if next_header.level <= header.level {
                break;
            }

            // Otherwise, it's a child section
            child_counter += 1;
            let child_path = format!("{}.{}", section.section_path, child_counter);
            if let Some((child_section, next_index)) =
                build_section(elements, i, header.level, child_path)
            {
                section.children.push(child_section);
                i = next_index;
            } else {
                break;
            }
        } else {
            // Regular content
            section.content.push(&elements[i]);
            i += 1;
        }
    }

    Some((section, i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sevenmark_parser::ast::{Location, TextElement};

    fn text(s: &str) -> SevenMarkElement {
        SevenMarkElement::Text(TextElement {
            location: Location::synthesized(),
            content: s.to_string(),
        })
    }

    fn header(level: usize, section_index: usize) -> SevenMarkElement {
        SevenMarkElement::Header(Header {
            location: Location::synthesized(),
            level,
            is_folded: false,
            section_index,
            content: vec![],
        })
    }

    #[test]
    fn test_preamble_only() {
        let elements = vec![text("hello"), text("world")];
        let tree = build_section_tree(&elements);

        assert_eq!(tree.preamble.len(), 2);
        assert!(tree.sections.is_empty());
    }

    #[test]
    fn test_single_section() {
        let elements = vec![header(1, 0), text("content")];
        let tree = build_section_tree(&elements);

        assert!(tree.preamble.is_empty());
        assert_eq!(tree.sections.len(), 1);
        assert_eq!(tree.sections[0].section_path, "1");
        assert_eq!(tree.sections[0].content.len(), 1);
        assert!(tree.sections[0].children.is_empty());
    }

    #[test]
    fn test_nested_sections() {
        let elements = vec![
            text("preamble"),
            header(1, 0),
            text("content1"),
            header(2, 1),
            text("content1.1"),
            header(3, 2),
            text("content1.1.1"),
            header(2, 3),
            text("content1.2"),
            header(1, 4),
            text("content2"),
        ];
        let tree = build_section_tree(&elements);

        assert_eq!(tree.preamble.len(), 1);
        assert_eq!(tree.sections.len(), 2);

        // Section 1
        let s1 = &tree.sections[0];
        assert_eq!(s1.section_path, "1");
        assert_eq!(s1.content.len(), 1);
        assert_eq!(s1.children.len(), 2);

        // Section 1.1
        let s1_1 = &s1.children[0];
        assert_eq!(s1_1.section_path, "1.1");
        assert_eq!(s1_1.content.len(), 1);
        assert_eq!(s1_1.children.len(), 1);

        // Section 1.1.1
        let s1_1_1 = &s1_1.children[0];
        assert_eq!(s1_1_1.section_path, "1.1.1");
        assert_eq!(s1_1_1.content.len(), 1);
        assert!(s1_1_1.children.is_empty());

        // Section 1.2
        let s1_2 = &s1.children[1];
        assert_eq!(s1_2.section_path, "1.2");
        assert_eq!(s1_2.content.len(), 1);
        assert!(s1_2.children.is_empty());

        // Section 2
        let s2 = &tree.sections[1];
        assert_eq!(s2.section_path, "2");
        assert_eq!(s2.content.len(), 1);
        assert!(s2.children.is_empty());
    }
}
