// This file is part of sv_parser and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::{SvError};
use crate::token::{TokenKind};
use crate::tokenizer::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;
use crate::ast::module_hdr::{parse_module_hdr};
use crate::ast::module_body::*;

// TODO: rework to reuse a maximum of what s already done in module body parser

/// This function should be called after a keyword interface
pub fn parse_interface(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut node = AstNode::new(AstNodeKind::Interface);
    // Parse interface header
    parse_module_hdr(ts, &mut node)?;
    // Parse package body
    loop {
        let t = next_t!(ts,true);
        // println!("[parse_module_body] Token = {}", t);
        match t.kind {
            // Modport
            TokenKind::KwModport => parse_modport(ts,&mut node)?,
            // clocking block
            TokenKind::KwDefault | TokenKind::KwClocking | TokenKind::KwGlobal => parse_clocking(ts,&mut node)?,
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
            // Identifier -> lookahead to detect if it is a signal declaration or an instantiation
            TokenKind::Ident => {
                let nt = next_t!(ts,true);
                // println!("[Module body] Ident followed by {}", nt.kind);
                match nt.kind {
                    // Scope -> this is a type definition
                    TokenKind::Scope => parse_signal_decl_list(ts,&mut node)?,
                    // Identifier : could be a signal declaration or a module/interface instantiation
                    TokenKind::Ident => {
                        let nnt = next_t!(ts,true);
                        // println!("[Module body] (Ident Ident) followed by {}", nnt.kind);
                        match nnt.kind {
                            // Opening parenthesis indicates
                            // Semi colon or comma indicate signal declaration
                            TokenKind::SemiColon |
                            TokenKind::Comma     =>  parse_signal_decl_list(ts,&mut node)?,
                            // Range -> can be either an unpacked array declaration or an array of instance ...
                            // TODO: handle case of array of instances
                            TokenKind::SquareLeft =>  {
                                parse_signal_decl_list(ts,&mut node)?;
                            }
                            // Open parenthesis -> instance
                            TokenKind::ParenLeft => {
                                let node_inst = parse_instance(ts)?;
                                node.child.push(node_inst);
                            }
                            _ => return Err(SvError::syntax(t, " signal declaration or instance".to_string()))
                        }
                    }
                    // Dash is a clear indicator of an instance -> TODO
                    TokenKind::Hash => {
                        let node_inst = parse_instance(ts)?;
                        node.child.push(node_inst);
                    }
                    // Untreated token are forbidden
                    _ => return Err(SvError::syntax(t, " signal declaration or instance, expecting type or instance".to_string()))
                }
            }
            // End module -> parsing of body is done
            TokenKind::KwAssign => {
                ts.flush(0); // Consume assign keyword (should not be present to parse it)
                let node_assign = parse_assign_c(ts)?;
                node.child.push(node_assign);
            }
            // Always keyword
            TokenKind::KwAlways  |
            TokenKind::KwAlwaysC |
            TokenKind::KwAlwaysF |
            TokenKind::KwAlwaysL => {
                let node_proc = parse_always(ts)?;
                node.child.push(node_proc);
            }
            //
            TokenKind::Macro => parse_macro(ts,&mut node)?,
            // TokenKind::KwGenerate if cntxt==ModuleCntxt::Top => parse_module_body(ts,node,ModuleCntxt::Generate)?,
            TokenKind::KwFor  => parse_for(ts,&mut node)?,
            TokenKind::KwIf   => {
                ts.flush(0);
                parse_if_else(ts,&mut node, "if".to_string(), true)?;
            }
            // End of loop depends on context
            // TokenKind::KwEnd         if cntxt == ModuleCntxt::ForBlock => break,
            // TokenKind::KwEnd         if cntxt == ModuleCntxt::IfBlock  => break,
            // TokenKind::KwEndGenerate if cntxt == ModuleCntxt::Generate => break,
            // End module -> parsing of body is done
            TokenKind::KwEndIntf => break,
            // Any un-treated token is an error
            _ => {
                // println!("{}", node);
                return Err(SvError::syntax(t, "interface".to_string()))
            }
        }
    }
    ts.flush(0);
    Ok(node)
}

/// Parse an always block
pub fn parse_modport(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush(0); // Suppose modport keyword is now consumed
    let mut node_mp = AstNode::new(AstNodeKind::Modport);
    // Expect identifier for the modport name
    let mut t = next_t!(ts,false);
    if t.kind!=TokenKind::Ident {
        return Err(SvError::syntax(t,"modport. Expecting Identifier".to_string()));
    }
    node_mp.attr.insert("name".to_string(),t.value);
    // Expect open parenthesis
    t = next_t!(ts,false);
    if t.kind!=TokenKind::ParenLeft {
        return Err(SvError::syntax(t,"modport. Expecting (".to_string()));
    }
    // Expect a list of (input|output|inout|ref|clocking|import|export) id1, id0, ...
    // In case of import should also support import with task prototype
    // In case of port (in/out/inout) need to support port expresison in the form .ID(expr)
    loop {
        t = next_t!(ts,false);
        let mut node_p = AstNode::new(AstNodeKind::Port);
        match t.kind {
            TokenKind::KwInput | TokenKind::KwOutput | TokenKind::KwInout | TokenKind::KwRef => {
                node_p.attr.insert("dir".to_string(), t.value);
                t = next_t!(ts,false);
                match t.kind {
                    TokenKind::Ident => {
                        node_p.attr.insert("name".to_string(), t.value);
                        t = next_t!(ts,false);
                        match t.kind {
                            TokenKind::Comma => {},
                            TokenKind::ParenRight => break,
                            _ =>  return Err(SvError::syntax(t,"modport. Expecting , or )".to_string())),
                        }
                    }
                    // modport expression
                    TokenKind::Dot => {
                        t = next_t!(ts,false);
                        if t.kind!=TokenKind::Ident {
                            return Err(SvError::syntax(t,"modport. Expecting identifier".to_string()));
                        }
                        node_p.attr.insert("name".to_string(), t.value);
                    }
                    _ =>  return Err(SvError::syntax(t,"modport. Expecting port name/expression".to_string())),
                }
            }
            TokenKind::KwClocking => {
                node_p.kind = AstNodeKind::Clocking;
                t = next_t!(ts,false);
                match t.kind {
                    TokenKind::Ident => {
                        node_p.attr.insert("name".to_string(), t.value);
                        t = next_t!(ts,false);
                        match t.kind {
                            TokenKind::Comma => {},
                            TokenKind::ParenRight => break,
                            _ =>  return Err(SvError::syntax(t,"modport. Expecting , or )".to_string())),
                        }
                    }
                    _ =>  return Err(SvError::syntax(t,"modport. Expecting port name/expression".to_string())),
                }
            }
            TokenKind::Ident => {
                node_p.attr.insert("name".to_string(), t.value);
                t = next_t!(ts,false);
                match t.kind {
                    TokenKind::Comma => {},
                    TokenKind::ParenRight => break,
                    _ =>  return Err(SvError::syntax(t,"modport. Expecting , or )".to_string())),
                }
            }
            // TODO : support port expression
            _ =>  return Err(SvError::syntax(t,"modport. Expecting direction/identifier/cloking/import/export (".to_string())),
        }
        node_mp.child.push(node_p);
    }
    // Expect semi-colon
    t = next_t!(ts,false);
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t,"modport. Expecting ;".to_string()));
    }
    node.child.push(node_mp);
    Ok(())
}

/// Parse an always block
pub fn parse_clocking(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut node_c = AstNode::new(AstNodeKind::Clocking);
    // Optionnal default/global
    let mut t = next_t!(ts,false);
    let has_id = t.kind!=TokenKind::KwDefault;
    let need_event = t.kind==TokenKind::KwClocking;
    if t.kind== TokenKind::KwDefault || t.kind== TokenKind::KwDefault {
        node_c.attr.insert("scope".to_string(), t.value);
        t = next_t!(ts,false);
    }
    // Expect clocking keyword
    if t.kind!=TokenKind::KwClocking {
        return Err(SvError::syntax(t,"clocking block. Expecting ;".to_string()));
    }
    t = next_t!(ts,false);
    // Clocking block identifier : optional when default
    if t.kind == TokenKind::Ident {
        node_c.attr.insert("name".to_string(), t.value);
        t = next_t!(ts,false);
    } else if has_id {
        return Err(SvError::syntax(t,"clocking block. Expecting identifier".to_string()));
    }
    // Expect clocking event
    match t.kind {
        TokenKind::At => {},
        TokenKind::SemiColon if !need_event => {
            node.child.push(node_c);
            return Ok(());
        },
        _ => return Err(SvError::syntax(t,"clocking block. Expecting @".to_string()))
    }
    node_c.child.push(parse_sensitivity(ts,true)?);
    //
    t = next_t!(ts,false);
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t,"clocking block. Expecting ;".to_string()));
    }
    loop {
        t = next_t!(ts,false);
        match t.kind {
            TokenKind::KwEndClocking => break,
            // TODO: actual parsing of clocking block
            _ => {},
        }
    }

    node.child.push(node_c);
    Ok(())
}
