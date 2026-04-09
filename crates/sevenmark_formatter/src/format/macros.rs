use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{AgeElement, AnchorElement, DdayElement, PageCountElement, VariableElement};

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

pub fn format_clear<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[clear]")
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

pub fn format_anchor<'a>(a: &'a Arena<'a>, e: &AnchorElement) -> DocBuilder<'a, Arena<'a>> {
    a.text(format!("[anchor({})]", e.name))
}

pub fn format_toc<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[toc]")
}

pub fn format_date<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[date]")
}

pub fn format_datetime<'a>(a: &'a Arena<'a>) -> DocBuilder<'a, Arena<'a>> {
    a.text("[datetime]")
}

pub fn format_dday<'a>(a: &'a Arena<'a>, e: &DdayElement) -> DocBuilder<'a, Arena<'a>> {
    a.text(format!("[dday({})]", e.date))
}

pub fn format_pagecount<'a>(a: &'a Arena<'a>, e: &PageCountElement) -> DocBuilder<'a, Arena<'a>> {
    match &e.namespace {
        Some(ns) => a.text(format!("[pagecount({})]", ns)),
        None => a.text("[pagecount]"),
    }
}
