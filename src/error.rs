// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::lex::position::Position;
use crate::lex::token::Token;
use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum SvErrorKind {
    Null,
    Io,
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
        SvError {kind:SvErrorKind::Eof, pos: Position::new(), txt: "".to_owned()}
    }

    pub fn syntax(t: Token, s: String) -> SvError {
        SvError {
            kind:SvErrorKind::Syntax,
            pos: t.pos,
            txt: format!("Unexpected {} ({:?}) in {}", t.value, t.kind, s)
        }
    }
}


impl fmt::Display for SvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            SvErrorKind::Null         => write!(f, "End of file reached."),
            SvErrorKind::Eof          => write!(f, "Unexpected end of file !"),
            SvErrorKind::Io           => write!(f, "{}", self.txt),
            SvErrorKind::Token        => write!(f, "line {} -- Unable to parse token \"{}\" !",self.pos, self.txt),
            SvErrorKind::Syntax       => write!(f, "line {} -- {} !",self.pos, self.txt),
            SvErrorKind::NotSupported => write!(f, "line {} -- Unsuported syntax : {} !",self.pos, self.txt),
        }
    }
}

impl From<std::io::Error> for SvError {
    fn from(cause: std::io::Error) -> SvError {
        SvError{kind:SvErrorKind::Io, pos: Position::new(), txt:format!("{:?}", cause) }
    }
}