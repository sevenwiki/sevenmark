use crate::classes;
use maud::{Markup, html};

pub fn render(namespace: Option<&str>) -> Markup {
    html! {
        span class=(classes::PAGECOUNT) data-namespace=[namespace] {}
    }
}
