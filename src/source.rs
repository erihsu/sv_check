use crate::position::Position;

use std::{fs,path,io, mem, str, iter};

/// Structure holding source code to parse with function to read char by char
///  and keeping information on current position in line/column.
#[derive(Debug, Clone)]
pub struct Source {
    /// filename used to initialize the code
    pub filename : path::PathBuf,
    /// String representing the source code to analyze
    _code : String,
    /// Current position in the code
    pub pos : Position,
    // // Character iterator
    chars : iter::Peekable<str::Chars<'static>>
}

impl Source {

    /// Create a Source struct from a file.
    /// Return an io error if unable to open the file
    pub fn from_file(filename: path::PathBuf) -> Result<Source,io::Error>  {
        let _code = fs::read_to_string(&filename)?;
        let chars = unsafe { mem::transmute(_code.chars().peekable()) };
        let pos = Position::new();
        Ok(Source {filename, _code, pos, chars})
    }

    pub fn get_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos.incr(&c);
        // println!("Parse_ident: char={} at {:?}", c,self.pos );
        Some(c)
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}
