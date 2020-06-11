// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::reporter::{REPORTER, MsgID};
use crate::error::SvError;
use crate::lex::token::{TokenKind};
use crate::lex::token_stream::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;
use crate::ast::class::{parse_class,parse_func,parse_task,parse_class_stmt,parse_assign_or_call};

// TODO
// - when parsing named block, ensure the name is unique

#[allow(dead_code)]
#[derive(PartialEq,Debug)]
pub enum ModuleCntxt {
    Top, Generate, Block, ForStmt, IfStmt
}


/// This function should be called after a keyword module/macromodule
pub fn parse_module_body(ts : &mut TokenStream, node : &mut AstNode, cntxt : ModuleCntxt) -> Result<(), SvError> {
    loop {
        let t = ts.next_t(true)?;
        // println!("[parse_module_body] Token = {}", t);
        match t.kind {
            // Import statement
            TokenKind::KwImport => parse_import(ts,node)?,
            // Param/local param declaration
            TokenKind::KwParam | TokenKind::KwLParam | TokenKind::KwSpecParam => {
                ts.rewind(1); // put back the token so that it can be read by the parse param function
                // potential list of param (the parse function extract only one at a time)
                loop {
                    node.child.push(parse_param_decl(ts,true)?);
                    loop_args_break_cont!(ts,"parameter declaration",SemiColon);
                }
            }
            // Port
            TokenKind::KwInput | TokenKind::KwOutput | TokenKind::KwInout | TokenKind::KwRef => {
                ts.rewind(1); // put back the token so that it can be read by the parse param function
                node.child.push(parse_port_decl(ts,false,ExprCntxt::StmtList)?);
            }
            // Nettype
            TokenKind::KwNetType |
            TokenKind::KwSupply  =>  parse_signal_decl_list(ts,node)?,
            // Basetype
            TokenKind::KwConst       |
            TokenKind::KwReg         |
            TokenKind::KwVar         |
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     => parse_signal_decl_list(ts,node)?,
            TokenKind::KwInterconnect => {
                let mut node_d = AstNode::new(AstNodeKind::Declaration, t.pos);
                node.attr.insert("type".to_owned(), "interconnect".to_string());
                ts.flush(1);
                let mut nt = ts.next_t(true)?;
                if nt.kind == TokenKind::KwSigning {
                    node_d.attr.insert("signing".to_owned(), nt.value);
                    ts.flush(1);
                    nt = ts.next_t(true)?;
                }
                if nt.kind == TokenKind::SquareLeft {
                    ts.rewind(1);
                    parse_opt_slice(ts,&mut node_d,true,false)?;
                }
                parse_var_decl_name(ts, &mut node_d,ExprCntxt::StmtList,false,false)?;
                // allow list of interconnect
                loop {
                    loop_args_break_cont!(ts,"interconnect declaration",SemiColon);
                    let mut node_l = AstNode::new(AstNodeKind::Declaration, ts.get_pos());
                    parse_var_decl_name(ts, &mut node_l,ExprCntxt::StmtList,false,false)?;
                    node_d.child.push(node_l);
                }
                node.child.push(node_d);
            }
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
            TokenKind::KwTypedef => parse_typedef(ts,node)?,
            TokenKind::KwNettype => parse_nettype(ts,node)?,
            TokenKind::TypeGenvar => {
                ts.flush_rd();
                loop {
                    let nt = ts.next_t(false)?;
                    match nt.kind {
                        TokenKind::Ident => {
                            let mut n = AstNode::new(AstNodeKind::Declaration, nt.pos);
                            n.attr.insert("type".to_owned(), "genvar".to_owned());
                            n.attr.insert("name".to_owned(),t.value.clone());
                            node.child.push(n);
                            loop_args_break_cont!(ts,"genvar declaration",SemiColon);
                        }
                        _ =>  return Err(SvError::syntax(t,"genvar. Expecting identifier")),
                    }
                }
            }
            // Class (interface class)
            TokenKind::KwIntf |
            TokenKind::KwVirtual |
            TokenKind::KwClass => node.child.push(parse_class(ts)?),
            // Primite
            TokenKind::KwOr       |
            TokenKind::KwPrimCmos |
            TokenKind::KwPrimMos  |
            TokenKind::KwPrimEn   |
            TokenKind::KwPrimIn   |
            TokenKind::KwPrimOut  |
            TokenKind::KwPrimTran |
            TokenKind::KwPrimTranif => node.child.push(parse_primitive(ts)?),
            // Identifier -> lookahead to detect if it is a signal declaration or an instantiation
            TokenKind::Ident => {
                let mut nt = ts.next_t(true)?;
                // println!("[Module body] Ident followed by {}", nt.kind);
                match nt.kind {
                    // Scope -> this is a type definition
                    TokenKind::Scope => parse_signal_decl_list(ts,node)?,
                    // Colon : this was a label
                    TokenKind::Colon => {
                        ts.flush(2);
                        let mut n = AstNode::new(AstNodeKind::Statement, t.pos);
                        n.attr.insert("label".to_owned(),t.value);
                        nt = ts.next_t(true)?;
                        match nt.kind {
                            TokenKind::KwAssert => parse_assert(ts,node)?,
                            u => {
                                println!("[parse_module_body] Labeled stateent {} not supported", u);
                                // Expect assertion: not support for the moment ...
                                ts.skip_until(TokenKind::SemiColon)?;
                            }
                        }

                    }
                    // Identifier : could be a signal declaration or a module/interface instantiation
                    TokenKind::Ident => {
                        nt = ts.next_t(true)?;
                        // println!("[Module body] (Ident Ident) followed by {}", nt.kind);
                        match nt.kind {
                            // Semi colon, comma or equal indicate signal declaration
                            TokenKind::SemiColon |
                            TokenKind::OpEq      |
                            TokenKind::Comma     =>  parse_signal_decl_list(ts,node)?,
                            // Slice -> can be either an unpacked array declaration or an array of instance ...
                            TokenKind::SquareLeft =>  {
                                ts.peek_until(TokenKind::SquareRight)?;
                                nt = ts.next_t(true)?;
                                if nt.kind == TokenKind::ParenLeft {
                                    node.child.push(parse_instance(ts)?);
                                } else {
                                    parse_signal_decl_list(ts,node)?;
                                }
                            }
                            // Open parenthesis -> instance
                            TokenKind::ParenLeft => {
                                let node_inst = parse_instance(ts)?;
                                node.child.push(node_inst);
                            }
                            _ => return Err(SvError::syntax(t, "signal declaration/instance"))
                        }
                    }
                    // Open bracket indicate a packet dimension, i.e. a signal declaration
                    TokenKind::SquareLeft =>  parse_signal_decl_list(ts,node)?,
                    // Dash : Can be a parametiyed class of a parameterized interface
                    TokenKind::Hash => {
                        let node_inst = parse_instance(ts)?;
                        node.child.push(node_inst);
                    }
                    // Untreated token are forbidden
                    _ => return Err(SvError::syntax(t, "signal declaration. Expecting type or instance"))
                }
            }
            TokenKind::KwBind => parse_bind(ts,node)?,
            //
            TokenKind::KwAssign | TokenKind::KwDefparam => {
                ts.rewind(1);
                node.child.push(parse_assign_c(ts)?);
            }
            // Always keyword
            TokenKind::KwAlways  |
            TokenKind::KwAlwaysC |
            TokenKind::KwAlwaysF |
            TokenKind::KwAlwaysL  => parse_always(ts, node)?,
            TokenKind::KwInitial  => parse_initial(ts, node)?,
            TokenKind::KwFinal    => parse_initial(ts, node)?,
            TokenKind::KwFunction => parse_func(ts, node, false, false)?,
            TokenKind::KwTask     => parse_task(ts, node)?,
            //
            TokenKind::KwTimeunit | TokenKind::KwTimeprec => parse_timescale(ts,node)?,
            //
            TokenKind::KwGenerate if cntxt==ModuleCntxt::Top => {
                ts.flush_rd();
                parse_module_body(ts,node,ModuleCntxt::Generate)?;
            }
            TokenKind::KwFor  => parse_for(ts,node,true)?,
            TokenKind::KwIf   => parse_if_else(ts,node, true)?,
            TokenKind::KwBegin => {
                ts.flush_rd();
                let mut n = AstNode::new(AstNodeKind::Block, t.pos);
                parse_label(ts,&mut n,"block".to_owned())?;
                parse_module_body(ts,&mut n, ModuleCntxt::Block)?;
                if n.attr["block"]!="" {
                    check_label(ts, &n.attr["block"])?;
                }
            }
            TokenKind::KwAssert     => parse_assert(ts,node)?,
            TokenKind::KwCovergroup => parse_covergroup(ts,node)?,
            TokenKind::KwProperty   => parse_sva_property(ts,node)?,
            TokenKind::SemiColon    => {ts.flush(1);}, // TODO: generate a warning
            // End of loop depends on context
            TokenKind::KwEnd         if cntxt == ModuleCntxt::Block    => {ts.flush(1); break},
            TokenKind::KwEndGenerate if cntxt == ModuleCntxt::Generate => {ts.flush(1); break},
            TokenKind::KwEndModule   if cntxt == ModuleCntxt::Top      => {ts.flush(1); break},
            TokenKind::Macro => parse_macro(ts,node)?,
            TokenKind::CompDir => parse_macro(ts,node)?,
            TokenKind::SystemTask => {
                // allow only elaboration system task: fatal/error/warning/info
                // But still parse the whole systemTask and try to continue
                let node_c = parse_system_task(ts)?;
                match t.value.as_ref() {
                    "$error" | "$fatal" | "$warning" | "$info" => node.child.push(node_c),
                    _ => rpt_t!(MsgID::ErrInvalid,&t,"module body")
                }

            }
            // Any un-treated token is an error
            _ => {
                // println!("{}", node);
                return Err(SvError::syntax(t, "module body"))
            }
        }

        if cntxt == ModuleCntxt::ForStmt || cntxt == ModuleCntxt::IfStmt {
            break;
        }
    }
    // ts.flush_rd();
    Ok(())
    // Err(SvError {kind:SvErrorKind::NotSupported, pos: t.pos, txt: "Module body".to_owned()})
}

// Parse a continous assignment / defparam
pub fn parse_assign_c(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut t = ts.next_t(false)?; // Get first word: expect assign or defparam
    let mut node = AstNode::new(AstNodeKind::Assign, t.pos);
    node.attr.insert("kind".to_owned(),t.value);
    t = ts.next_t(true)?;
    if t.kind==TokenKind::ParenLeft {
        ts.flush(1);
        t = expect_t!(ts,"drive strength",TokenKind::KwDrive);
        let drive_is_0 = t.value.ends_with('0');
        if drive_is_0 {
            node.attr.insert("strength0".to_owned(),t.value);
        } else {
            node.attr.insert("strength1".to_owned(),t.value);
        }
        expect_t!(ts,"drive strength",TokenKind::Comma);
        t = expect_t!(ts,"drive strength",TokenKind::KwDrive);
        if drive_is_0 {
            if t.value.ends_with('0') {
                return Err(SvError::syntax(t, "drive strength. Expecting drive strength 1"))
            }
            node.attr.insert("strength1".to_owned(),t.value);
        } else {
            if t.value.ends_with('1') {
                return Err(SvError::syntax(t, "drive strength. Expecting drive strength 0"))
            }
            node.attr.insert("strength0".to_owned(),t.value);
        }
        expect_t!(ts,"drive strength",TokenKind::ParenRight);
    }
    // Optional delay
    if t.kind == TokenKind::Hash {
        node.child.push(parse_delay(ts)?);
        t = ts.next_t(true)?; // Get first word: expect assign or defparam
    }
    match t.kind {
        // Concatenation operator
        TokenKind::CurlyLeft => {
            ts.flush(1);
            let mut nm = AstNode::new(AstNodeKind::Concat,t.pos);
            loop {
                nm.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                loop_args_break_cont!(ts,"concatenation",CurlyRight);
            }
            node.child.push(nm);
        }
        _ => {
            ts.rewind(0);
            node.child.push(parse_ident_hier(ts)?);
        }
    }
    expect_t!(ts,"continuous assignment",TokenKind::OpEq);
    node.child.push(parse_expr(ts,ExprCntxt::Stmt,false)?);
    // Consume last token (the semicolon)
    ts.flush(1); // Consume last token
    // println!("[parse_assign_c] {}", node);
    Ok(node)
}

pub fn parse_assign_bnb(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut node = AstNode::new(AstNodeKind::Assign, ts.get_pos());
    // Parse LHS
    node.child.push(parse_ident_hier(ts)?);
    // Expect <=, = or composed asisgnement
    let mut t = expect_t!(ts,"assign",TokenKind::OpLTE,TokenKind::OpEq,TokenKind::OpCompAss);
    node.attr.insert("kind".to_owned(),t.value);
    // Optional delay
    if t.kind==TokenKind::OpLTE {
        t = ts.next_t(true)?;
        if t.kind==TokenKind::Hash {
            node.child.push(parse_delay(ts)?);
        } else {
            ts.rewind(1);
        }
    }
    //
    node.child.push(parse_expr(ts,ExprCntxt::Stmt,false)?);
    ts.flush_rd(); // consume the ;
    // println!("[parse_assign_c] {}", node);
    Ok(node)
}

// Parse an instance
#[allow(unused_assignments)]
pub fn parse_instance(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    // First token is the module type
    ts.rewind(0);
    let mut t = ts.next_t(false)?;
    let mut node = AstNode::new(AstNodeKind::Instances, t.pos);
    node.attr.insert("type".to_owned(), t.value);
    t = ts.next_t(true)?;
    parse_opt_params!(ts,node,t);
    ts.rewind(0);
    // Instances can be a list
    loop {
        t = expect_t!(ts,"instance name",TokenKind::Ident);
        let mut node_i = AstNode::new(AstNodeKind::Instance, t.pos);
        node_i.attr.insert("name".to_owned(), t.value);
        // Test for array of instance
        parse_opt_slice(ts, &mut node_i, true,false)?;
        parse_port_connection(ts,&mut node_i,false)?;
        node.child.push(node_i);
        loop_args_break_cont!(ts,"param declaration",SemiColon);
    }
    // println!("[Instance] {}",node);
    Ok(node)
}

// Parse an instance
#[allow(unused_assignments)]
pub fn parse_bind(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush(1); // consume the bind keyword
    let mut n = AstNode::new(AstNodeKind::Bind, ts.get_pos());
    n.child.push(parse_ident_hier(ts)?); // TODO: handle variant of binding style
    n.child.push(parse_instance(ts)?);
    node.child.push(n);
    Ok(())
}

/// Parse an always block
pub fn parse_always(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let t0 = ts.next_t(false)?;
    let mut n = AstNode::new(AstNodeKind::Process, t0.pos);
    n.attr.insert("kind".to_owned(),t0.value.clone());
    let t = ts.next_t(true)?;
    // println!("[parse_always] Node {}\nFirst Token {}",n, t);
    if t.kind == TokenKind::At {
        ts.flush_rd();
        match t0.kind {
            TokenKind::KwAlwaysL |
            TokenKind::KwAlwaysC => return Err(SvError::syntax(t, &format!("in {} process sensitivity list",t0.value))),
            // Parse the sensitivity list
            _ => {
                let node_s = parse_sensitivity(ts,true)?;
                n.child.push(node_s);
            }
        }
        // println!("[parse_always] Token post sensitivity list: {}", t);
    } else {
        ts.rewind(0);
    }
    //
    parse_class_stmt(ts, &mut n, false, false, false, true)?;
    // parse_class_stmt_or_block(ts,&mut n)?;
    node.child.push(n);
    Ok(())
}

/// Parse sensitivity list
/// Suppose Token @ has been consumed
/// An empty sensitivity node corresponds to @(*) or @*
pub fn parse_sensitivity(ts : &mut TokenStream, is_process: bool) -> Result<AstNode, SvError> {
    // Check next character: open parenthesis or *
    let mut t = ts.next_t(true)?;
    let mut node = AstNode::new(AstNodeKind::Sensitivity, t.pos);
    // println!("[parse_sensitivity] First Token {}", t);
    match t.kind {
        TokenKind::OpStar   |
        TokenKind::SensiAll => {ts.flush(1); return Ok(node);}
        TokenKind::Ident if !is_process => {
            node.child.push(parse_ident_hier(ts)?);
            // node.attr.insert("clk_event".to_owned(), t.value);
            return Ok(node);
        }
        TokenKind::ParenLeft => {
            ts.flush(1);
            t = ts.next_t(true)?;
            if t.kind == TokenKind::OpStar {
                ts.flush(1);
                expect_t!(ts,"sensitivity list",TokenKind::ParenRight);
                return Ok(node);
            }
        }
        _ => return Err(SvError::syntax(t, "sensitivity list. Expecting *, (*) or ( event list )"))
    }
    // Parse event list
    loop {
        // println!("[parse_sensitivity] First Token of event expression {}", t);
        // Capture optionnal edge
        let mut n = AstNode::new(AstNodeKind::Event, t.pos);
        if t.kind == TokenKind::KwEdge {
            n.attr.insert("edge".to_owned(),t.value );
            ts.flush_rd(); // consume keyword
        } else {
            ts.rewind(1);
        }
        // Capture event name
        n.child.push(parse_expr(ts,ExprCntxt::Sensitivity,false)?);
        // Check for iff
        t = ts.next_t(false)?;
        if t.kind==TokenKind::KwIff {
            n.child.push(parse_expr(ts,ExprCntxt::Sensitivity,false)?);
            // n.child.push(parse_ident_hier(ts)?);
            t = ts.next_t(false)?;
        }
        node.child.push(n);
        // Expecting closing parenthesis, comma, or keyword "or"
        // println!("[parse_sensitivity] event expression separator {}", t);
        match t.kind {
            TokenKind::ParenRight => break,
            TokenKind::KwOr  |
            TokenKind::Comma => {},
            _ => return Err(SvError::syntax(t, "sensitivity list. Expecting comma, keyword 'or' or )"))
        }
        t = ts.next_t(true)?;
    }
    // println!("[parse_sensitivity] {}", node);
    Ok(node)
}

/// Parse an always block
pub fn parse_initial(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(1);
    let t = ts.next_t(false)?;
    let mut n = AstNode::new(AstNodeKind::Process, t.pos);
    n.attr.insert("kind".to_owned(),t.value);
    parse_class_stmt(ts, &mut n, false, false, false, true)?;
    node.child.push(n);
    Ok(())
}


/// Parse any statement in a process
pub fn parse_stmt(ts : &mut TokenStream, node: &mut AstNode, is_block: bool) -> Result<(), SvError> {
    ts.rewind(0);
    loop {
        let mut t = ts.next_t(true)?;
        // println!("[parse_stmt] Token = {}", t);
        // ts.display_status("");
        match t.kind {
            TokenKind::KwIf   => parse_if_else(ts,node, false)?,
            TokenKind::KwCase => parse_case(ts,node)?,
            TokenKind::KwPriority |
            TokenKind::KwUnique   |
            TokenKind::KwUnique0   => {
                t = ts.next_t(true)?;
                match t.kind {
                    TokenKind::KwIf   => parse_if_else(ts,node, false)?,
                    TokenKind::KwCase => parse_case(ts,node)?,
                    _ => return Err(SvError::syntax(t,"priority statement. Expecting case/if"))
                }
            }
            TokenKind::KwFor  => parse_for(ts,node,false)?,
            TokenKind::Ident  => {
                t = ts.next_t(true)?;
                // Two cases: task call or assignment
                match t.kind {
                    TokenKind::ParenLeft => {
                        let mut n = AstNode::new(AstNodeKind::MethodCall, t.pos);
                        n.attr.insert("name".to_owned(),t.value);
                        ts.flush(1); // consume identifier
                        parse_func_call(ts, &mut n, false)?;
                        expect_t!(ts,"type",TokenKind::SemiColon);
                        node.child.push(n);
                    }
                    _ => {
                        ts.rewind(0);
                        node.child.push(parse_assign_bnb(ts)?);
                    }
                }
            },
            TokenKind::KwEnd if is_block => {
                ts.flush_rd();
                break;
            },
            TokenKind::KwBegin => {
                ts.flush_rd();
                let mut n = AstNode::new(AstNodeKind::Block, t.pos);
                parse_label(ts,&mut n,"block".to_owned())?;
                parse_stmt(ts,&mut n, true)?;
                if n.attr["block"]!="" {
                    check_label(ts, &n.attr["block"])?;
                }
                node.child.push(n);
            }
            _ => return Err(SvError::syntax(t, "statement"))
        }
        // Stop parsing if not in a block
        if ! is_block {break;}
    }
    Ok(())
}

/// Parse If/Else if/Else statements
/// Suppose first IF has been consumed
pub fn parse_if_else(ts : &mut TokenStream, node: &mut AstNode, is_gen: bool) -> Result<(), SvError> {
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
    node_if.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
    ts.flush(1); // Consume last token
    // Check for begin
    let mut is_block = false;
    let mut t = ts.next_t(true)?;
    if t.kind == TokenKind::KwBegin {
        is_block = true;
        ts.flush(1);
        parse_label(ts,&mut node_if,"block".to_owned())?;
    } else {
        ts.rewind(0);
    }
    // Loop on statement, if/else / case
    if is_gen {
        parse_module_body(ts,&mut node_if, if is_block {ModuleCntxt::Block} else {ModuleCntxt::IfStmt})?;
        node_if.attr.get("block").map(|n| check_label(ts,n)).unwrap_or(Ok(()))?;
    } else {
        parse_stmt(ts,&mut node_if, is_block)?;
    }
    node.child.push(node_if);

    // Check for else if/else statement
    loop {
        t = ts.next_t(true)?;
        // println!("[parse_if_else] Else Token ? {}", t);
        if t.kind == TokenKind::KwElse {
            t = ts.next_t(true)?;
            // println!("[parse_if_else] If Token ? {}", t);
            if t.kind == TokenKind::KwIf {
                parse_if_else(ts,node, is_gen)?;
            } else {
                ts.flush(1); // Consume else
                let mut node_else = AstNode::new(AstNodeKind::Branch, t.pos);
                node_else.attr.insert("kind".to_owned(),"else".to_owned());
                is_block = t.kind == TokenKind::KwBegin;
                // println!("[parse_if_else] Else token : is_block {}, is_gen {}", is_block, is_gen);
                if is_block {
                    ts.flush(1);
                    parse_label(ts,&mut node_else,"block".to_owned())?;
                }
                if is_gen {
                    ts.rewind(0);
                    parse_module_body(ts,&mut node_else, if is_block {ModuleCntxt::Block} else {ModuleCntxt::IfStmt})?;
                    if is_block {
                        node_else.attr.get("block").map(|n| check_label(ts,n)).unwrap_or(Ok(()))?;
                    }
                } else {
                    parse_stmt(ts,&mut node_else, is_block)?;
                }
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

pub fn parse_for(ts : &mut TokenStream, node: &mut AstNode, is_generate: bool) -> Result<(), SvError> {
    ts.flush_rd();
    let mut t = ts.next_t(false)?;
    if t.kind!=TokenKind::ParenLeft {
        return Err(SvError::syntax(t,"for loop. Expecting ("));
    }
    let mut node_for = AstNode::new(AstNodeKind::LoopFor, t.pos);
    // Parse init part : end on ;
    let mut node_hdr = AstNode::new(AstNodeKind::Header, t.pos);
    let mut ns = AstNode::new(AstNodeKind::Declaration, t.pos);
    parse_data_type(ts, &mut ns, 4)?;
    parse_var_decl_name(ts, &mut ns,ExprCntxt::StmtList,true,false)?;
    ns.attr.insert("loop".to_owned(), "init".to_owned());
    node_hdr.child.push(ns);
    ts.flush(1); // clear semi-colon
    // Parse test part : end on ;
    ns = parse_expr(ts,ExprCntxt::Stmt,false)?;
    ns.attr.insert("loop".to_owned(), "test".to_owned());
    node_hdr.child.push(ns);
    ts.flush(1); // clear semi-colon
    // Parse incr part : end on )
    loop {
        ns = AstNode::new(AstNodeKind::Expr, ts.get_pos());
        parse_assign_or_call(ts,&mut ns,ExprCntxt::ArgList)?;
        ns.attr.insert("loop".to_owned(), "incr".to_owned());
        node_hdr.child.push(ns);
        loop_args_break_cont!(ts,"for test arguments",ParenRight);
    }
    ts.flush_rd(); // Clear parenthesis
    node_for.child.push(node_hdr);
    // Check for begin
    let mut cntxt_body = ModuleCntxt::ForStmt;
    t = ts.next_t(true)?;
    let is_block = t.kind == TokenKind::KwBegin;
    if is_block {
        ts.flush(1); // Consume begin keyword
        cntxt_body = ModuleCntxt::Block;
        parse_label(ts,&mut node_for,"block".to_owned())?;
    }
    ts.rewind(0);
    // Parse content of for loop as if inside a module body
    if is_generate {
        parse_module_body(ts,&mut node_for,cntxt_body)?;
    } else {
        parse_stmt(ts,&mut node_for, is_block)?;
    }
    // println!("{}", node_for);
    node.child.push(node_for);
    Ok(())
}


pub fn parse_timescale(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut t = ts.next_t(false)?;
    let mut node_ts = AstNode::new(AstNodeKind::Timescale, t.pos);
    let allow_timeprec = t.kind==TokenKind::KwTimeunit;
    let mut time = parse_time(ts)?;
    node_ts.attr.insert(t.value, time);
    // Check if followed
    t = ts.next_t(false)?;
    match t.kind {
        TokenKind::SemiColon => {}
        TokenKind::OpDiv if allow_timeprec => {
            time = parse_time(ts)?;
            node_ts.attr.insert("timeprecision".to_owned(), time);
            t = ts.next_t(false)?;
            if t.kind != TokenKind::SemiColon {
                return Err(SvError::syntax(t,"timescale. Expecting ;"));
            }
        }
        _ => return Err(SvError::syntax(t,"timescale. Expecting ; or /"))
    }
    node.child.push(node_ts);
    Ok(())
}


/// Parse primitive
pub fn parse_primitive(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    // Capture primitive type: allows to know expected arguments
    let mut t = ts.next_t(false)?;
    let mut node = AstNode::new(AstNodeKind::Primitive, t.pos);
    // let nb_port;
    match t.kind {
        TokenKind::KwPrimCmos   => {} //{nb_port = 4;}
        TokenKind::KwPrimMos    => {} //{nb_port = 3;}
        TokenKind::KwPrimEn     => {} //{nb_port = 3;}
        TokenKind::KwPrimIn     => {} //{nb_port = 3;}
        TokenKind::KwPrimOut    => {} //{nb_port = 3;}
        TokenKind::KwPrimTran   => {} //{nb_port = 2;}
        TokenKind::KwPrimTranif => {} //{nb_port = 3;}
        _ => return Err(SvError::syntax(t, "primitive. Expecting primitive keyword"))
    }
    node.attr.insert("type".to_owned(), t.value);
    t = ts.next_t(true)?;
    // TODO: Hanndle optionnal strength
    // TODO: Hanndle optionnal delay
    // Optionnal identifier
    if t.kind==TokenKind::Ident {
        node.attr.insert("name".to_owned(), t.value);
        ts.flush(1); // consume the identifier
    }
    parse_opt_slice(ts, &mut node, true, false)?;
    expect_t!(ts,"primitive",TokenKind::ParenLeft);
    loop {
        let mut node_p = AstNode::new(AstNodeKind::Port, ts.get_pos());
        node_p.child.push(parse_expr(ts,ExprCntxt::ArgList,false)?);
        node.child.push(node_p);
        loop_args_break_cont!(ts,"port connection",ParenRight);
    }
    // TODO: allow list of instances
    expect_t!(ts,"primitive",TokenKind::SemiColon);
    Ok(node)
}