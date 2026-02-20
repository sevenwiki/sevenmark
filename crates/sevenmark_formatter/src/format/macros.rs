use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{AgeElement, VariableElement};

pub fn format_null<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[null]")
}

pub fn format_footnote_ref<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[fn]")
}

pub fn format_time_now<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[now]")
}

pub fn format_hard_break<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[br]")
}

pub fn format_hline<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("----")
}

pub fn format_variable<'a>(a: &'a Arena<'a>, e: &VariableElement) -> DocBuilder<'a, Arena<'a>> {
    a.text(format!("[var({})]", e.name))
}

pub fn format_age<'a>(a: &'a Arena<'a>, e: &AgeElement) -> DocBuilder<'a, Arena<'a>> {
    a.text(format!("[age({})]", e.date))
}
