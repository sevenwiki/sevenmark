//! Render configuration

/// Configuration for HTML rendering
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderConfig<'a> {
    /// Edit URL for section edit links.
    /// - `Some(url)` - Render edit links with this base URL
    /// - `None` - Don't render edit links (for discussions, etc.)
    pub edit_url: Option<&'a str>,
    /// Base URL for file/media (e.g., Cloudflare CDN URL)
    pub file_base_url: Option<&'a str>,
    /// Base URL for document links (e.g., "/Document/")
    pub document_base_url: Option<&'a str>,
    /// Base URL for category links (e.g., "/Category/")
    pub category_base_url: Option<&'a str>,
}

/// Config for discussion rendering (no edit links)
pub const DISCUSSION_CONFIG: RenderConfig<'static> = RenderConfig {
    edit_url: None,
    file_base_url: None,
    document_base_url: None,
    category_base_url: None,
};
