mod astnode;
mod module;

use astnode::*;
use crate::token::*;
use crate::tokenizer::*;
use crate::error::*;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Ast {
    pub tree  : AstNode,
    token_buf : Vec<Token>
}

impl Ast {

    pub fn new() -> Ast {
        Ast {
            tree: AstNode::new(AstNodeKind::Root),
            token_buf: Vec::new(),
        }
    }

    pub fn build(&mut self, ts : &mut TokenStream) -> Result<(),SvError> {
        loop {
            if let Some(x) = ts.next() {
                match x {
                    Ok(t)  => {
                        match t.kind {
                            // Skip Comment / Atrribute (TEMP)
                            // TODO: actually use them to try associate comment with a node
                            TokenKind::Comment => continue,
                            TokenKind::KwModule => {
                                match module::parse_module_hdr(ts) {
                                    Ok(n) => self.tree.child.push(n),
                                    Err(e) => return Err(e)
                                }
                            },
                            // Display all un-implemented token (TEMP)
                            _ => println!("{}", t),
                        }
                    },
                    Err(e) => {println!("{}", e); break;},
                }
            }
            else {break;}

        }
        Ok(())
    }

}

