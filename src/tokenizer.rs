use crate::position::Position;
use crate::token::*;
use crate::source::Source;

pub struct TokenStream<'a> {
    source: &'a mut Source,
    last_char: char,
    last_pos : Position,
}

/// Enum for the state machine parsing number
#[derive(PartialEq, Debug)]
enum NumParseState {Start, Base, Int, Dec, Exp}

impl<'a> TokenStream<'a> {

    // Create a token stream with for a source code
    pub fn new(src: &'a mut Source) -> TokenStream<'a> {
        TokenStream {source: src, last_char : ' ', last_pos : Position::new() }
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
    fn parse_ident(&mut self, first_char: char) -> Result<Token,TokenError> {
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
                    return Err(TokenError::new(Kind::SystemTask,p,s));
                }
                Kind::SystemTask
            }
            else if first_char == '`' {
                if is_pathpulse || is_casting || s.len()==1 {
                    return Err(TokenError::new(Kind::Macro,p,s));
                }
                Kind::Macro
            }
            else if is_casting {
                Kind::Casting
            }
            else if BASETYPES.contains(&&s[..]) {
                Kind::BaseType
            } else if KEYWORDS.contains(&&s[..]) || is_pathpulse {
                if is_casting {
                    return Err(TokenError::new(Kind::Casting,p,s));
                }
                Kind::Keyword
            } else {
                Kind::Ident
            }
        };
        Ok(Token::new(k,s,p))
    }

    /// Get all characters until end of string
    fn parse_string(&mut self) -> Result<Token,TokenError> {
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
        Ok(Token::new(Kind::Str,s,p))
    }

    /// Get all characters until end of line
    fn parse_comment_line(&mut self) -> Result<Token,TokenError> {
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
        Ok(Token::new(Kind::Comment,s,p))
    }

    /// Get all characters until */
    fn parse_comment_block(&mut self) -> Result<Token,TokenError> {
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
        Ok(Token::new(Kind::Comment,s,p))
    }

    /// Get all characters until *)
    fn parse_attribute(&mut self) -> Result<Token,TokenError> {
        let mut s = String::from("(");
        let p = self.last_pos;
        while let Some(c) = self.source.get_char() {
            s.push(c);
            if c == ')' && self.last_char == '*' {
                self.last_char = ' '; // Last char is consume
                break;
            }
            self.last_char = c;
        }
        self.last_pos = self.source.pos;
        Ok(Token::new(Kind::Attribute,s,p))
    }


    /// Parse a number (real or integer)
    fn parse_number(&mut self, first_char: char) -> Result<Token,TokenError> {
        let mut s = String::new();
        let p = self.last_pos;
        let mut has_xz = false;
        let mut fsm = if first_char=='\'' {NumParseState::Base} else  {NumParseState::Start};
        s.push(first_char);
        while let Some(c) = self.source.get_char() {
            match c {
                // x/z allowed for integer number only
                'x'|'X'|'z'|'Z' => {
                    if fsm != NumParseState::Start && fsm != NumParseState::Int {
                        return Err(TokenError::new(Kind::Integer,p,s));
                    }
                    has_xz = true;
                    s.push(c);
                }
                // Base specifier
                '\'' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(TokenError::new(Kind::Integer,p,s));
                    }
                    fsm = NumParseState::Base;
                    s.push(c);
                },
                's'|'S' => {
                    if fsm != NumParseState::Base || self.last_char!='\'' {
                        return Err(TokenError::new(Kind::Integer,p,s));
                    }
                    s.push(c);
                }
                'b'|'o'|'h'|'d'|'B'|'O'|'H'|'D' => {
                    if fsm != NumParseState::Base {
                        return Err(TokenError::new(Kind::Integer,p,s));
                    }
                    fsm = NumParseState::Int;
                    s.push(c);
                }
                // Dot -> real number
                '.' => {
                    if fsm != NumParseState::Start || has_xz {
                        return Err(TokenError::new(Kind::Real,p,s));
                    }
                    fsm = NumParseState::Dec;
                    s.push(c);
                }
                // Exponent -> real number
                'e' | 'E' => {
                    if (fsm != NumParseState::Start && fsm != NumParseState::Dec) || has_xz {
                        return Err(TokenError::new(Kind::Real,p,s));
                    }
                    fsm = NumParseState::Exp;
                    s.push(c);
                }
                // Sign exponent
                '+' | '-' => {
                    if fsm != NumParseState::Exp || (self.last_char!='e' && self.last_char!='E') || has_xz {
                        return Err(TokenError::new(Kind::Real,p,s));
                    }
                    s.push(c);
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
        let k = if fsm==NumParseState::Dec || fsm==NumParseState::Exp {Kind::Real} else {Kind::Integer};
        Ok(Token::new(k,s,p))
    }

    /// Return the next token from the source code
    pub fn get_next_token(&mut self) -> Result<Token,TokenError> {
        let c = self.get_first_char().ok_or(TokenError::new(Kind::None,self.last_pos,"".to_string()))?;
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
                        let t = Token::new(Kind::OpCompAss,"/=".to_string(),p);
                        self.source.get_char();  // Consume peeked character
                        return Ok(t)
                    }
                    _ => return Ok(Token::new(Kind::OpDiv,"/".to_string(),p))
                }
            }
            '%' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => return Ok(Token::new(Kind::OpCompAss ,"%=".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpMod,"%".to_string(),p))
                    }
                }
            }
            '+' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '+' => return Ok(Token::new(Kind::OpIncrDecr,"++".to_string(),p)) ,
                    '=' => return Ok(Token::new(Kind::OpCompAss ,"+=".to_string(),p)) ,
                    ':' => return Ok(Token::new(Kind::OpRange   ,"+:".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpPlus,"+".to_string(),p))
                    }
                }
            }
            '-' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '-' => return Ok(Token::new(Kind::OpIncrDecr,"--".to_string(),p)) ,
                    '=' => return Ok(Token::new(Kind::OpCompAss ,"-=".to_string(),p)) ,
                    ':' => return Ok(Token::new(Kind::OpRange   ,"-:".to_string(),p)) ,
                    '>' => return Ok(Token::new(Kind::OpImpl    ,"->".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpMinus,"-".to_string(),p))
                    }
                }
            }
            '*' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '*' => return Ok(Token::new(Kind::OpPow,"**".to_string(),p)) ,
                    '=' => return Ok(Token::new(Kind::OpCompAss ,"*=".to_string(),p)) ,
                    '>' => return Ok(Token::new(Kind::OpStarLT ,"*>".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpStar,"*".to_string(),p))
                    }
                }
            }
            '^' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '~' => return Ok(Token::new(Kind::OpXnor    ,"^~".to_string(),p)) ,
                    '=' => return Ok(Token::new(Kind::OpCompAss ,"^=".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpXor,"^".to_string(),p))
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
                                return Ok(Token::new(Kind::OpTimingAnd,"&&&".to_string(),p))
                            },
                            _ => return Ok(Token::new(Kind::OpLogicAnd,"&&".to_string(),p))
                        }
                    }
                    '=' => return Ok(Token::new(Kind::OpCompAss ,"&=".to_string(),p)) ,
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpAnd,"&".to_string(),p))
                    }
                }
            }
            '|' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '|' => return Ok(Token::new(Kind::OpLogicOr,"||".to_string(),p)) ,
                    '=' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        match nnc {
                            '>' => {
                                self.source.get_char();  // Consume peeked character
                                return Ok(Token::new(Kind::OpSeqRel,"|=>".to_string(),p))
                            },
                            _ => return Ok(Token::new(Kind::OpCompAss,"|=".to_string(),p))
                        }
                    }
                    '-' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        match nnc {
                            '>' => return Ok(Token::new(Kind::OpSeqRel,"|->".to_string(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(Kind::OpOr,"|".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpOr,"|".to_string(),p))
                    }
                }
            }
            '<' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => return Ok(Token::new(Kind::OpLTE,"<=".to_string(),p)) ,
                    '<' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(Kind::OpCompAss,"<<=".to_string(),p)) ,
                            '<' => {
                                let nc = self.source.get_char().unwrap_or(' ');
                                match nc {
                                    '=' => return Ok(Token::new(Kind::OpCompAss,"<<<=".to_string(),p)) ,
                                    _ => {
                                        self.updt_last(nc);
                                        return Ok(Token::new(Kind::OpSShift,"<<<".to_string(),p))
                                    }
                                }
                            }
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(Kind::OpSL,"<<".to_string(),p))
                            }
                        }
                    }
                    // Check for equivalence operator <->
                    '-' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        if nnc == &'>' {
                            self.source.get_char().unwrap(); // Consume next char
                            return Ok(Token::new(Kind::OpEquiv,"<->".to_string(),p))
                        } else {
                            self.updt_last(nc);
                            return Ok(Token::new(Kind::OpLT,"<".to_string(),p))
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpLT,"<".to_string(),p))
                    }
                }
            }
            '>' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => return Ok(Token::new(Kind::OpGTE,">=".to_string(),p)) ,
                    '>' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(Kind::OpCompAss,">>=".to_string(),p)) ,
                            '>' => {
                                let nc = self.source.get_char().unwrap_or(' ');
                                match nc {
                                    '=' => return Ok(Token::new(Kind::OpCompAss,">>>=".to_string(),p)) ,
                                    _ => {
                                        self.updt_last(nc);
                                        return Ok(Token::new(Kind::OpSShift,">>>".to_string(),p))
                                    }
                                }
                            }
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(Kind::OpSR,">>".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpGT,">".to_string(),p))
                    }
                }
            }
            '!' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '=' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(Kind::OpDiff2,"!==".to_string(),p)) ,
                            '?' => return Ok(Token::new(Kind::OpDiffQue,"!=?".to_string(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(Kind::OpDiff,"!=".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpBang,"!".to_string(),p))
                    }
                }
            }
            '=' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '>' => return Ok(Token::new(Kind::OpFatArrL,"=>".to_string(),p)) ,
                    '=' => {
                        let nc = self.source.get_char().unwrap_or(' ');
                        match nc {
                            '=' => return Ok(Token::new(Kind::OpEq3,"===".to_string(),p)) ,
                            '?' => return Ok(Token::new(Kind::OpEq2Que,"==?".to_string(),p)) ,
                            _ => {
                                self.updt_last(nc);
                                return Ok(Token::new(Kind::OpEq2,"==".to_string(),p))
                            }
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::OpEq,"=".to_string(),p))
                    }
                }
            }
            '~' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '&' => return Ok(Token::new(Kind::OpNand ,"~&".to_string(),p)),
                    '|' => return Ok(Token::new(Kind::OpNor  ,"~|".to_string(),p)),
                    '^' => return Ok(Token::new(Kind::OpXnor ,"~^".to_string(),p)),
                    _   => return Ok(Token::new(Kind::OpTilde,"~".to_string() ,p)),
                }
            }
            // Parenthesis
            '(' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '*' => return self.parse_attribute() ,
                    _ => return Ok(Token::new(Kind::ParenLeft  ,"(".to_string(),p)),
                }
            }
            ')' => return Ok(Token::new(Kind::ParenRight ,")".to_string(),p)),
            '[' => return Ok(Token::new(Kind::SquareLeft ,"[".to_string(),p)),
            ']' => return Ok(Token::new(Kind::SquareRight,"]".to_string(),p)),
            '{' => return Ok(Token::new(Kind::CurlyLeft  ,"{".to_string(),p)),
            '}' => return Ok(Token::new(Kind::CurlyRight ,"}".to_string(),p)),
            // Special characters
            ',' => return Ok(Token::new(Kind::Comma    ,",".to_string(),p)),
            '?' => return Ok(Token::new(Kind::Que      ,"?".to_string(),p)),
            ';' => return Ok(Token::new(Kind::SemiColon,";".to_string(),p)),
            '@' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '@' => return Ok(Token::new(Kind::At2 ,"@@".to_string(),p)),
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::At ,"@".to_string(),p))
                    }
                }
            }
            '.' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    '*' => return Ok(Token::new(Kind::DotStar,".*".to_string(),p)),
                    _ => return Ok(Token::new(Kind::Dot      ,".".to_string(),p)),
                }

            }
            '$' => {
                let nc = self.source.peek_char().unwrap_or(&' ');
                match nc {
                    'a'...'z' | 'A'...'Z' | '_'  => return self.parse_ident(c),
                    _ => return Ok(Token::new(Kind::Dollar ,"$".to_string(),p)),
                }

            }
            '#' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '#' => return Ok(Token::new(Kind::Hash2,"##".to_string(),p)),
                    '-' | '=' => {
                        let nnc = self.source.peek_char().unwrap_or(&' ');
                        if nnc == &'#' {
                            self.source.get_char().unwrap(); // Consume next char
                            return Ok(Token::new(Kind::OpEquiv, format!("#{}#", nc),p))
                        } else {
                            self.updt_last(nc);
                            return Ok(Token::new(Kind::Hash2,"##".to_string(),p))
                        }
                    }
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::Hash,"#".to_string(),p))
                    }
                }
            }
            ':' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    ':' => return Ok(Token::new(Kind::Scope,"::".to_string(),p)),
                    '/' => return Ok(Token::new(Kind::OpDist,":/".to_string(),p)),
                    '=' => return Ok(Token::new(Kind::OpDist,":=".to_string(),p)),
                    _ => {
                        self.updt_last(nc);
                        return Ok(Token::new(Kind::Colon,":".to_string(),p))
                    }
                }
            }
            //
            '\'' => {
                let nc = self.source.get_char().unwrap_or(' ');
                match nc {
                    '{' => Ok(Token::new(Kind::TickCurly ,"'{".to_string(),p)) ,
                    _ => Err(TokenError::new(Kind::Casting,p,'\''.to_string()))
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

}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token,TokenError>;

    fn next(&mut self) -> Option<Result<Token,TokenError>> {
        match self.get_next_token() {
            Err(e) => if e.kind != Kind::None {Some(Err(e))} else {None},
            Ok(t) => Some(Ok(t))
        }
    }
}
