use super::*;

#[test]
fn allows_safe_properties() {
    let result = sanitize_inline_style("color: red; font-size: 16px; margin: 10px");
    assert!(result.contains("color"));
    assert!(result.contains("font-size"));
    assert!(result.contains("margin"));
}

#[test]
fn allows_layout_properties() {
    let result = sanitize_inline_style(
        "display: grid; float: right; clear: both; overflow: hidden; grid-template-columns: 1fr 1fr; \
         flex-direction: row; place-items: center",
    );
    assert!(result.contains("display"));
    assert!(result.contains("float"));
    assert!(result.contains("clear"));
    assert!(result.contains("overflow"));
    assert!(result.contains("grid-template-columns"));
    assert!(result.contains("flex-direction"));
    assert!(result.contains("place-items"));
}

#[test]
fn blocks_position() {
    assert_eq!(sanitize_inline_style("position: fixed"), "");
}

#[test]
fn blocks_inset() {
    assert_eq!(sanitize_inline_style("inset: 0"), "");
}

#[test]
fn blocks_z_index() {
    assert_eq!(sanitize_inline_style("z-index: 999999"), "");
}

#[test]
fn blocks_pointer_events() {
    assert_eq!(sanitize_inline_style("pointer-events: none"), "");
}

#[test]
fn strips_dangerous_keeps_safe() {
    let result = sanitize_inline_style(
        "color: red; display: inline-block; font-size: 16px; position: fixed; background: url(evil.png)",
    );
    assert!(result.contains("color"));
    assert!(result.contains("display"));
    assert!(result.contains("font-size"));
    assert!(!result.contains("position"));
    assert!(!result.contains("background"));
    assert!(!result.contains("evil.png"));
}

#[test]
fn blocks_important() {
    assert_eq!(sanitize_inline_style("color: red !important"), "");
}

#[test]
fn blocks_url_function() {
    assert_eq!(sanitize_inline_style("background: url(evil.png)"), "");
}

#[test]
fn blocks_var_function() {
    assert_eq!(sanitize_inline_style("color: var(--theme-color)"), "");
}

#[test]
fn blocks_env_function() {
    assert_eq!(
        sanitize_inline_style("padding: env(safe-area-inset-top)"),
        ""
    );
}

#[test]
fn blocks_expression_function() {
    assert_eq!(sanitize_inline_style("width: expression(alert(1))"), "");
}

#[test]
fn allows_gradient_in_background() {
    let result = sanitize_inline_style("background: linear-gradient(to right, red, blue)");
    assert!(result.contains("linear-gradient"));
}

#[test]
fn stylesheet_allows_class_selector() {
    let result = sanitize_css_block(".card { color: red }");
    assert!(result.contains(".card"));
    assert!(result.contains("color"));
}

#[test]
fn stylesheet_blocks_bare_tag_selector() {
    assert_eq!(sanitize_css_block("div { color: red }"), "");
}

#[test]
fn stylesheet_blocks_universal_selector() {
    assert_eq!(sanitize_css_block("* { margin: 0 }"), "");
}

#[test]
fn stylesheet_blocks_id_selector() {
    assert_eq!(sanitize_css_block("#main { color: red }"), "");
}

#[test]
fn stylesheet_drops_rule_with_mixed_selectors() {
    assert_eq!(sanitize_css_block(".safe, div { color: red }"), "");
}

#[test]
fn stylesheet_blocks_class_with_tag_descendant() {
    assert_eq!(sanitize_css_block(".parent p { color: red }"), "");
}

#[test]
fn stylesheet_allows_class_with_class_descendant() {
    let result = sanitize_css_block(".parent .child { color: red }");
    assert!(result.contains(".parent"));
    assert!(result.contains(".child"));
}

#[test]
fn stylesheet_allows_class_with_pseudo() {
    let result = sanitize_css_block(".link:hover { color: blue }");
    assert!(result.contains(":hover"));
    assert!(result.contains("color"));
}

#[test]
fn stylesheet_drops_import() {
    let result = sanitize_css_block("@import url('evil.css'); .card { color: red }");
    assert!(!result.contains("@import"));
    assert!(result.contains(".card"));
    assert!(result.contains("color"));
}

#[test]
fn stylesheet_drops_keyframes() {
    assert_eq!(
        sanitize_css_block("@keyframes slide { from { opacity: 0 } to { opacity: 1 } }"),
        ""
    );
}

#[test]
fn stylesheet_drops_font_face() {
    assert_eq!(
        sanitize_css_block("@font-face { font-family: Evil; src: url(evil.woff) }"),
        ""
    );
}

#[test]
fn stylesheet_preserves_media() {
    let result = sanitize_css_block("@media (max-width: 600px) { .card { color: red } }");
    assert!(result.contains("@media"));
    assert!(result.contains(".card"));
}

#[test]
fn stylesheet_media_drops_nested_bare_tag_selector() {
    let result = sanitize_css_block(
        "@media (max-width: 600px) { body { color: red } .card { color: blue } }",
    );
    assert!(result.contains("@media"));
    assert!(result.contains(".card"));
    assert!(!result.contains("body"));
}

#[test]
fn stylesheet_preserves_supports() {
    let result = sanitize_css_block("@supports (color: red) { .card { color: red } }");
    assert!(result.contains("@supports"));
    assert!(result.contains(".card"));
}

#[test]
fn stylesheet_supports_strips_nested_dangerous_property() {
    let result = sanitize_css_block(
        "@supports (color: red) { .card { color: red; position: fixed; display: grid } }",
    );
    assert!(result.contains("@supports"));
    assert!(result.contains(".card"));
    assert!(result.contains("color"));
    assert!(result.contains("display"));
    assert!(!result.contains("position"));
}

#[test]
fn stylesheet_strips_dangerous_values_in_rule() {
    let result =
        sanitize_css_block(".evil { display: grid; color: red; background: url(evil.png) }");
    assert!(result.contains("color"));
    assert!(result.contains("display"));
    assert!(!result.contains("background"));
    assert!(!result.contains("evil.png"));
}

#[test]
fn full_overlay_attack_neutralized() {
    let attack = r#".red {
        position: fixed;
        inset: 0;
        display: grid;
        place-items: center;
        background: rgba(37, 99, 235, 0.9);
        color: white;
        z-index: 999999;
        font-size: 32px;
    }"#;

    let result = sanitize_css_block(attack);
    assert!(result.contains("display"));
    assert!(result.contains("place-items"));
    assert!(!result.contains("position"));
    assert!(!result.contains("inset"));
    assert!(!result.contains("z-index"));
    assert!(result.contains("color"));
    assert!(result.contains("font-size"));
}
