use line_span::LineSpanExt;
use std::fs;

fn main() {
    let input = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let normalized_input = input.replace("\r\n", "\n");
    
    println!("Input:");
    for (i, line) in normalized_input.lines().enumerate() {
        println!("{}: {}", i + 1, line);
    }
    
    println!("\nLine spans:");
    for (i, span) in normalized_input.line_spans().enumerate() {
        println!("Line {}: byte range {:?}, content: {:?}", 
                 i + 1, 
                 span.range(), 
                 &normalized_input[span.range()]);
    }
    
    println!("\nLine starts (bytes):");
    let line_starts: Vec<usize> = normalized_input
        .line_spans()
        .map(|span| span.range().start)
        .collect();
    for (i, start) in line_starts.iter().enumerate() {
        println!("Line {}: starts at byte {}", i + 1, start);
    }
    
    // O(1) 바이트→줄 매핑 테스트
    println!("\nO(1) 바이트→줄 매핑 테스트:");
    let mapper = ByteToLineMapper::new(&normalized_input);
    
    for test_byte in [27, 36, 59, 68, 0, 13, 91] {
        let (line, column) = mapper.byte_to_line_column(test_byte);
        println!("Byte {}: Line {}, Column {}", test_byte, line, column);
    }
}

struct ByteToLineMapper {
    byte_to_line: Vec<usize>,     // 모든 바이트 위치 → 줄 번호 (0-based)
    byte_to_column: Vec<usize>,   // 모든 바이트 위치 → 컬럼 번호 (0-based)
}

impl ByteToLineMapper {
    fn new(input: &str) -> Self {
        let mut byte_to_line = vec![0; input.len() + 1];
        let mut byte_to_column = vec![0; input.len() + 1];
        
        let mut current_line = 0;
        let mut current_column = 0;
        let mut byte_pos = 0;
        
        for ch in input.chars() {
            let char_byte_len = ch.len_utf8();
            
            // 현재 문자의 모든 바이트에 같은 line/column 할당
            for i in 0..char_byte_len {
                if byte_pos + i < byte_to_line.len() {
                    byte_to_line[byte_pos + i] = current_line;
                    byte_to_column[byte_pos + i] = current_column;
                }
            }
            
            if ch == '\n' {
                current_line += 1;
                current_column = 0;
            } else {
                current_column += 1;
            }
            
            byte_pos += char_byte_len;
        }
        
        // 마지막 위치 처리
        if byte_pos < byte_to_line.len() {
            byte_to_line[byte_pos] = current_line;
            byte_to_column[byte_pos] = current_column;
        }
        
        Self {
            byte_to_line,
            byte_to_column,
        }
    }
    
    /// O(1) 바이트 위치를 줄/컬럼으로 변환 (1-based)
    fn byte_to_line_column(&self, byte_offset: usize) -> (usize, usize) {
        let safe_offset = byte_offset.min(self.byte_to_line.len() - 1);
        let line = self.byte_to_line[safe_offset] + 1;     // 1-based
        let column = self.byte_to_column[safe_offset] + 1; // 1-based
        (line, column)
    }
}