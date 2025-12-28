//! Mention element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::{MentionElement, MentionType};

use crate::classes;

pub fn render(e: &MentionElement) -> Markup {
    let class = match e.mention_type {
        MentionType::User => classes::MENTION_USER,
        MentionType::Discussion => classes::MENTION_DISCUSSION,
    };

    html! {
        span class=(class) data-uuid=(&e.uuid) {}
    }
}
