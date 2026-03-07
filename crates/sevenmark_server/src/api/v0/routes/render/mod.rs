use std::collections::HashSet;

pub mod openapi;
pub mod render_discussion;
pub mod render_document;
pub mod routes;

fn sort_strings(values: HashSet<String>) -> Vec<String> {
    let mut values: Vec<_> = values.into_iter().collect();
    values.sort();
    values
}
