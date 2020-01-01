// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::{SvError};
use crate::lex::token::{TokenKind};
use crate::lex::token_stream::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;
use crate::ast::module_body::{parse_timescale};
use crate::ast::class::{parse_class,parse_func,parse_task};


/// This function should be called after a keyword package
pub fn parse_package(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut node = AstNode::new(AstNodeKind::Package);
    // Parse package header
    let mut t = next_t!(ts,false);
    if t.kind==TokenKind::KwStatic || t.kind==TokenKind::KwAutomatic {
        node.attr.insert("lifetime".to_owned(),t.value);
        t = next_t!(ts,false);
    }
    if t.kind!=TokenKind::Ident {
        return Err(SvError::syntax(t, "package header. Expecting identifier".to_owned()));
    }
    node.attr.insert("name".to_owned(),t.value);
    t = next_t!(ts,false);
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "package header. Expecting ;".to_owned()));
    }
    // Parse package body
    loop {
        t = next_t!(ts,true);
        // println!("[parse_module_body] Token = {}", t);
        match t.kind {
            // Import statement
            TokenKind::KwImport | TokenKind::KwExport => parse_import(ts,&mut node)?,
            TokenKind::KwTimeunit | TokenKind::KwTimeprec => parse_timescale(ts,&mut node)?,
            // Param declaration
            TokenKind::KwParam |
            TokenKind::KwLParam => {
                ts.rewind(1); // put back the token so that it can be read by the parse param function
                // potential list of param (the parse function extract only one at a time)
                loop {
                    let node_param = parse_param_decl(ts,true)?;
                    node.child.push(node_param);
                    let nt = next_t!(ts,false);
                    match nt.kind {
                        TokenKind::Comma => {}, // Comma indicate a list -> continue
                        TokenKind::SemiColon => {break;}, // Semi colon indicate end of statement, stop the loop
                        _ => return Err(SvError::syntax(t, "param declaration, expecting , or ;".to_owned()))
                    }
                }
            }
            TokenKind::KwClass => node.child.push(parse_class(ts)?),
            // Nettype (might need another function to parse the signal to include strength/charge, delay, ...)
            TokenKind::KwConst   |
            TokenKind::KwNetType |
            TokenKind::KwSupply  =>  parse_signal_decl_list(ts,&mut node)?,
            // Basetype
            TokenKind::KwReg         |
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     => parse_signal_decl_list(ts,&mut node)?,
            TokenKind::KwEnum        => {
                let mut node_e = parse_enum(ts,false)?;
                parse_ident_list(ts,&mut node_e)?;
                node.child.push(node_e);
            }
            TokenKind::KwStruct |
            TokenKind::KwUnion  => {
                let mut node_s = parse_struct(ts)?;
                parse_ident_list(ts,&mut node_s)?;
                node.child.push(node_s);
            }
            TokenKind::KwTypedef => parse_typedef(ts,&mut node)?,
            TokenKind::TypeGenvar => {
                ts.flush(0);
                loop {
                    let nt = next_t!(ts,false);
                    match nt.kind {
                        TokenKind::Ident => {
                            let mut n = AstNode::new(AstNodeKind::Declaration);
                            n.attr.insert("type".to_owned(), "genvar".to_owned());
                            n.attr.insert("name".to_owned(),t.value.clone());
                            node.child.push(n);
                            loop_args_break_cont!(ts,"genvar declaration",SemiColon);
                        }
                        _ =>  return Err(SvError::syntax(t,"virtual interface. Expecting identifier".to_owned())),
                    }
                }
            }
            // Identifier -> In a package it can only be a signal declaration
            TokenKind::Ident        => parse_signal_decl_list(ts,&mut node)?,
            TokenKind::Macro        => parse_macro(ts,&mut node)?,
            TokenKind::CompDir      => parse_macro(ts,&mut node)?,
            TokenKind::KwFunction   => parse_func(ts, &mut node, false, false)?,
            TokenKind::KwTask       => parse_task(ts, &mut node)?,
            TokenKind::KwCovergroup => parse_covergroup(ts,&mut node)?,
            // Extra semi-colon
            TokenKind::SemiColon => {ts.flush(1);}, // TODO: generate a warning
            // End module -> parsing of body is done
            TokenKind::KwEndPackage => {
                ts.flush(1);
                check_label(ts, &node.attr["name"])?;
                break;
            },
            // Any un-treated token is an error
            _ => {
                // println!("{}", node);
                return Err(SvError::syntax(t, "package".to_owned()))
            }
        }
    }
    Ok(node)
}
