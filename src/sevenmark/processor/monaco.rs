use crate::sevenmark::ast::{Location, SevenMarkElement};
use line_span::LineSpanExt;
use serde::Serialize;

/// Location information for Monaco Editor (1-based line/column)
#[derive(Debug, Clone, Serialize)]
pub struct LineColumnLocation {
    pub start_line: usize,   // 1-based line number
    pub start_column: usize, // 1-based column number
    pub end_line: usize,     // 1-based line number
    pub end_column: usize,   // 1-based column number
}

/// Conversion result for Monaco Editor
#[derive(Debug, Serialize)]
pub struct MonacoResult {
    pub elements: serde_json::Value,
}

/// Memory-efficient byte-to-line/column converter using line spans and binary search
///
/// This struct uses O(lines) memory instead of O(bytes) by storing only line start positions
/// and using binary search for O(log n) position lookups. This is much more memory efficient
/// for large documents while maintaining good performance.
struct ByteToLineMapper {
    line_starts: Vec<usize>, // Start byte position of each line (0-based)
    input: String,           // Keep reference to input for column calculation
}

impl ByteToLineMapper {
    /// Creates a new mapper using line_span to efficiently find line boundaries
    ///
    /// Time complexity: O(n) for initial scan
    /// Space complexity: O(lines) - much more efficient than O(bytes)
    fn new(input: &str) -> Self {
        let line_starts: Vec<usize> = input.line_spans().map(|span| span.range().start).collect();

        Self {
            line_starts,
            input: input.to_string(),
        }
    }

    /// Converts byte position to line/column coordinates using binary search
    ///
    /// Time complexity: O(log n) where n is the number of lines
    /// Returns 1-based line and column numbers as expected by Monaco Editor
    fn byte_to_line_column(&self, byte_offset: usize) -> (usize, usize) {
        let clamped_pos = byte_offset.min(self.input.len());

        // Binary search to find the line containing this position
        let line = self
            .line_starts
            .binary_search(&clamped_pos)
            .unwrap_or_else(|index| {
                if index == 0 {
                    0 // Position is before first line (shouldn't happen)
                } else {
                    index - 1 // Position is in the line that starts at index-1
                }
            });

        // Calculate column by subtracting line start from position
        let line_start = self.line_starts[line];
        let column = clamped_pos - line_start;

        // Convert to 1-based as expected by Monaco Editor
        (line + 1, column + 1)
    }
}

/// Monaco Editor location converter with memory-efficient position lookup
///
/// This processor converts SevenMark AST elements with byte-based locations
/// to Monaco Editor-compatible JSON with 1-based line/column positions.
pub struct MonacoVisitor {
    mapper: ByteToLineMapper,
}

impl MonacoVisitor {
    /// Creates a new Monaco processor for the given input text
    pub fn new(input: &str) -> Self {
        let mapper = ByteToLineMapper::new(input);
        Self { mapper }
    }

    /// Converts a byte-based Location to Monaco Editor LineColumnLocation
    pub fn convert_location(&self, location: &Location) -> LineColumnLocation {
        let (start_line, start_column) = self.mapper.byte_to_line_column(location.start);
        let (end_line, end_column) = self.mapper.byte_to_line_column(location.end);

        LineColumnLocation {
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }

    /// Converts SevenMark AST elements to Monaco Editor-compatible JSON
    ///
    /// This method:
    /// 1. Serializes the AST to JSON
    /// 2. Recursively finds all "location" fields
    /// 3. Converts byte positions to line/column positions
    /// 4. Returns the transformed JSON
    pub fn convert_elements(&self, elements: &[SevenMarkElement]) -> MonacoResult {
        // First serialize to regular JSON
        let mut json_value = serde_json::to_value(elements).unwrap_or(serde_json::Value::Null);

        // Transform all location fields in the JSON tree
        self.convert_locations_in_json(&mut json_value);

        MonacoResult {
            elements: json_value,
        }
    }

    /// Recursively transforms location fields in JSON values
    ///
    /// Traverses the entire JSON structure and converts any "location" field
    /// from byte-based Location to line/column-based LineColumnLocation
    fn convert_locations_in_json(&self, value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                // Check for location field and convert it
                if let Some(location_value) = map.get("location") {
                    if let Ok(location) = serde_json::from_value::<Location>(location_value.clone())
                    {
                        let line_col_location = self.convert_location(&location);
                        map.insert(
                            "location".to_string(),
                            serde_json::to_value(line_col_location).unwrap(),
                        );
                    }
                }

                // Recursively process all other fields
                for (_, v) in map.iter_mut() {
                    self.convert_locations_in_json(v);
                }
            }
            serde_json::Value::Array(arr) => {
                for item in arr.iter_mut() {
                    self.convert_locations_in_json(item);
                }
            }
            _ => {}
        }
    }
}

/// Converts SevenMark AST to Monaco Editor-compatible JSON string
///
/// This is the main entry point for converting parsed SevenMark elements
/// to a JSON format suitable for Monaco Editor decorations. All byte-based
/// locations in the AST are converted to 1-based line/column positions.
///
/// # Arguments
/// * `elements` - The parsed SevenMark AST elements
/// * `input` - The original input string used for position mapping
///
/// # Returns
/// Pretty-formatted JSON string with line/column-based locations
pub fn convert_ast_to_monaco_json(elements: &[SevenMarkElement], input: &str) -> String {
    let visitor = MonacoVisitor::new(input);
    let result = visitor.convert_elements(elements);

    serde_json::to_string_pretty(&result.elements).unwrap_or_default()
}
