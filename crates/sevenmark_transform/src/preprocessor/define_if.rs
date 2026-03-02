use crate::expression_evaluator::evaluate_condition;
use sevenmark_ast::{
    Element, ListContentItem, Span, TableCellItem, TableRowItem, TextElement, Traversable,
};
use std::collections::{HashMap, HashSet};

pub(super) fn process_defines_and_ifs(
    elements: &mut Vec<Element>,
    variables: &mut HashMap<String, String>,
) {
    process_defines_and_ifs_with_protected_keys(elements, variables, None);
}

pub(super) fn process_defines_and_ifs_with_protected_keys(
    elements: &mut Vec<Element>,
    variables: &mut HashMap<String, String>,
    protected_keys: Option<&HashSet<String>>,
) {
    let mut i = 0;
    while i < elements.len() {
        // 1. Define: variable registration
        if let Element::Define(define_elem) = &mut elements[i] {
            for (key, param) in &mut define_elem.parameters {
                // Allow define values to reference already-known variables.
                substitute_variables_in_elements(&mut param.value, variables);
                let value = sevenmark_utils::extract_plain_text(&param.value);
                if !value.is_empty() {
                    if protected_keys.is_some_and(|keys| keys.contains(key)) {
                        continue;
                    }
                    variables.insert(key.clone(), value);
                }
            }
            i += 1;
            continue;
        }

        // Resolve variables inside parameter values before element-level logic.
        elements[i].for_each_parameter_value_vec(&mut |vec| {
            substitute_variables_in_elements(vec, variables);
        });

        // 2. Variable: substitution
        if let Element::Variable(var_elem) = &elements[i] {
            if let Some(value) = variables.get(&var_elem.name) {
                elements[i] = Element::Text(TextElement {
                    span: Span::synthesized(),
                    value: value.clone(),
                });
            }
            i += 1;
            continue;
        }

        // 3. If: evaluate and expand/remove
        if let Element::If(if_elem) = &elements[i] {
            if evaluate_condition(&if_elem.condition, variables) {
                let content = if_elem.children.clone();
                elements.splice(i..i + 1, content);
            } else {
                elements.remove(i);
            }
            continue;
        }

        // 4. Table: process row/cell-level conditionals
        if let Element::Table(table_elem) = &mut elements[i] {
            process_table_conditionals(&mut table_elem.children, variables, protected_keys);
            i += 1;
            continue;
        }

        // 5. List: process item-level conditionals
        if let Element::List(list_elem) = &mut elements[i] {
            process_list_conditionals(&mut list_elem.children, variables, protected_keys);
            i += 1;
            continue;
        }

        // 6. Others: recurse into children
        elements[i].for_each_children_vec(&mut |vec| {
            process_defines_and_ifs_with_protected_keys(vec, variables, protected_keys);
        });

        i += 1;
    }
}

fn process_table_conditionals(
    rows: &mut Vec<TableRowItem>,
    variables: &mut HashMap<String, String>,
    protected_keys: Option<&HashSet<String>>,
) {
    let mut i = 0;
    while i < rows.len() {
        match &mut rows[i] {
            TableRowItem::Row(row) => {
                process_table_cell_conditionals(&mut row.children, variables, protected_keys);
                i += 1;
            }
            TableRowItem::Conditional(cond) => {
                if evaluate_condition(&cond.condition, variables) {
                    let expanded: Vec<TableRowItem> = std::mem::take(&mut cond.rows)
                        .into_iter()
                        .map(TableRowItem::Row)
                        .collect();
                    rows.splice(i..i + 1, expanded);
                } else {
                    rows.remove(i);
                }
            }
        }
    }
}

fn process_table_cell_conditionals(
    cells: &mut Vec<TableCellItem>,
    variables: &mut HashMap<String, String>,
    protected_keys: Option<&HashSet<String>>,
) {
    let mut i = 0;
    while i < cells.len() {
        match &mut cells[i] {
            TableCellItem::Cell(cell) => {
                substitute_variables_in_elements(&mut cell.x, variables);
                substitute_variables_in_elements(&mut cell.y, variables);
                process_defines_and_ifs_with_protected_keys(
                    &mut cell.children,
                    variables,
                    protected_keys,
                );
                i += 1;
            }
            TableCellItem::Conditional(cond) => {
                if evaluate_condition(&cond.condition, variables) {
                    let expanded: Vec<TableCellItem> = std::mem::take(&mut cond.cells)
                        .into_iter()
                        .map(TableCellItem::Cell)
                        .collect();
                    cells.splice(i..i + 1, expanded);
                } else {
                    cells.remove(i);
                }
            }
        }
    }
}

fn process_list_conditionals(
    items: &mut Vec<ListContentItem>,
    variables: &mut HashMap<String, String>,
    protected_keys: Option<&HashSet<String>>,
) {
    let mut i = 0;
    while i < items.len() {
        match &mut items[i] {
            ListContentItem::Item(item) => {
                process_defines_and_ifs_with_protected_keys(
                    &mut item.children,
                    variables,
                    protected_keys,
                );
                i += 1;
            }
            ListContentItem::Conditional(cond) => {
                if evaluate_condition(&cond.condition, variables) {
                    let expanded: Vec<ListContentItem> = std::mem::take(&mut cond.items)
                        .into_iter()
                        .map(ListContentItem::Item)
                        .collect();
                    items.splice(i..i + 1, expanded);
                } else {
                    items.remove(i);
                }
            }
        }
    }
}

fn substitute_variables_in_elements(elements: &mut [Element], variables: &HashMap<String, String>) {
    for element in elements {
        substitute_variables_in_element(element, variables);
    }
}

fn substitute_variables_in_element(element: &mut Element, variables: &HashMap<String, String>) {
    if let Element::Variable(var_elem) = element {
        if let Some(value) = variables.get(&var_elem.name) {
            *element = Element::Text(TextElement {
                span: Span::synthesized(),
                value: value.clone(),
            });
        }
        return;
    }

    element.traverse_children(&mut |child| {
        substitute_variables_in_element(child, variables);
    });
}
