use std::collections::BTreeSet;

use ls_types::{CompletionItem, CompletionItemKind};
use sevenmark_ast::Element;

use crate::ast_walk::visit_elements;
use crate::document::DocumentState;

pub(super) fn variable_completions(state: &DocumentState) -> Vec<CompletionItem> {
    let mut names = BTreeSet::new();
    visit_elements(&state.elements, &mut |element| {
        if let Element::Define(d) = element {
            for name in d.parameters.keys() {
                names.insert(name.clone());
            }
        }
    });
    names
        .into_iter()
        .map(|name| CompletionItem {
            label: name,
            kind: Some(CompletionItemKind::VARIABLE),
            ..Default::default()
        })
        .collect()
}
