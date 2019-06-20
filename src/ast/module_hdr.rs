// This file is part of sv_parser and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::{SvErrorKind, SvError, };
use crate::token::{TokenKind};
use crate::tokenizer::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;

/// This function should be called after a keyword module/macromodule
pub fn parse_module_hdr(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    // First extract next token: can be lifetime or identifier.
    // let t = ts.next_non_comment(false);
    let mut has_lifetime : Option<TokenKind> = None;
    let mut node_h = AstNode::new(AstNodeKind::Header);
    let mut t = next_t!(ts,false);

    if t.kind==TokenKind::KwStatic || t.kind==TokenKind::KwAutomatic {
        has_lifetime = Some(t.kind);
        t = next_t!(ts,false);
    }
    match t.kind {
        TokenKind::Ident => {
            node.attr.insert("name".to_string(),t.value);
            t = next_t!(ts,false);
        },
        _ => return Err(SvError::syntax(t, "module/interface declaration, expecting identifier or lifetime (static/automatic)".to_string()))
    }
    // Create Node woth two child: header/body

    // Add lifetime info
    if has_lifetime.is_some() {
        node_h.attr.insert("lifetime".to_string(),format!("{:?}", has_lifetime.unwrap()));
    }
    // Optional package import
    while t.kind == TokenKind::KwImport {
        parse_import(ts,&mut node_h)?;
        t = next_t!(ts,false);
    }
    // Optional parameter list
    if t.kind==TokenKind::Hash {
        t = next_t!(ts,false);
        if t.kind!=TokenKind::ParenLeft {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting ( after #, got {} ({:?})",t.value, t.kind)));
        }
        loop {
            let node_port = parse_param_decl(ts,false)?;
            node_h.child.push(node_port);
            t = next_t!(ts,false);
            match t.kind {
                // Comma -> the port list continue
                TokenKind::Comma => {},
                // Right parenthesis, port list is done, get next token
                TokenKind::ParenRight => {
                    t = next_t!(ts,false);
                    break;
                },
                // Any other token is an error
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Expecting , or ) in param declaration, got {}", t.value)))
            }
        }
    }
    // Optional Port list
    if t.kind==TokenKind::ParenLeft {
        loop {
            let node_port = parse_port_decl(ts)?;
            node_h.child.push(node_port);
            t = next_t!(ts,false);
            match t.kind {
                // Comma -> the port list continue
                TokenKind::Comma => {},
                // Right parenthesis, port list is done, get next token
                TokenKind::ParenRight => {
                    t = next_t!(ts,false);
                    break;
                },
                // Any other token is an error
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Expecting , or ) in port declaration, got {}", t.value)))
            }
        }
    }
    // End of header
    if t.kind != TokenKind::SemiColon {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting ; in port declaration, got {}", t.value)))
    }
    node.child.push(node_h);
    // println!("{}", node);
    Ok(())
}


/// Parse a port declaration
fn parse_port_decl(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut t = next_t!(ts,true);
    let mut node = AstNode::new(AstNodeKind::Port);
    let mut type_found = false;
    // println!("[parse_port_decl] First token = {:?}", t);
    // direction/interface
    match t.kind {
        TokenKind::KwInput | TokenKind::KwOutput | TokenKind::KwInout | TokenKind::KwRef => {
            node.attr.insert("dir".to_string(), t.value);
            ts.flush(0);
            t = next_t!(ts,true);
        }
        // Interface
        TokenKind::Ident => {
            type_found = true;
            // Check if mod port is available
            let nt = next_t!(ts,true);
            match nt.kind {
                // Dot : t is the interface type and token is the modport (expect identifier)
                TokenKind::Dot => {
                    let nnt = next_t!(ts,true);
                    if nnt.kind != TokenKind::Ident {
                        return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) for port type",nnt.value, nnt.kind)))
                    }
                    node.attr.insert("intf".to_string(), t.value);
                    node.attr.insert("modport".to_string(), nnt.value);
                    ts.flush(0);
                }
                // Another ident : No modport, nt is the port name, rewind it
                TokenKind::Ident => ts.rewind(1),
                // Comma/parenthesis -> t was the port name
                TokenKind::Comma | TokenKind::ParenRight  => {
                    type_found = true;
                    ts.rewind(2);
                }

                // Any other token is an error
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in port declaration.",nt.value, nt.kind)))
            }
        }
        TokenKind::KwIntf => {
            type_found = true;
            // Check if mod port is available
            let nt = next_t!(ts,true);
            match nt.kind {
                // Dot : t is the interface type and token is the modport (expect identifier)
                TokenKind::Dot => {
                    let nnt = next_t!(ts,true);
                    if nnt.kind != TokenKind::Ident {
                        return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) for port type, expecting modport",nnt.value, nnt.kind)))
                    }
                    node.attr.insert("intf".to_string(), t.value);
                    node.attr.insert("modport".to_string(), nnt.value);
                    ts.flush(0);
                }
                // Another ident : No modport, nt is the port name, rewind it
                TokenKind::Ident => ts.rewind(1),
                // Any other token is an error
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in port declaration.",nt.value, nt.kind)))
            }
        }
        // Handle Ident
        _ => {}
    }
    if ! type_found {
        // Optional net type
        match t.kind {
            TokenKind::KwSupply | TokenKind::KwNetType => {ts.flush(0)},
            _ => {}
        }
        // Optional data type
        parse_data_type(ts,&mut node)?;
    }
    // Port name
    // ts.display_status();
    t = next_t!(ts,false);
    // println!("[parse_port_decl] Expecting port name = {:?}", t);
    if t.kind != TokenKind::Ident {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting port identifier, got {} ({:?})", t.value, t.kind)))
    }
    node.attr.insert("name".to_string(), t.value);
    // Optional Unpacked dimension : [x][y:z]
    t = next_t!(ts,true);
    // println!("[parse_port_decl] After port name token = {:?}", t);
    if t.kind == TokenKind::SquareLeft {
        ts.flush(0);
        node.attr.insert("unpacked".to_string(), parse_range(ts)?);
    }
    // Optional Default value i.e. "= expr"
    // t = next_t!(ts,false);

    // println!("{}", node.to_string_lvl(1));
    Ok(node)
}
