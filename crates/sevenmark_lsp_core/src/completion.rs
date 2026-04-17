mod context;
mod items;
mod params;
mod variables;

use ls_types::{CompletionItem, Position};

use crate::document::DocumentState;

use context::context_and_bracket_depth;
use items::{
    brace_hash_completions, bracket_completions_ctx, bracket_hash_completions_ctx,
    macro_completions,
};
use params::parameter_completions;
use variables::variable_completions;

/// Returns completion items based on the cursor context.
pub fn get_completions(
    state: &DocumentState,
    position: Position,
    byte_offset: usize,
) -> Vec<CompletionItem> {
    let prefix = &state.text[..byte_offset];

    if prefix.ends_with("[var(") {
        return variable_completions(state);
    }

    let ctx = context_and_bracket_depth(prefix);

    if prefix.ends_with("{{{#") {
        return brace_hash_completions(ctx);
    }

    if prefix.ends_with("[[#") {
        return bracket_hash_completions_ctx(ctx, position);
    }

    if prefix.ends_with("[[") {
        return bracket_completions_ctx(ctx, position);
    }

    if prefix.ends_with('#') {
        if let Some(items) = parameter_completions(prefix, ctx) {
            return items;
        }
    }

    if prefix.ends_with('[') {
        return macro_completions(position);
    }

    Vec::new()
}

#[cfg(test)]
mod tests;
