// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::position::Position;
use crate::token::*;
use crate::error::*;
use crate::source::Source;

use std::path::PathBuf;
use std::collections::VecDeque;

pub struct TokenStream<'a> {
    source: &'a mut Source,
    last_char: char,
    last_pos : Position,
    buffer : VecDeque<Token>,
    rd_ptr : u8,
    pub inc_files : Vec<PathBuf>,
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
                else {TokenKind::Macro}
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
    fn parse_string(&mut self, is_macro: bool) -> Result<Token,SvError> {
        let mut s = if is_macro {"`\"".to_string()} else {"".to_string()};
        let p = self.last_pos;
        while let Some(c) = self.source.get_char() {
            s.push(c);
            if c == '"' && self.last_char != '\\' {
                if !is_macro {s.pop();}
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
            if fsm==NumParseState::IntStart && !c.is_whitespace() {
                fsm = NumParseState::Int;
            }
            // println!("[parse_number] char {} ({}), fsm={:?}, base={:?}, has_xz={}", c,c.is_whitespace(),fsm,base,has_xz);
            match c {
                // x/z allowed for integer number only
                'x'|'X'|'z'|'Z' => {
                    if base != NumBase::Binary && base != NumBase::Hexa && fsm!=NumParseState::Int && &s!="'" {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    has_xz = true;
                }
                // Base specifier
                '\'' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    fsm = NumParseState::Base;
                },
                // s following a number can be the time unit
                's' if fsm != NumParseState::Base => {
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
                    return Err(SvError::new(SvErrorKind::Token,p,s));
                }
                // _ can be used inside the value as a separator
                '_' if fsm != NumParseState::Base => {}
                // Dot -> real number
                '.' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
                    }
                    fsm = NumParseState::Dec;
                }
                // Exponent -> real number
                'e' | 'E' => {
                    if (fsm != NumParseState::Start && fsm != NumParseState::Dec) || has_xz {
                        return Err(SvError::new(SvErrorKind::Token,p,s));
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
        let c = self.get_first_char().ok_or(SvError::new(SvErrorKind::Null,self.last_pos,"".to_string()))?;
        let p = self.last_pos; // Save position of this first char since it will
        self.last_char = ' ';
        // println!("[get_next_token] Character {} @ {}", c, self.source.pos );
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
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '{' => {
                        self.source.get_char().unwrap(); // Consume next char
                        Ok(Token::new(TokenKind::TickCurly ,"'{".to_string(),p))
                    },
                    'h'|'d'|'o'|'b'|'x'|'z'|'H'|'D'|'O'|'B'|'X'|'Z'|'1'|'0' => return self.parse_number(c),
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
            '"' => return self.parse_string(false),
            //
            '`' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '"' => {
                        self.source.get_char().unwrap();
                        return self.parse_string(true);
                    },
                    _ => return self.parse_ident(c)
                }
            }
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
                    if t.kind!=TokenKind::Comment && t.kind!=TokenKind::Attribute {
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
                self.rd_ptr -= 1;
            }
        }
    }

    pub fn rewind(&mut self, nb : u8) {
        self.rd_ptr = if nb==0 || self.rd_ptr < nb {0} else {self.rd_ptr - nb};
    }

    pub fn skip_until(&mut self, tk_end: TokenKind) -> Result<(),SvError> {
        // Count the (), {} and begin/end
        let mut cnt_p = 0;
        let mut cnt_c = 0;
        let mut cnt_b = 0;
        self.flush(0);
        loop {
            match self.next() {
                Some(Ok(t)) => {
                    // if t.kind==tk_end && cnt_p<=0 && cnt_b<=0 && cnt_c<=0 {
                    //     break;
                    // }
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
        self.last_pos = self.source.pos;
        Ok(())
    }

    ///
    pub fn add_inc(&mut self, fname: &str) {
        let mut fname_path = PathBuf::new();
        for s in fname.to_string().split("/") {
            fname_path.push(s);
        }
        self.inc_files.push(fname_path);
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
