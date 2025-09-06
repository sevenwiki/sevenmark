use std::fs;

fn main() {
    let input = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let normalized_input = input.replace("\r\n", "\n");
    
    // O(1) 바이트→줄 매핑 구현
    let mapper = ByteToLineMapper::new(&normalized_input);
    
    // 테스트: "2줄 셀1"의 바이트 27-36을 문자로 변환
    println!("테스트 바이트 위치들 (O(1) 방식):");
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