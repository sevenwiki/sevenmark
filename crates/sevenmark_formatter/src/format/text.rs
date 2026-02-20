use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{MentionType, CommentElement, EscapeElement, ErrorElement, MentionElement, TextElement};

pub fn format_text<'a>(a: &'a Arena<'a>, e: &TextElement) -> DocBuilder<'a, Arena<'a>> {
    a.text(e.value.clone())
}

pub fn format_escape<'a>(a: &'a Arena<'a>, e: &EscapeElement) -> DocBuilder<'a, Arena<'a>> {
    a.text(format!("\\{}", e.value))
}

pub fn format_error<'a>(a: &'a Arena<'a>, e: &ErrorElement) -> DocBuilder<'a, Arena<'a>> {
    a.text(e.value.clone())
}

pub fn format_comment<'a>(a: &'a Arena<'a>, e: &CommentElement) -> DocBuilder<'a, Arena<'a>> {
    // Multiline comments contain newlines; inline comments do not
    if e.value.contains('\n') {
        a.text(format!("/*{}*/", e.value))
    } else {
        a.text(format!("//{}", e.value))
    }
}

pub fn format_mention<'a>(a: &'a Arena<'a>, e: &MentionElement) -> DocBuilder<'a, Arena<'a>> {
    match e.kind {
        MentionType::User => a.text(format!("<@{}>", e.id)),
        MentionType::Discussion => a.text(format!("<#{}>", e.id)),
    }
}
