// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::{SvError};
use crate::lex::token::{TokenKind};
use crate::lex::token_stream::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;
use crate::ast::module_body::parse_sensitivity;
// use crate::ast::module_body::*;

/// This function should be called after a keyword interface
pub fn parse_class(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    // Parse declaration :
    // [ virtual ] class [ lifetime ] class_identifier
    //                   [ parameter_port_list ] [ extends class_type [ ( list_of_arguments ) ] ]
    //                   [ implements interface_class_type { , interface_class_type } ] ;
    //
    let mut t = ts.next_t(false)?;
    let mut node = AstNode::new(AstNodeKind::Class, t.pos);
    let mut is_class_intf = false;
    // Optionnal virtual/interface keyword
    match t.kind {
        TokenKind::KwVirtual => {
            node.attr.insert("qualifier".to_owned(),t.value);
            t = ts.next_t(false)?;
        }
        TokenKind::KwIntf => {
            node.attr.insert("qualifier".to_owned(),t.value);
            is_class_intf = true;
            t = ts.next_t(false)?;
        }
        _ => {}
    }
    // Mandatory class keyword
    if t.kind!=TokenKind::KwClass {
        return Err(SvError::syntax(t,"class. Expecting class"));
    }
    // Optional lifetime indicator
    t = ts.next_t(false)?;
    if t.kind==TokenKind::KwStatic || t.kind==TokenKind::KwAutomatic {
        node.attr.insert("lifetime".to_owned(),t.value);
        t = ts.next_t(false)?;
    }
    // Mandatory class name
    match t.kind {
        TokenKind::Ident => {
            node.attr.insert("name".to_owned(),t.value);
            t = ts.next_t(false)?;
        },
        _ => return Err(SvError::syntax(t, "class declaration, expecting identifier or lifetime (static/automatic)"))
    }
    // Optional parameter list
    if t.kind==TokenKind::Hash {
        let mut node_p = AstNode::new(AstNodeKind::Params, t.pos);
        t = ts.next_t(false)?;
        if t.kind!=TokenKind::ParenLeft {
            return Err(SvError::syntax(t, "param declaration. Expecting ("));
        }
        loop {
            node_p.child.push(parse_param_decl(ts,false)?);
            loop_args_break_cont!(ts,"param declaration",ParenRight);
        }
        t = ts.next_t(false)?;
        node.child.push(node_p);
    }
    // Optional extends
    if t.kind==TokenKind::KwExtends {
        let mut node_e = AstNode::new(AstNodeKind::Extends, t.pos);
        parse_class_type(ts, &mut node_e, false)?;
        node.child.push(node_e);
        t = ts.next_t(false)?;
        if t.kind == TokenKind::Comma && is_class_intf {
            loop {
                let mut node_e = AstNode::new(AstNodeKind::Extends, t.pos);
                parse_class_type(ts, &mut node_e,true)?;
                node.child.push(node_e);
                loop_args_break_cont!(ts,"extends declaration",SemiColon);
            }
            t.kind = TokenKind::SemiColon; // Just to ensure next check is fine
        }
    }
    // Optional extends
    if t.kind==TokenKind::KwImplements {
        loop {
            let mut node_e = AstNode::new(AstNodeKind::Implements, t.pos);
            parse_class_type(ts, &mut node_e,true)?;
            node.child.push(node_e);
            loop_args_break_cont!(ts,"implements declaration",SemiColon);
        }
        t.kind = TokenKind::SemiColon; // Just to ensure next check is fine
    }
    // Check the declaration ends with a ;
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t,"class. Expecting ;"));
    }

    // Loop on class item
    loop {
        t = ts.next_t(true)?;
        // println!("[class] Top level token={}", t);
        match t.kind {
            // Class members : known types, identifier, rand qualifier
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     |
            TokenKind::KwRand        |
            TokenKind::KwConst       |
            TokenKind::Ident       => parse_class_members(ts, &mut node)?,
            // Class parameters
            TokenKind::KwParam | TokenKind::KwLParam => {
                ts.rewind(1); // put back the token so that it can be read by the parse param function
                // potential list of param (the parse function extract only one at a time)
                loop {
                    let node_param = parse_param_decl(ts,true)?;
                    node.child.push(node_param);
                    loop_args_break_cont!(ts,"param declaration",SemiColon);
                }
            }
            // Typedef
            TokenKind::KwTypedef => parse_typedef(ts,&mut node)?,
            // A class can be defined inside a class
            TokenKind::KwClass  => node.child.push(parse_class(ts)?),
            TokenKind::KwImport => parse_import(ts,&mut node)?,
            // Local/protected/static : can be members or method -> continue parsing
            TokenKind::KwLocal     |
            TokenKind::KwProtected => {
                loop {
                    t = ts.next_t(true)?;
                    match t.kind {
                        TokenKind::KwFunction => { parse_func(ts, &mut node, false, false)?; break; },
                        TokenKind::KwTask     => { parse_task(ts, &mut node)?;               break; },
                        TokenKind::TypeIntAtom   |
                        TokenKind::TypeIntVector |
                        TokenKind::TypeReal      |
                        TokenKind::TypeString    |
                        TokenKind::TypeCHandle   |
                        TokenKind::TypeEvent     |
                        TokenKind::KwRand        |
                        TokenKind::KwVar         |
                        TokenKind::KwConst       |
                        TokenKind::Macro         |
                        TokenKind::Ident         => {parse_class_members(ts, &mut node)?; break; },
                        TokenKind::KwVirtual   |
                        TokenKind::KwStatic     => {}
                        _ => return Err(SvError::syntax(t, "extern function/task declaration")),
                    }
                }
            }
            TokenKind::KwStatic    => {
                t = ts.next_t(true)?;
                if t.kind == TokenKind::KwLocal || t.kind == TokenKind::KwProtected {
                    t = ts.next_t(true)?;
                }
                // println!("[class] local/protected/static followed by {}", t);
                match t.kind {
                    TokenKind::TypeIntAtom   |
                    TokenKind::TypeIntVector |
                    TokenKind::TypeReal      |
                    TokenKind::TypeString    |
                    TokenKind::TypeCHandle   |
                    TokenKind::TypeEvent     |
                    TokenKind::KwRand        |
                    TokenKind::KwVar         |
                    TokenKind::KwConst       |
                    TokenKind::Ident         => parse_class_members(ts, &mut node)?,
                    TokenKind::KwFunction    => parse_func(ts, &mut node, false, false)?,
                    TokenKind::KwTask        => parse_task(ts, &mut node)?,
                    TokenKind::KwConstraint  => parse_constraint(ts,&mut node)?,
                    _ => return Err(SvError::syntax(t, "virtual task/function/interface")),
                }
            }
            TokenKind::KwExtern => {
                loop {
                    t = ts.next_t(true)?;
                    match t.kind {
                        TokenKind::KwFunction => { parse_func(ts, &mut node, false, false)?; break;},
                        TokenKind::KwTask     => { parse_task(ts, &mut node)?; break;},
                        TokenKind::KwLocal     |
                        TokenKind::KwProtected |
                        TokenKind::KwVirtual   |
                        TokenKind::KwStatic     => {}
                        _ => return Err(SvError::syntax(t, "extern function/task declaration")),
                    }
                }
            }
            // Function/task
            TokenKind::KwFunction => parse_func(ts, &mut node, false, false)?,
            TokenKind::KwTask     => parse_task(ts, &mut node)?,
            TokenKind::KwVirtual => {
                t = ts.next_t(true)?;
                if t.kind == TokenKind::KwProtected {
                    t = ts.next_t(true)?;
                }
                match t.kind {
                    TokenKind::Ident | TokenKind::KwIntf | TokenKind::Macro => parse_vintf(ts, &mut node)?,
                    TokenKind::KwFunction => parse_func(ts, &mut node, false, false)?,
                    TokenKind::KwTask     => parse_task(ts, &mut node)?,
                    _ => return Err(SvError::syntax(t, "virtual task/function/interface")),
                }

            }
            TokenKind::KwPure => {
                t = ts.next_t(true)?;
                if t.kind != TokenKind::KwVirtual {
                    return Err(SvError::syntax(t, "virtual task/function/interface"))
                }
                t = ts.next_t(true)?;
                if t.kind == TokenKind::KwProtected {
                    t = ts.next_t(true)?;
                }
                match t.kind {
                    TokenKind::KwFunction => parse_func(ts, &mut node, false, false)?,
                    TokenKind::KwTask     => parse_task(ts, &mut node)?,
                    _ => return Err(SvError::syntax(t, "pure virtual task/function")),
                }
            }
            TokenKind::Macro => {
                parse_macro(ts,&mut node)?;
                // Check for trailing ; after a macro
                t = ts.next_t(true)?;
                if t.kind == TokenKind::SemiColon {ts.flush(1);} else {ts.rewind(1);}
            },
            TokenKind::CompDir => parse_macro(ts,&mut node)?,
            TokenKind::KwConstraint => parse_constraint(ts,&mut node)?,
            TokenKind::KwCovergroup => parse_covergroup(ts,&mut node)?,
            TokenKind::SemiColon => {ts.flush(1);}, // TODO: generate a warning
            TokenKind::KwEndClass => {
                ts.flush(1);
                check_label(ts,&node.attr["name"])?;
                break;
            },
            _ => return Err(SvError::syntax(t, "class body")),
        }
    }
    // println!("[class] {}", node);
    Ok(node)
}


/// Parse a data type
pub fn parse_class_type(ts : &mut TokenStream, node : &mut AstNode, is_intf : bool) -> Result<(), SvError> {
    parse_opt_scope(ts,node)?;
    let t = expect_t!(ts,"data type",TokenKind::Ident);
    node.attr.insert("type".to_owned(),t.value);
    node.pos = t.pos;
    // Check for param
    parse_opt_params!(ts,node);
    // Check arguments
    if !is_intf {
        let nt = ts.next_t(true)?;
        if nt.kind==TokenKind::ParenLeft {
            ts.rewind(0);
            let mut node_p = AstNode::new(AstNodeKind::Ports, nt.pos);
            parse_func_call(ts,&mut node_p, false)?;
            node.child.push(node_p);
        }
    }
    // Put back token we read
    ts.rewind(0);
    Ok(())
}

///
pub fn parse_class_members(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut node_m = AstNode::new(AstNodeKind::Declaration, ts.get_pos());
    let mut t;
    let mut allow_const = true;
    let mut allow_lifetime = true;
    let mut allow_access = true;
    let mut allow_rand = true;
    let mut allow_var = true;
    let mut allow_dir = true;
    // Parse optional qualifier (no order really enforced by standard)
    loop {
        t = ts.next_t(true)?;
        match t.kind {
            TokenKind::KwConst  if allow_const => {
                node_m.attr.insert("const".to_owned(), "".to_owned());
                allow_const = false;
            }
            TokenKind::KwAutomatic |
            TokenKind::KwStatic if allow_lifetime => {
                node_m.attr.insert("lifetime".to_owned(), t.value);
                allow_lifetime = false;
            }
            TokenKind::KwLocal |
            TokenKind::KwProtected if allow_access => {
                node_m.attr.insert("access".to_owned(), t.value);
                allow_access = false;
            }
            TokenKind::KwRand if allow_rand => {
                node_m.attr.insert("rand".to_owned(), t.value);
                allow_rand = false;
            }
            TokenKind::KwReg |
            TokenKind::KwVar if allow_var => {
                node_m.attr.insert("net".to_owned(), t.value);
                allow_var = false;
            }
            TokenKind::KwInput  |
            TokenKind::KwOutput |
            TokenKind::KwInout  |
            TokenKind::KwRef    if allow_dir => {
                node.attr.insert("dir".to_owned(), t.value);
                allow_dir = false
            }
            _ => break
        }
        ts.flush(1);
    }
    // Parse data type
    parse_data_type(ts, &mut node_m, 2)?;
    // Parse data name
    let mut n = AstNode::new(AstNodeKind::Identifier, ts.get_pos());
    parse_var_decl_name(ts, &mut n,ExprCntxt::StmtList,false,false)?;
    // println!("[parse_class_members] {}", node_m);
    // if node_m.attr.contains_key("net") {println!("{:?}", node);}
    node_m.child.push(n);
    // Check for extra signals
    loop {
        // loop_args_break_cont!(ts,"signal declaration",SemiColon);
        t = ts.next_t(false)?;
        // println!("[parse_class_members] Next token : {}", t);
        match t.kind {
            TokenKind::Comma => {}, // Comma indicate a list -> continue
            TokenKind::SemiColon => break, // Semi colon indicate end of statement, stop the loop
            _ => return Err(SvError::syntax(t, "signal declaration, expecting , or ;"))
        }
        n = AstNode::new(AstNodeKind::Identifier, t.pos);
        parse_var_decl_name(ts, &mut n,ExprCntxt::StmtList,false,false)?;
        // println!("[class members] {}", node_m);
        // ts.display_status("class_members");
        node_m.child.push(n);
    }
    node.child.push(node_m);
    ts.rewind(0);
    // ts.display_status("Post parse_class_members");
    Ok(())
}

/// Parse a data type
pub fn parse_func(ts : &mut TokenStream, node : &mut AstNode, is_oob : bool, is_decl: bool) -> Result<(), SvError> {
    ts.rewind(0);
    let mut node_f = AstNode::new(AstNodeKind::Function, ts.get_pos());
    let mut t;
    let mut allow_virtual = true;
    let mut allow_protloc = true;
    let mut allow_static = true;
    let mut no_body = is_decl;
    // ts.display_status("parse_func: start");
    // optional pure
    loop {
        t = ts.next_t(false)?;
        match t.kind {
            TokenKind::KwExtern => {
                no_body = true;
                node_f.attr.insert("extern".to_owned(),"".to_owned());
            }
            TokenKind::KwPure => {
                no_body = true;
                node_f.attr.insert("pure".to_owned(),"".to_owned());
                expect_t!(ts,"pure method", TokenKind::KwVirtual);
                allow_virtual = false;
            }
            TokenKind::KwVirtual if allow_virtual => {
                node_f.attr.insert(t.value,"".to_owned());
                allow_virtual = false;
            }
            TokenKind::KwLocal | TokenKind::KwProtected if allow_protloc => {
                node_f.attr.insert("access".to_owned(),t.value);
                allow_protloc = false;
            }
            TokenKind::KwStatic if allow_static => {
                node_f.attr.insert(t.value, "".to_owned());
                allow_static = false;
            }
            TokenKind::KwFunction => break,
            _ => return Err(SvError::syntax(t, "function header. Expecting method qualifier"))
        }
    }
    t = ts.next_t(true)?;
    if t.kind==TokenKind::KwAutomatic || t.kind==TokenKind::KwStatic {
        node_f.attr.insert("lifetime".to_owned(),t.value);
        ts.flush_rd();
        t = ts.next_t(true)?;
    }
    // Expect return type or function name
    let mut nr = AstNode::new(AstNodeKind::Type, t.pos);
    match t.kind {
        TokenKind::KwNew => ts.rewind(0),
        _ => {
            let mut nt = ts.next_t(true)?;
            match nt.kind {
                // In case of ident check if this is not a class specifier followed by new (only for out-of-block definition)
                TokenKind::Scope if is_oob => {
                    nt = ts.next_t(true)?;
                    match nt.kind {
                        TokenKind::KwNew => ts.rewind(0),
                        _ => {
                            parse_data_type(ts,&mut nr, 1)?;
                            node_f.child.push(nr);
                        }
                    }
                }
                // Implicit return type, i.e logic
                // TODO: generate a warning ?
                TokenKind::ParenLeft => ts.rewind(0),
                _ => {
                    parse_data_type(ts,&mut nr, 1)?;
                    node_f.child.push(nr);
                }
            }
        }
    }
    // Expect function name
    t = ts.next_t(false)?;
    match t.kind {
        TokenKind::KwNew => {node_f.attr.insert("name".to_owned(),t.value);},
        // In case of ident check if this is not a class specifier followed by new (only for out-of-block definition)
        TokenKind::Ident => {
            if is_oob {
                node_f.attr.insert("scope".to_owned(),t.value);
                t = ts.next_t(false)?;
                if t.kind==TokenKind::Scope {
                    t = ts.next_t(false)?;
                    if t.kind == TokenKind::KwNew {
                        node_f.attr.insert("name".to_owned(),t.value);
                    } else {
                        return Err(SvError::syntax(t, "function header. Expecting function name"));
                    }
                } else {
                    return Err(SvError::syntax(t, "function header. Expecting function name"));
                }
            } else {
                node_f.attr.insert("name".to_owned(),t.value);
            }
        }
        // Expect data type
        _ => return Err(SvError::syntax(t, "function header. Expecting function name"))
    }
    // Optional function port definition
    t = ts.next_t(false)?;
    let has_args = t.kind==TokenKind::ParenLeft;
    if has_args {
        t = ts.next_t(true)?;
        if t.kind!=TokenKind::ParenRight {
            ts.rewind(1);
            let mut node_ports = AstNode::new(AstNodeKind::Ports, ts.get_pos());
            loop {
                node_ports.child.push(parse_port_decl(ts, true,ExprCntxt::ArgList)?); // TBD if we want a parse specific to function instead of allowing same I/O as a module ...
                loop_args_break_cont!(ts,"port declaration",ParenRight);
            }
            // println!("{}", node_ports);
            node_f.child.push(node_ports);
        } else {
            ts.flush(1);
        }
        t = ts.next_t(false)?;
    }
    // Expect ;
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "function declaration. Expecting ;"));
    }
    if no_body {
        // println!("Function declaration {:?}", node_f);
        // ts.display_status("parse_func: decl done");
        node.child.push(node_f);
        return Ok(());
    }
    //
    let mut allow_decl = true;
    loop {
        t = ts.next_t(true)?;
        // if node_f.attr["name"]=="create" {println!("[parse_func] Next statement start : {} : {} ({})", t.kind, t.value, t.pos);}
        match t.kind {
            TokenKind::KwEndFunction => break,
            _ => allow_decl = parse_class_stmt(ts,&mut node_f,false, allow_decl, !has_args, false)?,
        }

    }
    // ts.display_status("parse_func: before get label");
    ts.flush_rd();
    // Check for optional end label
    check_label(ts,&node_f.attr["name"])?;
    // ts.display_status("parse_func: done");
    node.child.push(node_f);
    Ok(())
}

/// Parse a data type
pub fn parse_task(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut node_task = AstNode::new(AstNodeKind::Task, ts.get_pos());
    let mut t;
    let mut allow_virtual = true;
    let mut allow_protloc = true;
    let mut allow_static = true;
    let mut no_body = false;
    // optional pure
    loop {
        t = ts.next_t(false)?;
        match t.kind {
            TokenKind::KwExtern => {
                no_body = true;
                node_task.attr.insert("extern".to_owned(),"".to_owned());
            }
            TokenKind::KwPure => {
                node_task.attr.insert("pure".to_owned(),"".to_owned());
                no_body = true;
                t = ts.next_t(false)?;
                if t.kind != TokenKind::KwVirtual {
                    return Err(SvError::syntax(t, "pure method. Expecting virtual"));
                }
                allow_virtual = false;
            }
            TokenKind::KwVirtual if allow_virtual => {
                node_task.attr.insert(t.value,"".to_owned());
                allow_virtual = false;
            }
            TokenKind::KwLocal | TokenKind::KwProtected if allow_protloc => {
                node_task.attr.insert("access".to_owned(),t.value);
                allow_protloc = false;
            }
            TokenKind::KwStatic if allow_static => {
                node_task.attr.insert(t.value, "".to_owned());
                allow_static = false;
            }
            TokenKind::KwTask => break,
            _ => return Err(SvError::syntax(t, "task declaration. Expecting method qualifier"))
        }
    }
    t = ts.next_t(false)?;
    if t.kind==TokenKind::KwAutomatic || t.kind==TokenKind::KwStatic {
        node_task.attr.insert("lifetime".to_owned(),t.value);
        t = ts.next_t(false)?;
    }

    // Expect task name
    if t.kind != TokenKind::Ident {
        return Err(SvError::syntax(t, "task declaration. Expecting task name"))
    }
    node_task.attr.insert("name".to_owned(),t.value);

    // Optional function port definition
    t = ts.next_t(false)?;
    let has_args = t.kind==TokenKind::ParenLeft;
    if has_args {
        t = ts.next_t(true)?;
        if t.kind!=TokenKind::ParenRight {
            ts.rewind(1);
            let mut node_ports = AstNode::new(AstNodeKind::Ports, ts.get_pos());
            loop {
                node_ports.child.push(parse_port_decl(ts, true,ExprCntxt::ArgList)?); // TBD if we want a parse specific to function instead of allowing same I/O as a module ...
                loop_args_break_cont!(ts,"task port declaration",ParenRight);
            }
            // println!("{}", node_ports);
            node_task.child.push(node_ports);
        } else {
            ts.flush(1);
        }
        t = ts.next_t(false)?;
    }
    // Expect ;
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "function declaration. Expecting ;"));
    }

    if no_body {
        node.child.push(node_task);
        return Ok(());
    }
    //
    let mut allow_decl = true;
    loop {
        t = ts.next_t(true)?;
        match t.kind {
            TokenKind::KwEndTask => break,
            TokenKind::KwReturn => {
                node.child.push(AstNode::new(AstNodeKind::Return, ts.get_pos()));
                t = ts.next_t(false)?;
                if t.kind!=TokenKind::SemiColon {
                    return Err(SvError::syntax(t,"return statement. Expecting ;"));
                }
            },
            _ => allow_decl = parse_class_stmt(ts,&mut node_task,false,allow_decl,!has_args, false)?,
        }
    }
    ts.flush_rd();
    // Check for optional end label
    check_label(ts,&node_task.attr["name"])?;

    // println!("[parse_task] {}", node_task);
    node.child.push(node_task);
    Ok(())
}


// TODO: check if this can be removed (likely)
pub fn parse_class_stmt_or_block(ts : &mut TokenStream, node: &mut AstNode, allow_assign: bool) -> Result<(), SvError> {
    let is_block = parse_has_begin(ts,node)?;
    parse_class_stmt(ts,node, is_block, is_block, false, allow_assign)?;
    if is_block && node.attr["block"]!="" {
        check_label(ts, &node.attr["block"])?;
    }
    Ok(())
}


/// Parse any statement in a process
pub fn parse_class_stmt(ts : &mut TokenStream, node: &mut AstNode, is_block: bool, mut allow_decl: bool, allow_port: bool, allow_assign: bool) -> Result<bool, SvError> {
    ts.rewind(0);
    let mut was_decl;
    loop {
        let mut t = ts.next_t(true)?;
        was_decl = false;
        // println!("[parse_stmt] Token = {} (block={}, allow decl={}, port={})", t, is_block,allow_decl,allow_port);
        // ts.display_status("");
        match t.kind {
            TokenKind::KwBegin => {
                let mut n = AstNode::new(AstNodeKind::Block, t.pos);
                ts.flush(1);
                parse_label(ts,&mut n,"block".to_owned())?;
                parse_class_stmt(ts,&mut n, true, true,false, allow_assign)?;
                if n.attr["block"]!="" {
                    check_label(ts, &n.attr["block"])?;
                }
                node.child.push(n);
            }
            TokenKind::KwIf   => parse_class_if_else(ts,node, allow_assign)?,
            TokenKind::KwFor   => {
                ts.flush(1);
                parse_class_for(ts,node)?;
            }
            TokenKind::KwForever => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop, t.pos);
                n.attr.insert("kind".to_owned(), t.value);
                parse_class_stmt_or_block(ts, &mut n, allow_assign)?;
                node.child.push(n);
            }
            TokenKind::KwForeach => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop, t.pos);
                n.attr.insert("kind".to_owned(), t.value);
                t = ts.next_t(false)?;
                if t.kind!=TokenKind::ParenLeft {
                    return Err(SvError::syntax(t,"foreach. Expecting ("));
                }
                // TODO: handle multi-dimensionnal foreach loop !
                n.child.push(parse_ident_hier(ts)?);
                t = ts.next_t(false)?;
                if t.kind!=TokenKind::ParenRight {
                    return Err(SvError::syntax(t,"foreach. Expecting )"));
                }
                ts.flush_rd(); // Clear parenthesis
                parse_class_stmt_or_block(ts, &mut n, allow_assign)?;
                // println!("Foreach: {}", n);
                node.child.push(n);
            }
            TokenKind::KwAssign if allow_assign => {
                was_decl = parse_assign_or_call(ts,node,ExprCntxt::Stmt)?;
            }
            TokenKind::KwRepeat | TokenKind::KwWhile  => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop, t.pos);
                n.attr.insert("kind".to_owned(), t.value);
                t = ts.next_t(false)?;
                if t.kind != TokenKind::ParenLeft {
                    return Err(SvError::syntax(t, &format!("{} loop, expecting (", n.attr["kind"].clone() )));
                }
                let node_cond = parse_expr(ts,ExprCntxt::Arg,false)?;
                ts.flush(1); // Consume right parenthesis
                n.child.push(node_cond);
                parse_class_stmt_or_block(ts, &mut n, allow_assign)?;
                node.child.push(n);
            }
            TokenKind::KwDo   => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop, t.pos);
                n.attr.insert("kind".to_owned(), t.value);
                let mut ns = AstNode::new(AstNodeKind::Statement, ts.get_pos());
                parse_class_stmt_or_block(ts, &mut ns, allow_assign)?;
                t = ts.next_t(false)?;
                if t.kind != TokenKind::KwWhile {
                    return Err(SvError::syntax(t, "do loop, expecting while"));
                }
                t = ts.next_t(false)?;
                if t.kind != TokenKind::ParenLeft {
                    return Err(SvError::syntax(t, "do loop, expecting ("));
                }
                let nc = parse_expr(ts,ExprCntxt::Arg,false)?;
                ts.flush(1); // Consume right parenthesis
                expect_t!(ts,"do loop",TokenKind::SemiColon);
                n.child.push(nc);
                n.child.push(ns);
                node.child.push(n);
            }
            TokenKind::KwCase     => parse_case(ts,node)?,
            TokenKind::KwPriority |
            TokenKind::KwUnique   |
            TokenKind::KwUnique0   => {
                t = ts.next_t(true)?;
                match t.kind {
                    TokenKind::KwIf   => parse_class_if_else(ts,node, allow_assign)?,
                    TokenKind::KwCase => parse_case(ts,node)?,
                    _ => return Err(SvError::syntax(t,"priority statement. Expecting case/if"))
                }
            }
            TokenKind::KwFork => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Fork, ts.get_pos());
                let has_label = parse_label(ts,&mut n,"label".to_string())?;
                loop {
                    t = ts.next_t(true)?;
                    if t.kind == TokenKind::KwJoin {
                        ts.flush(1);
                        n.attr.insert("join".to_owned(), t.value);
                        if has_label {
                            check_label(ts, &n.attr["label"])?;
                        }
                        break;
                    } else {
                        ts.rewind(1);
                        let mut ns = AstNode::new(AstNodeKind::Statement, ts.get_pos());
                        parse_class_stmt_or_block(ts, &mut ns, allow_assign)?;
                        n.child.push(ns);
                    }
                }
                // println!("Fork {}", n);
                node.child.push(n);
            }
            TokenKind::KwDisable => {
                let mut n = AstNode::new(AstNodeKind::Statement, t.pos);
                n.attr.insert("kind".to_owned(), t.value);
                ts.flush(1);
                t = ts.next_t(true)?;
                match t.kind {
                    TokenKind::KwFork => ts.flush(1),
                    TokenKind::Ident => n.child.push(parse_ident_hier(ts)?),
                    _ => return Err(SvError::syntax(t, "disable statement, expecting fork or identifier" ))
                }
                n.attr.insert("value".to_owned(), t.value);
                expect_t!(ts,"disable statement",TokenKind::SemiColon);
            }
            TokenKind::Macro => {
                parse_macro(ts,node)?;
                // Allow extra ;
                t = ts.next_t(true)?;
                if t.kind == TokenKind::SemiColon {ts.flush(1);} else {ts.rewind(1);}
            }
            TokenKind::CompDir => {
                parse_macro(ts,node)?;
                // Allow declaration after some ifdef/else/... : lazy check, but should be good enough
                was_decl = true;
            }
            TokenKind::SystemTask  => {
                node.child.push(parse_system_task(ts)?);
                expect_t!(ts,"system task call",TokenKind::SemiColon);
            }
            TokenKind::KwSuper | TokenKind::KwThis => {parse_assign_or_call(ts,node,ExprCntxt::Stmt)?;},
            TokenKind::Ident => {
                let name = t.value;
                t = ts.next_t(true)?;
                let has_label = t.kind == TokenKind::Colon;
                if has_label {
                    ts.flush(2); // Label
                    was_decl = false;
                    let mut n = AstNode::new(AstNodeKind::Statement, ts.get_pos());
                    n.attr.insert("label".to_owned(),name);
                    parse_class_stmt(ts,&mut n, false, false,false, allow_assign)?;
                }
                else {
                    // if node.attr.get("name")==Some(&"create".to_string()) {println!("Ident {} followed by {} (allow={})",name, t.kind,allow_decl);}
                    if allow_decl {
                        match t.kind {
                            TokenKind::Ident => was_decl=true,
                            // Parameterized class ? can be a declaration or a function call
                            TokenKind::Hash  => {
                                t = ts.next_t(true)?;
                                if t.kind != TokenKind::ParenLeft {
                                    return Err(SvError::syntax(t, "parameterized class, expecting ("));
                                }
                                ts.peek_until(TokenKind::ParenRight)?;
                                // ts.display_status("[peek_until] done");
                                t = ts.next_t(true)?;
                                // println!("[class_stmt] Post param got {}", t);
                                was_decl=t.kind==TokenKind::Ident;
                            },
                            TokenKind::Scope => {
                                t = ts.next_t(true)?;
                                if t.kind != TokenKind::Ident {
                                    return Err(SvError::syntax(t, "type/package, expecting identifier"));
                                }
                                t = ts.next_t(true)?;
                                was_decl = t.kind==TokenKind::Ident;
                            }
                            _ => was_decl=false,
                        }
                    }
                    if was_decl {
                        parse_class_members(ts, node)?;
                    } else {
                        was_decl = parse_assign_or_call(ts,node,ExprCntxt::Stmt)?;
                    }
                }
            },
            TokenKind::KwTypedef if allow_decl => {parse_typedef(ts,node)?; was_decl = true;},
            TokenKind::KwInput  |
            TokenKind::KwOutput |
            TokenKind::KwInout  |
            TokenKind::KwRef    if allow_port && allow_decl => {
                ts.rewind(1);
                loop {
                    node.child.push(parse_port_decl(ts,true,ExprCntxt::StmtList)?);
                    loop_args_break_cont!(ts,"ort declaration",SemiColon);
                }
                was_decl = true;
            }
            TokenKind::KwAutomatic   |
            TokenKind::KwStatic      |
            TokenKind::KwConst       |
            TokenKind::KwReg         |
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     if allow_decl => {
                parse_class_members(ts, node)?;
                was_decl = true;
            },
            TokenKind::CurlyLeft => {
                was_decl = parse_assign_or_call(ts,node,ExprCntxt::Stmt)?;
            }
            TokenKind::KwEnd if is_block => {
                ts.flush_rd();
                break;
            },
            TokenKind::KwBreak | TokenKind::KwContinue => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Branch, t.pos);
                n.attr.insert("kind".to_owned(),t.value);
                node.child.push(n);
                expect_t!(ts,"control flow",TokenKind::SemiColon);
            },
            TokenKind::OpImpl => {
                ts.flush(1);
                let mut n = parse_ident_hier(ts)?;
                n.kind = AstNodeKind::Event;
                // println!("Event {:?}", n);
                node.child.push(n);
                expect_t!(ts,"event trigger",TokenKind::SemiColon);
            }
            TokenKind::At => {
                ts.flush(1); // Consume At
                let mut n = AstNode::new(AstNodeKind::EventCtrl, t.pos);
                n.attr.insert("kind".to_owned(),t.value);
                n.child.push(parse_sensitivity(ts,false)?);
                t = ts.next_t(true)?;
                if t.kind != TokenKind::SemiColon {
                    parse_class_stmt_or_block(ts, &mut n, allow_assign)?;
                } else {
                    ts.flush(1);
                    // break;
                }
            }
            TokenKind::Hash => {
                let mut n = parse_delay(ts)?;
                t = ts.next_t(true)?;
                if t.kind != TokenKind::SemiColon {
                    ts.rewind(1);
                    parse_class_stmt_or_block(ts, &mut n, allow_assign)?;
                } else {
                    ts.flush(1);
                }
                node.child.push(n);
            }
            TokenKind::KwWait => {
                let mut n = AstNode::new(AstNodeKind::Wait, t.pos);
                n.attr.insert("kind".to_owned(),t.value);
                ts.flush(1); // Consume token
                t = ts.next_t(false)?;
                match t.kind {
                    TokenKind::KwFork => {
                        n.attr.insert("kind".to_owned()," fork".to_owned());
                        expect_t!(ts,"wait",TokenKind::SemiColon);
                    }
                    TokenKind::ParenLeft => {
                        n.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                        ts.flush(1); // Consume right parenthesis
                        t = ts.next_t(true)?;
                        match t.kind {
                            TokenKind::SemiColon => ts.flush(1),
                            _ => parse_class_stmt_or_block(ts, &mut n, allow_assign)?,
                        }
                    }
                    _ => return Err(SvError::syntax(t, "wait statement. Expecting fork or (event_name)"))
                }
                node.child.push(n);
            }
            TokenKind::KwAssert => parse_assert(ts,node)?,
            TokenKind::Casting => {
                ts.flush(1); // Consume
                if t.value!="void'" {
                    return Err(SvError::syntax(t,"casting statement. Expecting void"));
                }
                t = ts.next_t(false)?;
                if t.kind!=TokenKind::ParenLeft {
                    return Err(SvError::syntax(t,"casting statement. Expecting ("));
                }
                node.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                ts.flush(1); // Consume right parenthesis
                expect_t!(ts,"casting statement",TokenKind::SemiColon);
            }
            TokenKind::KwReturn => {
                let mut n = AstNode::new(AstNodeKind::Return, t.pos);
                ts.flush(1);
                t = ts.next_t(true)?;
                if t.kind!=TokenKind::SemiColon {
                    ts.rewind(1);
                    n.child.push(parse_expr(ts,ExprCntxt::Stmt,false)?);
                    ts.flush(1);
                }
                // println!("Return statement: token = {}\n node={}", t,n);
                node.child.push(n);
                ts.flush_rd();
            },
            // Semi-colon : empty statement
            TokenKind::SemiColon => ts.flush(1),
            TokenKind::KwEndFunction | TokenKind::KwEndTask => {ts.rewind(0); break;},
            _ => return Err(SvError::syntax(t, "class statement"))
        }
        // Stop parsing if not in a block
        if ! is_block {break;}
        allow_decl = was_decl;
    }
    // ts.display_status("parse_class_stmt: done");
    Ok(was_decl)
}

pub fn parse_assign_or_call(ts : &mut TokenStream, node: &mut AstNode, cntxt: ExprCntxt) -> Result<bool, SvError> {
    ts.rewind(0);
    let mut t = ts.next_t(true)?;
    let mut n = AstNode::new(AstNodeKind::Assign, t.pos);
    let mut is_decl = false;
    let mut nm;
    if t.kind == TokenKind::KwAssign {
        n.attr.insert("assign".to_owned(),t.value);
        ts.flush(1);
        t = ts.next_t(true)?;
    }
    match t.kind {
        TokenKind::OpIncrDecr => {
            n.attr.insert("incr_decr".to_owned(),t.value);
            n.attr.insert("op".to_owned(),"pre".to_owned());
            ts.flush(1);
            n.child.push(parse_member_or_call(ts, false)?);
            node.child.push(n);
            return Ok(is_decl);
        }
        // Concatenation operator
        TokenKind::CurlyLeft => {
            ts.flush(1);
            nm = AstNode::new(AstNodeKind::Concat, t.pos);
            loop {
                nm.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                loop_args_break_cont!(ts,"concatenation",CurlyRight);
            }
            // println!("{}", nm);
        }
        _ => {
            ts.rewind(1);
            nm = parse_member_or_call(ts, true)?;
            is_decl = nm.kind==AstNodeKind::Declaration;
        }
    }
    t = ts.next_t(true)?;
    match t.kind {
        TokenKind::OpEq | TokenKind::OpCompAss if !is_decl => {
            ts.flush(1); // Consume the operand
            n.attr.insert("kind".to_owned(),t.value);
            n.child.push(nm);
            n.child.push(parse_expr(ts,cntxt.clone(),false)?);
        }
        TokenKind::OpLTE if !is_decl => {
            ts.flush(1); // Consume the operand
            n.attr.insert("kind".to_owned(),t.value);
            n.child.push(nm);
            t = ts.next_t(true)?;
            if t.kind==TokenKind::Hash {
                n.child.push(parse_delay(ts)?);
            } else {
                ts.rewind(1);
            }
            n.child.push(parse_expr(ts,cntxt.clone(),false)?);
        }
        TokenKind::OpIncrDecr if !is_decl => {
            ts.flush(1); // Consume the operand
            n.attr.insert("incr_decr".to_owned(),t.value);
            n.attr.insert("op".to_owned(),"post".to_owned());
            n.child.push(nm);
        }
        TokenKind::SemiColon if cntxt==ExprCntxt::Stmt || cntxt==ExprCntxt::StmtList => n = nm,
        _ => return Err(SvError::syntax(t, "class assign/call. Expecting assignment operator or ;")),
    }
    // Rewind ending token when in a list, otherwise consume it
    match cntxt {
        ExprCntxt::Stmt => {expect_t!(ts,"assign/call",TokenKind::SemiColon);},
        ExprCntxt::Arg  => {expect_t!(ts,"assign/call",TokenKind::ParenRight);},
        ExprCntxt::ArgList  => {
            t = ts.next_t(true)?;
            if t.kind != TokenKind::ParenRight && t.kind != TokenKind::Comma {
                return Err(SvError::syntax(t, "assign/call. Expecting , or )"))
            }
            ts.rewind(1);
        },
        ExprCntxt::StmtList => {
            t = ts.next_t(true)?;
            if t.kind != TokenKind::SemiColon && t.kind != TokenKind::Comma {
                return Err(SvError::syntax(t, "assign/call. Expecting , or ;"))
            }
            ts.rewind(1);
        },
        _ => panic!("Invalid context {:?} for parse_assign_or_call",cntxt)
    }

    node.child.push(n);
    Ok(is_decl)
}

/// Parse If/Else if/Else statements
/// Suppose first IF has been consumed
#[allow(unused_variables, unused_mut)]
pub fn parse_class_if_else(ts : &mut TokenStream, node: &mut AstNode, allow_assign: bool) -> Result<(), SvError> {
    ts.rewind(0);
    let mut t = ts.next_t(false)?;
    let mut node_if = AstNode::new(AstNodeKind::Branch, t.pos);
    if t.kind==TokenKind::KwPriority || t.kind==TokenKind::KwUnique || t.kind==TokenKind::KwUnique0 {
        node_if.attr.insert("prio".to_owned(),t.value);
        t = ts.next_t(false)?;
    }
    if t.kind==TokenKind::KwElse {
        t = ts.next_t(false)?;
        node_if.attr.insert("kind".to_owned(),"else if".to_owned());
    } else {
        node_if.attr.insert("kind".to_owned(),"if".to_owned());
    }
    if t.kind!=TokenKind::KwIf {
        return Err(SvError::syntax(t, " if statement. Expecting if"));
    }
    expect_t!(ts,"if statement",TokenKind::ParenLeft);
    let n = parse_expr(ts,ExprCntxt::Arg,false)?;
    ts.flush(1); // Consume right parenthesis
    node_if.child.push(n);
    // parse_class_stmt(ts, &mut node_if, false, false, false, allow_assign)?;
    parse_class_stmt_or_block(ts,&mut node_if, allow_assign)?;
    node.child.push(node_if);
    // Check for else if/else statement
    let mut t;
    loop {
        t = ts.next_t(true)?;
        // println!("[parse_if_else] Else Token ? {}", t);
        if t.kind == TokenKind::KwElse {
            t = ts.next_t(true)?;
            // println!("[parse_if_else] If Token ? {}", t);
            if t.kind == TokenKind::KwIf {
                parse_class_if_else(ts,node, allow_assign)?;
            } else {
                ts.flush(1); // Consume else
                ts.rewind(0);
                let mut node_else = AstNode::new(AstNodeKind::Branch, t.pos);
                node_else.attr.insert("kind".to_owned(),"else".to_owned());
                parse_class_stmt_or_block(ts, &mut node_else, allow_assign)?;
                // parse_class_stmt(ts, &mut node_else, false, false, false, allow_assign)?;
                node.child.push(node_else);
                break;
            }
        }
        else {
            ts.rewind(0);
            break;
        }
    }

    Ok(())
}


pub fn parse_class_for(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush_rd();
    let mut t = expect_t!(ts,"for",TokenKind::ParenLeft);
    let mut node_for = AstNode::new(AstNodeKind::LoopFor, t.pos);
    // Parse init part : end on ;
    let mut node_hdr = AstNode::new(AstNodeKind::Header, t.pos);
    // Handle empty init
    let mut is_decl = false;
    loop {
        t = ts.next_t(true)?;
        let mut ns = AstNode::new(AstNodeKind::Declaration, t.pos);
        match t.kind {
            TokenKind::SemiColon => break, // Handle empty init
            TokenKind::Ident => {
                if !is_decl {
                    ns.kind = AstNodeKind::Assign;
                }
                ts.rewind(1);
            },
            _ => {
                is_decl = true;
                if t.kind==TokenKind::KwVar {
                    ts.flush(1);
                    node.attr.insert("nettype".to_owned(), t.value);
                } else {
                    ts.rewind(1);
                }
                parse_data_type(ts, &mut ns, 2)?;
            }
        }
        parse_var_decl_name(ts, &mut ns,ExprCntxt::StmtList,true,false)?;
        ns.attr.insert("loop".to_owned(), "init".to_owned());
        node_hdr.child.push(ns);
        // Stop loop on semicolon, consume comma if any
        loop_args_break_cont!(ts,"for loop init statement",SemiColon);
    }
    ts.flush(1); // clear semi-colon
    // Parse test part : end on ;
    let mut ns = parse_expr(ts,ExprCntxt::Stmt,false)?;
    ns.attr.insert("loop".to_owned(), "test".to_owned());
    node_hdr.child.push(ns);
    ts.flush(1); // clear semi-colon
    // Parse incr part : end on )
    t = ts.next_t(true)?;
    if t.kind != TokenKind::ParenRight {
        ts.rewind(1);
        loop {
            ns = AstNode::new(AstNodeKind::Expr, t.pos);
            parse_assign_or_call(ts,&mut ns,ExprCntxt::ArgList)?;
            ns.attr.insert("loop".to_owned(), "incr".to_owned());
            node_hdr.child.push(ns);
            loop_args_break_cont!(ts,"for test arguments",ParenRight);
        }
    }
    ts.flush_rd(); // Clear parenthesis
    node_for.child.push(node_hdr);
    // Parse content of for loop
    parse_class_stmt_or_block(ts,&mut node_for, false)?;
    // println!("parse_class_for {}", node_for);
    node.child.push(node_for);
    Ok(())
}

