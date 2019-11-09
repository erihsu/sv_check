// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::*;
use crate::lex::position::Position;
use crate::lex::token::*;
use crate::lex::source::Source;

use std::collections::VecDeque;

pub struct TokenStream<'a> {
    pub source: &'a mut Source,
    last_char: char,
    last_pos : Position,
    buffer : VecDeque<Token>,
    rd_ptr : u8,
    pub inc_files : Vec<String>,
}

/// Enum for the state machine parsing number
#[derive(PartialEq, Debug)]
enum NumParseState {Start, Base, IntStart, Int, Dec, Exp}

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
            rd_ptr   : 0,
            inc_files: Vec::new()
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
        let mut second_char = ' ';
        while let Some(c) = self.source.get_char() {
            if s.len()==1 {
                second_char = c;
            }
            if !c.is_alphanumeric() && c!='_' && !(s.len()==1 && second_char=='`' && first_char=='`'){
                match c {
                    '$' => {
                        s.push(c);
                        if s=="PATHPULSE" {
                            is_pathpulse = true;
                            break;
                        }
                    },
                    '\'' => {
                        s.push(c);
                        is_casting = true;
                        break;
                    }
                    _ => {
                        self.last_char = c;
                        self.last_pos = self.source.pos;
                        break;
                    }
                }
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
                if is_pathpulse || s.len()==1 {
                    return Err(SvError::new(SvErrorKind::Token,p,s));
                } else if is_casting {
                    let nc = self.source.peek_char().unwrap_or(&' ');
                    match nc {
                        'b'|'B'|'o'|'O'|'h'|'H'|'d'|'D' => {
                            let t = self.parse_number('\'')?;
                            s.push_str(&t.value);
                            return Ok(Token::new(t.kind,s,p));
                        }
                        '(' => {},
                        _ => {return Err(SvError::new(SvErrorKind::Token,p,s));}
                    }
                }
                if is_casting {TokenKind::Casting}
                else if second_char=='`' && first_char=='`' {TokenKind::IdentInterpolated}
                else {
                    match s.as_ref() {
                        "`ifndef" | "`ifdef" | "`elsif" | "`else"  | "`endif" => TokenKind::CompDir,
                        _ => TokenKind::Macro
                    }
                }
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


    /// Escaped Identifier: get all characters until whitespace
    fn parse_esc_ident(&mut self) -> Result<Token,SvError> {
        let mut s = "\\".to_owned();
        let p = self.last_pos;
        while let Some(c) = self.source.get_char() {
            if ! c.is_whitespace() {
                s.push(c);
            } else { break;}
        }
        self.last_pos = self.source.pos;
        self.last_char = ' ';
        Ok(Token::new(TokenKind::Ident,s,p))
    }

    /// Get all characters until end of string
    fn parse_string(&mut self, is_macro: bool) -> Result<Token,SvError> {
        let mut s = if is_macro {"`\"".to_owned()} else {"".to_owned()};
        let p = self.last_pos;
        let mut cpp = ' ';
        while let Some(c) = self.source.get_char() {
            s.push(c);
            if c == '"' && (self.last_char != '\\' || cpp == '\\') {
                if !is_macro {s.pop();}
                break;
            }
            cpp = self.last_char;
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
            if c == '\n'{
                break;
            } else {
                s.push(c);
            }
        }
        self.last_pos = self.source.pos;
        self.last_char = ' ' ;
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
            if fsm==NumParseState::IntStart && !c.is_whitespace() {
                fsm = NumParseState::Int;
            }
            // println!("[parse_number] char {} ({}), fsm={:?}, base={:?}, has_xz={}", c,c.is_whitespace(),fsm,base,has_xz);
            match c {
                // x/z allowed for integer number only
                'x'|'X'|'z'|'Z' => {
                    if base != NumBase::Binary && base != NumBase::Hexa && fsm!=NumParseState::Int && &s!="'" {
                        return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));
                    }
                    has_xz = true;
                }
                // Base specifier
                '\'' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));
                    }
                    fsm = NumParseState::Base;
                },
                // s following a number can be the time unit
                's' if fsm != NumParseState::Base => {
                    // Check for 1step keyword
                    if s == "1" {
                        let nc = self.source.peek_char().unwrap_or(&' ');
                        if nc == &'t' {
                            self.source.get_char(); // consume t, and check the next two char are e and p
                            if self.source.get_char().unwrap_or(' ') != 'e' {return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));}
                            if self.source.get_char().unwrap_or(' ') != 'p' {return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));}
                            let lc = self.source.peek_char().unwrap_or(&'/');
                            if lc.is_whitespace() || lc==&';' {
                                self.last_pos = self.source.pos;
                                return Ok(Token::new(TokenKind::Kw1step,"1step".to_owned(),p));
                            } else {
                                return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));
                            }
                        }
                    }
                    self.last_char = c;
                    break;
                }
                // s in the base part indicates a signed value
                's'|'S' if fsm == NumParseState::Base => {}
                'b'|'B' if fsm == NumParseState::Base => {
                    fsm = NumParseState::IntStart;
                    base = NumBase::Binary;
                }
                'o'|'O' if fsm == NumParseState::Base => {
                    fsm = NumParseState::IntStart;
                    base = NumBase::Octal;
                }
                'h'|'H' if fsm == NumParseState::Base => {
                    fsm = NumParseState::IntStart;
                    base = NumBase::Hexa;
                }
                'd'|'D' if fsm == NumParseState::Base => {
                    fsm = NumParseState::IntStart;
                }
                'a'|'b'|'c'|'d'|'e'|'f'|'A'|'B'|'C'|'D'|'E'|'F' if base==NumBase::Hexa => {},
                '?' if base==NumBase::Binary && fsm != NumParseState::Int => {
                    return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));
                }
                // _ can be used inside the value as a separator
                '_' if fsm != NumParseState::Base => {}
                // Dot -> real number
                '.' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));
                    }
                    fsm = NumParseState::Dec;
                }
                // Exponent -> real number
                'e' | 'E' => {
                    if (fsm != NumParseState::Start && fsm != NumParseState::Dec) || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,self.source.pos,s));
                    }
                    fsm = NumParseState::Exp;
                }
                // Sign exponent
                '+' | '-' => {
                    if fsm != NumParseState::Exp || (self.last_char!='e' && self.last_char!='E') {
                        self.last_char = c;
                        break;
                    }
                }
                // Standard number
                _ if c.is_digit(10) => {},
                // Standard number
                _ if c.is_whitespace() && fsm==NumParseState::IntStart => {},
                // Not part of a number -> Token is ready
                _ => {
                    self.last_char = c;
                    break
                }
            }
            s.push(c);
            self.last_char = c;
            if &s=="'0" || &s=="'1" || &s=="'x" || &s=="'z" || &s=="'X" || &s=="'Z" {
                self.last_char = ' ';
                break;
            }
        }
        self.last_pos = self.source.pos;
        let k = if fsm==NumParseState::Dec || fsm==NumParseState::Exp {TokenKind::Real} else {TokenKind::Integer};
        Ok(Token::new(k,s,p))
    }

    /// Return the next token from the source code
    pub fn get_next_token(&mut self) -> Result<Token,SvError> {
        let c = self.get_first_char().ok_or_else(|| SvError::new(SvErrorKind::Null,self.last_pos,"".to_owned()))?;
        let p = self.last_pos; // Save position of this first char since it will
        self.last_char = ' ';
        // println!("[get_next_token] Character {} @ {}", c, self.source.pos );
        match c {
            // Operator
            '/' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '/' => self.parse_comment_line() ,
                    '*' => self.parse_comment_block() ,
                    '=' => {
                        let t = Token::new(TokenKind::OpCompAss,"/=".to_owned(),p);
                        self.source.get_char();  // Consume peeked character
                        Ok(t)
                    }
                    _ => Ok(Token::new(TokenKind::OpDiv,"/".to_owned(),p))
                }
            }
            '%' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => Ok(Token::new(TokenKind::OpCompAss ,"%=".to_owned(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpMod,"%".to_owned(),p))
                    }
                }
            }
            '+' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '+' => Ok(Token::new(TokenKind::OpIncrDecr,"++".to_owned(),p)) ,
                    '=' => Ok(Token::new(TokenKind::OpCompAss ,"+=".to_owned(),p)) ,
                    ':' => Ok(Token::new(TokenKind::OpRange   ,"+:".to_owned(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpPlus,"+".to_owned(),p))
                    }
                }
            }
            '-' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '-' => Ok(Token::new(TokenKind::OpIncrDecr,"--".to_owned(),p)) ,
                    '=' => Ok(Token::new(TokenKind::OpCompAss ,"-=".to_owned(),p)) ,
                    ':' => Ok(Token::new(TokenKind::OpRange   ,"-:".to_owned(),p)) ,
                    '>' => Ok(Token::new(TokenKind::OpImpl    ,"->".to_owned(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpMinus,"-".to_owned(),p))
                    }
                }
            }
            '*' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '*' => Ok(Token::new(TokenKind::OpPow,"**".to_owned(),p)) ,
                    '=' => Ok(Token::new(TokenKind::OpCompAss ,"*=".to_owned(),p)) ,
                    '>' => Ok(Token::new(TokenKind::OpStarLT ,"*>".to_owned(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpStar,"*".to_owned(),p))
                    }
                }
            }
            '^' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '~' => Ok(Token::new(TokenKind::OpXnor    ,"^~".to_owned(),p)) ,
                    '=' => Ok(Token::new(TokenKind::OpCompAss ,"^=".to_owned(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpXor,"^".to_owned(),p))
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
                                Ok(Token::new(TokenKind::OpTimingAnd,"&&&".to_owned(),p))
                            },
                            _ => Ok(Token::new(TokenKind::OpLogicAnd,"&&".to_owned(),p))
                        }
                    }
                    '=' => Ok(Token::new(TokenKind::OpCompAss ,"&=".to_owned(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpAnd,"&".to_owned(),p))
                    }
                }
            }
            '|' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '|' => Ok(Token::new(TokenKind::OpLogicOr,"||".to_owned(),p)) ,
                    '=' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        match nnc {
                            '>' => {
                                self.source.get_char();  // Consume peeked character
                                Ok(Token::new(TokenKind::OpSeqRel,"|=>".to_owned(),p))
                            },
                            _ => Ok(Token::new(TokenKind::OpCompAss,"|=".to_owned(),p))
                        }
                    }
                    '-' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        match nnc {
                            '>' => Ok(Token::new(TokenKind::OpSeqRel,"|->".to_owned(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                Ok(Token::new(TokenKind::OpOr,"|".to_owned(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpOr,"|".to_owned(),p))
                    }
                }
            }
            '<' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => Ok(Token::new(TokenKind::OpLTE,"<=".to_owned(),p)) ,
                    '<' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => Ok(Token::new(TokenKind::OpCompAss,"<<=".to_owned(),p)) ,
                            '<' => {
                                let nc = self.source.get_char().unwrap_or(' ');
                                match nc {
                                    '=' => Ok(Token::new(TokenKind::OpCompAss,"<<<=".to_owned(),p)) ,
                                    _ => {
                                        self.updt_last(nc);
                                        Ok(Token::new(TokenKind::OpSShift,"<<<".to_owned(),p))
                                    }
                                }
                            }
                            _ => {
                                self.updt_last(nc);
                                Ok(Token::new(TokenKind::OpSL,"<<".to_owned(),p))
                            }
                        }
                    }
                    // Check for equivalence operator <->
                    '-' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        if nnc == &'>' {
                            self.source.get_char().unwrap(); // Consume next char
                            Ok(Token::new(TokenKind::OpEquiv,"<->".to_owned(),p))
                        } else {
                            self.updt_last(nc);
                            Ok(Token::new(TokenKind::OpLT,"<".to_owned(),p))
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpLT,"<".to_owned(),p))
                    }
                }
            }
            '>' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => Ok(Token::new(TokenKind::OpGTE,">=".to_owned(),p)) ,
                    '>' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => Ok(Token::new(TokenKind::OpCompAss,">>=".to_owned(),p)) ,
                            '>' => {
                                let nc = self.source.get_char().unwrap_or(' ');
                                match nc {
                                    '=' => Ok(Token::new(TokenKind::OpCompAss,">>>=".to_owned(),p)) ,
                                    _ => {
                                        self.updt_last(nc);
                                        Ok(Token::new(TokenKind::OpSShift,">>>".to_owned(),p))
                                    }
                                }
                            }
                            _ => {
                                self.updt_last(nc);
                                Ok(Token::new(TokenKind::OpSR,">>".to_owned(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpGT,">".to_owned(),p))
                    }
                }
            }
            '!' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => Ok(Token::new(TokenKind::OpDiff2,"!==".to_owned(),p)) ,
                            '?' => Ok(Token::new(TokenKind::OpDiffQue,"!=?".to_owned(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                Ok(Token::new(TokenKind::OpDiff,"!=".to_owned(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpBang,"!".to_owned(),p))
                    }
                }
            }
            '=' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '>' => Ok(Token::new(TokenKind::OpFatArrL,"=>".to_owned(),p)) ,
                    '=' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => Ok(Token::new(TokenKind::OpEq3,"===".to_owned(),p)) ,
                            '?' => Ok(Token::new(TokenKind::OpEq2Que,"==?".to_owned(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                Ok(Token::new(TokenKind::OpEq2,"==".to_owned(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpEq,"=".to_owned(),p))
                    }
                }
            }
            '~' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '&' => Ok(Token::new(TokenKind::OpNand ,"~&".to_owned(),p)),
                    '|' => Ok(Token::new(TokenKind::OpNor  ,"~|".to_owned(),p)),
                    '^' => Ok(Token::new(TokenKind::OpXnor ,"~^".to_owned(),p)),
                    _   => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::OpTilde,"~".to_owned() ,p))
                    }
                }
            }
            // Parenthesis
            '(' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '*' => self.parse_attribute() ,
                    _ => Ok(Token::new(TokenKind::ParenLeft  ,"(".to_owned(),p)),
                }
            }
            ')' => Ok(Token::new(TokenKind::ParenRight ,")".to_owned(),p)),
            '[' => Ok(Token::new(TokenKind::SquareLeft ,"[".to_owned(),p)),
            ']' => Ok(Token::new(TokenKind::SquareRight,"]".to_owned(),p)),
            '{' => Ok(Token::new(TokenKind::CurlyLeft  ,"{".to_owned(),p)),
            '}' => Ok(Token::new(TokenKind::CurlyRight ,"}".to_owned(),p)),
            // Special characters
            ',' => Ok(Token::new(TokenKind::Comma    ,",".to_owned(),p)),
            '?' => Ok(Token::new(TokenKind::Que      ,"?".to_owned(),p)),
            ';' => Ok(Token::new(TokenKind::SemiColon,";".to_owned(),p)),
            '@' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '@' => Ok(Token::new(TokenKind::At2 ,"@@".to_owned(),p)),
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::At ,"@".to_owned(),p))
                    }
                }
            }
            '.' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '*' => {
                        self.source.get_char().unwrap(); // Consume next char
                        Ok(Token::new(TokenKind::DotStar,".*".to_owned(),p))
                    }
                    _ => Ok(Token::new(TokenKind::Dot      ,".".to_owned(),p)),
                }

            }
            '$' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    'a'..='z' | 'A'..='Z' | '_'  => self.parse_ident(c),
                    _ => Ok(Token::new(TokenKind::Dollar ,"$".to_owned(),p)),
                }

            }
            '#' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '#' => Ok(Token::new(TokenKind::Hash2,"##".to_owned(),p)),
                    '-' | '=' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        if nnc == &'#' {
                            self.source.get_char().unwrap(); // Consume next char
                            Ok(Token::new(TokenKind::OpEquiv, format!("#{}#", nc),p))
                        } else {
                            self.updt_last(nc);
                            Ok(Token::new(TokenKind::Hash2,"##".to_owned(),p))
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::Hash,"#".to_owned(),p))
                    }
                }
            }
            ':' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    ':' => Ok(Token::new(TokenKind::Scope,"::".to_owned(),p)),
                    '/' => Ok(Token::new(TokenKind::OpDist,":/".to_owned(),p)),
                    '=' => Ok(Token::new(TokenKind::OpDist,":=".to_owned(),p)),
                    _ => {
                        self.updt_last(nc);
                        Ok(Token::new(TokenKind::Colon,":".to_owned(),p))
                    }
                }
            }
            //
            '\'' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '{' => {
                        self.source.get_char().unwrap(); // Consume next char
                        Ok(Token::new(TokenKind::TickCurly ,"'{".to_owned(),p))
                    },
                    'h'|'d'|'o'|'b'|'x'|'z'|'H'|'D'|'O'|'B'|'X'|'Z'|'1'|'0' => self.parse_number(c),
                    _ => Err(SvError::new(SvErrorKind::Token,p,'\''.to_string()))
                }
            }
            '\\' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '\n' => Ok(Token::new(TokenKind::LineCont ,"".to_owned(),p)) ,
                    _ => self.parse_esc_ident()
                }
            }
            // String
            '"' => self.parse_string(false),
            //
            '`' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '"' => {
                        self.source.get_char().unwrap();
                        self.parse_string(true)
                    },
                    _ => self.parse_ident(c)
                }
            }
            // Identifier
            _ => {
                if c.is_digit(10) {
                    self.parse_number(c)
                } else {
                    self.parse_ident(c)
                }
            }
        }
    }

    pub fn next_non_comment(&mut self, peek: bool) -> Option<Result<Token,SvError>> {
        // println!("Buffer = {:?} , rd_ptr = {}", self.buffer, self.rd_ptr);
        if self.buffer.len() as u8>self.rd_ptr || (!peek && !self.buffer.is_empty()) {
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
                    if t.kind!=TokenKind::Comment && t.kind!=TokenKind::Attribute {
                        if peek {
                            self.buffer.push_back(t.clone());
                            self.rd_ptr += 1;
                        }
                        return Some(Ok(t));
                    } else {
                        // println!("Comment = {:?}", t);
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
                self.rd_ptr -= 1;
            }
        }
    }

    // Flush to keep nb element
    pub fn flush_keep(&mut self, mut nb : u8) {
        let l = self.buffer.len() as u8;
        nb = l - nb;
        for _i in 0..nb {
            self.buffer.pop_front();
        }
        self.rewind(0);
    }

    pub fn rewind(&mut self, nb : u8) {
        self.rd_ptr = if nb==0 || self.rd_ptr < nb {0} else {self.rd_ptr - nb};
    }

    pub fn skip_until(&mut self, tk_end: TokenKind) -> Result<(),SvError> {
        // Count the (), {} and begin/end
        let mut cnt_p = 0;
        let mut cnt_c = 0;
        let mut cnt_b = 0;
        // self.display_status("Starting skip_until");
        // Check buffer first
        while !self.buffer.is_empty() {
            if let Some(t) = self.buffer.pop_front() {
                // println!("Buffer = {:?}", t);
                match t.kind {
                    TokenKind::ParenLeft  => cnt_p += 1,
                    TokenKind::ParenRight => cnt_p -= 1,
                    TokenKind::CurlyLeft  => cnt_c += 1,
                    TokenKind::CurlyRight => cnt_c -= 1,
                    TokenKind::KwBegin => cnt_b += 1,
                    TokenKind::KwEnd   => cnt_b -= 1,
                    _ => {}
                }
            }
        }
        loop {
            match self.next() {
                Some(Ok(t)) => {
                    match t.kind {
                        TokenKind::ParenLeft  => cnt_p += 1,
                        TokenKind::ParenRight => cnt_p -= 1,
                        TokenKind::CurlyLeft  => cnt_c += 1,
                        TokenKind::CurlyRight => cnt_c -= 1,
                        TokenKind::KwBegin => cnt_b += 1,
                        TokenKind::KwEnd   => cnt_b -= 1,
                        _ => {}
                    }
                    if t.kind==tk_end && cnt_p<=0 && cnt_b<=0 && cnt_c<=0 {
                        break;
                    }
                    // println!("Skipping {} (cnt {}/{}/{})", t,cnt_p,cnt_b,cnt_c);
                }
                Some(Err(t)) => return Err(t),
                None => return Err(SvError::eof())
            };
        }
        // println!("[skip_until] File = {:#?} new pos = {}", self.source.filename, self.source.pos);
        self.last_pos = self.source.pos;
        Ok(())
    }

    ///
    pub fn add_inc(&mut self, fname: &str) {
        self.inc_files.push(fname.to_string());
    }

    // Debug function
    #[allow(dead_code)]
    pub fn display_status(&self, comment : &str) {
        let mut s = format!("{} : Last pos = {}, Buffer with {} element / ptr = {}",comment,self.last_pos, self.buffer.len(),  self.rd_ptr);
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
