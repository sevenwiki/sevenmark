use sevenmark_ast::{Element, Traversable};

/// Recursively visits every element in the AST via depth-first traversal.
pub fn visit_elements(elements: &[Element], visitor: &mut dyn FnMut(&Element)) {
    for element in elements {
        visit_element(element, visitor);
    }
}

fn visit_element(element: &Element, visitor: &mut dyn FnMut(&Element)) {
    visitor(element);
    element.traverse_children_ref(&mut |child| {
        visit_element(child, visitor);
    });
}