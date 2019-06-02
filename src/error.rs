use crate::position::Position;
use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum SvErrorKind {
    Null,
    Eof,
    Token,
    Syntax,
}
#[derive(Debug)]
pub struct SvError {
    pub kind: SvErrorKind,
    pub pos: Position,
    pub txt: String,
}

impl SvError {
    pub fn new(k : SvErrorKind, p : Position, t : String) -> SvError {
        SvError {kind:k, pos: p, txt: t}
    }
    pub fn eof(t : String) -> SvError {
        SvError {kind:SvErrorKind::Eof, pos: Position::new(), txt: t}
    }
}


impl fmt::Display for SvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match &self.kind {
    		SvErrorKind::Null   => write!(f, "End of file reached."),
            SvErrorKind::Eof    => write!(f, "Unexpected end of file."),
            SvErrorKind::Token  => write!(f, "Invalid token \"{}\" at line {}.", self.txt,self.pos),
    		SvErrorKind::Syntax => write!(f, "Invalid Syntax at line {} : {}",self.pos, self.txt),
    	}

    }
}
