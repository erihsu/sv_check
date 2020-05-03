// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::{SvError};
use crate::lex::token::{TokenKind};
use crate::lex::token_stream::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;

/// This function should be called after a keyword module/macromodule
pub fn parse_module_hdr(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    // First extract next token: can be lifetime or identifier.
    // let t = ts.next_non_comment(false);
    let mut t = next_t!(ts,true);
    let mut node_h = AstNode::new(AstNodeKind::Header, t.pos);

    if t.kind==TokenKind::KwStatic || t.kind==TokenKind::KwAutomatic {
        node_h.attr.insert("lifetime".to_owned(),t.value);
        ts.flush(1);
        t = next_t!(ts,true);
    }
    match t.kind {
        TokenKind::Ident => {
            node.attr.insert("name".to_owned(),t.value);
            ts.flush(1);
            t = next_t!(ts,true);
        },
        _ => return Err(SvError::syntax(t, "module/interface declaration, expecting identifier or lifetime (static/automatic)"))
    }
    // Optional package import
    while t.kind == TokenKind::KwImport {
        parse_import(ts,&mut node_h)?;
        t = next_t!(ts,true);
    }
    // Optional parameter list
    if t.kind==TokenKind::Hash {
        ts.flush(1);
        expect_t!(ts,"parameter declaration",TokenKind::ParenLeft);
        t = next_t!(ts,true);
        if t.kind!=TokenKind::ParenRight {
            ts.rewind(1);
            loop {
                let node_port = parse_param_decl(ts,false)?;
                node_h.child.push(node_port);
                loop_args_break_cont!(ts,"parameter declaration",ParenRight);
            }
        } else {ts.flush(1);}
        t = next_t!(ts,true);
    }
    // Optional Port list
    if t.kind==TokenKind::ParenLeft {
        ts.flush(1);
        t = next_t!(ts,true);
        if t.kind!=TokenKind::ParenRight {
            ts.rewind(1);
            loop {
                let node_port = parse_port_decl(ts, false,ExprCntxt::ArgList)?;
                node_h.child.push(node_port);
                loop_args_break_cont!(ts,"port declaration",ParenRight);
            }
        } else {ts.flush(1);}
        t = next_t!(ts,false);
    }
    // End of header
    if t.kind != TokenKind::SemiColon {
        return Err(SvError::syntax(t, "port declaration. Expecting ;"));
    }
    ts.flush(1);
    node.child.push(node_h);
    Ok(())
}
