// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::*;
use crate::project::Project;
use crate::lex::position::Position;
use crate::lex::token::*;
use crate::lex::source::Source;

use std::collections::{VecDeque, HashMap};



pub struct TokenStream<'a,'b> {
    pub source: &'a mut Source,
    last_char: char,
    pub last_pos : Position,
    buffer : VecDeque<Token>,
    rd_ptr : usize,
    pub inc_files : Vec<String>,
    pub project : &'b mut Project,
}

/// Enum for the state machine parsing number
#[derive(PartialEq, Debug,  Clone)]
enum NumParseState {Start, Base, IntStart, Int, Dec, Exp}

#[derive(PartialEq, Debug)]
enum NumBase {Binary, Octal, Hexa, Decimal}

impl<'a,'b> TokenStream<'a,'b> {

    // Create a token stream with for a source code
    pub fn new(src: &'a mut Source, project: &'b mut Project) -> TokenStream<'a,'b> {
        TokenStream {
            source: src,
            last_char : ' ',
            project : project,
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
                    return Err(SvError::token(p,s));
                }
                TokenKind::SystemTask
            }
            else if first_char == '`' {
                if is_pathpulse || s.len()==1 {
                    return Err(SvError::token(p,s));
                } else if is_casting {
                    let nc = self.source.peek_char().unwrap_or(&' ');
                    match nc {
                        'b'|'B'|'o'|'O'|'h'|'H'|'d'|'D' => {
                            let t = self.parse_number('\'')?;
                            s.push_str(&t.value);
                            return Ok(Token::new(t.kind,s,p));
                        }
                        '(' => {},
                        _ => {return Err(SvError::token(p,s));}
                    }
                }
                if is_casting {TokenKind::Casting}
                else if second_char=='`' && first_char=='`' {
                    s = s.chars().skip(2).collect();
                    TokenKind::IdentInterpolated
                }
                else {
                    match s.as_ref() {
                        "`ifndef" | "`ifdef" | "`elsif" | "`else"  | "`endif" => TokenKind::CompDir,
                        "`undefineall" | "`resetall" | "`celldefine" | "`endcelldefine" |
                        "`nounconnected_drive" | "`end_keywords" | "`undef" | "`begin_keywords" |
                        "`unconnected_drive" | "`define" | "`pragma" | "`default_nettype" |
                        "`timescale" | "`line" | "`include"  => TokenKind::Macro,
                        _ => TokenKind::MacroCall
                    }
                }
            }
            else if is_casting {TokenKind::Casting}
            else if let Some(k) = basetype_from_str(s.as_ref()) {k}
            else if let Some(k) = keyword_from_str(s.as_ref()) {
                if is_casting {
                    return Err(SvError::token(p,s));
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
        // TODO: Macro interpolated string to be handled
        let mut s = if is_macro {"".to_owned()} else {"".to_owned()};
        // let mut s = "".to_owned();
        let p = self.last_pos;
        let mut cpp = ' ';
        while let Some(c) = self.source.get_char() {
            s.push(c);
            if c == '"' && (self.last_char != '\\' || cpp == '\\') {
                s.pop();
                break;
            }
            cpp = self.last_char;
            self.last_char = c;
        }
        self.last_char = ' ';
        // if is_macro {println!("Macro String = {}", s);}
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
        let mut fsm_next;
        let mut done = false;
        s.push(first_char);
        while let Some(c) = self.source.get_char() {
            // if fsm==NumParseState::IntStart && !c.is_whitespace() {
            //
            // }
            fsm_next = fsm.clone(); // Default next state is current state
            // println!("[parse_number] char {} ({}), fsm={:?}, base={:?}, has_xz={}", c,c.is_whitespace(),fsm,base,has_xz);
            match c {
                // unsized literal
                '0'|'1'|'x'|'z'|'X'|'Z'|'?' if fsm == NumParseState::Base => {
                    fsm_next = NumParseState::Int;
                    done = true;
                }
                //
                'x'|'X'|'z'|'Z'|'?' if fsm==NumParseState::IntStart && base==NumBase::Decimal => done = true,
                // x/z allowed for integer number only
                'x'|'X'|'z'|'Z' => {
                    if base != NumBase::Binary && base != NumBase::Hexa && fsm!=NumParseState::Int && fsm!=NumParseState::IntStart && fsm!=NumParseState::Start && &s!="'" {
                        return Err(SvError::token(self.source.pos,s));
                    }
                    has_xz = true;
                }
                '?' if base==NumBase::Binary && (fsm == NumParseState::Int || fsm == NumParseState::IntStart) => {
                    has_xz = true;
                }
                // Base specifier
                '\'' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::token(self.source.pos,s));
                    }
                    fsm_next = NumParseState::Base;
                },
                // s following a number can be the time unit
                's' if fsm != NumParseState::Base => {
                    // Check for 1step keyword
                    if s == "1" {
                        let nc = self.source.peek_char().unwrap_or(&' ');
                        if nc == &'t' {
                            self.source.get_char(); // consume t, and check the next two char are e and p
                            if self.source.get_char().unwrap_or(' ') != 'e' {return Err(SvError::token(self.source.pos,s));}
                            if self.source.get_char().unwrap_or(' ') != 'p' {return Err(SvError::token(self.source.pos,s));}
                            let lc = self.source.peek_char().unwrap_or(&'/');
                            if lc.is_whitespace() || lc==&';' {
                                self.last_pos = self.source.pos;
                                return Ok(Token::new(TokenKind::Kw1step,"1step".to_owned(),p));
                            } else {
                                return Err(SvError::token(self.source.pos,s));
                            }
                        }
                    }
                    self.last_char = c;
                    break;
                }
                // s in the base part indicates a signed value
                's'|'S' if fsm == NumParseState::Base => {}
                'b'|'B' if fsm == NumParseState::Base => {
                    fsm_next = NumParseState::IntStart;
                    base = NumBase::Binary;
                }
                'o'|'O' if fsm == NumParseState::Base => {
                    fsm_next = NumParseState::IntStart;
                    base = NumBase::Octal;
                }
                'h'|'H' if fsm == NumParseState::Base => {
                    fsm_next = NumParseState::IntStart;
                    base = NumBase::Hexa;
                }
                'd'|'D' if fsm == NumParseState::Base => {
                    fsm_next = NumParseState::IntStart;
                }
                'a'|'b'|'c'|'d'|'e'|'f'|'A'|'B'|'C'|'D'|'E'|'F' if base==NumBase::Hexa => {},
                // _ can be used inside the value as a separator
                '_' if fsm != NumParseState::Base => {}
                // Dot -> real number
                '.' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(SvError::token(self.source.pos,s));
                    }
                    fsm_next = NumParseState::Dec;
                }
                // Exponent -> real number
                'e' | 'E' => {
                    if (fsm != NumParseState::Start && fsm != NumParseState::Dec) || has_xz {
                        return Err(SvError::token(self.source.pos,s));
                    }
                    fsm_next = NumParseState::Exp;
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
            if fsm==NumParseState::IntStart && !c.is_whitespace() {
                fsm_next = NumParseState::Int;
            }
            fsm = fsm_next.clone();
            s.push(c);
            self.last_char = c;
            if done {
                self.last_char = ' ';
                break;
            }
        }
        self.last_pos = self.source.pos;
        // if fsm == NumParseState::Base {println!("[TokenStream] Integer {} | {:?} | {}", s,fsm,p);}
        // let k = if fsm==NumParseState::Dec || fsm==NumParseState::Exp {TokenKind::Real} else {TokenKind::Integer};
        let k = match fsm {
            NumParseState::Dec | NumParseState::Exp => TokenKind::Real,
            NumParseState::Base => {s.pop(); TokenKind::Casting}
            _ => TokenKind::Integer
        };
        // println!("[TokenStream] Parse_number @ {} = {} | kind={} | FSM={:?}", p, s,k,fsm);
        Ok(Token::new(k,s,p))
    }

    /// Return the next token from the source code
    pub fn get_next_token(&mut self) -> Result<Token,SvError> {
        let c = self.get_first_char().ok_or_else(|| SvError::null(self.last_pos))?;
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
                    '(' => Ok(Token::new(TokenKind::Casting,"\'".to_owned(),p)),
                    'h'|'d'|'o'|'b'|'x'|'z'|'H'|'D'|'O'|'B'|'X'|'Z'|'1'|'0' => self.parse_number(c),
                    _ => Err(SvError::token(p,'\''.to_string()))
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
        if self.buffer.len()>self.rd_ptr || (!peek && !self.buffer.is_empty()) {
            if !peek {
                if self.rd_ptr>0 {self.rd_ptr -= 1;}
                return Some(Ok(self.buffer.pop_front()?));
            }
            else {
                let t = self.buffer.get(self.rd_ptr)?;
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
                None => {
                    if self.buffer.len() > self.rd_ptr || (!peek && !self.buffer.is_empty()) {
                        if !peek {
                            if self.rd_ptr>0 {self.rd_ptr -= 1;}
                            return Some(Ok(self.buffer.pop_front()?));
                        }
                        else {
                            let t = self.buffer.get(self.rd_ptr)?;
                            self.rd_ptr += 1;
                            return Some(Ok(t.clone()));
                        }
                    }
                    return None
                }
            };
        }
    }

    pub fn get_pos(&self) -> Position {
        if self.buffer.len() > self.rd_ptr {
            self.buffer[self.rd_ptr].pos
        } else {
            self.last_pos
        }
    }

    pub fn flush(&mut self, nb : usize) {
        // println!("Flushing: Buffer size = {:?} , rd_ptr = {}", self.buffer, self.rd_ptr);
        if nb==0 || nb>=self.buffer.len() {
            self.buffer.clear();
            self.rd_ptr = 0;
        } else {
            self.buffer.drain(0..nb);
            if nb >= self.rd_ptr {self.rd_ptr = 0;} else {self.rd_ptr -= nb;}            
        }
    }

    pub fn flush_rd(&mut self) {
        self.buffer.drain(0..self.rd_ptr);
        self.rd_ptr = 0;
    }

    // Flush to keep nb element
    pub fn flush_keep(&mut self, mut nb : usize) {
        let l = self.buffer.len();
        nb = l - nb;
        self.buffer.drain(0..nb);
        self.rd_ptr = 0;
    }

    pub fn rewind(&mut self, nb : usize) {
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
                if t.kind==tk_end && cnt_p<=0 && cnt_b<=0 && cnt_c<=0 {
                    self.last_pos = self.source.pos;
                    return Ok(());
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
                None => {
                    // TODO: handle macro case where buffer is filled after macro expansion
                    return Err(SvError::eof(self.source.pos))
                }
            };
        }
        // println!("[skip_until] File = {:#?} new pos = {}", self.source.filename, self.source.pos);
        self.last_pos = self.source.pos;
        Ok(())
    }

    pub fn peek_until(&mut self, tk_end: TokenKind) -> Result<(),SvError> {
        // Count the (), {} and begin/end
        let mut cnt_p = 0;
        let mut cnt_c = 0;
        let mut cnt_b = 0;
        let mut rd_stream = self.buffer.is_empty();
        // self.display_status("[peek_until] start");
        loop {
            let t;
            if !rd_stream && self.rd_ptr < self.buffer.len() {
                t = self.buffer[self.rd_ptr].clone();
            } else {
                rd_stream = true;
                match self.next() {
                    Some(Ok(x)) => {
                        self.buffer.push_back(x.clone());
                        t = x.clone();
                    }
                    Some(Err(e)) => return Err(e),
                    None => return Err(SvError::eof(self.source.pos)),
                }
            };
            // Increment read pointer in all cases since the token is always in thebuffer
            self.rd_ptr += 1;
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
                self.last_pos = self.source.pos;
                // self.display_status("[peek_until] done");
                return Ok(());
            }
        }
    }

    pub fn collect_until(&mut self, is_list: bool) -> Result<Vec<Token>,SvError> {
        let mut v = Vec::new();
        let mut line_num = self.last_pos.line;
        // Count the (), {} and begin/end
        let mut cnt_p = 0;
        let mut cnt_c = 0;
        let mut cnt_b = 0;
        // Check buffer first
        while !self.buffer.is_empty() {
            if let Some(t) = self.buffer.pop_front() {
                // println!("Buffer = {:?} ({})", t,line_num);
                // Update line number if we went back in the buffer
                if t.pos.line<line_num {line_num = t.pos.line;}
                match t.kind {
                    TokenKind::ParenLeft  => cnt_p += 1,
                    TokenKind::ParenRight => cnt_p -= 1,
                    TokenKind::CurlyLeft  => cnt_c += 1,
                    TokenKind::CurlyRight => cnt_c -= 1,
                    TokenKind::KwBegin => cnt_b += 1,
                    TokenKind::KwEnd   => cnt_b -= 1,
                    _ => {}
                }
                if t.pos.line != line_num || (is_list && cnt_p<=0 && cnt_b<=0 && cnt_c<=0  && (t.kind==TokenKind::Comma || t.kind==TokenKind::ParenRight)) {
                    self.buffer.push_back(t);
                    self.last_pos = self.source.pos;
                    return Ok(v);
                }
                else if t.kind== TokenKind::LineCont {
                    line_num += 1;
                } else {
                    v.push(t);
                }
            }
        }
        while let Ok(t) = self.get_next_token() {
            // println!("Stream = {:?} ({})", t,line_num);
            match t.kind {
                TokenKind::ParenLeft  => cnt_p += 1,
                TokenKind::ParenRight => cnt_p -= 1,
                TokenKind::CurlyLeft  => cnt_c += 1,
                TokenKind::CurlyRight => cnt_c -= 1,
                TokenKind::KwBegin => cnt_b += 1,
                TokenKind::KwEnd   => cnt_b -= 1,
                TokenKind::Comment => {
                    if t.value.ends_with("\\") {line_num+=1;}
                    continue;
                }
                TokenKind::Attribute => continue,
                _ => {}
            }
            if t.pos.line != line_num || (is_list && cnt_p<=0 && cnt_b<=0 && cnt_c<=0  && (t.kind==TokenKind::Comma || t.kind==TokenKind::ParenRight)) {
                self.buffer.push_back(t);
                break;
            }
            else if t.kind== TokenKind::LineCont {
                line_num += 1;
            } else {
                v.push(t);
            }
            // println!("Skipping {} (cnt {}/{}/{})", t,cnt_p,cnt_b,cnt_c);
        }
        // println!("[collect_until] File = {:#?} new pos = {} -> {:?}", self.source.filename, self.source.pos, v);
        self.last_pos = self.source.pos;
        Ok(v)
    }

    //
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

    pub fn macro_expand(&mut self, t: Token, pos: Position, top: bool, macro_body: &mut std::vec::IntoIter<Token>, args_caller: &HashMap<String,Vec<Token>>) -> Option<Result<Token,SvError>> {
        match t.value.as_ref() {
            "`__FILE__" => Some(Ok(Token::new(TokenKind::Str,self.source.get_filename(),pos))),
            "`__LINE__" => Some(Ok(Token::new(TokenKind::Integer, pos.line.to_string() ,pos))),
            _ => {
                let macro_name = t.value.clone();
                let def = self.project.defines.get(&macro_name);
                if def.is_none() {
                    println!("[macro_expand] {} : line {} | Empty macro {} ", self.source.get_filename(),pos,macro_name);
                    return None;
                }
                if def.unwrap().is_none() {
                    println!("[macro_expand] {} : line {} | Unknown macro {} ", self.source.get_filename(),pos,macro_name);
                    return None;
                }
                let macro_def = def.unwrap().as_ref().unwrap().clone();
                let mut body = macro_def.body.clone().into_iter();
                let mut args = HashMap::new();
                // Check for macro param
                if !macro_def.ports.is_empty() {
                    // println!("[macro_expand] {} : line {} | macro {} has ports : {:?}", self.source.get_filename(),pos,macro_name,macro_def.ports);
                    let mut cnt_a : isize = -1;
                    let mut cnt_p = 0;
                    let mut cnt_c = 0;
                    let mut v = Vec::new();
                    loop {
                        let t;
                        if top {
                            if let Ok(nt) = self.get_next_token() {t = nt.clone();}
                            else {break;}
                        } else {
                            if let Some(nt) = macro_body.next() {t = nt.clone();}
                            else {break;}
                        };
                        // println!("[macro_expand] {} : macro {} | cnt: arg={}, ()={}, {{}}={} | token = {}", self.source.get_filename(),macro_name,cnt_a,cnt_p,cnt_c,t);
                        match t.kind {
                            TokenKind::Comment => {}
                            TokenKind::ParenLeft  => { if cnt_a==-1 { cnt_a += 1; } else { cnt_p += 1; v.push(t);}}
                            TokenKind::ParenRight if cnt_p!=0 => { cnt_p -= 1; v.push(t);}
                            TokenKind::CurlyLeft  => { cnt_c += 1; v.push(t); }
                            TokenKind::CurlyRight => { cnt_c -= 1; v.push(t); }
                            TokenKind::ParenRight if cnt_p==0 => {
                                let arg_name = macro_def.ports[cnt_a as usize].0.clone();
                                args.insert(arg_name,v);
                                break;
                            }
                            TokenKind::Comma if cnt_p==0 && cnt_c==0 => {
                                let arg_name = macro_def.ports[cnt_a as usize].0.clone();
                                // println!("[macro_expand] {} : macro {} port {} = {:?}", self.source.get_filename(),macro_name,arg_name,v);
                                if v.len() == 0 {
                                    println!("[macro_expand] {} : macro {} port {} is empty : default length = {}", self.source.get_filename(),macro_name,arg_name,v.len());
                                }
                                args.insert(arg_name,v);
                                cnt_a += 1;
                                if (cnt_a as usize)==macro_def.ports.len() {break;}
                                v = Vec::new();
                            }
                            _ if cnt_a == -1 => {return Some(Err(SvError::syntax(t,"Macro call parameter")));}
                            // Replace identifier by argument value
                            TokenKind::Ident if args_caller.contains_key(&t.value) => {
                                // println!("[macro_expand] {} : macro {} port {} token {} replaced by {:?}", self.source.get_filename(),macro_name,macro_def.ports[cnt_a as usize].0,t.value,args_caller[&t.value]);
                                for at in args_caller[&t.value].clone() {
                                    v.push(at);
                                }
                            }
                            _ => {v.push(t);}
                        }
                    }
                    // TODO: handle default value
                    while (cnt_a as usize) < macro_def.ports.len()-1 {
                        cnt_a += 1;
                        let (arg_name,dt) = macro_def.ports[cnt_a as usize].clone();
                        if dt.len() == 0 {
                            return Some(Err(SvError::syntax(t, &format!("macro call. Only {} parameters over {}. No default value for {}.", cnt_a+1,macro_def.ports.len(),arg_name))));
                        } else {
                            args.insert(arg_name,dt);
                        }
                    }
                    // println!("[macro_expand] {} : macro {} arguments = \n{:#?}", self.source.get_filename(),macro_name,args);
                }
                // if top {println!("[macro_expand] {} : macro {} -> ", self.source.get_filename(),macro_name);}
                // let mut s = "".to_string();
                let mut prev_was_inc = false;
                while let Some(bt) = body.next() {
                    match bt.kind {
                        TokenKind::MacroCall => {
                            if let Some(Err(e)) = self.macro_expand(bt, pos, false,&mut body, &args) {
                                return Some(Err(e));
                            }
                        }
                        // Replace identifier by argument value
                        TokenKind::IdentInterpolated => {
                            if args.contains_key(&bt.value) {
                                for at in args[&bt.value].clone() {
                                    self.buffer.push_back(Token{kind:at.kind,value:at.value,pos:pos});
                                }
                            } else {
                                return Some(Err(SvError::syntax(bt,&format!("macro call. Parameter not found: {:?}",args.keys()))));
                            }
                        }
                        TokenKind::Ident if args.contains_key(&bt.value) => {
                            // println!("[macro_expand] Replacing {} by {:?}", bt.value, args[&bt.value]);
                            for at in args[&bt.value].clone() {
                                // if at.kind==TokenKind::Str {s = format!("{} \"{}\"",s, at.value);} else {s = format!("{} {}",s, at.value);}
                                self.buffer.push_back(Token{kind:at.kind,value:at.value,pos:pos});
                            }
                        }
                        _ => {
                            if prev_was_inc && bt.kind==TokenKind::Str {
                                if self.project.ast_inc.contains_key(&bt.value) {
                                    // println!("[macro_expand] Found include {} : updating defines", bt.value);
                                    for (k,v) in self.project.ast_inc[&bt.value].defines.clone() {
                                        self.project.defines.insert(k,v);
                                    }
                                } else {
                                    // println!("[macro_expand] Found include {} : compiling", bt.value);
                                    if let Err(e) = self.project.compile_inc(bt.value.clone(), t.clone()) {
                                        return Some(Err(e));
                                    }
                                }
                            }
                            prev_was_inc = bt.kind == TokenKind::Macro && bt.value=="`include";
                            // if bt.kind==TokenKind::Str {s = format!("{} \"{}\"",s, bt.value);} else {s = format!("{} {}",s, bt.value);}
                            self.buffer.push_back(Token{kind:bt.kind,value:bt.value,pos:pos});
                        }
                    }
                }
                // if s.len() != 0 {println!("{}", s);}
                None
            }
        }
    }
}

impl<'a,'b> Iterator for TokenStream<'a,'b> {
    type Item = Result<Token,SvError>;

    fn next(&mut self) -> Option<Result<Token,SvError>> {
        match self.get_next_token() {
            Err(e) => if e.kind != SvErrorKind::Null {Some(Err(e))} else {None},
            Ok(t) if t.kind==TokenKind::MacroCall => {
                let b = Vec::new(); // Empty body: unused since top caller
                let a = HashMap::new(); // Empty argument list: unused since top caller
                let pos = t.pos.clone();
                self.macro_expand(t, pos, true, &mut b.into_iter(), &a)
            }
            Ok(t) => Some(Ok(t))
        }
    }
}
