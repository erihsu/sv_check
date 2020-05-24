// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::lex::position::Position;
use crate::lex::token::{Token, TokenKind};
use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum SvErrorKind {
    Null,
    Io,
    Include,
    Eof,
    Token,
    Syntax,
    Missing,
    // NotSupported,
}

#[derive(Debug)]
pub struct SvError {
    pub kind: SvErrorKind,
    pub token: Token,
    pub txt: String,
}

impl SvError {

    pub fn new(k : SvErrorKind, t : Token, txt : String) -> SvError {
        SvError {kind:k, token: t, txt}
    }

    #[allow(dead_code)]
    pub fn eof(pos: Position) -> SvError {
        SvError {kind:SvErrorKind::Eof, token: Token::new(TokenKind::EOF, "".to_string(), pos), txt: "".to_string()}
    }

    pub fn null(pos: Position) -> SvError {
        SvError {kind:SvErrorKind::Null, token: Token::new(TokenKind::EOF, "".to_owned(), pos), txt: "".to_string()}
    }

    pub fn missing(txt: &str) -> SvError {
        SvError {kind:SvErrorKind::Missing, token: Token::new(TokenKind::EOF, "".to_owned(), Position::new()), txt: txt.to_string()}
    }

    pub fn token(pos: Position, s: String) -> SvError {
        SvError {
            kind:SvErrorKind::Syntax,
            token: Token::new(TokenKind::Unknown, s, pos),
            txt: "".to_string()
        }
    }

    pub fn syntax(t: Token, s: &str) -> SvError {
        SvError { kind:SvErrorKind::Syntax, token: t, txt: s.to_string() }
    }
}


impl fmt::Display for SvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            SvErrorKind::Null         => write!(f, "End of file reached."),
            SvErrorKind::Io           => write!(f, "{}", self.txt),
            SvErrorKind::Include      => write!(f, ":{} | File {} not found", self.token.pos, self.token.value),
            SvErrorKind::Eof          => write!(f, ":{} | Unexpected end of file !", self.token.pos),
            SvErrorKind::Token        => write!(f, ":{} | Unable to parse token \"{}\" !",self.token.pos, self.token.value),
            SvErrorKind::Syntax       => write!(f, ":{} | Unexpected '{}' ({}) in {} !",self.token.pos, self.token.value, self.token.kind, self.txt),
            SvErrorKind::Missing      => write!(f, "Missing {} !", self.txt),
            // SvErrorKind::NotSupported => write!(f, ":{} -- Unsuported syntax : {} !",self.token.pos, self.txt),
        }
    }
}

impl From<std::io::Error> for SvError {
    fn from(cause: std::io::Error) -> SvError {
        SvError{kind:SvErrorKind::Io, token: Token::new(TokenKind::EOF, "".to_string(), Position::new()), txt:format!("{:?}", cause) }
    }
}