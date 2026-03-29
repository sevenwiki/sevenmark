use maud::{Markup, html};

use crate::classes;

pub fn render(name: &str) -> Markup {
    html! {
        span id=(name) class=(classes::ANCHOR) {}
    }
}
