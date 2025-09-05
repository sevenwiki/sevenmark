use crate::sevenmark::ast::{Location, SevenMarkElement};
use line_span::LineSpanExt;
use serde::Serialize;

/// Location information for Monaco Editor (1-based line/column)
#[derive(Debug, Clone, Serialize)]
pub struct LineColumnLocation {
    pub start_line: usize,    // 1-based line number
    pub start_column: usize,  // 1-based column number
    pub end_line: usize,      // 1-based line number
    pub end_column: usize,    // 1-based column number
}

/// Conversion result for Monaco Editor
#[derive(Debug, Serialize)]
pub struct MonacoResult {
    pub elements: serde_json::Value,
}

/// O(1) byte-to-line/column converter using pre-computed mapping arrays
/// 
/// This struct trades memory for speed by pre-computing two arrays that map
/// every byte position in the input string to its corresponding line and column.
/// This enables O(1) position lookups, which is crucial for efficient Monaco Editor
/// decoration when dealing with large documents.
struct ByteToLineMapper {
    byte_to_line: Vec<usize>,     // Maps each byte position → line number (0-based)
    byte_to_column: Vec<usize>,   // Maps each byte position → column number (0-based)
}

impl ByteToLineMapper {
    /// Creates a new mapper by pre-computing line/column positions for every byte
    /// 
    /// Time complexity: O(n) where n is the length of input in bytes
    /// Space complexity: O(n) - stores two arrays of size input.len() + 1
    fn new(input: &str) -> Self {
        let mut byte_to_line = vec![0; input.len() + 1];
        let mut byte_to_column = vec![0; input.len() + 1];
        
        let mut current_line = 0;
        let mut current_column = 0;
        let mut byte_pos = 0;
        
        // Iterate through each Unicode character
        for ch in input.chars() {
            let char_byte_len = ch.len_utf8();
            
            // Assign the same line/column to all bytes of this character
            // This handles multi-byte UTF-8 characters correctly
            for i in 0..char_byte_len {
                if byte_pos + i < byte_to_line.len() {
                    byte_to_line[byte_pos + i] = current_line;
                    byte_to_column[byte_pos + i] = current_column;
                }
            }
            
            // Update position counters
            if ch == '\n' {
                current_line += 1;
                current_column = 0;
            } else {
                current_column += 1;
            }
            
            byte_pos += char_byte_len;
        }
        
        // Handle the final position (end of input)
        if byte_pos < byte_to_line.len() {
            byte_to_line[byte_pos] = current_line;
            byte_to_column[byte_pos] = current_column;
        }
        
        Self {
            byte_to_line,
            byte_to_column,
        }
    }
    
    /// Converts byte position to line/column coordinates in O(1) time
    /// 
    /// Returns 1-based line and column numbers as expected by Monaco Editor
    fn byte_to_line_column(&self, byte_offset: usize) -> (usize, usize) {
        let safe_offset = byte_offset.min(self.byte_to_line.len() - 1);
        let line = self.byte_to_line[safe_offset] + 1;     // Convert to 1-based
        let column = self.byte_to_column[safe_offset] + 1; // Convert to 1-based
        (line, column)
    }
}

/// Monaco Editor location converter with O(1) position lookup optimization
/// 
/// This visitor converts SevenMark AST elements with byte-based locations
/// to Monaco Editor-compatible JSON with 1-based line/column positions.
pub struct MonacoVisitor {
    mapper: ByteToLineMapper,
}

impl MonacoVisitor {
    /// Creates a new Monaco visitor for the given input text
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
                    if let Ok(location) = serde_json::from_value::<Location>(location_value.clone()) {
                        let line_col_location = self.convert_location(&location);
                        map.insert("location".to_string(), serde_json::to_value(line_col_location).unwrap());
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