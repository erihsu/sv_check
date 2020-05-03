// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

pub mod astnode;
#[macro_use]
mod common;
mod module_hdr;
mod module_body;
mod package;
mod interface;
mod class;
pub mod uvm_macro;

use std::collections::HashMap;
use std::path::PathBuf;


use astnode::*;
use common::*;
use module_hdr::*;
use module_body::*;
use crate::lex::{
    token::*,
    position::Position,
    token_stream::*
};
use crate::error::*;
use crate::reporter::MsgID;

#[derive(Debug,  Clone)]
pub struct MacroDef {
    pub ports : Vec<(String,Vec<Token>)>,
    pub body  : Vec<Token>,
}

impl MacroDef {
    pub fn new() -> MacroDef { MacroDef{ports:Vec::new(),body:Vec::new()}}
}

pub type Defines = HashMap<String,Option<MacroDef>>;

#[derive(Debug, Clone)]
pub struct Ast {
    pub filename: PathBuf,
    pub tree    : AstNode,
    pub defines : Defines,
}

impl Ast {

    pub fn new(filename: PathBuf) -> Ast {
        Ast {
            filename,
            tree: AstNode::new(AstNodeKind::Root, Position::new()),
            defines: HashMap::new(),
        }
    }

    pub fn build(&mut self, ts : &mut TokenStream) -> Result<(),SvError> {
        loop {
            if let Some(x) = ts.next_non_comment(true) {
                // println!("[AST] {:?}", x);
                match x {
                    Ok(t) => {
                        match t.kind {
                            // Skip Comment / Attribute (TEMP)
                            // TODO: actually use them to try associate comment with a node
                            TokenKind::Comment => {},
                            TokenKind::Macro => parse_macro(ts,&mut self.tree)?,
                            TokenKind::CompDir => parse_macro(ts,&mut self.tree)?,
                            TokenKind::KwModule => {
                                ts.flush_rd();
                                let mut node_m = AstNode::new(AstNodeKind::Module, t.pos);
                                parse_module_hdr(ts,&mut node_m)?;
                                let mut node_b = AstNode::new(AstNodeKind::Body, t.pos);
                                parse_module_body(ts,&mut node_b, ModuleCntxt::Top)?;
                                node_m.child.push(node_b);
                                check_label(ts, &node_m.attr["name"])?;
                                self.tree.child.push(node_m);
                            },
                            TokenKind::KwIntf => {
                                let nt = next_t!(ts,true);
                                ts.rewind(0);
                                match nt.kind {
                                    TokenKind::KwClass => self.tree.child.push(class::parse_class(ts)?),
                                    _ => self.tree.child.push(interface::parse_interface(ts)?)
                                }
                            },
                            TokenKind::KwPackage => {
                                ts.rewind(1);
                                self.tree.child.push(package::parse_package(ts)?);
                            },
                            TokenKind::KwTypedef => parse_typedef(ts,&mut self.tree)?,
                            TokenKind::KwImport  => parse_import(ts,&mut self.tree)?,
                            TokenKind::KwClass => self.tree.child.push(class::parse_class(ts)?),
                            TokenKind::KwVirtual => {
                                let nt = next_t!(ts,true);
                                match nt.kind {
                                    TokenKind::KwClass => self.tree.child.push(class::parse_class(ts)?),
                                    _ => return Err(SvError::syntax(nt, "virtual declaration. Expecting class"))
                                }
                            }
                            TokenKind::KwLParam => {
                                ts.rewind(1); // put back the token so that it can be read by the parse param function
                                // potential list of param (the parse function extract only one at a time)
                                loop {
                                    self.tree.child.push(parse_param_decl(ts,true)?);
                                    loop_args_break_cont!(ts,"parameter declaration",SemiColon);
                                }
                            }
                            TokenKind::SemiColon => ts.flush(1),
                            // Display all un-implemented token (TEMP)
                            _ => {
                                ts.project.log.msg_s(MsgID::DbgSkip, &format!("{} {} | Skipping {}", ts.source.get_filename(), t.pos, t.kind));
                                // println!("[Warning] {:?} -- Root skipping {}",ts.source.get_filename(), t);
                                ts.flush_rd();
                            }
                        }
                    }
                    Err(t) => return Err(t),
                }
            } else {
                self.defines = ts.project.defines.clone();
                // println!("Macro: {:#?}", self.defines);
                return Ok(());
            }
        }
    }

}
