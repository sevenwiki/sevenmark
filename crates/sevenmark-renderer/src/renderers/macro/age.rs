//! Age macro renderer

use crate::context::RenderContext;
use chrono::NaiveDate;
use maud::{Markup, html};
use sevenmark_parser::ast::AgeElement;

/// Render [age(date)] macro - calculate days elapsed since date
pub fn render_age(elem: &AgeElement, ctx: &RenderContext) -> Markup {
    let days = calculate_days_elapsed(&elem.content, ctx);
    html! { (days) }
}

fn calculate_days_elapsed(date_str: &str, ctx: &RenderContext) -> String {
    // Try to parse the date string
    let target_date = parse_date(date_str);

    match target_date {
        Some(target) => {
            let today = ctx.now.date_naive();
            let duration = today.signed_duration_since(target);
            duration.num_days().to_string()
        }
        None => "?".to_string(),
    }
}

fn parse_date(s: &str) -> Option<NaiveDate> {
    // Try various date formats
    let formats = [
        "%Y-%m-%d", // 2000-01-15
        "%Y/%m/%d", // 2000/01/15
        "%Y.%m.%d", // 2000.01.15
        "%d-%m-%Y", // 15-01-2000
        "%d/%m/%Y", // 15/01/2000
    ];

    for fmt in formats {
        if let Ok(date) = NaiveDate::parse_from_str(s.trim(), fmt) {
            return Some(date);
        }
    }

    None
}
