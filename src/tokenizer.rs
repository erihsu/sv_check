// This file is part of sv_parser and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::position::Position;
use crate::token::*;
use crate::error::*;
use crate::source::Source;

use std::collections::VecDeque;

pub struct TokenStream<'a> {
    source: &'a mut Source,
    last_char: char,
    last_pos : Position,
    buffer : VecDeque<Token>,
    rd_ptr : u8,
}

/// Enum for the state machine parsing number
#[derive(PartialEq, Debug)]
enum NumParseState {Start, Base, Int, Dec, Exp}

#[derive(PartialEq, Debug)]
enum NumBase {Binary, Octal, Hexa, Decimal}

impl<'a> TokenStream<'a> {

    // Create a token stream with for a source code
    pub fn new(src: &'a mut Source) -> TokenStream<'a> {
        TokenStream {
            source: src,
            last_char : ' ',
            last_pos : Position::new() ,
            buffer   : VecDeque::new() ,
            rd_ptr   : 0
        }
    }

    fn updt_last(&mut self, c: char) {
        self.last_char = c;
        self.last_pos = self.source.pos;
    }

    // Retrieve the first non-white character in the source code
    fn get_first_char(&mut self) -> Option<char> {
        if ! self.last_char.is_whitespace() {
            return Some(self.last_char);
        }
        while let Some(c) = self.source.get_char() {
            if ! c.is_whitespace() {
                self.last_pos = self.source.pos;
                return Some(c);
            }
        }
        None
    }

    /// Get a valid identifier
    fn parse_ident(&mut self, first_char: char) -> Result<Token,SvError> {
        let mut s = String::new();
        s.push(first_char);
        let p = self.last_pos;
        let mut is_pathpulse = false;
        let mut is_casting = false;
        while let Some(c) = self.source.get_char() {
            if !c.is_alphanumeric() && c!='_' {
                match c {
                    '$' if s=="PATHPULSE" => {
                        s.push(c);
                        is_pathpulse = true;

                    },
                    '\'' => {
                        s.push(c);
                        is_casting = true;
                    }
                    _ => {
                        self.last_char = c;
                        self.last_pos = self.source.pos;
                    }
                }
                break;
            } else {
                s.push(c);
            }
        }
        // Check if the word is a base type of a keyword
        let k = {
            if first_char == '$' {
                if is_pathpulse || is_casting {
                    return Err(SvError::new(SvErrorKind::Token,p,s));
                }
                TokenKind::SystemTask
            }
            else if first_char == '`' {
                if is_pathpulse || is_casting || s.len()==1 {
                    return Err(SvError::new(SvErrorKind::Token,p,s));
                }
                TokenKind::Macro
            }
            else if is_casting {TokenKind::Casting}
            else if let Some(k) = basetype_from_str(s.as_ref()) {k}
            else if let Some(k) = keyword_from_str(s.as_ref()) {
                if is_casting {
                    return Err(SvError::new(SvErrorKind::Token,p,s));
                }
                k
            } else {TokenKind::Ident}
        };
        Ok(Token::new(k,s,p))
    }

    /// Get all characters until end of string
    fn parse_string(&mut self) -> Result<Token,SvError> {
        let mut s = String::from("\"");
        let p = self.last_pos;
        while let Some(c) = self.source.get_char() {
            s.push(c);
            if c == '"' && self.last_char != '\\' {
                break;
            }
            self.last_char = c;
        }
        self.last_char = ' ';
        Ok(Token::new(TokenKind::Str,s,p))
    }

    /// Get all characters until end of line
    fn parse_comment_line(&mut self) -> Result<Token,SvError> {
        let mut s = String::from("/");
        let p = self.last_pos;
        while let Some(c) = self.source.get_char() {
            self.last_char = c;
            if c == '\n'{
                break;
            } else {
                s.push(c);
            }
        }
        self.last_pos = self.source.pos;
        Ok(Token::new(TokenKind::Comment,s,p))
    }

    /// Get all characters until */
    fn parse_comment_block(&mut self) -> Result<Token,SvError> {
        let mut s = String::from("/");
        let p = self.last_pos;
        while let Some(c) = self.source.get_char() {
            s.push(c);
            if c == '/' && self.last_char == '*' {
                self.last_char = ' '; // Last char is consume
                break;
            }
            self.last_char = c;
        }
        self.last_pos = self.source.pos;
        Ok(Token::new(TokenKind::Comment,s,p))
    }

    /// Get all characters until *)
    fn parse_attribute(&mut self) -> Result<Token,SvError> {
        let mut s = String::from("(");
        let p = self.last_pos;
        while let Some(c) = self.source.get_char() {
            s.push(c);
            if c == ')' && self.last_char == '*' {
                self.last_char = ' '; // Last char is consumed
                break;
            }
            self.last_char = c;
        }
        self.last_pos = self.source.pos;
        if s.len() == 3 {
            Ok(Token::new(TokenKind::SensiAll,s,p))
        } else {
            Ok(Token::new(TokenKind::Attribute,s,p))
        }
    }


    /// Parse a number (real or integer)
    fn parse_number(&mut self, first_char: char) -> Result<Token,SvError> {
        let mut s = String::new();
        let p = self.last_pos;
        let mut has_xz = false;
        let mut base = NumBase::Decimal;
        let mut fsm = if first_char=='\'' {NumParseState::Base} else  {NumParseState::Start};
        s.push(first_char);
        while let Some(c) = self.source.get_char() {
            match c {
                // x/z allowed for integer number only
                'x'|'X'|'z'|'Z' => {
                    if base != NumBase::Binary && base != NumBase::Hexa && fsm!=NumParseState::Int {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    has_xz = true;
                    s.push(c);
                }
                // Base specifier
                '\'' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    fsm = NumParseState::Base;
                    s.push(c);
                },
                's'|'S' => {
                    if fsm != NumParseState::Base || self.last_char!='\'' {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    s.push(c);
                }
                'b'|'B' if fsm == NumParseState::Base => {
                    fsm = NumParseState::Int;
                    s.push(c);
                    base = NumBase::Binary;
                }
                'o'|'O' if fsm == NumParseState::Base => {
                    fsm = NumParseState::Int;
                    s.push(c);
                    base = NumBase::Octal;
                }
                'h'|'H' if fsm == NumParseState::Base => {
                    fsm = NumParseState::Int;
                    s.push(c);
                    base = NumBase::Hexa;
                }
                'd'|'D' if fsm == NumParseState::Base => {
                    fsm = NumParseState::Int;
                    s.push(c);
                }
                'a'|'b'|'c'|'d'|'e'|'f'|'A'|'B'|'C'|'D'|'E'|'F' if base==NumBase::Hexa => s.push(c),
                '?' if base==NumBase::Binary => {
                    if fsm != NumParseState::Int {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    s.push(c);
                }
                // Dot -> real number
                '.' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    fsm = NumParseState::Dec;
                    s.push(c);
                }
                // Exponent -> real number
                'e' | 'E' => {
                    if (fsm != NumParseState::Start && fsm != NumParseState::Dec) || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    fsm = NumParseState::Exp;
                    s.push(c);
                }
                // Sign exponent
                '+' | '-' => {
                    if fsm == NumParseState::Exp && (self.last_char=='e' || self.last_char=='E') {
                        s.push(c);
                    } else {break;}
                }
                // Standard number
                _ if c.is_digit(10) => s.push(c),
                // Not part of a number -> Token is ready
                _ => {
                    self.last_char = c;
                    break
                }
            }
            self.last_char = c;
        }
        self.last_pos = self.source.pos;
        let k = if fsm==NumParseState::Dec || fsm==NumParseState::Exp {TokenKind::Real} else {TokenKind::Integer};
        Ok(Token::new(k,s,p))
    }

    /// Return the next token from the source code
    pub fn get_next_token(&mut self) -> Result<Token,SvError> {
        let c = self.get_first_char().ok_or(SvError::new(SvErrorKind::Null,self.last_pos,"".to_string()))?;
        let p = self.last_pos; // Save position of this first char since it will
        self.last_char = ' ';
        match c {
            // Operator
            '/' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '/' => return self.parse_comment_line() ,
                    '*' => return self.parse_comment_block() ,
                    '=' => {
                        let t = Token::new(TokenKind::OpCompAss,"/=".to_string(),p);
                        self.source.get_char();  // Consume peeked character
                        return Ok(t)
                    }
                    _ => return Ok(Token::new(TokenKind::OpDiv,"/".to_string(),p))
                }
            }
            '%' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => return Ok(Token::new(TokenKind::OpCompAss ,"%=".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpMod,"%".to_string(),p))
                    }
                }
            }
            '+' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '+' => return Ok(Token::new(TokenKind::OpIncrDecr,"++".to_string(),p)) ,
                    '=' => return Ok(Token::new(TokenKind::OpCompAss ,"+=".to_string(),p)) ,
                    ':' => return Ok(Token::new(TokenKind::OpRange   ,"+:".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpPlus,"+".to_string(),p))
                    }
                }
            }
            '-' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '-' => return Ok(Token::new(TokenKind::OpIncrDecr,"--".to_string(),p)) ,
                    '=' => return Ok(Token::new(TokenKind::OpCompAss ,"-=".to_string(),p)) ,
                    ':' => return Ok(Token::new(TokenKind::OpRange   ,"-:".to_string(),p)) ,
                    '>' => return Ok(Token::new(TokenKind::OpImpl    ,"->".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpMinus,"-".to_string(),p))
                    }
                }
            }
            '*' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '*' => return Ok(Token::new(TokenKind::OpPow,"**".to_string(),p)) ,
                    '=' => return Ok(Token::new(TokenKind::OpCompAss ,"*=".to_string(),p)) ,
                    '>' => return Ok(Token::new(TokenKind::OpStarLT ,"*>".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpStar,"*".to_string(),p))
                    }
                }
            }
            '^' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '~' => return Ok(Token::new(TokenKind::OpXnor    ,"^~".to_string(),p)) ,
                    '=' => return Ok(Token::new(TokenKind::OpCompAss ,"^=".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpXor,"^".to_string(),p))
                    }
                }
            }
            '&' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '&' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        match nnc {
                            '&' => {
                                self.source.get_char();  // Consume peeked character
                                return Ok(Token::new(TokenKind::OpTimingAnd,"&&&".to_string(),p))
                            },
                            _ => return Ok(Token::new(TokenKind::OpLogicAnd,"&&".to_string(),p))
                        }
                    }
                    '=' => return Ok(Token::new(TokenKind::OpCompAss ,"&=".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpAnd,"&".to_string(),p))
                    }
                }
            }
            '|' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '|' => return Ok(Token::new(TokenKind::OpLogicOr,"||".to_string(),p)) ,
                    '=' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        match nnc {
                            '>' => {
                                self.source.get_char();  // Consume peeked character
                                return Ok(Token::new(TokenKind::OpSeqRel,"|=>".to_string(),p))
                            },
                            _ => return Ok(Token::new(TokenKind::OpCompAss,"|=".to_string(),p))
                        }
                    }
                    '-' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        match nnc {
                            '>' => return Ok(Token::new(TokenKind::OpSeqRel,"|->".to_string(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(TokenKind::OpOr,"|".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpOr,"|".to_string(),p))
                    }
                }
            }
            '<' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => return Ok(Token::new(TokenKind::OpLTE,"<=".to_string(),p)) ,
                    '<' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(TokenKind::OpCompAss,"<<=".to_string(),p)) ,
                            '<' => {
                                let nc = self.source.get_char().unwrap_or(' ');
                                match nc {
                                    '=' => return Ok(Token::new(TokenKind::OpCompAss,"<<<=".to_string(),p)) ,
                                    _ => {
                                        self.updt_last(nc);
                                        return Ok(Token::new(TokenKind::OpSShift,"<<<".to_string(),p))
                                    }
                                }
                            }
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(TokenKind::OpSL,"<<".to_string(),p))
                            }
                        }
                    }
                    // Check for equivalence operator <->
                    '-' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        if nnc == &'>' {
                            self.source.get_char().unwrap(); // Consume next char
                            return Ok(Token::new(TokenKind::OpEquiv,"<->".to_string(),p))
                        } else {
                            self.updt_last(nc);
                            return Ok(Token::new(TokenKind::OpLT,"<".to_string(),p))
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpLT,"<".to_string(),p))
                    }
                }
            }
            '>' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => return Ok(Token::new(TokenKind::OpGTE,">=".to_string(),p)) ,
                    '>' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(TokenKind::OpCompAss,">>=".to_string(),p)) ,
                            '>' => {
                                let nc = self.source.get_char().unwrap_or(' ');
                                match nc {
                                    '=' => return Ok(Token::new(TokenKind::OpCompAss,">>>=".to_string(),p)) ,
                                    _ => {
                                        self.updt_last(nc);
                                        return Ok(Token::new(TokenKind::OpSShift,">>>".to_string(),p))
                                    }
                                }
                            }
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(TokenKind::OpSR,">>".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpGT,">".to_string(),p))
                    }
                }
            }
            '!' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(TokenKind::OpDiff2,"!==".to_string(),p)) ,
                            '?' => return Ok(Token::new(TokenKind::OpDiffQue,"!=?".to_string(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(TokenKind::OpDiff,"!=".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpBang,"!".to_string(),p))
                    }
                }
            }
            '=' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '>' => return Ok(Token::new(TokenKind::OpFatArrL,"=>".to_string(),p)) ,
                    '=' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(TokenKind::OpEq3,"===".to_string(),p)) ,
                            '?' => return Ok(Token::new(TokenKind::OpEq2Que,"==?".to_string(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(TokenKind::OpEq2,"==".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpEq,"=".to_string(),p))
                    }
                }
            }
            '~' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '&' => return Ok(Token::new(TokenKind::OpNand ,"~&".to_string(),p)),
                    '|' => return Ok(Token::new(TokenKind::OpNor  ,"~|".to_string(),p)),
                    '^' => return Ok(Token::new(TokenKind::OpXnor ,"~^".to_string(),p)),
                    _   => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::OpTilde,"~".to_string() ,p));
                    }
                }
            }
            // Parenthesis
            '(' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '*' => return self.parse_attribute() ,
                    _ => return Ok(Token::new(TokenKind::ParenLeft  ,"(".to_string(),p)),
                }
            }
            ')' => return Ok(Token::new(TokenKind::ParenRight ,")".to_string(),p)),
            '[' => return Ok(Token::new(TokenKind::SquareLeft ,"[".to_string(),p)),
            ']' => return Ok(Token::new(TokenKind::SquareRight,"]".to_string(),p)),
            '{' => return Ok(Token::new(TokenKind::CurlyLeft  ,"{".to_string(),p)),
            '}' => return Ok(Token::new(TokenKind::CurlyRight ,"}".to_string(),p)),
            // Special characters
            ',' => return Ok(Token::new(TokenKind::Comma    ,",".to_string(),p)),
            '?' => return Ok(Token::new(TokenKind::Que      ,"?".to_string(),p)),
            ';' => return Ok(Token::new(TokenKind::SemiColon,";".to_string(),p)),
            '@' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '@' => return Ok(Token::new(TokenKind::At2 ,"@@".to_string(),p)),
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::At ,"@".to_string(),p))
                    }
                }
            }
            '.' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '*' => {
                        self.source.get_char().unwrap(); // Consume next char
                        return Ok(Token::new(TokenKind::DotStar,".*".to_string(),p));
                    }
                    _ => return Ok(Token::new(TokenKind::Dot      ,".".to_string(),p)),
                }

            }
            '$' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    'a'...'z' | 'A'...'Z' | '_'  => return self.parse_ident(c),
                    _ => return Ok(Token::new(TokenKind::Dollar ,"$".to_string(),p)),
                }

            }
            '#' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '#' => return Ok(Token::new(TokenKind::Hash2,"##".to_string(),p)),
                    '-' | '=' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        if nnc == &'#' {
                            self.source.get_char().unwrap(); // Consume next char
                            return Ok(Token::new(TokenKind::OpEquiv, format!("#{}#", nc),p))
                        } else {
                            self.updt_last(nc);
                            return Ok(Token::new(TokenKind::Hash2,"##".to_string(),p))
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::Hash,"#".to_string(),p))
                    }
                }
            }
            ':' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    ':' => return Ok(Token::new(TokenKind::Scope,"::".to_string(),p)),
                    '/' => return Ok(Token::new(TokenKind::OpDist,":/".to_string(),p)),
                    '=' => return Ok(Token::new(TokenKind::OpDist,":=".to_string(),p)),
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(TokenKind::Colon,":".to_string(),p))
                    }
                }
            }
            //
            '\'' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '{' => Ok(Token::new(TokenKind::TickCurly ,"'{".to_string(),p)) ,
                    _ => Err(SvError::new(SvErrorKind::Token,p,'\''.to_string()))
                }
            }
            '\\' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '\n' => Ok(Token::new(TokenKind::LineCont ,"".to_string(),p)) ,
                    _ => Err(SvError::new(SvErrorKind::Token,p,'\\'.to_string()))
                }
            }
            // String
            '"' => return self.parse_string(),
            // Identifier
            _ => {
                if c.is_digit(10) {
                    return self.parse_number(c)
                } else {
                    return self.parse_ident(c)
                }
            }
        }
    }

    pub fn next_non_comment(&mut self, peek: bool) -> Option<Result<Token,SvError>> {
        // println!("Buffer = {:?} , rd_ptr = {}", self.buffer, self.rd_ptr);
        if self.buffer.len() as u8>self.rd_ptr || (!peek && self.buffer.len()>0) {
            if !peek {
                if self.rd_ptr>0 {self.rd_ptr -= 1;}
                return Some(Ok(self.buffer.pop_front()?));
            }
            else {
                let t = self.buffer.get(self.rd_ptr as usize)?;
                self.rd_ptr += 1;
                return Some(Ok(t.clone()));
            }
        }
        loop {
            match self.next() {
                Some(Ok(t)) => {
                    if t.kind!=TokenKind::Comment {
                        if peek {
                            self.buffer.push_back(t.clone());
                            self.rd_ptr += 1;
                        }
                        return Some(Ok(t));
                    } else {
                        continue
                    }
                }
                Some(Err(t)) => return Some(Err(t)),
                None => return None
            };
        }
    }

    pub fn flush(&mut self, nb : u8) {
        // println!("Flushing: Buffer size = {:?} , rd_ptr = {}", self.buffer, self.rd_ptr);
        if nb==0 || nb>=self.buffer.len() as u8 {
            self.buffer.clear();
            self.rd_ptr = 0;
        } else {
            for _i in 0..nb {
                self.buffer.pop_front();
            }
        }
    }

    pub fn rewind(&mut self, nb : u8) {
        self.rd_ptr = if nb==0 || self.rd_ptr < nb {0} else {self.rd_ptr - nb};
    }

    #[allow(dead_code)]
    pub fn display_status(&self) {
        let mut s = format!("Buffer with {} element / ptr = {}",self.buffer.len(),  self.rd_ptr);
        for t in &self.buffer {
            s = format!("{}\n - {:?}",s,t);
        }
        println!("{}", s);
    }

}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token,SvError>;

    fn next(&mut self) -> Option<Result<Token,SvError>> {
        match self.get_next_token() {
            Err(e) => if e.kind != SvErrorKind::Null {Some(Err(e))} else {None},
            Ok(t) => Some(Ok(t))
        }
    }
}
