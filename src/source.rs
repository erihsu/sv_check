use crate::position::Position;

use std::{fs,path,io, mem, str};

/// Structure holding source code to parse with function to read char by char
///  and keeping information on current position in line/column.
#[derive(Debug, Clone)]
pub struct Source {
    /// filename used to initialize the code
    pub filename : String,
    /// String representing the source code to analyze
    _code : String, 
    /// Current position in the 
    pub pos : Position,
    // // Character iterator
    chars : str::Chars<'static>
}

impl Source {

    /// Create a Source struct from a file.
    /// Return an io error if unable to open the file
    pub fn from_file(fname: &str) -> Result<Source,io::Error>  {
        let filename = fname.to_string();
        let _code = fs::read_to_string(path::Path::new(&filename))?;
        let chars = unsafe { mem::transmute(_code.chars()) };
        let pos = Position::new();
        Ok(Source {filename, _code, pos, chars})
    }

    pub fn get_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos.incr(&c);
        Some(c)
    }
}
