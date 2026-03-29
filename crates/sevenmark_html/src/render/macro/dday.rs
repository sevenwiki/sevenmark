use crate::classes;
use maud::{Markup, html};

pub fn render(date: &str) -> Markup {
    html! { span class=(classes::DDAY) data-date=(date) {} }
}
