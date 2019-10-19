// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::fmt;

/// Structure holding source code to parse with function to read char by char
///  and keeping information on current position in line/column.
#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub line    : u32,
    pub col     : u32,
}

impl Position {
    /// Create a new pposition initialize to 0,0
    pub fn new() -> Self {
        Position {line:1, col: 0}
    }
    /// Increment the current position
    pub fn incr(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        // println!("Char = {} -> Position => {} {}", *c, self.line, self.col);
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}",self.line,self.col)
    }
}
