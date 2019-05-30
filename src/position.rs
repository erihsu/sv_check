/// Structure holding source code to parse with function to read char by char
///  and keeping information on current position in line/column.
#[derive(Debug, Copy, Clone)]
pub struct Position {
    line    : u32,
    col     : u32,
}

impl Position {
    /// Create a new pposition initialize to 0,0
    pub fn new() -> Self {
        Position {line:0, col: 0}
    }
    /// Increment the current position
    pub fn incr(mut self, c: &char) {
        if *c == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}