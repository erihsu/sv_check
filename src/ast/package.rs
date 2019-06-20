// This file is part of sv_parser and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::{SvError};
use crate::token::{TokenKind};
use crate::tokenizer::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;


/// This function should be called after a keyword package
pub fn parse_package(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut node = AstNode::new(AstNodeKind::Package);
    // Parse package header
    let mut t = next_t!(ts,false);
    if t.kind==TokenKind::KwStatic || t.kind==TokenKind::KwAutomatic {
        node.attr.insert("lifetime".to_string(),t.value);
        t = next_t!(ts,false);
    }
    if t.kind!=TokenKind::Ident {
        return Err(SvError::syntax(t, "package header. Expecting identifier".to_string()));
    }
    node.attr.insert("name".to_string(),t.value);
    t = next_t!(ts,false);
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "package header. Expecting ;".to_string()));
    }
    // Parse package body
    loop {
        t = next_t!(ts,true);
        // println!("[parse_module_body] Token = {}", t);
        match t.kind {
            // Import statement
            TokenKind::KwImport => parse_import(ts,&mut node)?,
            // Only local param declaration
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
                        _ => return Err(SvError::syntax(t, "param declaration, expecting , or ;".to_string()))
                    }
                }
            }
            // Nettype (might need another function to parse the signal to include strength/charge, delay, ...)
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
                let mut node_e = parse_enum(ts)?;
                let s = parse_ident_list(ts)?;
                node_e.attr.insert("name".to_string(),s);
                node.child.push(node_e);
            }
            TokenKind::KwTypedef => parse_typedef(ts,&mut node)?,
            TokenKind::TypeGenvar => {
                ts.flush(0);
                let mut s = "".to_string();
                loop {
                    let mut nt = next_t!(ts,false);
                    if nt.kind!=TokenKind::Ident {
                        return Err(SvError::syntax(t, "after genvar, expecting identifier".to_string()));
                    }
                    s.push_str(&nt.value);
                    nt = next_t!(ts,false);
                    match nt.kind {
                        TokenKind::Comma => s.push_str(", "),
                        TokenKind::SemiColon => break,
                        _ => return Err(SvError::syntax(t, "genvar, expecting  , or ;".to_string()))
                    }
                }
                node.child.push(AstNode::new(AstNodeKind::Genvar(s)));
            }
            // Identifier -> In a package it can only be a signal declaration
            TokenKind::Ident => parse_signal_decl_list(ts,&mut node)?,
            // End module -> parsing of body is done
            TokenKind::KwEndPackage => break,
            // Any un-treated token is an error
            _ => {
                // println!("{}", node);
                return Err(SvError::syntax(t, "package".to_string()))
            }
        }
    }
    ts.flush(0);
    Ok(node)
}
