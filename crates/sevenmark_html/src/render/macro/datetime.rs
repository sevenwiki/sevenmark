use crate::classes;
use maud::{Markup, html};

pub fn render() -> Markup {
    html! { span class=(classes::DATETIME) {} }
}
