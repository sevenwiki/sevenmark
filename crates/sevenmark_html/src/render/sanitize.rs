//! CSS sanitization policy for SevenMark
//!
//! Fail-closed policy: allow a curated set of presentation and layout
//! properties while still blocking overlay primitives and dynamic or
//! external-value constructs such as url(), var(), env(), and expression().
//! Stylesheet selectors remain class-scoped.

use std::collections::HashSet;
use std::convert::Infallible;
use std::sync::LazyLock;

use css_sanitizer::lightningcss::properties::Property;
use css_sanitizer::lightningcss::properties::custom::{EnvironmentVariable, Function, Variable};
use css_sanitizer::lightningcss::rules::CssRule;
use css_sanitizer::lightningcss::selector::{Component, SelectorList};
use css_sanitizer::lightningcss::values::image::Image;
use css_sanitizer::lightningcss::values::url::Url;
use css_sanitizer::lightningcss::visitor::{Visit, VisitTypes, Visitor};
use css_sanitizer::{
    CssSanitizationPolicy, NodeAction, PropertyContext, RuleContext, SelectorContext,
    clean_declaration_list_with_policy, clean_stylesheet_with_policy,
};

// ---------------------------------------------------------------------------
// Property whitelist
// ---------------------------------------------------------------------------

static ALLOWED_PROPERTIES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // Colors & visibility
        "color",
        "opacity",
        "visibility",
        // Background (url() blocked separately by value scan)
        "background",
        "background-color",
        "background-size",
        "background-position",
        "background-repeat",
        "background-clip",
        "background-origin",
        "background-attachment",
        // Typography
        "font",
        "font-size",
        "font-weight",
        "font-style",
        "font-family",
        "font-variant",
        "font-stretch",
        "line-height",
        "letter-spacing",
        "word-spacing",
        "word-break",
        "word-wrap",
        "overflow-wrap",
        "text-align",
        "text-decoration",
        "text-decoration-color",
        "text-decoration-line",
        "text-decoration-style",
        "text-decoration-thickness",
        "text-indent",
        "text-transform",
        "text-shadow",
        "text-overflow",
        "text-underline-offset",
        "white-space",
        "vertical-align",
        "writing-mode",
        "tab-size",
        // Margin
        "margin",
        "margin-top",
        "margin-right",
        "margin-bottom",
        "margin-left",
        "margin-block",
        "margin-block-start",
        "margin-block-end",
        "margin-inline",
        "margin-inline-start",
        "margin-inline-end",
        // Padding
        "padding",
        "padding-top",
        "padding-right",
        "padding-bottom",
        "padding-left",
        "padding-block",
        "padding-block-start",
        "padding-block-end",
        "padding-inline",
        "padding-inline-start",
        "padding-inline-end",
        // Sizing
        "width",
        "height",
        "min-width",
        "min-height",
        "max-width",
        "max-height",
        "inline-size",
        "block-size",
        "min-inline-size",
        "max-inline-size",
        "min-block-size",
        "max-block-size",
        "aspect-ratio",
        // Border
        "border",
        "border-top",
        "border-right",
        "border-bottom",
        "border-left",
        "border-color",
        "border-top-color",
        "border-right-color",
        "border-bottom-color",
        "border-left-color",
        "border-width",
        "border-top-width",
        "border-right-width",
        "border-bottom-width",
        "border-left-width",
        "border-style",
        "border-top-style",
        "border-right-style",
        "border-bottom-style",
        "border-left-style",
        "border-radius",
        "border-top-left-radius",
        "border-top-right-radius",
        "border-bottom-left-radius",
        "border-bottom-right-radius",
        "border-inline",
        "border-block",
        "border-inline-start",
        "border-inline-end",
        "border-block-start",
        "border-block-end",
        "border-inline-start-color",
        "border-inline-end-color",
        "border-block-start-color",
        "border-block-end-color",
        "border-inline-start-width",
        "border-inline-end-width",
        "border-block-start-width",
        "border-block-end-width",
        "border-inline-start-style",
        "border-inline-end-style",
        "border-block-start-style",
        "border-block-end-style",
        "border-inline-color",
        "border-block-color",
        "border-inline-width",
        "border-block-width",
        "border-inline-style",
        "border-block-style",
        "border-collapse",
        "border-spacing",
        // Outline
        "outline",
        "outline-color",
        "outline-style",
        "outline-width",
        "outline-offset",
        // Box
        "box-shadow",
        "box-sizing",
        // List
        "list-style",
        "list-style-type",
        "list-style-position",
        // Table
        "table-layout",
        "caption-side",
        "empty-cells",
        // Cursor (url() blocked separately)
        "cursor",
        // Multi-column
        "columns",
        "column-count",
        "column-width",
        "column-rule",
        "column-rule-color",
        "column-rule-style",
        "column-rule-width",
        // Layout
        "display",
        "overflow",
        "overflow-x",
        "overflow-y",
        "gap",
        "row-gap",
        "column-gap",
        "flex",
        "flex-direction",
        "flex-wrap",
        "flex-flow",
        "flex-grow",
        "flex-shrink",
        "flex-basis",
        "order",
        "justify-content",
        "justify-items",
        "justify-self",
        "align-content",
        "align-items",
        "align-self",
        "place-content",
        "place-items",
        "place-self",
        "grid",
        "grid-template",
        "grid-template-columns",
        "grid-template-rows",
        "grid-template-areas",
        "grid-auto-flow",
        "grid-auto-columns",
        "grid-auto-rows",
        "grid-column",
        "grid-column-start",
        "grid-column-end",
        "grid-row",
        "grid-row-start",
        "grid-row-end",
        "grid-area",
        // Misc
        "object-fit",
        "object-position",
    ])
});

// ---------------------------------------------------------------------------
// Property value security scan (detects dangerous CSS functions)
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct PropertySecurityScan {
    has_expression: bool,
    has_url: bool,
    has_var: bool,
    has_env: bool,
}

impl PropertySecurityScan {
    fn inspect(property: &mut Property<'_>) -> Self {
        let mut scan = Self::default();
        property
            .visit(&mut scan)
            .expect("property security scan should not fail");
        scan
    }

    fn scan_image(&mut self, image: &Image<'_>) {
        match image {
            Image::Url(_) => {
                self.has_url = true;
            }
            Image::ImageSet(image_set) => {
                for option in &image_set.options {
                    self.scan_image(&option.image);
                }
            }
            Image::Gradient(_) | Image::None => {}
        }
    }

    fn is_dangerous(&self) -> bool {
        self.has_url || self.has_var || self.has_env || self.has_expression
    }
}

impl<'i> Visitor<'i> for PropertySecurityScan {
    type Error = Infallible;

    fn visit_types(&self) -> VisitTypes {
        VisitTypes::URLS
            | VisitTypes::IMAGES
            | VisitTypes::VARIABLES
            | VisitTypes::ENVIRONMENT_VARIABLES
            | VisitTypes::FUNCTIONS
    }

    fn visit_url(&mut self, _url: &mut Url<'i>) -> Result<(), Self::Error> {
        self.has_url = true;
        Ok(())
    }

    fn visit_image(&mut self, image: &mut Image<'i>) -> Result<(), Self::Error> {
        self.scan_image(image);
        image.visit_children(self)
    }

    fn visit_variable(&mut self, variable: &mut Variable<'i>) -> Result<(), Self::Error> {
        self.has_var = true;
        variable.visit_children(self)
    }

    fn visit_environment_variable(
        &mut self,
        environment_variable: &mut EnvironmentVariable<'i>,
    ) -> Result<(), Self::Error> {
        self.has_env = true;
        environment_variable.visit_children(self)
    }

    fn visit_function(&mut self, function: &mut Function<'i>) -> Result<(), Self::Error> {
        if function.name.0.eq_ignore_ascii_case("expression") {
            self.has_expression = true;
        }
        function.visit_children(self)
    }
}

// ---------------------------------------------------------------------------
// SevenMark CSS sanitization policy
// ---------------------------------------------------------------------------

struct SevenmarkStylePolicy;

impl CssSanitizationPolicy for SevenmarkStylePolicy {
    fn visit_rule(&self, rule: &mut CssRule<'_>, _ctx: RuleContext) -> NodeAction {
        match rule {
            CssRule::Style(_)
            | CssRule::Media(_)
            | CssRule::Supports(_)
            | CssRule::Nesting(_)
            | CssRule::NestedDeclarations(_) => NodeAction::Continue,
            _ => NodeAction::Drop,
        }
    }

    fn visit_selector_list(
        &self,
        selectors: &mut SelectorList<'_>,
        _ctx: SelectorContext,
    ) -> NodeAction {
        let all_safe = selectors.0.iter().all(|selector| {
            let mut has_class = false;
            let mut has_forbidden = false;

            for component in selector.iter_raw_match_order() {
                match component {
                    Component::Class(_) | Component::Nesting => {
                        has_class = true;
                    }
                    Component::ID(_)
                    | Component::LocalName(_)
                    | Component::ExplicitUniversalType => {
                        has_forbidden = true;
                    }
                    _ => {}
                }
            }

            has_class && !has_forbidden
        });

        if all_safe {
            NodeAction::Continue
        } else {
            NodeAction::Drop
        }
    }

    fn visit_property(&self, property: &mut Property<'_>, ctx: PropertyContext) -> NodeAction {
        if ctx.important {
            return NodeAction::Drop;
        }

        let property_id = property.property_id();
        let name = property_id.name();
        if !ALLOWED_PROPERTIES.contains(name) {
            return NodeAction::Drop;
        }

        if PropertySecurityScan::inspect(property).is_dangerous() {
            return NodeAction::Drop;
        }

        NodeAction::Continue
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

static POLICY: SevenmarkStylePolicy = SevenmarkStylePolicy;

/// Sanitize a full CSS stylesheet (`{{{#css}}}` blocks).
pub(crate) fn sanitize_css_block(input: &str) -> String {
    clean_stylesheet_with_policy(input, &POLICY)
}

/// Sanitize inline CSS declarations (`style="..."` attributes).
pub(crate) fn sanitize_inline_style(input: &str) -> String {
    clean_declaration_list_with_policy(input, &POLICY)
}

#[cfg(test)]
mod tests;
