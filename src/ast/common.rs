// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::reporter::{REPORTER, MsgID};
use crate::lex::token::TokenKind;
use crate::lex::token_stream::{TokenStream};
use crate::ast::{
    MacroDef,
    astnode::{AstNode, AstNodeKind},
    class::{parse_func,parse_class_stmt_or_block}
};
use crate::error::{SvErrorKind, SvError};

macro_rules! expect_t {
    ($ts:expr, $cntxt:expr, $($exp_tk:expr),+) => {{
        let t = $ts.next_t(false)?;
        let mut m = false;
        // let mut s = "".to_owned();
        $(
            m |= t.kind == $exp_tk;
        )+
        if !m {
            return Err(SvError::syntax(t,  &format!("{}. Expecting {:?}", $cntxt.to_string(), ($($exp_tk),+) ) ));
        }
        t
    }};
}

macro_rules! loop_args_break_cont {
    ($ts:expr, $cntxt:expr, $tk:ident) => {{
        let t = $ts.next_t(false)?;
        match t.kind {
            // Comma -> the port list continue
            TokenKind::Comma => {},
            // Right parenthesis, port list is done
            TokenKind::$tk => break,
            // Any other token is an error
            _ => return Err(SvError::syntax(t, &format!("{}. Expecting , or {}", $cntxt,TokenKind::$tk)))
        }
    }}
}


macro_rules! parse_opt_params {
    ($ts:expr, $node:expr, $t:expr) => {
        if $t.kind==TokenKind::Hash {
            $ts.flush_rd(); // Consume the hash token
            let mut node_p = AstNode::new(AstNodeKind::Params, $ts.get_pos());
            parse_port_connection($ts,&mut node_p,true)?;
            $node.child.push(node_p);
            $t = $ts.next_t(true)?;
        }
    };
    ($ts:expr, $node:expr) => {
        let t = $ts.next_t(true)?;
        if t.kind==TokenKind::Hash {
            $ts.flush_rd(); // Consume the hash token
            let mut node_p = AstNode::new(AstNodeKind::Params, $ts.get_pos());
            parse_port_connection($ts,&mut node_p,true)?;
            $node.child.push(node_p);
        } else {
            $ts.rewind(1);
        }
    };
}

#[allow(dead_code)]
#[derive(PartialEq,Clone,Debug)]
pub enum ExprCntxt {
    ArgList, Arg, ExprGroup,
    StmtList, Stmt, CaseItemList,
    FieldList, Sensitivity,
    BracketMsb, BracketLsb,
    Question
}

/// Parse an import/export statement
pub fn parse_import(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(1);
    let mut t = expect_t!(ts,"import/export",TokenKind::KwImport,TokenKind::KwExport);
    let mut n = AstNode::new(AstNodeKind::Import, t.pos);
    n.attr.insert("kind".to_owned(),t.value);
    t = ts.next_t(true)?;
    match t.kind {
        TokenKind::Ident => {
            ts.rewind(1);
            loop {
                t = expect_t!(ts,"import",TokenKind::Ident);
                let mut ni = AstNode::new(AstNodeKind::Identifier, t.pos);
                ni.attr.insert("pkg_name".to_owned(),t.value);
                expect_t!(ts,"import",TokenKind::Scope);
                t = expect_t!(ts,"import",TokenKind::Ident,TokenKind::OpStar);
                ni.attr.insert("name".to_owned(),t.value);
                n.child.push(ni);
                t = ts.next_t(false)?;
                match t.kind {
                    TokenKind::SemiColon => break,
                    TokenKind::Comma => {},
                    _ => return Err(SvError::syntax(t, "package import. Expecting , or ;"))
                }
            }
        }
        TokenKind::Str => {
            if t.value!="DPI-C" && t.value!="DPI"  {
                return Err(SvError::syntax(t, "import DPI. Expecting DPI-C, DPI or package identifier"))
            }
            n.attr.insert("dpi".to_owned(),t.value);
            ts.flush(1);
            t = ts.next_t(true)?;
            if t.kind==TokenKind::KwPure || t.kind==TokenKind::KwContext {
                ts.flush(1);
                n.attr.insert("property".to_owned(),t.value);
            } else {ts.rewind(1);}
            parse_func(ts, &mut n, false, true)?;
        }
        _ => return Err(SvError::syntax(t, "import. Expecting DPI-C or package identifier"))
    }
    node.child.push(n);
    Ok(())
}

/// Parse a param/localparam declaration
pub fn parse_param_decl(ts : &mut TokenStream, is_body: bool) -> Result<AstNode, SvError> {
    let mut t = ts.next_t(true)?;
    let mut node = AstNode::new(AstNodeKind::Param, t.pos);
    // Handle conditionnal macro. TODO: handle directly in the next_t macro
    while t.kind == TokenKind::CompDir {
        parse_macro(ts,&mut node)?;
        t = ts.next_t(true)?;
    }
    // optionnal keyword param/localparam
    match t.kind {
        TokenKind::KwParam | TokenKind::KwLParam | TokenKind::KwSpecParam  => {
            node.attr.insert("kind".to_owned(), format!("{:?}",t.kind ) );
            ts.flush_rd();
        },
        _ => {}
    }

    // Optional data type
    parse_data_type(ts,&mut node, 2)?;
    // let allow_type = node.attr.get("type")==Some(&"type".to_string());
    // Parse name
    let cntxt = if is_body {ExprCntxt::StmtList} else {ExprCntxt::ArgList};
    let mut ni = AstNode::new(AstNodeKind::Identifier, ts.get_pos());
    parse_var_decl_name(ts,&mut ni,cntxt.clone(),false,true)?;
    node.child.push(ni);
    // Parse extra name
    parse_opt_ident_list(ts,&mut node,cntxt)?; // TODO : allow type when node.attr.get("type")==Some(&"type".to_string())

    // println!("{}", node);
    // ts.display_status("param_decl");
    Ok(node)
}

/// Parse a port declaration
pub fn parse_port_decl(ts : &mut TokenStream, allow_void : bool, cntxt: ExprCntxt) -> Result<AstNode, SvError> {
    let mut type_found = false;
    let mut t = ts.next_t(true)?;
    let mut node = AstNode::new(AstNodeKind::Port, t.pos);
    // println!("[parse_port_decl] First token = {:?}", t);
    // Handle conditionnal macro. TODO: handle directly in the next_t macro
    while t.kind == TokenKind::CompDir {
        parse_macro(ts,&mut node)?;
        t = ts.next_t(true)?;
    }
    if t.kind == TokenKind::KwConst {
        node.attr.insert("const".to_owned(), "".to_owned());
        t = ts.next_t(true)?;
    }
    // direction/interface
    match t.kind {
        TokenKind::KwInput | TokenKind::KwOutput | TokenKind::KwInout | TokenKind::KwRef => {
            node.attr.insert("dir".to_owned(), t.value);
            ts.flush_rd();
        }
        // Interface / User-defined type
        TokenKind::Ident => {
            type_found = true;
            // Check if mod port is available
            let nt = ts.next_t(true)?;
            // println!("[parse_port_decl] Second token = {:?}", nt);
            match nt.kind {
                // Dot : t is the interface type and token is the modport (expect identifier)
                TokenKind::Dot => {
                    let nnt = ts.next_t(true)?;
                    if nnt.kind != TokenKind::Ident {
                        return Err(SvError::syntax(t, "port type"))
                    }
                    node.attr.insert("intf".to_owned(), t.value);
                    node.attr.insert("modport".to_owned(), nnt.value);
                    ts.flush_rd();
                }
                // Another ident : No modport, nt is the port name, rewind it
                TokenKind::Ident => {
                    node.attr.insert("type".to_owned(), t.value);
                    ts.flush(1);
                    ts.rewind(1);
                }
                // Token , ) = or [ -> t was the port name
                TokenKind::Comma | TokenKind::SquareLeft | TokenKind::ParenRight | TokenKind::OpEq => {
                    ts.rewind(2);
                }
                // Hash : t is a parameterized class
                TokenKind::Hash => {
                    node.attr.insert("type".to_owned(), t.value);
                    ts.flush(2); // Consume Ident and hash
                    parse_port_connection(ts,&mut node,true)?;
                    // println!("parse_port_decl: {}", node);
                }
                // Scope : t is the type
                TokenKind::Scope => {
                    type_found = false;
                    ts.rewind(2);
                }
                // Any other token is an error
                _ =>  return Err(SvError::syntax(nt, "port declaration. Expecting identifer, comma, ), #"))
            }
        }
        TokenKind::KwIntf => {
            type_found = true;
            // Check if mod port is available
            let nt = ts.next_t(true)?;
            match nt.kind {
                // Dot : t is the interface type and token is the modport (expect identifier)
                TokenKind::Dot => {
                    let nnt = ts.next_t(true)?;
                    if nnt.kind != TokenKind::Ident {
                        return Err(SvError::new(SvErrorKind::Syntax, t, format!("Unexpected {} ({:?}) for port type, expecting modport",nnt.value, nnt.kind)))
                    }
                    node.attr.insert("intf".to_owned(), t.value);
                    node.attr.insert("modport".to_owned(), nnt.value);
                    ts.flush_rd();
                }
                // Another ident : No modport, nt is the port name, rewind it
                TokenKind::Ident => ts.rewind(1),
                // Any other token is an error
                _ => return Err(SvError::syntax(t, "interface declaratione. Expecting identifier"))
            }
        }
        // Virtual interface
        TokenKind::KwVirtual => {
            node.child.push(parse_vintf_type(ts)?);
            type_found = true;
        }
        // No port case
        // TokenKind::ParenRight => {return Ok(node);}
        //
        _ => {}
    }
    if ! type_found {
        // Optional net type
        parse_net_type(ts,&mut node)?;
        // Optional data type
        parse_data_type(ts,&mut node, if allow_void {1} else {0})?;
    }
    let mut ni = AstNode::new(AstNodeKind::Identifier, ts.get_pos());
    parse_var_decl_name(ts,&mut ni,cntxt.clone(),false,false)?;
    node.child.push(ni);
    parse_opt_ident_list(ts,&mut node,cntxt)?;
    // println!("port_decl: {:?}", node);
    // ts.display_status("port_decl");
    Ok(node)
}

/// Parse a list of signal declaration
pub fn parse_signal_decl_list(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    // ts.display_status("parse_signal_decl_list: start");
    let mut node_sig = AstNode::new(AstNodeKind::Declaration, ts.get_pos());
    // Check for rand qualifier
    let t = ts.next_t(true)?;
    match t.kind {
        TokenKind::KwRand => {
            node_sig.attr.insert("rand".to_owned(), t.value);
            ts.flush(1);
        }
        _ => ts.rewind(1)
    }
    // Parse potential net type
    parse_net_type(ts,&mut node_sig)?;
    // Parse data type
    parse_data_type(ts,&mut node_sig, 1)?;
    // Optional delay for net only
    if node_sig.attr.contains_key("nettype") {
        parse_opt_delay(ts,&mut node_sig)?;
    }
    // Parse name
    let mut ni = AstNode::new(AstNodeKind::Identifier, ts.get_pos());
    parse_var_decl_name(ts,&mut ni,ExprCntxt::StmtList,false,false)?;
    node_sig.child.push(ni);
    // Continue parsing
    parse_opt_ident_list(ts,&mut node_sig,ExprCntxt::StmtList)?;

    node.child.push(node_sig);
    // ts.display_status("parse_signal_decl_list: done");
    ts.flush(1); // consume the semicolon (the parse_opt_ident_list will stop on semicolon)
    // println!("[parse_signal_decl_list] {}", node);
    // ts.rewind(0); // put back any token we have not used
    Ok(())
}

pub fn parse_var_decl_name(ts : &mut TokenStream, node : &mut AstNode, cntxt: ExprCntxt, need_value: bool, allow_type: bool) -> Result<(), SvError> {
    // Signal name
    let mut t = expect_t!(ts,"variable declaration",TokenKind::Ident);
    node.attr.insert("name".to_owned(), t.value);
    // Optional Unpacked dimension : [x][y:z]
    parse_opt_slice(ts,node,true,true)?;
    // Optional Default value i.e. "= expr"
    t = ts.next_t(true)?;
    if t.kind == TokenKind::OpEq {
        ts.flush(1);
        node.child.push(parse_expr(ts,cntxt,allow_type)?);
    } else if need_value {
        return Err(SvError::syntax(t, "variable declaration. Expecting init value"))
    }
    ts.rewind(0);
    Ok(())
}

/// Tentatively parse a net type
pub fn parse_net_type(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let mut t = ts.next_t(true)?;
    if t.kind == TokenKind::KwConst {
        node.attr.insert("const".to_owned(),"".to_owned());
        ts.flush(1);
        t = ts.next_t(true)?;
    }
    if t.kind==TokenKind::KwNetType || t.kind==TokenKind::KwSupply || t.kind==TokenKind::KwVar || t.kind==TokenKind::KwInterconnect {
        let allow_strength = t.kind!=TokenKind::KwVar;
        node.attr.insert("nettype".to_owned(),t.value);
        ts.flush(1);
        t = ts.next_t(true)?;
       // println!("[parse_net_type] next = {}", t);
        // Check for optional strength
        if t.kind==TokenKind::ParenLeft && allow_strength {
            ts.flush(1);
            parse_strength(ts,node)?;
            t = ts.next_t(true)?;
        }
        // Check for optional vector info
        if t.kind==TokenKind::KwVector{
            node.attr.insert("vector".to_owned(),t.value);
            ts.flush(1);
        }
    }
    ts.rewind(0);
    Ok(())
}

/// Parse strength/charge info
/// Suppose the open parenthesis was already consumed
pub fn parse_strength(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let mut t = ts.next_t(false)?;
    match t.kind {
        TokenKind::KwCharge => {
            node.attr.insert("charge".to_owned(),t.value);
        }
        TokenKind::KwDrive | TokenKind::KwSupply => {
            let mut s = t.value;
            t = ts.next_t(false)?;
            if t.kind!=TokenKind::Comma {
                return Err(SvError::syntax(t, "drive strength declaration, expecting ,"))
            }
            s.push(',');
            t = ts.next_t(false)?;
            if t.kind!=TokenKind::KwDrive && t.kind!=TokenKind::KwSupply {
                return Err(SvError::syntax(t, "drive strength declaration, expecting ,"))
            }
            s.push_str(&t.value);
            node.attr.insert("drive".to_owned(),s);
            // TODO: Check combination are actually valid
        }
        _ => return Err(SvError::syntax(t, "strength declaration, expecting drive or charge"))
    }
    // Done, expecting closing parenthesis
    t = ts.next_t(false)?;
    if t.kind!=TokenKind::ParenRight {
        return Err(SvError::syntax(t, "strength declaration, expecting )"))
    }
    Ok(())
}

/// Parse optional scope
#[allow(dead_code)]
pub fn parse_opt_scope(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let t_ident = ts.next_t(true)?;
    if t_ident.kind != TokenKind::Ident {
        ts.rewind(1);
        return Ok(());
    }
    let t_scope = ts.next_t(true)?;
    if t_scope.kind != TokenKind::Scope {
        ts.rewind(2);
        return Ok(());
    }
    let mut n = AstNode::new(AstNodeKind::Scope, t_ident.pos);
    n.attr.insert("name".to_owned(),t_ident.value);
    ts.flush(2);
    // TODO : check for another scope after
    node.child.push(n);
    // println!("[parse_opt_scope] -> {}", node);
    Ok(())
}

/// Parse a data type
pub fn parse_data_type(ts : &mut TokenStream, node : &mut AstNode, allowed_flag: u8) -> Result<(), SvError> {
    let mut has_signing = true;
    let mut has_width   = true;
    let mut get_next    = false;
    ts.rewind(0); // Ensure we start analyzing data from
    let mut t = ts.next_t(true)?;
    let mut s = t.value.clone();
    // println!("[parse_data_type] First Token = {}", t);
    let allow_type_ref = node.attr.contains_key("nettype");
    // First word of a data type
    match t.kind {
        // Integer vector type -> has signing and packed dimension
        TokenKind::KwReg         |
        TokenKind::TypeIntVector => {ts.flush(1); get_next = true; }
        TokenKind::TypeVoid   if (allowed_flag & 1)!=0 => {ts.flush(1); get_next = true; }
        TokenKind::KwType     if (allowed_flag & 2)!=0 => {ts.flush(1); get_next = true; }
        TokenKind::TypeGenvar if (allowed_flag & 4)!=0 => {ts.flush(1); get_next = true; }
        TokenKind::KwType if allow_type_ref => {
            ts.flush(1);
            expect_t!(ts,"type reference", TokenKind::ParenLeft);
            node.child.push(parse_expr(ts,ExprCntxt::Arg,true)?);
            expect_t!(ts,"type reference", TokenKind::ParenRight);
        }
        TokenKind::TypeIntAtom => {ts.flush(1); get_next = true; has_width=false}
        TokenKind::TypeReal    |
        TokenKind::TypeString  |
        TokenKind::TypeCHandle |
        TokenKind::TypeEvent   => {ts.flush(1); get_next = true; has_width=false; has_signing=false}
        TokenKind::KwEnum => {
            has_signing = false;
            has_width   = false;
            node.attr.insert("type".to_owned(), "enum".to_owned());
            node.child.push(parse_enum(ts,false)?);
        }
        // Ident -> check next word, could be a user type
        TokenKind::Macro |
        TokenKind::Ident => {
            has_signing = false;
            has_width   = true;
            let nt = ts.next_t(true)?;
            // println!("[parse_data_type] Ident {} followed by {}", t.value, nt);
            match nt.kind {
                // Scope operator => custom type
                TokenKind::Scope => {
                    parse_opt_scope(ts,node)?;
                    let nnt = expect_t!(ts,"data type",TokenKind::Ident);
                    s = nnt.value;
                    get_next=true;
                }
                // Another ident : t is the type and nt is the port/signal name
                // -> consume first character and put back the one we read
                TokenKind::Ident => {get_next=true; ts.flush(1); ts.rewind(1);}
                // An open square braket: nt is the packed array dimension
                TokenKind::SquareLeft => {get_next=true; ts.flush(1); ts.rewind(1); }
                // Hash
                TokenKind::Hash => {
                    ts.flush(2);
                    get_next=true;
                    parse_port_connection(ts,node,true)?;
                }
                // Comma/parenthesis/Equal/semicolon -> t was the port name
                TokenKind::Comma |
                TokenKind::SemiColon |
                TokenKind::OpEq |
                TokenKind::ParenRight => ts.rewind(1),
                _ => return Err(SvError::syntax(nt, "data type."))
            }
        }
        // Sign/Slice start (ignore handling now, will be done after)
        TokenKind::KwSigning |
        TokenKind::SquareLeft => {}
        //
        TokenKind::Hash => {
            ts.rewind(1);
            return Ok(())
        }
        // Any token not listed here is an error
        _ => return Err(SvError::syntax(t, "data type. Expecting type or identifier"))
    }
    // println!("[parse_data_type] ->get next = {} : {}", get_next, t );
    if get_next {
        node.attr.insert("type".to_owned(), s);
        t = ts.next_t(true)?;
    }
    // println!("[parse_data_type] -> has_sign={} : {}", has_signing, t );
    //
    if has_signing && t.kind == TokenKind::KwSigning {
        node.attr.insert("signing".to_owned(), t.value);
        ts.flush(1);
        t = ts.next_t(true)?;
    }
    // println!("[parse_data_type] -> has_width={} : {}", has_width, t );
    if has_width && t.kind == TokenKind::KwVector {
        node.attr.insert("vector".to_owned(),t.value);
        ts.flush(1);
        t = ts.next_t(true)?;
    }
    if has_width && t.kind == TokenKind::SquareLeft {
        ts.rewind(1);
        parse_opt_slice(ts,node,true, false)?;
    }
    //
    ts.rewind(1); // Put back last token we did not used
    // println!("[parse_data_type] -> {}", node);
    // ts.display_status("Post parse_data_type");
    Ok(())
}


pub fn parse_opt_slice(ts : &mut TokenStream, node: &mut AstNode, allow_range: bool, allow_type: bool) -> Result<(),SvError> {
    let mut t;
    // println!("[parse_opt_slice] range = {}, type {} -> {}", allow_range, allow_type, node);
    // ts.display_status("parse_opt_slice : start");
    loop {
        t = ts.next_t(true)?;
        if t.kind != TokenKind::SquareLeft {break;} else {ts.flush(1);}
        let mut n = AstNode::new(AstNodeKind::Slice, t.pos);
        t = ts.next_t(true)?;
        match t.kind {
            TokenKind::SquareRight => ts.flush(1),
            TokenKind::OpStar if allow_type => {
                // TODO: better align with specified type ...
                n.attr.insert("associative".to_owned(),t.value);
                ts.flush(1);
                expect_t!(ts,"size",TokenKind::SquareRight);
            }
            _ => {
                ts.rewind(1);
                n.child.push(parse_expr(ts,ExprCntxt::BracketMsb,allow_type)?);
                // Check if range
                if allow_range {
                    // ts.display_status("parse_opt_slice : checking range");
                    // The expression parser ends either on : or ]
                    t = ts.next_t(false)?;
                    if t.kind == TokenKind::Colon || t.kind == TokenKind::OpRange {
                        n.attr.insert("range".to_owned(),t.value);
                        n.child.push(parse_expr(ts,ExprCntxt::BracketLsb,false)?);
                        ts.flush(1); // Consume the ]
                    }
                } else {
                    expect_t!(ts,"size",TokenKind::SquareRight);
                }
            }
        }
        node.child.push(n);
    }
    ts.rewind(0);
    // ts.display_status("parse_opt_slice : done");
    Ok(())
}

pub fn parse_opt_signess(ts : &mut TokenStream, node: &mut AstNode) -> Result<(),SvError> {
    let t = ts.next_t(true)?;
    if t.kind == TokenKind::KwSigning {
        node.attr.insert("signing".to_owned(), t.value);
        ts.flush(1);
    } else {
        ts.rewind(1);
    }
    Ok(())
}

pub fn parse_opt_delay(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let t = ts.next_t(true)?;
    if t.kind == TokenKind::Hash {
        node.child.push(parse_delay(ts)?);
    } else {
        ts.rewind(1);
    }
    Ok(())
}

// Parse identifier with potential hierarchy and range selection
pub fn parse_ident_hier(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    // ts.display_status("parse_ident_hier: start");
    let mut node = AstNode::new(AstNodeKind::Identifier, ts.get_pos());
    parse_opt_scope(ts,&mut node)?;
    let mut t = expect_t!(ts,"identifier", TokenKind::Ident, TokenKind::KwThis);
    node.attr.insert("name".to_owned(),t.value);
    parse_opt_slice(ts,&mut node,true,false)?;
    t = ts.next_t(true)?;
    if t.kind == TokenKind::Dot {
        ts.flush(1);
        node.child.push(parse_ident_hier(ts)?);
    } else {
        ts.rewind(1);
    }
    // println!("[parse_ident_hier] {}", node);
    // ts.display_status("parse_ident_hier: done");
    Ok(node)
}

pub fn parse_ident_list(ts : &mut TokenStream, node: &mut AstNode) -> Result<(),SvError> {
    let mut expect_ident = true;
    loop {
        let t = ts.next_t(false)?;
        match t.kind {
            TokenKind::Ident if expect_ident => {
                let mut n = AstNode::new(AstNodeKind::Identifier, t.pos);
                n.attr.insert("name".to_owned(),t.value);
                parse_opt_slice(ts,&mut n,true, true)?; // TODO CHECKD IF TYPE IS ALLOWED
                parse_opt_init_value(ts,&mut n,ExprCntxt::StmtList)?;
                node.child.push(n);
                expect_ident = false;
            }
            TokenKind::Comma if !expect_ident => {
                expect_ident = true;
            }
            TokenKind::SemiColon if !expect_ident => break, // Semi colon indicate end of statement, stop the loop
            _ => return Err(SvError::syntax(t, "ident list. Expecting identifier/comma/semicolon"))
        }
    }
    Ok(())
}

// Parse an optionnal identifier list: the next item in the list could be a type and so some pre-parsing is required
pub fn parse_opt_ident_list(ts : &mut TokenStream, node: &mut AstNode,cntxt: ExprCntxt) -> Result<(),SvError> {
    loop {
        let mut t = ts.next_t(true)?;
        if t.kind != TokenKind::Comma {break;}
        let tid = ts.next_t(true)?;
        if tid.kind != TokenKind::Ident {break;}
        // For port list we need to check if this is a type or an identifier
        if cntxt!=ExprCntxt::StmtList {
            t = ts.next_t(true)?;
            match t.kind {
                // TODO
                // SquareLeft could be a packed dimension of a custom type or the unpacked dimension of a variable
                // For the moment suppose the unpacked dimension ...
                TokenKind::SquareLeft => {
                    ts.rewind(1);
                    // println!("[parse_opt_ident_list] Found range in port list -> not supported yet\n{:?}",node);
                    // break;
                }
                TokenKind::Comma      |
                TokenKind::OpEq       |
                TokenKind::ParenRight |
                TokenKind::SemiColon  => {ts.rewind(1);}
                _ => break,
            }
        }
        ts.flush(2);
        let mut n = AstNode::new(AstNodeKind::Identifier, tid.pos);
        n.attr.insert("name".to_owned(),tid.value);
        // Optional Unpacked dimension : [x][y:z]
        parse_opt_slice(ts,&mut n,true,true)?;
        // Optional init value
        parse_opt_init_value(ts,&mut n,cntxt.clone())?;
        node.child.push(n);
    }
    ts.rewind(0);
    // ts.display_status("[parse_opt_ident_list] done");
    Ok(())
}

pub fn parse_opt_init_value(ts : &mut TokenStream, node: &mut AstNode, cntxt: ExprCntxt) -> Result<(),SvError> {
    let t = ts.next_t(true)?;
    if t.kind != TokenKind::OpEq {
        ts.rewind(1);
    } else {
        ts.flush(1);
        node.child.push(parse_expr(ts,cntxt,false)?);
    }
    Ok(())
}

/// Parse an enum declaration
pub fn parse_enum(ts : &mut TokenStream, is_typedef: bool) -> Result<AstNode,SvError> {
    let mut node_e = AstNode::new(AstNodeKind::Enum, ts.get_pos());
    ts.flush(1); // Suppose enum is on the stack
    let mut t = ts.next_t(true)?;
    // Optionnal data type
    match t.kind {
        TokenKind::TypeIntAtom => {
            let mut s = t.value;
            // Check for optional signing info
            t = ts.next_t(true)?;
            if t.kind == TokenKind::KwSigning {
                s.push_str(&t.value);
                t = ts.next_t(true)?;
            }
            node_e.attr.insert("type".to_owned(),s);
        }
        TokenKind::TypeIntVector => {
            node_e.attr.insert("type".to_owned(),t.value);
            ts.flush(1);
            // Check for optional signing info
            t = ts.next_t(true)?;
            // ts.display_status("");
            if t.kind == TokenKind::KwSigning {
                ts.flush(1); 
                node_e.attr.insert("signing".to_owned(), t.value);
                t = ts.next_t(true)?;
            }
            // Check for optional dimension
            if t.kind == TokenKind::SquareLeft {
                ts.rewind(1);
                parse_opt_slice(ts,&mut node_e,true,false)?;
                t = ts.next_t(true)?;
            }
        }
        TokenKind::Ident => {
            node_e.attr.insert("type".to_owned(),t.value);
            t = ts.next_t(true)?;
        }
        TokenKind::CurlyLeft => {}
        _ => return Err(SvError::syntax(t, "enum"))
    }
    // Expect opening brace for definition or ; in case of forward declaration
    match t.kind {
        TokenKind::CurlyLeft => ts.flush_rd(), // Consume all character up to the opening curly brace
        TokenKind::SemiColon if is_typedef => {
            node_e.attr.insert("forward".to_owned(),"".to_owned());
            ts.rewind(2); // Put back the type name and semi-colon, will be consume by the parse_typedef
            return Ok(node_e);
        }
        _ => return Err(SvError::syntax(t, "enum. Expecting {"))
    }

    loop {
        // Capture enum identifier
        t = ts.next_t(false)?;
        let mut node_id = AstNode::new(AstNodeKind::EnumIdent, t.pos);
        node_id.attr.insert("name".to_owned(), t.value);
        // Optional range
        parse_opt_slice(ts,&mut node_e,true,false)?;
        // Optional value
        parse_opt_init_value(ts,&mut node_id, ExprCntxt::FieldList)?;
        node_e.child.push(node_id);
        // Expect , or }
        loop_args_break_cont!(ts,"enum declaration",CurlyRight);
    }
    Ok(node_e)
}

/// Parse a struct declaration
pub fn parse_struct(ts : &mut TokenStream) -> Result<AstNode,SvError> {
    let mut t = ts.next_t(false)?;
    let mut node;
    match t.kind {
        TokenKind::KwStruct => node = AstNode::new(AstNodeKind::Struct, t.pos),
        TokenKind::KwUnion => {
            node = AstNode::new(AstNodeKind::Union, t.pos);
            t = ts.next_t(true)?;
            if t.kind == TokenKind::KwTagged {
                node.attr.insert("tagged".to_owned(),"".to_owned());
                ts.flush(1);
            } else {
                ts.rewind(1);
            }
        },
        _ => return Err(SvError::syntax(t, "struct. Expecting struct or union"))
    }
    t = ts.next_t(true)?;
    // Optional packed keyword
    if t.kind==TokenKind::KwPacked {
        ts.flush_rd();
        node.attr.insert("packed".to_owned(),"".to_owned());
        t = ts.next_t(true)?;
        // Optional signing
        if t.kind==TokenKind::KwSigning {
            ts.flush_rd();
            node.attr.insert("signing".to_owned(), t.value);
            t = ts.next_t(true)?;
        }
    }
    match t.kind {
        TokenKind::CurlyLeft => {ts.flush(1);},
        // Forward definition : no field defined
        TokenKind::Ident => {
            ts.rewind(1);
            node.attr.insert("forward".to_owned(),"".to_owned());
            return Ok(node);
        },
        _ => return Err(SvError::syntax(t, "struct. Expecting {"))
    }
    // Loop on type declaration until closing curly brace
    loop {
        t = ts.next_t(true)?;
        match t.kind {
            TokenKind::KwRand         |
            TokenKind::KwReg         |
            TokenKind::Ident         |
            TokenKind::TypeVoid      |
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     => parse_signal_decl_list(ts,&mut node)?,
            // anonymous enum
            TokenKind::KwEnum => {
                let mut node_e = parse_enum(ts,false)?;
                parse_ident_list(ts,&mut node_e)?;
                node.child.push(node_e);
            }
            // anonymous struct/union
            TokenKind::KwStruct |
            TokenKind::KwUnion  => {
                let mut node_s = parse_struct(ts)?;
                parse_ident_list(ts,&mut node_s)?;
                node.child.push(node_s);
            }
            // End of struct declaration
            TokenKind::CurlyRight => break,
            _ => return Err(SvError::syntax(t, "struct. Expecting data type")),
        }
        //
    }
    // Loop stop on } so can safely flush everything
    ts.flush_rd();
    // Check for packed dimension in packed struct
    if node.attr.contains_key("packed") {
        parse_opt_slice(ts,&mut node,true,false)?;
    }
    // println!("{}", node);
    Ok(node)
}

/// Parse typedef
pub fn parse_typedef(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush(1); // Consume the typedef keyword
    let mut t = ts.next_t(true)?;
    let mut node_def = AstNode::new(AstNodeKind::Typedef, t.pos);
    match t.kind {
        TokenKind::KwEnum => node_def.child.push(parse_enum(ts,true)?),
        TokenKind::KwStruct |
        TokenKind::KwUnion  => node_def.child.push(parse_struct(ts)?),
        TokenKind::KwReg         |
        TokenKind::Ident         |
        TokenKind::TypeIntAtom   |
        TokenKind::TypeIntVector |
        TokenKind::TypeReal      |
        TokenKind::TypeString    |
        TokenKind::TypeCHandle   |
        TokenKind::TypeEvent     => {
            let mut node_type = AstNode::new(AstNodeKind::Type, t.pos);
            parse_data_type(ts,&mut node_type, 0)?;
            node_def.child.push(node_type);
        }
        TokenKind::KwClass => {
            ts.flush(1);
            let mut node_type = AstNode::new(AstNodeKind::Type, t.pos);
            node_type.attr.insert("type".to_owned(),"class".to_owned());
            node_type.attr.insert("name".to_owned(),t.value);
            node_def.child.push(node_type);
        }
        TokenKind::KwVirtual => node_def.child.push(parse_vintf_type(ts)?),
        _ => return Err(SvError::syntax(t, "typedef declaration, expecting type/enum/struct"))
    }
    // Parse type name
    t = expect_t!(ts,"typedef", TokenKind::Ident, TokenKind::Macro);
    node_def.attr.insert("name".to_owned(),t.value);
    // Optional unpacked dimension
    parse_opt_slice(ts,&mut node_def,true,true)?;
    // Expect semi-colon
    expect_t!(ts,"typedef", TokenKind::SemiColon);
    // println!("Typedef done : {}", node_def);
    node.child.push(node_def);
    Ok(())
}

/// Parse user-defined nettype
pub fn parse_nettype(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush(1); // Consume the nettype keyword
    let mut t = ts.next_t(true)?;
    let mut node_def = AstNode::new(AstNodeKind::Typedef, t.pos);
    node_def.attr.insert("nettype".to_owned(),"user-defined".to_owned());
    // type
    match t.kind {
        TokenKind::Ident         |
        TokenKind::TypeIntAtom   |
        TokenKind::TypeIntVector |
        TokenKind::TypeReal      => {
            let mut node_type = AstNode::new(AstNodeKind::Type, t.pos);
            parse_data_type(ts,&mut node_type, 0)?;
            node_def.child.push(node_type);
        }
        _ => return Err(SvError::syntax(t, "nettype declaration, expecting type identifier"))
    }
    // type name
    t = expect_t!(ts,"user-defined nettype", TokenKind::Ident);
    node_def.attr.insert("name".to_owned(),t.value);
    // Optional resolution function name
    t = ts.next_t(true)?;
    match t.kind {
        TokenKind::KwWith => {
            ts.flush(1);
            t = expect_t!(ts,"nettype resolution function", TokenKind::Ident);
            node_def.attr.insert("resolution".to_owned(),t.value);
        }
        _ => ts.rewind(1)
    }

    // Expect semi-colon
    expect_t!(ts,"typedef", TokenKind::SemiColon);
    // println!("Typedef done : {}", node_def);
    node.child.push(node_def);
    Ok(())
}


/// Parse port/parameter connection
/// Stream should start at open parenthesis and will be consumed until the closing parenthesis included
pub fn parse_port_connection(ts : &mut TokenStream, node: &mut AstNode, is_param: bool) -> Result<(), SvError> {
    // Allow simple list until a named connection is found
    let mut allow_list = true;
    // Allow dot star if we are not in a parameter port connection
    // Also prevent dot star once we found one
    let mut allow_dot_star = !is_param;
    let mut cnt = 0;
    let mut is_first = true;
    expect_t!(ts,"port connection",TokenKind::ParenLeft);
    loop {
        let t = ts.next_t(true)?;
        // println!("[parse_port_connection] Token {}", t);
        match t.kind {
            TokenKind::Dot => {
                allow_list = false;
                ts.flush(1); // Consume the dot
                let mut nt = expect_t!(ts,"port name",TokenKind::Ident);
                let mut node_p = AstNode::new( if is_param {AstNodeKind::Param} else {AstNodeKind::Port}, nt.pos);
                node_p.attr.insert("name".to_owned(), nt.value);
                nt = ts.next_t(true)?;
                match nt.kind {
                    TokenKind::ParenLeft => {
                        ts.flush(1); // Consume the (
                        node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                        node_p.child.push(parse_expr(ts,ExprCntxt::Arg,is_param)?);
                        ts.flush(1); // Consume right parenthesis
                        node.child.push(node_p);
                        cnt += 1;
                    }
                    // Implicit named
                    TokenKind::Comma |
                    TokenKind::ParenRight => {
                        node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                        node.child.push(node_p);
                        cnt += 1;
                        ts.rewind(0);
                    }
                    _ => return Err(SvError::syntax(nt, "port connection. Expecting open parenthesis"))
                }
            },
            TokenKind::DotStar if allow_dot_star => {
                ts.flush(1); // Consume the .*
                allow_dot_star = false;
                let mut node_p = AstNode::new( if is_param {AstNodeKind::Param} else {AstNodeKind::Port}, t.pos);
                node_p.attr.insert("name".to_owned(), t.value);
                node.child.push(node_p);
            },
            TokenKind::ParenRight if is_first => break,
            TokenKind::CompDir => parse_macro(ts,node)?,
            //
            _ => {
                // ordered connection
                if allow_list {
                    ts.rewind(0);
                    let mut node_p = AstNode::new( if is_param {AstNodeKind::Param} else {AstNodeKind::Port}, t.pos);
                    node_p.attr.insert("name".to_owned(), "".to_owned());
                    node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                    node_p.child.push(parse_expr(ts,ExprCntxt::ArgList,is_param)?);
                    node.child.push(node_p);
                    cnt += 1;
                } else {
                    return Err(SvError::syntax(t, "port connection"));
                }
            }
        }
        is_first = false;
        loop_args_break_cont!(ts,"port connection",ParenRight);
    }
    // println!("parse_port_connection: {}", node);
    ts.flush_rd();
    Ok(())
}


/// Parse
pub fn parse_has_begin(ts : &mut TokenStream, node: &mut AstNode) -> Result<bool, SvError> {
    let mut is_block = false;
    let t = ts.next_t(true)?;
    if t.kind == TokenKind::KwBegin {
        is_block = true;
        ts.flush(1);
        if node.kind == AstNodeKind::Statement {
            node.kind = AstNodeKind::Block;
        }
        parse_label(ts,node,"block".to_owned())?;
    } else {
        ts.rewind(1);
    }
    Ok(is_block)
}

/// Parse the optional label after a begin keyword, and update
pub fn parse_label(ts : &mut TokenStream, node: &mut AstNode, attr_name: String) -> Result<bool, SvError> {
    let mut t = ts.next_t(true)?;
    // println!("[parse_label] Token = : {}", t);
    // Check for named block
    if t.kind == TokenKind::Colon {
        ts.flush(1);
        t = ts.next_t(false)?;
        if t.kind!=TokenKind::Ident {
            return Err(SvError::syntax(t, "block name"))
        }
        node.attr.insert(attr_name, t.value);
        Ok(true)
    } else {
        ts.rewind(1);
        node.attr.insert(attr_name, "".to_owned());
        Ok(false)
    }
}

pub fn check_label(ts : &mut TokenStream, name: &str) -> Result<(), SvError> {
    match ts.next_t(true) {
        Ok(t) => {
            if t.kind == TokenKind::Colon {
                ts.flush_rd();
                let nt = ts.next_t(false)?;
                if nt.kind!=TokenKind::Ident && nt.kind!=TokenKind::KwNew {
                    Err(SvError::syntax(nt, "label"))
                } else if nt.value != name {
                    Err(SvError::syntax(nt, &format!("label name. Expecting {}",name)))
                } else {
                    Ok(())
                }
            } else {
                ts.rewind(0);
                Ok(())
            }
        }
        Err(e) => {
            match e.kind {
                SvErrorKind::Null | SvErrorKind::Eof => Ok(()),
                _ => Err(e)
            }
        }
    }
}

pub fn parse_delay (ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    let mut t = ts.next_t(false)?;
    let mut node = AstNode::new(AstNodeKind::Wait, t.pos);
    node.attr.insert("kind".to_owned(),t.value);
    t = ts.next_t(true)?;
    match t.kind {
        TokenKind::Integer | TokenKind::Real | TokenKind::Kw1step => {
            let mut nv = AstNode::new(AstNodeKind::Value, t.pos);
            nv.attr.insert("value".to_owned(), t.value);
            ts.flush(1); // consume number
            // Optional time unit
            t = ts.next_t(true)?;
            match t.value.as_ref() {
                "fs" |"ps" |"ns" |"us" |"ms" | "s" => {
                    nv.attr.insert("value".to_owned(), t.value);
                    ts.flush(1);
                }
                _ => ts.rewind(1)
            }
        }
        TokenKind::Ident => node.child.push(parse_ident_hier(ts)?),
        TokenKind::ParenLeft => {
            ts.flush(1); // Consume open parenthesis
            node.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
            ts.flush(1); // Consume right parenthesis
        }
        _ => return Err(SvError::syntax(t, "wait statement. Expecting integer/real"))
    }
    Ok(node)
}

/// Parse Macro/Directive
pub fn parse_macro(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut t = ts.next_t(false)?;
    let mut node_m = AstNode::new(AstNodeKind::Directive, t.pos);
    node_m.attr.insert("name".to_owned(),t.value.clone());
    // println!("[parse_macro] First token {:?}", t);
    match t.value.as_ref() {
        // Directive with no parameters
        "`else"                |
        "`endif"               |
        "`undefineall"         |
        "`resetall"            |
        "`celldefine"          |
        "`endcelldefine"       |
        "`nounconnected_drive" |
        "`end_keywords"        => {}
        // Directive with one parameter
        "`ifndef" | "`ifdef" | "`elsif" | "`undef" => {
            t = expect_t!(ts,"Directive",TokenKind::Ident);
            node_m.attr.insert("param".to_owned(), t.value);
            ts.flush_rd();
        }
        "`begin_keywords" => {
            t = expect_t!(ts,"type",TokenKind::Str);
            node_m.attr.insert("version".to_owned(),t.value);
        }
        // Expect pull0 or pull1
        "`unconnected_drive" => {
            t = expect_t!(ts,"type",TokenKind::KwDrive);
            if t.value != "pull0" && t.value != "pull1" {
                return Err(SvError::syntax(t, "unconnected drive, Expecting pull0/1 !"));
            }
            node_m.attr.insert("drive".to_owned(),t.value);
        }
        // Include directive : `include <file> , `include "file" or `include `mymacro
        "`include" => {
            t = ts.next_t(true)?;        
            match t.kind {
                TokenKind::Macro => {}, // TO BE REMOVED
                TokenKind::Str => {},
                TokenKind::OpLT => {
                    t = ts.next_t(true)?;
                    if t.kind!=TokenKind::Ident {
                        return Err(SvError::syntax(t, "include directive"));
                    }
                    expect_t!(ts,"include directive",TokenKind::OpGT);
                }
                _ => return Err(SvError::syntax(t, "include directive"))
            }
            let fname = t.value.clone();
            // TO BE REMOVED
            ts.add_inc(&fname);
            node_m.attr.insert("include".to_owned(),fname.clone());

            // In case of include check if it has any define to be added to the tokenstream
            if ts.project.ast_inc.contains_key(&fname) {
                for (k,v) in ts.project.ast_inc[&fname].defines.clone() {
                    // println!("[Include] {:?} -> {:?}", k,v);
                    // println!("[parse_macro] Include Define {} ", k);
                    ts.project.defines.insert(k,v);
                }
                // ts.project.ast_inc[&fname].defines.iter().for_each(|(k, v)| ts.project.defines.insert(k.to_string(),v));
            } else {
                ts.project.compile_inc(fname, t)?;
            }
            ts.flush_rd();
        }
        // Define directive : first token is the name, followed by optional argument and then the content is all token until EOL
        "`define" => {
            t = ts.next_t(true)?;
            if t.kind!=TokenKind::Ident {
                return Err(SvError::syntax(t, "ifdef directive"))
            }
            // Check if macro is already defined
            let name = format!("`{}", t.value);
            // TODO: temporary disable of macro redefined warning until compilation directive are properly handle
            // if ts.project.defines.contains_key(&name) {println!("[parse_macro] {} : line {} | Macro {} already defined", ts.source.get_filename(),t.pos,t.value);}
            let line_num = t.pos.line;
            ts.flush(1);
            // Try next token: if new line, it was an empty define: add it to the list, rewind and return
            // Also support End Of file
            match ts.next_t(true) {
                Ok(t) if t.pos.line != line_num => {
                    ts.project.defines.insert(name,None);
                    ts.rewind(0);
                    return Ok(())
                }
                Err(e) => return if e.kind==SvErrorKind::Null {Ok(())} else {Err(e)},
                // Err(e) => return if e.kind==SvErrorKind::Null || e.kind==SvErrorKind::Eof {Ok(())} else {Err(e)},
                Ok(t_) => t = t_,
            }
            let mut macro_def = MacroDef::new();
            if t.kind == TokenKind::ParenLeft {
                // way to differentiate body from port might need more work ...
                t = ts.next_t(true)?;
                if t.kind == TokenKind::Ident {
                    ts.rewind(0);
                    ts.flush(1);
                    loop {
                        t = ts.next_t(false)?;
                        match t.kind {
                            TokenKind::Ident => {
                                let port_name = t.value;
                                // Optional Default value i.e. "= expr"
                                t = ts.next_t(true)?;
                                if t.kind == TokenKind::OpEq {
                                    ts.flush(1);
                                    let def_val = ts.collect_until(true)?;
                                    macro_def.ports.push((port_name,def_val));
                                } else {
                                    ts.rewind(1);
                                    macro_def.ports.push((port_name,Vec::new()));
                                }
                                loop_args_break_cont!(ts,"macro arguments",ParenRight);
                            }
                            TokenKind::LineCont => {},
                            _ =>  return Err(SvError::syntax(t,"define. Expecting port name/expression")),
                        }
                    }
                }
            }
            ts.rewind(0);
            // if name == "`" {ts.display_status("Macro : post port");}
            macro_def.body = ts.collect_until(false)?;
            // if name == "`" {println!("[parse_macro] Define {} = {:#?}", name, macro_def);}
            // println!("[parse_macro] Define {} ", name);
            ts.project.defines.insert(name,Some(macro_def));
        }
        "`pragma" => {
            t = expect_t!(ts,"type",TokenKind::Ident);
            let line = t.pos.line;
            node_m.attr.insert("pragma_name".to_owned(), t.value);
            // Silently consume any token on current line
            loop {
                t = ts.next_t(true)?;
                if t.pos.line != line {break;}
            }
            ts.flush_keep(1); // Cleanup everything except last token
        }
        "`default_nettype" => {
            t = ts.next_t(true)?;
            if t.kind!=TokenKind::KwNetType && (t.kind!=TokenKind::Ident || t.value != "none")  {
                return Err(SvError::syntax(t,"default_nettype. Expecting net type (wire/tri/...) or none"));
            }
            node_m.attr.insert("nettype".to_owned(),t.value);
            ts.flush_rd();
        }
        "`timescale" => {
            node_m.attr.insert("unit".to_owned(),parse_time(ts)?);
            expect_t!(ts,"timescale",TokenKind::OpDiv);
            node_m.attr.insert("precision".to_owned(),parse_time(ts)?);
        }
        // Line : expect number string number
        "`line" => {
            t = expect_t!(ts,"type",TokenKind::Integer);
            node_m.attr.insert("line".to_owned(),t.value);
            t = expect_t!(ts,"type",TokenKind::Str);
            node_m.attr.insert("filename".to_owned(),t.value);
            t = expect_t!(ts,"type",TokenKind::Integer);
            node_m.attr.insert("level".to_owned(),t.value);
        }
        // User define macro
        _ => {
            println!("[parse_macro] {} : line {} | Use of unknown macro {} ", ts.source.get_filename(),t.pos,t.value);
        }

    }
    // println!("[parse_macro] Done -> {}", node_m);
    // ts.display_status("[parse_macro]");
    node.child.push(node_m);
    Ok(())
}


/// Parse a virtual interface member declaration
pub fn parse_vintf_type(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    // Mandatory virtual keyword
    let mut t = expect_t!(ts,"virtual interface",TokenKind::KwVirtual);
    let mut node_i = AstNode::new(AstNodeKind::VIntf, t.pos);
    // Optional interface keyword
    t = ts.next_t(false)?;
    if t.kind==TokenKind::KwIntf {
        t = ts.next_t(false)?;
        node_i.attr.insert("keyword".to_owned(),"explicit".to_owned());
    } else {
        node_i.attr.insert("keyword".to_owned(),"implicit".to_owned());
    }
    // Mandatory virtual interface type
    if t.kind!=TokenKind::Ident && t.kind!=TokenKind::Macro {
        return Err(SvError::syntax(t, "virtual interface. Expecting type identifier"));
    }
    node_i.attr.insert("type".to_owned(),t.value);
    // Optional parameter
    parse_opt_params!(ts,node_i);
    Ok(node_i)
}

/// Parse a virtual interface member declaration
pub fn parse_vintf(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let mut node_i = parse_vintf_type(ts)?;
    loop {
        let t = ts.next_t(false)?;
        match t.kind {
            TokenKind::Ident => {
                let mut n = AstNode::new(AstNodeKind::Identifier, t.pos);
                n.attr.insert("name".to_owned(),t.value);
                node_i.child.push(n);
                loop_args_break_cont!(ts,"virtual interface",SemiColon);
            }
            _ =>  return Err(SvError::syntax(t,"virtual interface. Expecting identifier")),
        }
    }
    node.child.push(node_i);
    // println!("[parse_vintf] {}", node);
    Ok(())
}

pub fn parse_struct_init(ts : &mut TokenStream, node : &mut AstNode) -> Result<(),SvError> {
    // Suppose the function is called when a token '{ has been read -> consume it
    // ts.display_status("parse_struct_init: called");
    let mut node_s = AstNode::new(AstNodeKind::StructInit, ts.get_pos());
    ts.flush(1);
    // Check the second next token: a colon indicate a structure
    ts.next_t(true)?;
    let mut t = ts.next_t(true)?;
    let is_struct = t.kind == TokenKind::Colon;
    ts.rewind(0);
    // ts.display_status("parse_struct_init: start");
    loop {
        let mut s = "".to_owned();
        if is_struct {
            t = ts.next_t(false)?;
            match t.kind {
                TokenKind::Ident | TokenKind::Integer | TokenKind::Str |
                TokenKind::TypeIntAtom | TokenKind::TypeIntVector |
                TokenKind::TypeReal | TokenKind::TypeString |
                TokenKind::KwDefault => {}
                _ => return Err(SvError::syntax(t, "struct init. Expecting identifier/integer/default"))
            }
            s = t.value;
            expect_t!(ts,"struct init",TokenKind::Colon);
        }
        let mut n = parse_expr(ts,ExprCntxt::FieldList,false)?;
        if is_struct {
            n.attr.insert("fieldName".to_owned(),s);
        }
        node_s.child.push(n);
        loop_args_break_cont!(ts,"struct init",CurlyRight);
    }
    node.child.push(node_s);
    Ok(())
}

/// Parse case statement
pub fn parse_case(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut t = ts.next_t(false)?;
    let mut node_c = AstNode::new(AstNodeKind::Case, t.pos);
    // println!("[parse_case] First Token {}", t);
    if t.kind==TokenKind::KwPriority || t.kind==TokenKind::KwUnique || t.kind==TokenKind::KwUnique0 {
        node_c.attr.insert("prio".to_owned(),t.value);
        t = ts.next_t(false)?;
    }
    if t.kind!=TokenKind::KwCase {
        return Err(SvError::syntax(t,"case statement. Expecting case"));
    }
    node_c.attr.insert("kind".to_owned(),t.value);
    // Parse case expression
    expect_t!(ts,"case",TokenKind::ParenLeft);
    node_c.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
    ts.flush(1); // Consume right parenthesis
    // test for Match/inside
    t = ts.next_t(true)?;
    match t.kind {
        TokenKind::KwMatch => {
            ts.flush_rd();
            node_c.attr.insert("matching".to_owned(),t.value);
        }
        TokenKind::KwInside if node_c.attr["kind"]=="case" => {
            ts.flush_rd();
            node_c.attr.insert("matching".to_owned(),t.value);
        }
        _ => ts.rewind(0)
    }
    let allow_range = node_c.attr.get("matching")==Some(&"inside".to_string());
    // Loop on all case entry until endcase
    loop {
        // println!("[parse_case] case item {}", t);
        t = ts.next_t(true)?;
        let mut node_i =  AstNode::new(AstNodeKind::CaseItem, t.pos);
        if t.kind == TokenKind::KwEndcase {break;}
        // Collect all case item entries
        loop {
            match t.kind {
                TokenKind::KwDefault => {
                    node_i.attr.insert("kind".to_owned(),"default".to_owned());
                    ts.flush(1); // Consume the keyword
                    let nt = ts.next_t(true)?;
                    // Colon optional after default
                    if nt.kind==TokenKind::Colon {ts.flush(1);} else {ts.rewind(1);}
                    break;
                }
                TokenKind::SquareLeft if allow_range => {
                    ts.rewind(1);
                    parse_opt_slice(ts,&mut node_i,true,false)?;
                    // TODO: check actual range and each pasrt is constant ?
                }
                TokenKind::KwTagged  => {
                    node_i.attr.insert("kind".to_owned(),"tagged".to_owned());
                    ts.flush(1); // Consume the keyword
                }
                TokenKind::CompDir  => {
                    parse_macro(ts,&mut node_i)?;
                    t = ts.next_t(true)?;
                    continue;
                }
                TokenKind::Macro    => parse_macro(ts,&mut node_i)?,
                _ => {
                    ts.rewind(1);
                    node_i.child.push(parse_expr(ts,ExprCntxt::CaseItemList,false)?)
                }
            }
            // ts.display_status("CaseItem");
            loop_args_break_cont!(ts,"case item entry",Colon);
            t = ts.next_t(true)?;
        }
        // Parse statement
        parse_class_stmt_or_block(ts,&mut node_i, false)?;
        // println!("[parse_case] case item node {}", node_i);
        node_c.child.push(node_i);
    }
    ts.flush_rd();
    // println!("[parse_case] {}", node_c);
    node.child.push(node_c);
    Ok(())
}

pub fn parse_expr(ts : &mut TokenStream, cntxt: ExprCntxt, allow_type: bool) -> Result<AstNode, SvError> {
    let mut node_e = AstNode::new(if cntxt==ExprCntxt::ExprGroup {AstNodeKind::ExprGroup} else {AstNodeKind::Expr}, ts.get_pos());
    let mut is_first = true;
    let mut allow_ident = true;
    let mut allow_op    = true;
    let mut prev_tkind = TokenKind::SemiColon;
    let mut t;
    // ts.display_status("parse_expr: start");
    loop {
        t = ts.next_t(true)?;
        // println!("[parse_expr] Token = {}, (cntxt={:?}, first={}, allow ident={} / op={} )", t,cntxt, is_first, allow_ident,allow_op);
        match t.kind {
            // Statement: end on semi-colon or comma: rewind it and end
            TokenKind::SemiColon if cntxt==ExprCntxt::StmtList || cntxt==ExprCntxt::Stmt => { ts.rewind(0); break; },
            TokenKind::SemiColon if cntxt!=ExprCntxt::StmtList && cntxt!=ExprCntxt::Stmt  => return Err(SvError::syntax(t, "expression")),
            // End on comma (if not inside curly braces)
            TokenKind::Comma => {
                if cntxt==ExprCntxt::Stmt || cntxt==ExprCntxt::Arg || cntxt==ExprCntxt::ExprGroup  {
                    return Err(SvError::syntax(t, "expression"));
                }
                ts.rewind(1); // reset to comma token to be used by caller
                break;
            }
            // Count parenthesis/braces to check if it is balanced
            TokenKind::TickCurly if is_first  => parse_struct_init(ts, &mut node_e)?,
            TokenKind::CurlyLeft => {
                ts.flush_rd();
                allow_op = true;
                // println!("[parse_expr] CurlyLeft, (cntxt={:?}, first={}, allow ident={} / op={} ) \n Last = {:?}",cntxt, is_first, allow_ident,allow_op,node_e.child.last());
                if  cntxt == ExprCntxt::FieldList {
                    let nb_child = node_e.child.len();
                    if let Some(c) = node_e.child.last_mut() {
                        match c.kind {
                            AstNodeKind::Value      |
                            AstNodeKind::Identifier |
                            AstNodeKind::ExprGroup  => {
                                let mut nc = AstNode::new(AstNodeKind::Replication, t.pos);
                                loop {
                                    nc.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                                    loop_args_break_cont!(ts,"replication",CurlyRight);
                                }
                                node_e.child.push(nc);
                                allow_ident = false;
                                continue;
                            }
                            AstNodeKind::Concat |
                            AstNodeKind::SystemTask if nb_child == 1 => {
                                c.kind = AstNodeKind::Replication;
                                allow_ident = true;
                                // println!("[parse_expr] CurlyLeft, (cntxt={:?}, first={}, allow ident={} / op={} ) \n Last = {:?}",cntxt, is_first, allow_ident,allow_op,c);
                            }
                            _ => {}
                        }
                    }
                }
                if allow_ident {
                    // Concatenation operator
                    let mut nc = AstNode::new(AstNodeKind::Concat, t.pos);
                    loop {
                        nc.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                        loop_args_break_cont!(ts,"concatenation",CurlyRight);
                    }
                    allow_ident = false;
                    node_e.child.push(nc);
                } else {
                    return Err(SvError::syntax(t, "expression"));
                }
                // println!("{}", node_e);
            }
            TokenKind::CurlyRight  => {
                if cntxt==ExprCntxt::FieldList {
                    ts.rewind(1); // reset to } token to be used by caller
                    break;
                }
                return Err(SvError::syntax(t, "expression"));
            }
            //
            TokenKind::SquareLeft  => {
                ts.rewind(1);
                parse_opt_slice(ts,&mut node_e,true,allow_type)?;
                allow_ident = false;
                allow_op    = true;
            }
            TokenKind::SquareRight => {
                if cntxt == ExprCntxt::BracketMsb || cntxt == ExprCntxt::BracketLsb {
                    ts.rewind(1);
                    break;
                }
                return Err(SvError::syntax(t, "expression"));
            }
            TokenKind::Dollar if cntxt == ExprCntxt::BracketMsb => {
                node_e.attr.insert("value".to_owned(),"$".to_owned());
                ts.flush(1);
            }
            TokenKind::OpRange if cntxt == ExprCntxt::BracketMsb => {ts.rewind(1);break;},
            TokenKind::Colon if cntxt == ExprCntxt::BracketMsb   => {ts.rewind(1);break;},
            TokenKind::Colon if cntxt == ExprCntxt::Question     => {ts.rewind(1);break;},
            TokenKind::Colon if cntxt == ExprCntxt::CaseItemList => {ts.rewind(1);break;},
            TokenKind::ParenLeft if allow_ident => {
                ts.flush(1); // Consume left parenthesis
                node_e.child.push(parse_expr(ts,ExprCntxt::ExprGroup,false)?);
                ts.flush(1); // Consume right parenthesis
                allow_ident = false;
                allow_op    = true;
            },
            TokenKind::ParenRight => {
                match cntxt {
                    ExprCntxt::ArgList | ExprCntxt::Arg | ExprCntxt::ExprGroup | ExprCntxt::Sensitivity => {
                        ts.rewind(1);
                        break;
                    },
                    _ => return Err(SvError::syntax(t, "expression"))
                }
            }
            TokenKind::KwIff if cntxt==ExprCntxt::Sensitivity => {ts.rewind(1);break;},
            TokenKind::KwOr if cntxt==ExprCntxt::Sensitivity => {ts.rewind(1);break;},
            TokenKind::Casting => {
                ts.flush(1); // Consume Casting operator
                expect_t!(ts,"casting expression",TokenKind::ParenLeft);
                let mut nc = parse_expr(ts,ExprCntxt::Arg,false)?;
                nc.attr.insert("casting".to_owned(),t.value);
                ts.flush(1); // Consume right parenthesis
                node_e.child.push(nc);
                allow_ident = false;
                allow_op    = true;
            }
            //
            TokenKind::KwNew if is_first => {
                node_e.kind = AstNodeKind::New;
                // Allow array
                ts.flush(1);
                // println!("[parse_expr] new followed by {}", t);
                parse_opt_slice(ts,&mut node_e,false,false)?;
                t = ts.next_t(true)?;
                if t.kind == TokenKind::ParenLeft {
                    parse_func_call(ts, &mut node_e, false)?;
                    t = ts.next_t(true)?;
                }
                // Check that the statement will finish on next token and rewind
                match t.kind {
                    TokenKind::SemiColon  if cntxt==ExprCntxt::Stmt || cntxt==ExprCntxt::StmtList => {},
                    TokenKind::ParenRight if cntxt==ExprCntxt::Arg  || cntxt==ExprCntxt::ArgList  => {},
                    TokenKind::Comma      if cntxt==ExprCntxt::StmtList || cntxt==ExprCntxt::ArgList => {},
                    _ => return Err(SvError::syntax(t, "new statement."))
                }
                ts.rewind(0);
                allow_ident = false;
                allow_op    = false;
            }
            //
            TokenKind::KwSuper |
            TokenKind::KwThis |
            TokenKind::Ident => {
                if ! allow_ident {
                    match t.value.as_ref() {
                        "fs" |"ps" |"ns" |"us" |"ms" | "s" => {
                            match prev_tkind {
                                TokenKind::Integer | TokenKind::Real => {
                                    node_e.attr.insert("value".to_owned(), t.value);
                                    ts.flush(1);
                                }
                                _ => return Err(SvError::syntax(t, "expression. Unexpected identifier"))
                            };
                        }
                        _ => return Err(SvError::syntax(t, "expression."))
                    }
                } else {
                    node_e.child.push(parse_member_or_call(ts,true)?);
                }
                allow_ident = false;
                allow_op    = true;
            },
            TokenKind::Macro if allow_ident => {
                parse_macro(ts,&mut node_e)?;
                allow_ident = false;
                allow_op    = true;
            }
            TokenKind::Str     |
            TokenKind::Integer |
            TokenKind::KwNull  |
            TokenKind::Real     if allow_ident => {
                let mut nc = AstNode::new(AstNodeKind::Value, t.pos);
                nc.attr.insert("value".to_owned(), t.value);
                node_e.child.push(nc);
                allow_ident = false;
                allow_op    = true;
                ts.flush(1);
            }
            TokenKind::Integer if prev_tkind==TokenKind::Integer => {
                if t.value.starts_with('\'') {
                    node_e.child.last_mut().unwrap().attr.insert("value".to_owned(), t.value);
                    // node_e.child.last().unwrap().attr.insert("value".to_owned(), format!("{}{}", node_e.attr["value"],t.value));
                    ts.flush(1);
                } else {
                    return Err(SvError::syntax(t, "expression"))
                }
            }
            // Operator with one operand
            // TokenKind::OpBang => {
            TokenKind::OpBang if allow_ident => {
                allow_ident = true;
                allow_op    = false;
                ts.flush(1);
                let mut nc = AstNode::new(AstNodeKind::Operation, t.pos);
                nc.attr.insert("kind".to_owned(),t.value.clone());
                node_e.child.push(nc);
                // println!("{}", node_e);
            }
            // Operator with one or two operand
            TokenKind::OpTilde    |
            TokenKind::OpPlus     |
            TokenKind::OpMinus    |
            TokenKind::OpAnd      |
            TokenKind::OpNand     |
            TokenKind::OpOr       |
            TokenKind::OpNor      |
            TokenKind::OpXor      |
            TokenKind::OpXnor     if allow_op || allow_ident => {
                allow_ident = true;
                allow_op    = false;
                ts.flush(1);
                let mut nc = AstNode::new(AstNodeKind::Operation, t.pos);
                nc.attr.insert("kind".to_owned(),t.value.clone());
                node_e.child.push(nc);
                // println!("{}", node_e);
            },
            // Streaming operator
            TokenKind::OpSL |
            TokenKind::OpSR if is_first && cntxt==ExprCntxt::FieldList=> {
                allow_ident = false;
                allow_op    = false;
                ts.flush(1);
                let mut nc = AstNode::new(AstNodeKind::Operation, t.pos);
                nc.attr.insert("kind".to_owned(),t.value.clone());
                // Check optional slice size : type/identifier/constant expression
                t = ts.next_t(true)?;
                match t.kind {
                    TokenKind::TypeIntVector |
                    TokenKind::TypeIntAtom   => {
                        let mut ncc = AstNode::new(AstNodeKind::Type, t.pos);
                        ncc.attr.insert("type".to_owned(), t.value.clone());
                        ts.flush(1);
                        parse_opt_signess(ts,&mut ncc)?;
                        nc.child.push(ncc);
                    }
                    TokenKind::Ident => nc.child.push(parse_ident_hier(ts)?),
                    TokenKind::Integer => {
                        let mut ncc = AstNode::new(AstNodeKind::Value, t.pos);
                        ncc.attr.insert("value".to_owned(), t.value);
                        nc.child.push(ncc);
                    }
                    _ => ts.rewind(1)
                }
                // Concatenation expression
                expect_t!(ts,"streaming expression",TokenKind::CurlyLeft);
                let mut ncc = AstNode::new(AstNodeKind::Concat, t.pos);
                loop {
                    // TODO: support "with" after each expression ...
                    ncc.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                    loop_args_break_cont!(ts,"concatenation",CurlyRight);
                }
                nc.child.push(ncc);
                node_e.child.push(nc);
            }
            // Operator with two operand
            TokenKind::OpStar     |
            TokenKind::OpDiv      |
            TokenKind::OpSL       |
            TokenKind::OpSR       |
            TokenKind::OpSShift   |
            TokenKind::OpPow      |
            TokenKind::OpMod      |
            TokenKind::OpLogicAnd |
            TokenKind::OpLogicOr  |
            TokenKind::OpLT       |
            TokenKind::OpLTE      |
            TokenKind::OpGT       |
            TokenKind::OpGTE      |
            TokenKind::OpEq2      |
            TokenKind::OpEq3      |
            TokenKind::OpEq2Que   |
            TokenKind::OpDiff     |
            TokenKind::OpDiff2    |
            TokenKind::OpDiffQue  if allow_op && !is_first => {
                allow_ident = true;
                allow_op    = false;
                ts.flush(1);
                let mut nc = AstNode::new(AstNodeKind::Operation, t.pos);
                nc.attr.insert("kind".to_owned(),t.value.clone());
                node_e.child.push(nc);
            },
            TokenKind::KwInside if !is_first => {
                let mut nc = AstNode::new(AstNodeKind::Constraint, t.pos);
                nc.attr.insert("kind".to_owned(),t.value.clone());
                ts.flush(1);
                expect_t!(ts,"inside expression",TokenKind::CurlyLeft);
                loop {
                    nc.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                    loop_args_break_cont!(ts,"inside expression",CurlyRight);
                }
                node_e.child.push(nc);
                // println!("{}", node_e);
                allow_ident = false;
                allow_op    = true;
            }
            TokenKind::OpIncrDecr => {
                let mut nc = AstNode::new(AstNodeKind::Operation, t.pos);
                nc.attr.insert("kind".to_owned(),t.value.clone());
                if is_first || !allow_op {
                    nc.attr.insert("op".to_owned(),"pre".to_owned());
                    allow_ident = true;
                    allow_op    = false;
                }
                else if allow_op {
                    nc.attr.insert("op".to_owned(),"post".to_owned());
                    allow_ident = false;
                    allow_op    = true;
                } else {
                    return Err(SvError::syntax(t, "with constraint. Expecting ( or {"));
                }
                node_e.child.push(nc);
                ts.flush(1);
            }
            // Composed assignement are allowed in expression when surrounded by parenthesis
            TokenKind::OpCompAss if cntxt==ExprCntxt::ExprGroup && !is_first && allow_op => {
                let mut nc = AstNode::new(AstNodeKind::Operation, t.pos);
                nc.attr.insert("kind".to_owned(),t.value.clone());
                node_e.child.push(nc);
                allow_ident = true;
                allow_op    = false;
                ts.flush(1);
            }
            //
            TokenKind::Que if !is_first => {
                let node_cond = node_e;
                node_e = AstNode::new(AstNodeKind::Branch, t.pos);
                node_e.attr.insert("kind".to_owned(),"?".to_owned());
                node_e.child.push(node_cond);
                ts.flush(1);
                // Parse first expression
                node_e.child.push(parse_expr(ts,ExprCntxt::Question,false)?);
                ts.flush(1); // Consume the :
                // Parse second expression
                node_e.child.push(parse_expr(ts,cntxt,false)?);
                break;
            }
            //
            TokenKind::SystemTask if allow_ident => {
                node_e.child.push(parse_system_task(ts)?);
                allow_ident = false;
                allow_op    = true;
            }
            //
            TokenKind::TypeIntVector if allow_type => {
                node_e.kind = AstNodeKind::Type;
                node_e.attr.insert("type".to_owned(), t.value.clone());
                ts.flush(1);
                parse_opt_signess(ts,&mut node_e)?;
                parse_opt_slice(ts,&mut node_e,true,false)?;
                break; // next character should be , or ) : no need to consume it, will be checked by caller
            }
            // Unbounded
            TokenKind::Dollar if allow_type => {
                node_e.kind = AstNodeKind::Value;
                node_e.attr.insert("value".to_owned(), t.value.clone());
                ts.flush(1);
                break; // next character should be , or ) : no need to consume it, will be checked by caller
            }
            TokenKind::TypeIntAtom |
            TokenKind::TypeReal    |
            TokenKind::TypeString  |
            TokenKind::TypeCHandle if allow_type => {
                node_e.kind = AstNodeKind::Type;
                node_e.attr.insert("type".to_owned(), t.value.clone());
                ts.flush(1);
                if t.kind == TokenKind::TypeIntAtom {
                    parse_opt_signess(ts,&mut node_e)?;
                }
                break; // next character should be , or ) : no need to consume it, will be checked by caller
            }
            TokenKind::KwVirtual if allow_type => {
                node_e = parse_vintf_type(ts)?;
                break;
            }

            _ => return Err(SvError::syntax(t, "expression"))
        }
        is_first = false;
        prev_tkind = t.kind;
    }
    // println!("[Expr] {}", node_e);
    // ts.display_status("parse_expr done");
    // Remove top layer Expr if only one child
    if node_e.child.len()==1 && (node_e.kind==AstNodeKind::ExprGroup || node_e.kind==AstNodeKind::Expr) {
        Ok(node_e.child.into_iter().nth(0).unwrap())
    } else {
        Ok(node_e)
    }
}

pub fn parse_member_or_call(ts : &mut TokenStream, is_first: bool) -> Result<AstNode, SvError> {
    // ts.display_status("parse_member_or_call: start");
    ts.rewind(0);
    let t0 = ts.next_t(false)?;
    let mut n = AstNode::new(AstNodeKind::Identifier, t0.pos);
    if t0.kind!=TokenKind::Ident && t0.kind!=TokenKind::KwNew {
        if (t0.kind != TokenKind::KwThis && t0.kind != TokenKind::KwSuper) || !is_first {
            return Err(SvError::syntax(t0, "member. Expecting identifier"));
        }
    }
    // ts.display_status("parse_member_or_call");
    let mut t = ts.next_t(true)?;
    // println!("[parse_member_or_call] next token = {}", t);
    // Could be a static method call from parameterized class
    if t.kind == TokenKind::Hash && is_first {
        ts.flush(1);

        let mut ns = AstNode::new(AstNodeKind::Scope, t0.pos);

        parse_func_call(ts, &mut ns, true)?;

        t = ts.next_t(true)?;
        // println!("[parse_member_or_call] static method call from parameterized class : next token = {}", t);
        match t.kind {
            TokenKind::Scope => {
                ns.attr.insert("name".to_owned(),t0.value);
                ts.flush(1);
                parse_opt_scope(ts,&mut ns)?;
                n.child.push(ns);
                t = expect_t!(ts,"type",TokenKind::Ident);
                n.attr.insert("name".to_owned(),t.value);
                t = ts.next_t(true)?;
            }
            TokenKind::Comma | TokenKind::ParenRight => {
                n = ns;
                n.kind = AstNodeKind::Type;
                n.attr.insert("name".to_owned(),t0.value);
                ts.rewind(1);
                return Ok(n);
            }
            // TODO : need to be reviewed, the scope handling seems incorrect ...
            TokenKind::Ident => {
                n = ns; // Might need to update some attr of ns ? Kind ?
                n.kind = AstNodeKind::Declaration;
                n.attr.insert("name".to_owned(),t0.value);
                ts.rewind(1);
                parse_var_decl_name(ts, &mut n,ExprCntxt::StmtList,false,false)?;
                loop {
                    t = ts.next_t(true)?;
                    // println!("[parse_member_or_call] Next token : {}", t);
                    match t.kind {
                        TokenKind::Comma => ts.flush(1), // Comma indicate a list -> continue
                        TokenKind::SemiColon => break, // Semi colon indicate end of statement, stop the loop
                        _ => return Err(SvError::syntax(t, "signal declaration, expecting , or ;"))
                    }
                    let mut node_m = AstNode::new(AstNodeKind::Declaration, ts.get_pos());
                    parse_var_decl_name(ts, &mut node_m,ExprCntxt::StmtList,false,false)?;
                    n.child.push(node_m);
                }
                ts.rewind(0);
                // ts.display_status("parse_member_or_call: Member declaration");
                return Ok(n);
            }
            _ => return Err(SvError::syntax(t, "statement. Expecting scope/type"))
        }

    }
    else if t.kind == TokenKind::Scope {
        ts.flush(1);
        let mut ns = AstNode::new(AstNodeKind::Scope, t0.pos);
        ns.attr.insert("name".to_owned(),t0.value);
        parse_opt_scope(ts,&mut ns)?;
        n.child.push(ns);
        t = ts.next_t(false)?;
        match t.kind {
            TokenKind::Ident => {
                n.attr.insert("name".to_owned(),t.value);
                t = ts.next_t(true)?;
            }
            TokenKind::Casting => {
                n.attr.insert("type".to_owned(),t.value);
                n.kind = AstNodeKind::Type;
                n.attr.insert("kind".to_owned(),"casting".to_owned());
                // ts.flush(1); // Consume Casting operator
                expect_t!(ts,"casting expression",TokenKind::ParenLeft);
                n.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                ts.flush(1); // Consume right parenthesis
                return Ok(n);
            }
            _ => return Err(SvError::syntax(t, "Expression. Expecting identifier"))
        }

    } else {
        n.attr.insert("name".to_owned(),t0.value);
    }
    // Check for function call
    if t.kind == TokenKind::ParenLeft {
        n.kind = AstNodeKind::MethodCall;
        ts.rewind(0);
        parse_func_call(ts, &mut n, false)?;
        t = ts.next_t(true)?;
    }
    // ts.display_status("parse_member_or_call: post parse_func_call");
    // Check for array selection
    if t.kind == TokenKind::SquareLeft {
        ts.rewind(1);
        parse_opt_slice(ts,&mut n,true,false)?;
        t = ts.next_t(true)?;
    }
    // Check for members
    if t.kind == TokenKind::Dot {
        ts.flush(1);
        n.child.push(parse_member_or_call(ts, false)?);
        t = ts.next_t(true)?;
    }
    // Check for constraint
    if t.kind == TokenKind::KwWith {
        t = ts.next_t(true)?;
        match t.kind {
            TokenKind::CurlyLeft => {
                ts.rewind(0);
                parse_constraint(ts,&mut n)?;
            }
            TokenKind::ParenLeft => {
                ts.flush(2);
                let mut nc = AstNode::new(AstNodeKind::Constraint, t.pos);
                nc.attr.insert("kind".to_owned(),"with".to_owned());
                nc.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                n.child.push(nc);
                ts.flush(1); // Consume closing parenthesis
            }
            _ => return Err(SvError::syntax(t, "with constraint. Expecting ( or {"))
        }
        // rpt_t!(MsgID::InfoStatus,&t,&format!("{}", nc));
    }

    ts.rewind(1);
    // ts.display_status("parse_member_or_call: end");
    // println!("[parse_member_or_call] {}", n);
    Ok(n)
}

pub fn parse_system_task(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    let mut t = expect_t!(ts,"system task",TokenKind::SystemTask);
    let mut n = AstNode::new(AstNodeKind::SystemTask, t.pos);
    let mut name = t.value.clone();
    let mut allow_type = false;
    // Handle special cases
    match t.value.as_ref() {
        "$test" | "$value" => {
            t = expect_t!(ts,"system task $plusargs",TokenKind::SystemTask);
            if t.value != "$plusargs" {
                return Err(SvError::syntax(t, "system task. Expecting $plusargs"))
            }
            name.push_str(&t.value);
        }
        "$async" => {
            t = expect_t!(ts,"system task $async",TokenKind::SystemTask);
            if t.value != "$and" && t.value != "$nand" && t.value != "$or" && t.value != "$nor" {
                return Err(SvError::syntax(t, "system task. Expecting $and/$nand/$or/$nor"))
            }
            name.push_str(&t.value);
            t = expect_t!(ts,"system task $async",TokenKind::SystemTask);
            if t.value != "$array" && t.value != "$plane" {
                return Err(SvError::syntax(t, "system task $async. Expecting $array/$plane"))
            }
            name.push_str(&t.value);
        }
        "$bits" => allow_type = true,
        _ => {},
    }

    t = ts.next_t(true)?;
    if t.kind==TokenKind::ParenLeft {
        ts.flush(1);
        loop {
            n.child.push(parse_expr(ts,ExprCntxt::ArgList,allow_type)?);
            loop_args_break_cont!(ts,"system task",ParenRight);
        }
    } else {
        ts.rewind(1);
    }
    // TODO: Check number/type of arguments depending on the macro name

    n.attr.insert("name".to_owned(),name);
    Ok(n)
}


#[allow(unused_assignments)]
pub fn parse_func_call(ts : &mut TokenStream, node: &mut AstNode, is_param: bool) -> Result<(), SvError> {
    // Allow simple list until a named connection is found
    let mut allow_list = true;
    let mut cnt = 0;
    ts.rewind(0);
    let mut t = expect_t!(ts,"function call. Expecting open parenthesis", TokenKind::ParenLeft);
    let mut nps = AstNode::new(AstNodeKind::Ports, t.pos);
    loop {
        t = ts.next_t(true)?;
        // println!("[parse_func_call] Token {}", t);
        match t.kind {
            TokenKind::Dot => {
                allow_list = false;
                ts.flush_rd(); // Consume the dot
                let mut nt = expect_t!(ts,"function argument name",TokenKind::Ident);
                let mut node_p = AstNode::new(AstNodeKind::Port, nt.pos);
                node_p.attr.insert("name".to_owned(), nt.value);
                nt = ts.next_t(true)?;
                match nt.kind {
                    TokenKind::ParenLeft => {
                        ts.flush(1); // Consume the open parenthesis
                        node_p.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                        ts.flush(1); // clear up last token (close parenthesis)
                        node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                        nps.child.push(node_p);
                        cnt += 1;
                    }
                    // Implicit named
                    TokenKind::Comma |
                    TokenKind::ParenRight => {
                        node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                        nps.child.push(node_p);
                        cnt += 1;
                        ts.rewind(0);
                    }
                    _ => return Err(SvError::syntax(t,"function arg. Expecting open parenthesis"))
                }
            },
            // Allow type for param
            TokenKind::TypeIntVector |
            TokenKind::TypeIntAtom if is_param => {
                ts.flush(1);
                let mut node_p = AstNode::new(AstNodeKind::Param, t.pos);
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("type".to_owned(), t.value);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                t = ts.next_t(true)?;
                if t.kind == TokenKind::KwSigning {
                    node_p.attr.insert("signing".to_owned(), t.value);
                    ts.flush(1);
                } else {
                    ts.rewind(1);
                }
                parse_opt_slice(ts,&mut node_p,true,true)?;
                nps.child.push(node_p);
            }
            TokenKind::TypeReal    |
            TokenKind::TypeString  |
            TokenKind::TypeCHandle if is_param => {
                ts.flush(1);
                let mut node_p = AstNode::new(AstNodeKind::Param, t.pos);
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("type".to_owned(), t.value);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                nps.child.push(node_p);
            }
            TokenKind::KwVirtual if is_param => {
                let mut s = t.value;
                ts.flush(1);
                t = ts.next_t(false)?;
                if t.kind == TokenKind::KwIntf {
                    s.push_str(" interface");
                    t = ts.next_t(false)?;
                }
                if t.kind!=TokenKind::Ident && t.kind!=TokenKind::Macro {
                    return Err(SvError::syntax(t,"function arg. Expecting port name"));
                }
                s.push_str(&t.value);
                let mut node_p = AstNode::new(AstNodeKind::Param, t.pos);
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("value".to_owned(), s);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                parse_opt_params!(ts,node_p);
                // println!("Virtual interface param : {}", node_p);
                nps.child.push(node_p);
            }
            TokenKind::KwNull => {
                ts.flush(1);
                let mut node_p = AstNode::new(AstNodeKind::Port, t.pos);
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("value".to_owned(), t.value);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                nps.child.push(node_p);
            }
            // Handle case of empty arguments
            TokenKind::ParenRight => {
                ts.flush(1);
                break;
            }
            //
            _ => {
                // ordered connection
                if allow_list {
                    ts.rewind(0);
                    let mut node_p = AstNode::new(AstNodeKind::Port, t.pos);
                    node_p.child.push(parse_expr(ts,ExprCntxt::ArgList,false)?);
                    node_p.attr.insert("name".to_owned(), "".to_owned());
                    node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                    nps.child.push(node_p);
                    cnt += 1;
                    ts.rewind(0); //
                } else {
                    return Err(SvError::syntax(t, "port connection"));
                }
            }
        }
        loop_args_break_cont!(ts,"argument list",ParenRight);
    }
    if !nps.child.is_empty() {
        node.child.push(nps);
    }
    // ts.display_status("parse_func_call");
    Ok(())
}

// Parse a time value
pub fn parse_time(ts : &mut TokenStream) -> Result<String,SvError> {
    let t1 = ts.next_t(false)?;
    if t1.kind!=TokenKind::Integer && t1.kind!=TokenKind::Real {
        return Err(SvError::syntax(t1,"timescale. Expecting time value (integer or floating)"));
    }
    let t2 = ts.next_t(false)?;
    if t2.kind!=TokenKind::Ident {
        return Err(SvError::syntax(t2,"timescale. Expecting time unit"));
    }
    match t2.value.as_ref() {
        "fs" |"ps" |"ns" |"us" |"ms" | "s" => {},
        _ => return Err(SvError::syntax(t2,"timescale. Expecting fs, ps, ns, ..."))
    }
    Ok(format!("{}{}",t1.value,t2.value))
}

/// Parse an assetr/else
pub fn parse_assert(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut n = AstNode::new(AstNodeKind::Assert, ts.get_pos());
    ts.flush(1); // Consume assert
    let mut t = ts.next_t(false)?;
    // Support for deferred assertion (#0/final)
    match t.kind {
        TokenKind::Hash => {
            t = expect_t!(ts,"deferred assertion",TokenKind::Integer);
            if t.value!="0" {rpt_t!(MsgID::ErrInvalid,&t,"deferred assertion. Expecting #0 or final")}
            n.attr.insert("deferred".to_string(),t.value);
            t = ts.next_t(false)?;
        }
        TokenKind::KwFinal => {
            n.attr.insert("deferred".to_string(),t.value);
            t = ts.next_t(false)?;
        }
        _ => {}
    }
    match t.kind {
        TokenKind::KwProperty => {
            ts.skip_until(TokenKind::ParenRight)?;
        }
        TokenKind::ParenLeft => {
            n.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
            ts.flush(1); // Consume right parenthesis
        }
        _ => return Err(SvError::syntax(t,"assert statement. Expecting ("))
    }
    t = ts.next_t(true)?;
    match t.kind {
        //
        TokenKind::SemiColon => ts.flush(1),
        // Else part handle after
        TokenKind::KwElse => {}
        // What ?
        _ => {
            ts.rewind(0);
            parse_class_stmt_or_block(ts, &mut n, false)?;
            t = ts.next_t(true)?;
        }
    }
    // Handle optionnal else part
    if t.kind==TokenKind::KwElse {
        ts.flush(1);
        let mut node_else = AstNode::new(AstNodeKind::Branch, t.pos);
        node_else.attr.insert("kind".to_owned(),"else".to_owned());
        parse_class_stmt_or_block(ts, &mut node_else, false)?;
        n.child.push(node_else);
    } else {
        ts.rewind(0);
    }
    node.child.push(n);
    Ok(())
}

/// Parse constraint block
/// Temporary implementation : just get the name of the constraint
pub fn parse_constraint(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let t = ts.next_t(false)?;
    let mut n = AstNode::new(AstNodeKind::Constraint, t.pos);
    if t.kind == TokenKind::KwConstraint {
        let nt = expect_t!(ts,"constraint",TokenKind::Ident);
        n.attr.insert("name".to_owned(),nt.value);
    }
    n.attr.insert("kind".to_owned(),t.value);
    node.child.push(n);
    ts.skip_until(TokenKind::CurlyRight)?;
    // ts.display_status("parse_constraint done");
    Ok(())
}

/// Parse covergroup block
/// Temporary implementation : just get the name of the covergroup
pub fn parse_covergroup(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut n = AstNode::new(AstNodeKind::Covergroup, ts.get_pos());
    ts.flush(1); // Consume the covergroup word
    let t = expect_t!(ts,"covergroup",TokenKind::Ident);
    n.attr.insert("name".to_owned(),t.value);
    ts.skip_until(TokenKind::KwEndGroup)?;
    check_label(ts, &n.attr["name"])?;
    node.child.push(n);
    Ok(())
}

/// Parse property block
/// Temporary implementation : just get the name of the covergroup
pub fn parse_sva_property(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut n = AstNode::new(AstNodeKind::SvaProperty, ts.get_pos());
    ts.flush(1); // Consume the property word
    let t = expect_t!(ts,"SVA property",TokenKind::Ident);
    n.attr.insert("name".to_owned(),t.value);
    ts.skip_until(TokenKind::KwEndProperty)?;
    check_label(ts, &n.attr["name"])?;
    node.child.push(n);
    Ok(())
}