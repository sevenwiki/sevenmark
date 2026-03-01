//! Clear macro rendering

use maud::{Markup, html};

use crate::classes;

pub fn render() -> Markup {
    html! {
        div class=(classes::CLEAR) style="clear: both;" {}
    }
}
