use maud::{Markup, html};
use crate::classes;

pub fn render(namespace: Option<&str>) -> Markup {
    html! {
        span class=(classes::PAGECOUNT) data-namespace=[namespace] {}
    }
}