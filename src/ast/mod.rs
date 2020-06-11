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
use crate::reporter::{REPORTER, MsgID};

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
            match ts.next_t(true) {
                Ok(t) => {
                    // rpt_t!(MsgID::InfoStatus,&t, "top token");
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
                            let nt = ts.next_t(true)?;
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
                        TokenKind::KwImport | TokenKind::KwExport  => parse_import(ts,&mut self.tree)?,
                        TokenKind::KwClass => self.tree.child.push(class::parse_class(ts)?),
                        TokenKind::KwVirtual => {
                            let nt = ts.next_t(true)?;
                            match nt.kind {
                                TokenKind::KwClass => self.tree.child.push(class::parse_class(ts)?),
                                _ => return Err(SvError::syntax(nt, "virtual declaration. Expecting class"))
                            }
                        }
                        // Parameters
                        TokenKind::KwParam |
                        TokenKind::KwLParam => {
                            ts.rewind(1); // put back the token so that it can be read by the parse param function
                            // potential list of param (the parse function extract only one at a time)
                            loop {
                                self.tree.child.push(parse_param_decl(ts,true)?);
                                loop_args_break_cont!(ts,"parameter declaration",SemiColon);
                            }
                        }
                        // Signal declaration
                        TokenKind::KwConst       |
                        TokenKind::KwReg         |
                        TokenKind::KwVar         |
                        TokenKind::TypeIntAtom   |
                        TokenKind::TypeIntVector |
                        TokenKind::TypeReal      |
                        TokenKind::TypeString    |
                        TokenKind::TypeCHandle   |
                        TokenKind::TypeEvent     |
                        TokenKind::Ident          => parse_signal_decl_list(ts,&mut self.tree)?,

                        TokenKind::KwFunction => class::parse_func(ts, &mut self.tree, true, false)?,
                        TokenKind::KwTask     => class::parse_task(ts, &mut self.tree)?,
                        //
                        TokenKind::SemiColon => ts.flush(1),
                        // Display all un-implemented token (TEMP)
                        _ => {
                            rpt_s!(MsgID::DbgSkip, &format!("{}:{} | Parser Skipping {}", ts.source.get_filename(), t.pos, t.kind));
                            // println!("[Warning] {:?} -- Root skipping {}",ts.source.get_filename(), t);
                            ts.flush_rd();
                        }
                    }
                }
                Err(t) => {
                    match t.kind {
                        SvErrorKind::Null |
                        SvErrorKind::Eof  => {
                            // rpt_s!(MsgID::DbgStatus, &format!("{:?}", ts.project.defines.keys()));
                            self.defines = ts.project.defines.clone();
                            return Ok(());
                        }
                        _ => return Err(t),
                    }
                }
            }
        }
    }

}
