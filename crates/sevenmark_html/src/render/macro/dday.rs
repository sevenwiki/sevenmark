use maud::{Markup, html};
use crate::classes;

pub fn render(date: &str) -> Markup {
    html! { span class=(classes::DDAY) data-date=(date) {} }
}