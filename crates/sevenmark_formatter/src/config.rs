/// Configuration for the SevenMark formatter.
pub struct FormatConfig {
    /// Maximum line width before the pretty printer breaks lines.
    pub width: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self { width: 80 }
    }
}
