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
    let mut node = AstNode::new(AstNodeKind::Class);
    let mut t = next_t!(ts,false);
    // Optionnal virtual keyword
    if t.kind==TokenKind::KwVirtual {
        node.attr.insert("virtual".to_string(),"".to_string());
        t = next_t!(ts,false);
    }
    // Mandatory class keyword
    if t.kind!=TokenKind::KwClass {
        return Err(SvError::syntax(t,"class. Expecting class".to_string()));
    }
    // Optional lifetime indicator
    t = next_t!(ts,false);
    if t.kind==TokenKind::KwStatic || t.kind==TokenKind::KwAutomatic {
        node.attr.insert("lifetime".to_string(),t.value);
        t = next_t!(ts,false);
    }
    // Mandatory class name
    match t.kind {
        TokenKind::Ident => {
            node.attr.insert("name".to_string(),t.value);
            t = next_t!(ts,false);
        },
        _ => return Err(SvError::syntax(t, "class declaration, expecting identifier or lifetime (static/automatic)".to_string()))
    }
    // Optional parameter list
    if t.kind==TokenKind::Hash {
        let mut node_p = AstNode::new(AstNodeKind::Params);
        t = next_t!(ts,false);
        if t.kind!=TokenKind::ParenLeft {
            return Err(SvError::syntax(t, "param declaration. Expecting (".to_string()));
        }
        loop {
            node_p.child.push(parse_param_decl(ts,false)?);
            loop_args_break_cont!(ts,"param declaration",ParenRight);
        }
        t = next_t!(ts,false);
        node.child.push(node_p);
    }
    // Optional extends
    if t.kind==TokenKind::KwExtends {
        let mut node_e = AstNode::new(AstNodeKind::Extends);
        parse_class_type(ts, &mut node_e, false)?;
        node.child.push(node_e);
        t = next_t!(ts,false);
    }
    // Optional extends
    if t.kind==TokenKind::KwImplements {
        let mut node_e = AstNode::new(AstNodeKind::Implements);
        parse_class_type(ts, &mut node_e,true)?;
        node.child.push(node_e);
        t = next_t!(ts,false);
    }
    // Check the declaration ends with a ;
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t,"class. Expecting ;".to_string()));
    }

    // Loop on class item
    loop {
        t = next_t!(ts,true);
        // println!("[class] token={}", t);
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
            TokenKind::KwParam | TokenKind::KwLParam => {
                ts.rewind(1); // put back the token so that it can be read by the parse param function
                // potential list of param (the parse function extract only one at a time)
                loop {
                    let node_param = parse_param_decl(ts,true)?;
                    node.child.push(node_param);
                    loop_args_break_cont!(ts,"param declaration",SemiColon);
                }
            }
            TokenKind::KwTypedef => parse_typedef(ts,&mut node)?,
            // Local/protected/static : can be members or method -> continue parsing
            TokenKind::KwLocal     |
            TokenKind::KwProtected => {
                loop {
                    t = next_t!(ts,true);
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
                        _ => return Err(SvError::syntax(t, "extern function/task declaration".to_string())),
                    }
                }
            }
            TokenKind::KwStatic    => {
                t = next_t!(ts,true);
                if t.kind == TokenKind::KwLocal || t.kind == TokenKind::KwProtected {
                    t = next_t!(ts,true);
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
                    TokenKind::KwFunction => parse_func(ts, &mut node, false, false)?,
                    TokenKind::KwTask     => parse_task(ts, &mut node)?,
                    _ => return Err(SvError::syntax(t, "virtual task/function/interface".to_string())),
                }
            }
            TokenKind::KwExtern => {
                loop {
                    t = next_t!(ts,true);
                    match t.kind {
                        TokenKind::KwFunction => { parse_func(ts, &mut node, false, false)?; break;},
                        TokenKind::KwTask     => { parse_task(ts, &mut node)?; break;},
                        TokenKind::KwLocal     |
                        TokenKind::KwProtected |
                        TokenKind::KwVirtual   |
                        TokenKind::KwStatic     => {}
                        _ => return Err(SvError::syntax(t, "extern function/task declaration".to_string())),
                    }
                }
            }
            // Function/task
            TokenKind::KwFunction => parse_func(ts, &mut node, false, false)?,
            TokenKind::KwTask     => parse_task(ts, &mut node)?,
            TokenKind::KwVirtual => {
                t = next_t!(ts,true);
                if t.kind == TokenKind::KwProtected {
                    t = next_t!(ts,true);
                }
                match t.kind {
                    TokenKind::Ident | TokenKind::KwIntf | TokenKind::Macro => parse_vintf(ts, &mut node)?,
                    TokenKind::KwFunction => parse_func(ts, &mut node, false, false)?,
                    TokenKind::KwTask     => parse_task(ts, &mut node)?,
                    _ => return Err(SvError::syntax(t, "virtual task/function/interface".to_string())),
                }

            }
            TokenKind::KwPure => {
                t = next_t!(ts,true);
                if t.kind != TokenKind::KwVirtual {
                    return Err(SvError::syntax(t, "virtual task/function/interface".to_string()))
                }
                t = next_t!(ts,true);
                if t.kind == TokenKind::KwProtected {
                    t = next_t!(ts,true);
                }
                match t.kind {
                    TokenKind::KwFunction => parse_func(ts, &mut node, false, false)?,
                    TokenKind::KwTask     => parse_task(ts, &mut node)?,
                    _ => return Err(SvError::syntax(t, "pure virtual task/function".to_string())),
                }
            }
            TokenKind::Macro => {
                parse_macro(ts,&mut node)?;
                // Check for trailing ; after a macro
                t = next_t!(ts,true);
                if t.kind == TokenKind::SemiColon {ts.flush(1);} else {ts.rewind(1);}
            },
            TokenKind::KwConstraint => parse_constraint(ts,&mut node)?,
            TokenKind::KwCovergroup => parse_covergroup(ts,&mut node)?,
            TokenKind::SemiColon => {ts.flush(1);}, // TODO: generate a warning
            TokenKind::KwEndClass => break,
            _ => return Err(SvError::syntax(t, "class body".to_string())),
        }
    }
    ts.flush(0);
    // TODO: handle endclass label
    // println!("[class] {}", node);

    Ok(node)
}


/// Parse a data type
pub fn parse_class_type(ts : &mut TokenStream, node : &mut AstNode, is_intf : bool) -> Result<(), SvError> {
    let t = next_t!(ts,false);
    if t.kind!=TokenKind::Ident {
        return Err(SvError::syntax(t, "param declaration. Expecting (".to_string()));
    }
    // Check for scope
    let mut nt = next_t!(ts,true);
    if nt.kind==TokenKind::Scope {
        nt = next_t!(ts,true);
        if nt.kind != TokenKind::Ident {
            return Err(SvError::syntax(t, "class type. Expecting identifier".to_string()));
        }
        node.attr.insert("name".to_string(),format!("{}::{}",t.value,nt.value));
        ts.flush(0);
        nt = next_t!(ts,true);
    } else {
        node.attr.insert("name".to_string(),t.value);
    }
    // Check for param
    parse_opt_params!(ts,node,nt);
    // Check arguments
    if nt.kind==TokenKind::ParenLeft && !is_intf {
        ts.rewind(0);
        let mut node_p = AstNode::new(AstNodeKind::Ports);
        parse_func_call(ts,&mut node_p, false)?;
        node.child.push(node_p);
    }
    // Put back token we read
    ts.rewind(0);
    Ok(())
}

///
pub fn parse_class_members(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let mut node_m = AstNode::new(AstNodeKind::Declaration);
    ts.rewind(0);
    let mut t;
    let mut allow_const = true;
    let mut allow_lifetime = true;
    let mut allow_access = true;
    let mut allow_rand = true;
    let mut allow_var = true;
    // Parse optional qualifier (no order really enforced by standard)
    loop {
        t = next_t!(ts,true);
        match t.kind {
            TokenKind::KwConst  if allow_const => {
                node_m.attr.insert("const".to_string(), "".to_string());
                allow_const = false;
            }
            TokenKind::KwAutomatic |
            TokenKind::KwStatic if allow_lifetime => {
                node_m.attr.insert("lifetime".to_string(), t.value);
                allow_lifetime = false;
            }
            TokenKind::KwLocal |
            TokenKind::KwProtected if allow_access => {
                node_m.attr.insert("access".to_string(), t.value);
                allow_access = false;
            }
            TokenKind::KwRand if allow_rand => {
                node_m.attr.insert("rand".to_string(), "".to_string());
                allow_rand = false;
            }
            TokenKind::KwVar if allow_var => {
                node_m.attr.insert("var".to_string(), "".to_string());
                allow_var = false;
            }
            _ => break
        }
        ts.flush(1);
    }
    // Parse data type
    parse_data_type(ts, &mut node_m, 2)?;
    // Parse data name
    parse_var_decl_name(ts, &mut node_m)?;
    // println!("[parse_class_members] {}", node_m);
    node.child.push(node_m);
    // Check for extra signals
    loop {
        t = next_t!(ts,false);
        // println!("[parse_class_members] Next token : {}", t);
        match t.kind {
            TokenKind::Comma => {}, // Comma indicate a list -> continue
            TokenKind::SemiColon => break, // Semi colon indicate end of statement, stop the loop
            _ => return Err(SvError::syntax(t, "signal declaration, expecting , or ;".to_string()))
        }
        node_m = AstNode::new(AstNodeKind::Declaration);
        parse_var_decl_name(ts, &mut node_m)?;
        // println!("[class members] {}", node_m);
        // ts.display_status("class_members");
        node.child.push(node_m);
    }
    ts.rewind(0);
    // ts.display_status("Post parse_class_members");
    Ok(())
}

/// Parse a data type
pub fn parse_func(ts : &mut TokenStream, node : &mut AstNode, is_oob : bool, is_decl: bool) -> Result<(), SvError> {
    ts.rewind(0);
    let mut node_f = AstNode::new(AstNodeKind::Function);
    let mut t;
    let mut allow_virtual = true;
    let mut allow_protloc = true;
    let mut allow_static = true;
    let mut no_body = is_decl;
    // ts.display_status("parse_func: start");
    // optional pure
    loop {
        t = next_t!(ts,false);
        match t.kind {
            TokenKind::KwExtern => {
                no_body = true;
                node_f.attr.insert("extern".to_string(),"".to_string());
            }
            TokenKind::KwPure => {
                no_body = true;
                node_f.attr.insert("pure".to_string(),"".to_string());
                expect_t!(ts,"pure method", TokenKind::KwVirtual);
                allow_virtual = false;
            }
            TokenKind::KwVirtual if allow_virtual => {
                node_f.attr.insert(t.value,"".to_string());
                allow_virtual = false;
            }
            TokenKind::KwLocal | TokenKind::KwProtected if allow_protloc => {
                node_f.attr.insert("access".to_string(),t.value);
                allow_protloc = false;
            }
            TokenKind::KwStatic if allow_static => {
                node_f.attr.insert(t.value, "".to_string());
                allow_static = false;
            }
            TokenKind::KwFunction => break,
            _ => return Err(SvError::syntax(t, "function header. Expecting method qualifier".to_string()))
        }
    }
    t = next_t!(ts,true);
    if t.kind==TokenKind::KwAutomatic || t.kind==TokenKind::KwStatic {
        node_f.attr.insert("lifetime".to_string(),t.value);
        ts.flush(0);
        t = next_t!(ts,true);
    }
    // Expect return type (or new)
    match t.kind {
        TokenKind::KwNew => ts.rewind(0),
        // In case of ident check if this is not a class specifier followed by new (only for out-of-block definition)
        TokenKind::Ident if is_oob => {
            let mut nt = next_t!(ts,true);
            if nt.kind==TokenKind::Scope {
                nt = next_t!(ts,true);
                if nt.kind == TokenKind::KwNew {
                    ts.rewind(0);
                } else {
                    parse_data_type(ts,&mut node_f, 1)?;
                }
            } else {
                parse_data_type(ts,&mut node_f, 1)?;
            }
        }
        // Expect data type
        _ => parse_data_type(ts,&mut node_f, 3)?
    }
    // Expect function name
    t = next_t!(ts,false);
    match t.kind {
        TokenKind::KwNew => {node_f.attr.insert("name".to_string(),t.value);},
        // In case of ident check if this is not a class specifier followed by new (only for out-of-block definition)
        TokenKind::Ident => {
            if is_oob {
                node_f.attr.insert("scope".to_string(),t.value);
                t = next_t!(ts,false);
                if t.kind==TokenKind::Scope {
                    t = next_t!(ts,false);
                    if t.kind == TokenKind::KwNew {
                        node_f.attr.insert("name".to_string(),t.value);
                    } else {
                        return Err(SvError::syntax(t, "function header. Expecting function name".to_string()));
                    }
                } else {
                    return Err(SvError::syntax(t, "function header. Expecting function name".to_string()));
                }
            } else {
                node_f.attr.insert("name".to_string(),t.value);
            }
        }
        // Expect data type
        _ => return Err(SvError::syntax(t, "function header. Expecting function name".to_string()))
    }
    // Optional function port definition
    t = next_t!(ts,false);
    if t.kind==TokenKind::ParenLeft {
        t = next_t!(ts,true);
        if t.kind!=TokenKind::ParenRight {
            ts.rewind(1);
            let mut node_ports = AstNode::new(AstNodeKind::Ports);
            loop {
                node_ports.child.push(parse_port_decl(ts, true)?); // TBD if we want a parse specific to function instead of allowing same I/O as a module ...
                loop_args_break_cont!(ts,"port declaration",ParenRight);
            }
            // println!("{}", node_ports);
            node_f.child.push(node_ports);
        } else {
            ts.flush(1);
        }
        t = next_t!(ts,false);
    }
    // Expect ;
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "function declaration. Expecting ;".to_string()));
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
        t = next_t!(ts,true);
        match t.kind {
            TokenKind::KwEndFunction => break,
            _ => allow_decl = parse_class_stmt(ts,&mut node_f,false, allow_decl)?,
        }

    }
    ts.flush(0);
    // Check for optional end label
    check_label(ts,&node_f.attr["name"])?;
    // println!("[parse_func] {}", node_f);
    node.child.push(node_f);
    Ok(())
}

/// Parse a data type
pub fn parse_task(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut node_task = AstNode::new(AstNodeKind::Task);
    let mut t;
    let mut allow_virtual = true;
    let mut allow_protloc = true;
    let mut allow_static = true;
    let mut no_body = false;
    // optional pure
    loop {
        t = next_t!(ts,false);
        match t.kind {
            TokenKind::KwExtern => {
                no_body = true;
                node_task.attr.insert("extern".to_string(),"".to_string());
            }
            TokenKind::KwPure => {
                node_task.attr.insert("pure".to_string(),"".to_string());
                no_body = true;
                t = next_t!(ts,false);
                if t.kind != TokenKind::KwVirtual {
                    return Err(SvError::syntax(t, "pure method. Expecting virtual".to_string()));
                }
                allow_virtual = false;
            }
            TokenKind::KwVirtual if allow_virtual => {
                node_task.attr.insert(t.value,"".to_string());
                allow_virtual = false;
            }
            TokenKind::KwLocal | TokenKind::KwProtected if allow_protloc => {
                node_task.attr.insert("access".to_string(),t.value);
                allow_protloc = false;
            }
            TokenKind::KwStatic if allow_static => {
                node_task.attr.insert(t.value, "".to_string());
                allow_static = false;
            }
            TokenKind::KwTask => break,
            _ => return Err(SvError::syntax(t, "task declaration. Expecting method qualifier".to_string()))
        }
    }
    t = next_t!(ts,false);
    if t.kind==TokenKind::KwAutomatic || t.kind==TokenKind::KwStatic {
        node_task.attr.insert("lifetime".to_string(),t.value);
        t = next_t!(ts,false);
    }

    // Expect task name
    if t.kind != TokenKind::Ident {
        return Err(SvError::syntax(t, "task declaration. Expecting task name".to_string()))
    }
    node_task.attr.insert("name".to_string(),t.value);

    // Optional function port definition
    t = next_t!(ts,false);
    if t.kind==TokenKind::ParenLeft {
        t = next_t!(ts,true);
        if t.kind!=TokenKind::ParenRight {
            ts.rewind(1);
            let mut node_ports = AstNode::new(AstNodeKind::Ports);
            loop {
                node_ports.child.push(parse_port_decl(ts, true)?); // TBD if we want a parse specific to function instead of allowing same I/O as a module ...
                loop_args_break_cont!(ts,"task port declaration",ParenRight);
            }
            // println!("{}", node_ports);
            node_task.child.push(node_ports);
        } else {
            ts.flush(1);
        }
        t = next_t!(ts,false);
    }
    // Expect ;
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "function declaration. Expecting ;".to_string()));
    }
    if no_body {
        node.child.push(node_task);
        return Ok(());
    }
    //
    let mut allow_decl = true;
    loop {
        t = next_t!(ts,true);
        match t.kind {
            TokenKind::KwEndTask => break,
            TokenKind::KwReturn => {
                node.child.push(AstNode::new(AstNodeKind::Return));
                t = next_t!(ts,false);
                if t.kind!=TokenKind::SemiColon {
                    return Err(SvError::syntax(t,"return statement. Expecting ;".to_string()));
                }
            },
            _ => allow_decl = parse_class_stmt(ts,&mut node_task,false,allow_decl)?,
        }
    }
    ts.flush(0);
    // Check for optional end label
    check_label(ts,&node_task.attr["name"])?;

    // println!("[parse_task] {}", node_task);
    node.child.push(node_task);
    Ok(())
}


///
pub fn parse_class_stmt_or_block(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let is_block = parse_has_begin(ts,node)?;
    parse_class_stmt(ts,node, is_block, is_block)?;
    if is_block && node.attr["block"]!="" {
        check_label(ts, &node.attr["block"])?;
    }
    Ok(())
}


/// Parse any statement in a process
pub fn parse_class_stmt(ts : &mut TokenStream, node: &mut AstNode, is_block: bool, mut allow_decl: bool) -> Result<bool, SvError> {
    ts.rewind(0);
    let mut was_decl;
    loop {
        let mut t = next_t!(ts,true);
        was_decl = false;
        // println!("[parse_stmt] Token = {}", t);
        // ts.display_status("");
        match t.kind {
            TokenKind::KwBegin => {
                let mut n = AstNode::new(AstNodeKind::Block);
                parse_label(ts,&mut n,"block".to_string())?;
                parse_class_stmt(ts,&mut n, true, true)?;
                if is_block && n.attr["block"]!="" {
                    check_label(ts, &n.attr["block"])?;
                }
                node.child.push(n);
            }
            TokenKind::KwIf   => {
                ts.flush(1);
                parse_class_if_else(ts,node, "if")?;
            }
            TokenKind::KwFor   => {
                ts.flush(1);
                parse_class_for(ts,node)?;
            }
            TokenKind::KwForever => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop);
                n.attr.insert("kind".to_string(), t.value);
                parse_class_stmt_or_block(ts, &mut n)?;
                node.child.push(n);
            }
            TokenKind::KwForeach => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop);
                n.attr.insert("kind".to_string(), t.value);
                t = next_t!(ts,false);
                if t.kind!=TokenKind::ParenLeft {
                    return Err(SvError::syntax(t,"foreach. Expecting (".to_string()));
                }
                n.child.push(parse_ident_hier(ts)?);
                t = next_t!(ts,false);
                if t.kind!=TokenKind::ParenRight {
                    return Err(SvError::syntax(t,"foreach. Expecting )".to_string()));
                }
                ts.flush(0); // Clear parenthesis
                parse_class_stmt_or_block(ts, &mut n)?;
                // println!("Foreach: {}", n);
                node.child.push(n);
            }
            TokenKind::KwRepeat | TokenKind::KwWhile  => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop);
                n.attr.insert("kind".to_string(), t.value);
                t = next_t!(ts,false);
                if t.kind != TokenKind::ParenLeft {
                    return Err(SvError::syntax(t, format!("{} loop, expecting (", n.attr["kind"].clone() )));
                }
                let node_cond = parse_expr(ts,ExprCntxt::Arg,false)?;
                ts.flush(1); // Consume right parenthesis
                n.child.push(node_cond);
                parse_class_stmt_or_block(ts, &mut n)?;
                node.child.push(n);
            }
            TokenKind::KwDo   => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Loop);
                n.attr.insert("kind".to_string(), t.value);
                let mut ns = AstNode::new(AstNodeKind::Statement);
                parse_class_stmt_or_block(ts, &mut ns)?;
                t = next_t!(ts,false);
                if t.kind != TokenKind::KwWhile {
                    return Err(SvError::syntax(t, "do loop, expecting while".to_string() ));
                }
                t = next_t!(ts,false);
                if t.kind != TokenKind::ParenLeft {
                    return Err(SvError::syntax(t, "do loop, expecting (".to_string() ));
                }
                let nc = parse_expr(ts,ExprCntxt::Arg,false)?;
                ts.flush(1); // Consume right parenthesis
                expect_t!(ts,"do loop",TokenKind::SemiColon);
                n.child.push(nc);
                n.child.push(ns);
                node.child.push(n);
            }
            TokenKind::KwCase     |
            TokenKind::KwPriority |
            TokenKind::KwUnique   |
            TokenKind::KwUnique0   => {
                ts.rewind(0);
                parse_class_case(ts,node)?;
            }
            TokenKind::KwFork => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Fork);
                let has_label = parse_label(ts,&mut n,"label".to_string())?;
                loop {
                    t = next_t!(ts,true);
                    if t.kind == TokenKind::KwJoin {
                        ts.flush(1);
                        n.attr.insert("join".to_string(), t.value);
                        if has_label {
                            check_label(ts, &n.attr["label"])?;
                        }
                        break;
                    } else {
                        ts.rewind(1);
                        let mut ns = AstNode::new(AstNodeKind::Statement);
                        parse_class_stmt_or_block(ts, &mut ns)?;
                        n.child.push(ns);
                    }
                }
                // println!("Fork {}", n);
                node.child.push(n);
            }
            TokenKind::KwDisable => {
                let mut n = AstNode::new(AstNodeKind::Statement);
                n.attr.insert("kind".to_string(), t.value);
                ts.flush(1);
                t = next_t!(ts,true);
                match t.kind {
                    TokenKind::KwFork => ts.flush(1),
                    TokenKind::Ident => n.child.push(parse_ident_hier(ts)?),
                    _ => return Err(SvError::syntax(t, "disable statement, expecting fork or identifier".to_string() ))
                }
                n.attr.insert("value".to_string(), t.value);
                expect_t!(ts,"disable statement",TokenKind::SemiColon);
            }
            TokenKind::Macro => {
                parse_macro(ts,node)?;
                // Allow extra ;
                if t.kind == TokenKind::SemiColon {
                    ts.flush(1);
                } else {
                    ts.rewind(0);
                }
                // Allow declaration after some ifdef/else/... : lazy check, but should be good enough
                match node.child.last().unwrap().attr["name"].as_ref() {
                    "`ifndef" | "`ifdef" | "`elsif" | "`else" => was_decl = true,
                    _ => {}
                }
            }
            TokenKind::SystemTask  => {
                node.child.push(parse_system_task(ts)?);
                expect_t!(ts,"system task call",TokenKind::SemiColon);
            }
            TokenKind::KwSuper | TokenKind::KwThis => {parse_assign_or_call(ts,node,ExprCntxt::Stmt)?;},
            TokenKind::Ident => {
                let name = t.value;
                t = next_t!(ts,true);
                let has_label = t.kind == TokenKind::Colon;
                if has_label {
                    ts.flush(2); // Label
                    was_decl = false;
                    let mut n = AstNode::new(AstNodeKind::Statement);
                    n.attr.insert("label".to_string(),name);
                    parse_class_stmt(ts,&mut n, false, false)?;
                }
                else {
                    if allow_decl {
                        match t.kind {
                            TokenKind::Ident => was_decl=true,
                            TokenKind::Scope => {
                                t = next_t!(ts,true);
                                if t.kind != TokenKind::Ident {
                                    return Err(SvError::syntax(t, "type/package, expecting identifier".to_string()));
                                }
                                t = next_t!(ts,true);
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
            TokenKind::KwAutomatic   |
            TokenKind::KwStatic      |
            TokenKind::KwConst       |
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     if allow_decl => {
                parse_class_members(ts, node)?;
                was_decl = true;
            },
            TokenKind::KwEnd if is_block => {
                ts.flush(0);
                break;
            },
            TokenKind::KwBreak | TokenKind::KwContinue => {
                ts.flush(1);
                let mut n = AstNode::new(AstNodeKind::Branch);
                n.attr.insert("kind".to_string(),t.value);
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
                let mut n = AstNode::new(AstNodeKind::EventCtrl);
                n.attr.insert("kind".to_string(),t.value);
                n.child.push(parse_sensitivity(ts,false)?);
                t = next_t!(ts,true);
                if t.kind != TokenKind::SemiColon {
                    parse_class_stmt_or_block(ts, &mut n)?;
                } else {
                    ts.flush(1);
                    // break;
                }
            }
            TokenKind::Hash => {
                let mut n = parse_delay(ts)?;
                t = next_t!(ts,true);
                if t.kind != TokenKind::SemiColon {
                    ts.rewind(1);
                    parse_class_stmt_or_block(ts, &mut n)?;
                } else {
                    ts.flush(1);
                }
                node.child.push(n);
            }
            TokenKind::KwWait => {
                let mut n = AstNode::new(AstNodeKind::Wait);
                n.attr.insert("kind".to_string(),t.value);
                ts.flush(1); // Consume token
                t = next_t!(ts,false);
                match t.kind {
                    TokenKind::KwFork => {
                        n.attr.insert("kind".to_string()," fork".to_string());
                        expect_t!(ts,"wait",TokenKind::SemiColon);
                    }
                    TokenKind::ParenLeft => {
                        let mut ns = parse_expr(ts,ExprCntxt::Arg,false)?;
                        ts.flush(1); // Consume right parenthesis
                        ns.kind = AstNodeKind::Event;
                        n.child.push(ns);
                        t = next_t!(ts,true);
                        match t.kind {
                            TokenKind::SemiColon => ts.flush(1),
                            _ => parse_class_stmt_or_block(ts, &mut n)?,
                        }
                    }
                    _ => return Err(SvError::syntax(t, "wait statement. Expecting fork or (event_name)".to_string()))
                }
                node.child.push(n);
            }
            TokenKind::KwAssert => {
                let mut n = AstNode::new(AstNodeKind::Assert);
                ts.flush(1); // Consume assert
                // TODO: support for deffered assertion (#0/final)
                t = next_t!(ts,false);
                if t.kind!=TokenKind::ParenLeft {
                    return Err(SvError::syntax(t,"assert statement. Expecting (".to_string()));
                }
                n.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                ts.flush(1); // Consume right parenthesis
                t = next_t!(ts,true);
                match t.kind {
                    //
                    TokenKind::SemiColon => ts.flush(1),
                    // Else part handle after
                    TokenKind::KwElse => {}
                    _ => {
                        ts.rewind(0);
                        parse_class_stmt_or_block(ts, &mut n)?;
                        t = next_t!(ts,true);
                    }
                }
                // Handle optionnal else part
                if t.kind==TokenKind::KwElse {
                    ts.flush(1);
                    let mut node_else = AstNode::new(AstNodeKind::Branch);
                    node_else.attr.insert("kind".to_string(),"else".to_string());
                    parse_class_stmt_or_block(ts, &mut node_else)?;
                    n.child.push(node_else);
                } else {
                    ts.rewind(0);
                }
                node.child.push(n);
            }
            TokenKind::Casting => {
                ts.flush(1); // Consume
                if t.value!="void'".to_string() {
                    return Err(SvError::syntax(t,"casting statement. Expecting void".to_string()));
                }
                t = next_t!(ts,false);
                if t.kind!=TokenKind::ParenLeft {
                    return Err(SvError::syntax(t,"casting statement. Expecting (".to_string()));
                }
                node.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                ts.flush(1); // Consume right parenthesis
                expect_t!(ts,"casting statement",TokenKind::SemiColon);
            }
            TokenKind::KwReturn => {
                let mut n = AstNode::new(AstNodeKind::Return);
                ts.flush(1);
                t = next_t!(ts,true);
                if t.kind!=TokenKind::SemiColon {
                    ts.rewind(1);
                    n.child.push(parse_expr(ts,ExprCntxt::Stmt,false)?);
                    ts.flush(1);
                }
                // println!("Return statement: token = {}\n node={}", t,n);
                node.child.push(n);
                ts.flush(0);
            },
            // Semi-colon : empty statement
            TokenKind::SemiColon => ts.flush(1),
            TokenKind::KwEndFunction | TokenKind::KwEndTask => {ts.rewind(0); break;},
            _ => return Err(SvError::syntax(t, "class statement".to_string()))
        }
        // Stop parsing if not in a block
        if ! is_block {break;}
        allow_decl = was_decl;
    }
    Ok(was_decl)
}

pub fn parse_assign_or_call(ts : &mut TokenStream, node: &mut AstNode, cntxt: ExprCntxt) -> Result<bool, SvError> {
    ts.rewind(0);
    let mut t = next_t!(ts,true);
    let mut n = AstNode::new(AstNodeKind::Assign);
    let mut is_decl = false;
    // Check for preincrement
    if t.kind==TokenKind::OpIncrDecr {
        n.attr.insert("incr_decr".to_string(),t.value);
        n.attr.insert("op".to_string(),"pre".to_string());
        ts.flush(1);
        n.child.push(parse_member_or_call(ts, false)?);
    } else {
        ts.rewind(1);
        let nm = parse_member_or_call(ts, true)?;
        t = next_t!(ts,true);
        is_decl = nm.kind==AstNodeKind::Declaration;
        match t.kind {
            TokenKind::OpEq | TokenKind::OpLTE | TokenKind::OpCompAss if !is_decl => {
                ts.flush(1); // Consume the operand
                n.attr.insert("kind".to_string(),t.value);
                n.child.push(nm);
                n.child.push(parse_expr(ts,cntxt.clone(),false)?);
            }
            TokenKind::OpIncrDecr if !is_decl => {
                ts.flush(1); // Consume the operand
                n.attr.insert("incr_decr".to_string(),t.value);
                n.attr.insert("op".to_string(),"post".to_string());
                n.child.push(nm);
            }
            TokenKind::SemiColon if cntxt==ExprCntxt::Stmt || cntxt==ExprCntxt::StmtList => n = nm,
            _ => return Err(SvError::syntax(t, "class assign/call. Expecting assignment operator or ;".to_string())),
        }
        // Rewind ending token when in a list, otherwise consume it
        match cntxt {
            ExprCntxt::Stmt => {expect_t!(ts,"assign/call",TokenKind::SemiColon);},
            ExprCntxt::Arg  => {expect_t!(ts,"assign/call",TokenKind::ParenRight);},
            ExprCntxt::ArgList  => {
                t = next_t!(ts,true);
                if t.kind != TokenKind::ParenRight && t.kind != TokenKind::Comma {
                    return Err(SvError::syntax(t, "assign/call. Expecting , or )".to_string()))
                }
                ts.rewind(1);
            },
            ExprCntxt::StmtList => {
                t = next_t!(ts,true);
                if t.kind != TokenKind::SemiColon && t.kind != TokenKind::Comma {
                    return Err(SvError::syntax(t, "assign/call. Expecting , or ;".to_string()))
                }
                ts.rewind(1);
            },
            _ => panic!("Invalid context {:?} for parse_assign_or_call",cntxt)
        }
    }
    node.child.push(n);
    Ok(is_decl)
}

/// Parse If/Else if/Else statements
/// Suppose first IF has been consumed
#[allow(unused_variables, unused_mut)]
pub fn parse_class_if_else(ts : &mut TokenStream, node: &mut AstNode, cond: &str) -> Result<(), SvError> {
    let mut node_if = AstNode::new(AstNodeKind::Branch);
    node_if.attr.insert("kind".to_string(),cond.to_string());
    expect_t!(ts,"if statement",TokenKind::ParenLeft);
    let n = parse_expr(ts,ExprCntxt::Arg,false)?;
    ts.flush(1); // Consume right parenthesis
    node_if.child.push(n);
    parse_class_stmt_or_block(ts,&mut node_if)?;
    node.child.push(node_if);

    // Check for else if/else statement
    let mut t;
    loop {
        t = next_t!(ts,true);
        // println!("[parse_if_else] Else Token ? {}", t);
        if t.kind == TokenKind::KwElse {
            ts.flush(0);
            t = next_t!(ts,true);
            // println!("[parse_if_else] If Token ? {}", t);
            if t.kind == TokenKind::KwIf {
                ts.flush(0);
                parse_class_if_else(ts,node,"else if")?;
            } else {
                ts.rewind(0);
                let mut node_else = AstNode::new(AstNodeKind::Branch);
                node_else.attr.insert("kind".to_string(),"else".to_string());
                parse_class_stmt_or_block(ts, &mut node_else)?;
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
    ts.flush(0);
    expect_t!(ts,"for",TokenKind::ParenLeft);
    let mut node_for = AstNode::new(AstNodeKind::LoopFor);
    // Parse init part : end on ;
    let mut node_hdr = AstNode::new(AstNodeKind::Header);
    let mut ns = AstNode::new(AstNodeKind::Declaration);
    let mut t = next_t!(ts,true);
    let mut is_type = true;
    if t.kind == TokenKind::Ident {
        t = next_t!(ts,true);
        is_type = t.kind != TokenKind::Dot && t.kind != TokenKind::SquareLeft ;
    }
    ts.rewind(0);
    if is_type {
        parse_data_type(ts, &mut ns, 2)?;
    } else {
        parse_assign_or_call(ts, &mut ns,ExprCntxt::Stmt)?;
    }

    parse_var_decl_name(ts, &mut ns)?;
    ns.attr.insert("loop".to_string(), "init".to_string());
    node_hdr.child.push(ns);
    ts.flush(0); // clear semi-colon
    // Parse test part : end on ;
    ns = parse_expr(ts,ExprCntxt::Stmt,false)?;
    ns.attr.insert("loop".to_string(), "test".to_string());
    node_hdr.child.push(ns);
    ts.flush(1); // clear semi-colon
    // Parse incr part : end on )
    loop {
        ns = AstNode::new(AstNodeKind::Expr);
        parse_assign_or_call(ts,&mut ns,ExprCntxt::ArgList)?;
        ns.attr.insert("loop".to_string(), "incr".to_string());
        node_hdr.child.push(ns);
        loop_args_break_cont!(ts,"for test arguments",ParenRight);
    }
    ts.flush(0); // Clear parenthesis
    node_for.child.push(node_hdr);
    // Parse content of for loop
    parse_class_stmt_or_block(ts,&mut node_for)?;
    // println!("parse_class_for {}", node_for);
    node.child.push(node_for);
    Ok(())
}


/// Parse case statement
pub fn parse_class_case(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut t = next_t!(ts,false);
    let mut node_c = AstNode::new(AstNodeKind::Case);
    // println!("[parse_class_case] First Token {}", t);
    if t.kind==TokenKind::KwPriority || t.kind==TokenKind::KwUnique || t.kind==TokenKind::KwUnique0 {
        node_c.attr.insert("prio".to_string(),t.value);
        t = next_t!(ts,false);
    }
    if t.kind!=TokenKind::KwCase {
        return Err(SvError::syntax(t,"case statement. Expecting case".to_string()));
    }
    node_c.attr.insert("kind".to_string(),t.value);
    // Parse case expression
    expect_t!(ts,"case",TokenKind::ParenLeft);
    node_c.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
    ts.flush(1); // Consume right parenthesis
    // TODO: test for Match/inside
    t = next_t!(ts,true);
    match t.kind {
        TokenKind::KwUnique   |
        TokenKind::KwUnique0  |
        TokenKind::KwPriority => {
            ts.flush(0);
            node_c.attr.insert("prio".to_string(),t.value);
        }
        _ => ts.rewind(0)
    }
    // Loop on all case entry until endcase
    loop {
        t = next_t!(ts,true);
        // println!("[parse_class_case] case item {}", t);
        let mut node_i =  AstNode::new(AstNodeKind::CaseItem);
        match t.kind {
            TokenKind::OpPlus   |
            TokenKind::OpMinus  |
            TokenKind::Integer  |
            TokenKind::Ident    |
            TokenKind::Str      |
            TokenKind::KwTagged  => {
                ts.rewind(0);
                // Collect case item expression
                let mut s = "".to_string();
                loop {
                    let nt = next_t!(ts,false);
                    match nt.kind {
                        // TODO : be more restrictive, and also handle case like ranges
                        TokenKind::Colon => break,
                        _ => s.push_str(&nt.value),
                    }
                }
                // println!("[parse_class_case] case item value {}", s);
                node_i.attr.insert("item".to_string(),s);
                ts.flush(0); // every character until the colon should be consumed
            }
            TokenKind::KwDefault => {
                let nt = next_t!(ts,true);
                // Check for colon after keyword default
                if nt.kind!=TokenKind::Colon {
                    return Err(SvError::syntax(t,"default case item. Expecting :".to_string()));
                }
                ts.flush(0);
                node_i.attr.insert("item".to_string(),"default".to_string());
            }
            TokenKind::KwEndcase => break,
            // TODO : support tagged keyword for case-matches
            _ => return Err(SvError::syntax(t,"case entry. Expecting identifier/value/endcase".to_string()))
        }
        // Parse statement
        parse_class_stmt_or_block(ts,&mut node_i)?;
        // println!("[parse_class_case] case item node {}", node_i);
        node_c.child.push(node_i);
    }
    ts.flush(0);
    // println!("[parse_class_case] {}", node_c);
    node.child.push(node_c);
    Ok(())
}
