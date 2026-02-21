/// Configuration for the SevenMark formatter.
pub struct FormatConfig {
    /// Maximum line width before the pretty printer breaks lines.
    pub width: usize,
    /// Indentation width (spaces) for structural blocks.
    pub indent: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            width: 80,
            indent: 2,
        }
    }
}
