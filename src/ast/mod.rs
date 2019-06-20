// This file is part of sv_parser and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

mod astnode;
#[macro_use]
mod common;
mod module_hdr;
mod module_body;
mod package;
mod interface;

use astnode::*;
use common::*;
use module_hdr::*;
use module_body::*;
use crate::token::*;
use crate::tokenizer::*;
use crate::error::*;


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
            if let Some(x) = ts.next_non_comment(true) {
                // println!("[AST] {:?}", x);
                match x {
                    Ok(t) => {
                        match t.kind {
                            // Skip Comment / Atrribute (TEMP)
                            // TODO: actually use them to try associate comment with a node
                            TokenKind::Comment => {},
                            TokenKind::Macro => parse_macro(ts,&mut self.tree)?,
                            TokenKind::KwModule => {
                                ts.flush(0);
                                let mut node_m = AstNode::new(AstNodeKind::Module);
                                parse_module_hdr(ts,&mut node_m)?;
                                let mut node_b = AstNode::new(AstNodeKind::Body);
                                parse_module_body(ts,&mut node_b, ModuleCntxt::Top)?;
                                node_m.child.push(node_b);
                                self.tree.child.push(node_m);
                            },
                            TokenKind::KwIntf => {
                                ts.flush(0);
                                match interface::parse_interface(ts) {
                                    Ok(n) => self.tree.child.push(n),
                                    Err(e) => return Err(e)
                                }
                            },
                            TokenKind::KwPackage => {
                                ts.flush(0);
                                match package::parse_package(ts) {
                                    Ok(n) => self.tree.child.push(n),
                                    Err(e) => return Err(e)
                                }
                            },
                            // Display all un-implemented token (TEMP)
                            _ => {
                                ts.flush(0);
                                //println!("{}", t),
                            }
                        }
                    }
                    Err(t) => return Err(t),
                }
            } else {
                return Ok(());
            }
        }
    }

}

