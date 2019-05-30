use crate::token::*;
use crate::source::Source;

pub struct TokenStream<'a> {
    source: &'a mut Source
}

impl<'a> TokenStream<'a> {


    pub fn new(src: &'a mut Source) -> TokenStream<'a> {
        TokenStream {source: src}
    }

}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;
    
    fn next(&mut self) -> Option<Token> {
        let mut s = String::new();
        while let Some(c) = self.source.get_char() {
            if c.is_whitespace() {
                if s.len() != 0 {
                    break;
                }
            } else {
                s.push(c);
            }
        }
        if s.len() != 0 {
            Some(Token::new(Kind::Ident,s,self.source.pos))
        }
        else {
            None
        }
    }
}
