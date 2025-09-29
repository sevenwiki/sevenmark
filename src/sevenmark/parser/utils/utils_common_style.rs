use crate::sevenmark::{CommonStyleAttributes, Parameters};

pub fn utils_get_common_style(parameters: Parameters) -> CommonStyleAttributes {
    let style = parameters
        .get("style")
        .map(|p| p.value.clone())
        .unwrap_or_else(Vec::new);
    let size = parameters
        .get("size")
        .map(|p| p.value.clone())
        .unwrap_or_else(Vec::new);
    let color = parameters
        .get("color")
        .map(|p| p.value.clone())
        .unwrap_or_else(Vec::new);
    let bg_color = parameters
        .get("bgcolor")
        .map(|p| p.value.clone())
        .unwrap_or_else(Vec::new);
    let opacity = parameters
        .get("opacity")
        .map(|p| p.value.clone())
        .unwrap_or_else(Vec::new);

    CommonStyleAttributes {
        style,
        size,
        color,
        bg_color,
        opacity,
    }
}
