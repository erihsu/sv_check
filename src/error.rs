use crate::position::Position;
use crate::token::Token;
use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum SvErrorKind {
    Null,
    Eof,
    Token,
    Syntax,
    NotSupported,
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

    pub fn eof() -> SvError {
        SvError {kind:SvErrorKind::Eof, pos: Position::new(), txt: "".to_string()}
    }

    pub fn syntax(t: Token, s: String) -> SvError {
        SvError {
            kind:SvErrorKind::Syntax,
            pos: t.pos,
            txt: format!("Unexpected {} {:?} in {}", t.value, t.kind, s)
        }
    }
}


impl fmt::Display for SvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            SvErrorKind::Null         => write!(f, "End of file reached."),
            SvErrorKind::Eof          => write!(f, "Unexpected end of file !"),
            SvErrorKind::Token        => write!(f, "Invalid token \"{}\" at line {} !", self.txt,self.pos),
            SvErrorKind::Syntax       => write!(f, "Invalid Syntax at line {} : {} !",self.pos, self.txt),
            SvErrorKind::NotSupported => write!(f, "Unsuported Syntax at line {} : {} !",self.pos, self.txt),
        }
    }
}