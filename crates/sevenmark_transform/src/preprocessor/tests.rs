use super::define_if::{process_defines_and_ifs, process_defines_and_ifs_with_protected_keys};
use super::metadata::collect_metadata;
use super::references::{collect_includes, substitute_includes};
use super::*;
use sevenmark_ast::{
    CategoryElement, DefineElement, FoldElement, FoldInnerElement, IncludeElement, ListContentItem,
    ListElement, ListItemElement, Parameter, Parameters, RedirectElement, Span, TableCellElement,
    TableCellItem, TableElement, TableRowElement, TableRowItem, TextElement, VariableElement,
};

fn span() -> Span {
    Span::synthesized()
}

fn text(value: &str) -> Element {
    Element::Text(TextElement {
        span: span(),
        value: value.to_string(),
    })
}

fn define(key: &str, value: &str) -> Element {
    define_with_value_elements(key, vec![text(value)])
}

fn define_with_value_elements(key: &str, value: Vec<Element>) -> Element {
    let mut parameters = Parameters::new();
    parameters.insert(
        key.to_string(),
        Parameter {
            span: span(),
            key: key.to_string(),
            value,
        },
    );

    Element::Define(DefineElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters,
    })
}

fn variable(name: &str) -> Element {
    Element::Variable(VariableElement {
        span: span(),
        name: name.to_string(),
    })
}

fn add_param(parameters: &mut Parameters, key: &str, value: &str) {
    add_param_elements(parameters, key, vec![text(value)]);
}

fn add_param_elements(parameters: &mut Parameters, key: &str, value: Vec<Element>) {
    parameters.insert(
        key.to_string(),
        Parameter {
            span: span(),
            key: key.to_string(),
            value,
        },
    );
}

fn include(title: &str, namespace: Option<&str>, params: &[(&str, &str)]) -> Element {
    let params_with_elements: Vec<_> = params
        .iter()
        .map(|(k, v)| ((*k).to_string(), vec![text(v)]))
        .collect();
    include_with_params_elements(title, namespace, params_with_elements)
}

fn include_with_params_elements(
    title: &str,
    namespace: Option<&str>,
    params: Vec<(String, Vec<Element>)>,
) -> Element {
    let mut parameters = Parameters::new();
    if let Some(ns) = namespace {
        add_param(&mut parameters, "namespace", ns);
    }
    for (k, v) in params {
        add_param_elements(&mut parameters, &k, v);
    }

    Element::Include(IncludeElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters,
        children: vec![text(title)],
    })
}

fn table_cell(params: Vec<(String, Vec<Element>)>, children: Vec<Element>) -> TableCellItem {
    let mut parameters = Parameters::new();
    for (k, v) in params {
        add_param_elements(&mut parameters, &k, v);
    }

    TableCellItem::Cell(TableCellElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters,
        x: Vec::new(),
        y: Vec::new(),
        children,
    })
}

fn table_row(params: Vec<(String, Vec<Element>)>, cells: Vec<TableCellItem>) -> TableRowItem {
    let mut parameters = Parameters::new();
    for (k, v) in params {
        add_param_elements(&mut parameters, &k, v);
    }

    TableRowItem::Row(TableRowElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters,
        children: cells,
    })
}

fn table(rows: Vec<TableRowItem>) -> Element {
    Element::Table(TableElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters: Parameters::new(),
        children: rows,
    })
}

fn list_item(params: Vec<(String, Vec<Element>)>, children: Vec<Element>) -> ListContentItem {
    let mut parameters = Parameters::new();
    for (k, v) in params {
        add_param_elements(&mut parameters, &k, v);
    }

    ListContentItem::Item(ListItemElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters,
        children,
    })
}

fn list(items: Vec<ListContentItem>) -> Element {
    Element::List(ListElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        kind: "unordered".to_string(),
        parameters: Parameters::new(),
        children: items,
    })
}

fn fold(
    summary_params: Vec<(String, Vec<Element>)>,
    summary_children: Vec<Element>,
    details_params: Vec<(String, Vec<Element>)>,
    details_children: Vec<Element>,
) -> Element {
    let mut summary_parameters = Parameters::new();
    for (k, v) in summary_params {
        add_param_elements(&mut summary_parameters, &k, v);
    }

    let mut details_parameters = Parameters::new();
    for (k, v) in details_params {
        add_param_elements(&mut details_parameters, &k, v);
    }

    Element::Fold(FoldElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters: Parameters::new(),
        summary: FoldInnerElement {
            span: span(),
            open_span: span(),
            close_span: span(),
            parameters: summary_parameters,
            children: summary_children,
        },
        details: FoldInnerElement {
            span: span(),
            open_span: span(),
            close_span: span(),
            parameters: details_parameters,
            children: details_children,
        },
    })
}

fn category(name: &str) -> Element {
    Element::Category(CategoryElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        children: vec![text(name)],
    })
}

fn redirect(title: &str, namespace: Option<&str>) -> Element {
    let mut parameters = Parameters::new();
    if let Some(ns) = namespace {
        add_param(&mut parameters, "namespace", ns);
    }

    Element::Redirect(RedirectElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters,
        children: vec![text(title)],
    })
}

#[test]
fn normalized_plain_text_trims_and_drops_empty() {
    assert_eq!(
        normalized_plain_text(&[text("  Name \n")]),
        Some("Name".to_string())
    );
    assert_eq!(normalized_plain_text(&[text(" \n\t ")]), None);
}

#[test]
fn collect_includes_trims_title_and_namespace() {
    let elements = vec![include("  Template \n", Some(" Document \n"), &[])];
    let mut includes = HashSet::new();

    collect_includes(&elements, &mut includes);

    assert_eq!(includes.len(), 1);
    assert!(includes.contains(&DocumentReference {
        namespace: DocumentNamespace::Document,
        title: "Template".to_string(),
    }));
}

#[test]
fn collect_metadata_trims_category_and_redirect() {
    let elements = vec![
        category(" Programming \n"),
        redirect(" Home \n", Some(" Category \n")),
    ];
    let mut categories = HashSet::new();
    let mut redirect = None;
    let mut media = HashSet::new();
    let mut sections = Vec::new();
    let mut user_mentions = HashSet::new();

    collect_metadata(
        &elements,
        &mut categories,
        &mut redirect,
        &mut media,
        &mut sections,
        &mut user_mentions,
        true,
    );

    assert!(categories.contains("Programming"));
    assert_eq!(
        redirect,
        Some(RedirectReference {
            namespace: DocumentNamespace::Category,
            title: "Home".to_string(),
        })
    );
}

#[test]
fn substitute_includes_uses_trimmed_target_title() {
    let mut elements = vec![include(" Template \n", None, &[("title", "caller")])];
    let docs_map = HashMap::from([(
        DocumentReference {
            namespace: DocumentNamespace::Document,
            title: "Template".to_string(),
        },
        vec![define("title", "template"), variable("title")],
    )]);
    let mut media = HashSet::new();

    substitute_includes(&mut elements, &docs_map, &mut media);

    let include_elem = match &elements[0] {
        Element::Include(include_elem) => include_elem,
        _ => panic!("expected include element"),
    };

    let rendered = include_elem.children.iter().find_map(|el| match el {
        Element::Text(t) => Some(t.value.as_str()),
        _ => None,
    });

    assert_eq!(rendered, Some("caller"));
}

#[test]
fn define_overrides_when_not_protected() {
    let mut elements = vec![define("title", "template"), variable("title")];
    let mut vars = HashMap::new();

    process_defines_and_ifs(&mut elements, &mut vars);

    match &elements[1] {
        Element::Text(t) => assert_eq!(t.value, "template"),
        _ => panic!("variable should be replaced by text"),
    }
}

#[test]
fn protected_key_keeps_include_parameter_value() {
    let mut elements = vec![define("title", "template"), variable("title")];
    let mut vars = HashMap::from([("title".to_string(), "caller".to_string())]);
    let protected = HashSet::from(["title".to_string()]);

    process_defines_and_ifs_with_protected_keys(&mut elements, &mut vars, Some(&protected));

    match &elements[1] {
        Element::Text(t) => assert_eq!(t.value, "caller"),
        _ => panic!("variable should be replaced by text"),
    }
    assert_eq!(vars.get("title").map(String::as_str), Some("caller"));
}

#[test]
fn unprotected_keys_still_use_template_defaults() {
    let mut elements = vec![define("subtitle", "template-sub"), variable("subtitle")];
    let mut vars = HashMap::from([("title".to_string(), "caller".to_string())]);
    let protected = HashSet::from(["title".to_string()]);

    process_defines_and_ifs_with_protected_keys(&mut elements, &mut vars, Some(&protected));

    match &elements[1] {
        Element::Text(t) => assert_eq!(t.value, "template-sub"),
        _ => panic!("variable should be replaced by text"),
    }
}

#[test]
fn define_parameter_can_reference_existing_variable() {
    let mut elements = vec![
        define("name", "caller"),
        define_with_value_elements("title", vec![variable("name")]),
        variable("title"),
    ];
    let mut vars = HashMap::new();

    process_defines_and_ifs(&mut elements, &mut vars);

    match &elements[2] {
        Element::Text(t) => assert_eq!(t.value, "caller"),
        _ => panic!("variable should be replaced by text"),
    }
    assert_eq!(vars.get("title").map(String::as_str), Some("caller"));
}

#[test]
fn include_parameter_variable_is_resolved() {
    let mut elements = vec![
        define("name", "caller"),
        include_with_params_elements(
            "Template",
            None,
            vec![("title".to_string(), vec![variable("name")])],
        ),
    ];
    let docs_map = HashMap::from([(
        DocumentReference {
            namespace: DocumentNamespace::Document,
            title: "Template".to_string(),
        },
        vec![define("title", "template"), variable("title")],
    )]);
    let mut media = HashSet::new();
    let mut vars = HashMap::new();

    process_defines_and_ifs(&mut elements, &mut vars);
    substitute_includes(&mut elements, &docs_map, &mut media);

    let include_elem = match &elements[1] {
        Element::Include(include_elem) => include_elem,
        _ => panic!("expected include element"),
    };

    let rendered = include_elem.children.iter().find_map(|el| match el {
        Element::Text(t) => Some(t.value.as_str()),
        _ => None,
    });

    assert_eq!(rendered, Some("caller"));
}

#[test]
fn table_nested_parameters_follow_document_order() {
    let mut elements = vec![table(vec![
        table_row(
            Vec::new(),
            vec![table_cell(Vec::new(), vec![define("row_cls", "later")])],
        ),
        table_row(
            vec![("class".to_string(), vec![variable("row_cls")])],
            vec![table_cell(Vec::new(), vec![text("content")])],
        ),
    ])];
    let mut vars = HashMap::new();

    process_defines_and_ifs(&mut elements, &mut vars);

    let Element::Table(table_elem) = &elements[0] else {
        panic!("expected table element");
    };
    let TableRowItem::Row(second_row) = &table_elem.children[1] else {
        panic!("expected table row");
    };
    let class_param = second_row
        .parameters
        .get("class")
        .expect("class parameter should exist");
    assert_eq!(
        sevenmark_utils::extract_plain_text(&class_param.value),
        "later"
    );
}

#[test]
fn list_nested_parameters_follow_document_order() {
    let mut elements = vec![list(vec![
        list_item(Vec::new(), vec![define("item_cls", "later")]),
        list_item(
            vec![("class".to_string(), vec![variable("item_cls")])],
            vec![text("content")],
        ),
    ])];
    let mut vars = HashMap::new();

    process_defines_and_ifs(&mut elements, &mut vars);

    let Element::List(list_elem) = &elements[0] else {
        panic!("expected list element");
    };
    let ListContentItem::Item(second_item) = &list_elem.children[1] else {
        panic!("expected list item");
    };
    let class_param = second_item
        .parameters
        .get("class")
        .expect("class parameter should exist");
    assert_eq!(
        sevenmark_utils::extract_plain_text(&class_param.value),
        "later"
    );
}

#[test]
fn fold_nested_parameters_follow_document_order() {
    let mut elements = vec![fold(
        Vec::new(),
        vec![define("detail_cls", "later")],
        vec![("class".to_string(), vec![variable("detail_cls")])],
        vec![text("content")],
    )];
    let mut vars = HashMap::new();

    process_defines_and_ifs(&mut elements, &mut vars);

    let Element::Fold(fold_elem) = &elements[0] else {
        panic!("expected fold element");
    };
    let class_param = fold_elem
        .details
        .parameters
        .get("class")
        .expect("class parameter should exist");
    assert_eq!(
        sevenmark_utils::extract_plain_text(&class_param.value),
        "later"
    );
}
