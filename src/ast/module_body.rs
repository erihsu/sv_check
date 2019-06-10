use crate::error::{SvErrorKind, SvError, };
use crate::token::{TokenKind};
use crate::tokenizer::TokenStream;
use crate::ast::astnode::*;
use crate::ast::common::*;

// TODO
// - when parsing named block, ensure the name is unique

#[allow(dead_code)]
#[derive(PartialEq,Debug)]
pub enum ModuleCntxt {
    Top, Generate, ForBlock, IfBlock, ForStmt, IfStmt
}


/// This function should be called after a keyword module/macromodule
pub fn parse_module_body(ts : &mut TokenStream, node : &mut AstNode, cntxt : ModuleCntxt) -> Result<(), SvError> {
    loop {
        let t = next_t!(ts,true);
        // println!("[parse_module_body] Token = {}", t);
        match t.kind {
            // Import statement
            TokenKind::KwImport => parse_import(ts,node)?,
            // Param/local param declaration
            TokenKind::KwParam | TokenKind::KwLParam => {
                ts.rewind(1); // put back the token so that it can be read by the parse param function
                // potential list of param (the parse function extract only one at a time)
                loop {
                    let node_param = parse_param_decl(ts,true)?;
                    node.child.push(node_param);
                    let nt = next_t!(ts,false);
                    match nt.kind {
                        TokenKind::Comma => {}, // Comma indicate a list -> continue
                        TokenKind::SemiColon => {break;}, // Semi colon indicate end of statement, stop the loop
                        _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Unexpected {} ({:?}) in param declaration, expecting , or ;",t.value, t.kind)))
                    }
                }
            }
            // Nettype
            TokenKind::KwNetType |
            TokenKind::KwSupply  =>  parse_signal_decl_list(ts,node)?,
            // Basetype
            TokenKind::KwReg         |
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     => parse_signal_decl_list(ts,node)?,
            TokenKind::KwEnum        => {
                let mut node_e = parse_enum(ts)?;
                let s = parse_ident_list(ts)?;
                node_e.attr.insert("name".to_string(),s);
                node.child.push(node_e);
            }
            TokenKind::KwTypedef => parse_typedef(ts,node)?,
            TokenKind::TypeGenvar => {
                ts.flush(0);
                let mut s = "".to_string();
                loop {
                    let mut nt = next_t!(ts,false);
                    if nt.kind!=TokenKind::Ident {
                        return Err(SvError::new(SvErrorKind::Syntax, nt.pos,
                                format!("Unexpected {} ({:?}) after genvar, expecting identifier",nt.value, nt.kind)));
                    }
                    s.push_str(&nt.value);
                    nt = next_t!(ts,false);
                    match nt.kind {
                        TokenKind::Comma => s.push_str(", "),
                        TokenKind::SemiColon => break,
                        _ => return Err(SvError::new(SvErrorKind::Syntax, nt.pos,
                                format!("Unexpected {} ({:?}) in genvar declaration, expecting , or ;",nt.value, nt.kind)))
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
                    TokenKind::Scope => parse_signal_decl_list(ts,node)?,
                    // Identifier : could be a signal declaration or a module/interface instantiation
                    TokenKind::Ident => {
                        let nnt = next_t!(ts,true);
                        // println!("[Module body] (Ident Ident) followed by {}", nnt.kind);
                        match nnt.kind {
                            // Opening parenthesis indicates
                            // Semi colon or comma indicate signal declaration
                            TokenKind::SemiColon |
                            TokenKind::Comma     =>  parse_signal_decl_list(ts,node)?,
                            // Range -> can be either an unpacked array declaration or an array of instance ...
                            // TODO: handle case of array of instances
                            TokenKind::SquareLeft =>  {
                                parse_signal_decl_list(ts,node)?;
                            }
                            // Open parenthesis -> instance
                            TokenKind::ParenLeft => {
                                let node_inst = parse_instance(ts)?;
                                node.child.push(node_inst);
                            }
                            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                    format!("Unexpected {} ({:?}) in signal declaration or instance",nnt.value, nnt.kind)))
                        }
                    }
                    // Dash is a clear indicator of an instance -> TODO
                    TokenKind::Hash => {
                        let node_inst = parse_instance(ts)?;
                        node.child.push(node_inst);
                    }
                    // Untreated token are forbidden
                    _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Unexpected '{} {}' in signal declaration, expecting type or instance",t.value, nt.value)))
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
            TokenKind::KwGenerate if cntxt==ModuleCntxt::Top => parse_module_body(ts,node,ModuleCntxt::Generate)?,
            TokenKind::KwFor  => parse_for(ts,node)?,
            TokenKind::KwIf   => {
                ts.flush(0);
                parse_if_else(ts,node, "if".to_string(), true)?;
            }
            // End of loop depends on context
            TokenKind::KwEnd         if cntxt == ModuleCntxt::ForBlock => break,
            TokenKind::KwEnd         if cntxt == ModuleCntxt::IfBlock  => break,
            TokenKind::KwEndGenerate if cntxt == ModuleCntxt::Generate => break,
            TokenKind::KwEndModule   if cntxt == ModuleCntxt::Top      => break,
            // Any un-treated token is an error
            _ => {
                // println!("{}", node);
                return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Unexpected {} ({:?}) in module body",t.value, t.kind)))
            }
        }

        if cntxt == ModuleCntxt::ForStmt || cntxt == ModuleCntxt::IfStmt {
            break;
        }
    }
    ts.flush(0);
    Ok(())
    // Err(SvError {kind:SvErrorKind::NotSupported, pos: t.pos, txt: "Module body".to_string()})
}

// Parse a continous assignment.
// Suppose assign keyword was already consumed
pub fn parse_assign_c(ts : &mut TokenStream) -> Result<AstNode, SvError> {

    let mut t = next_t!(ts,false);
    let mut node = AstNode::new(AstNodeKind::AssignC);
    let mut s;
    // println!("[parse_assign_c] First Token = {}", t);
    loop {
        match t.kind {
            TokenKind::Ident => {
                s = t.value;
                break;
            }
            // TODO: support drive/delay
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Unexpected {} ({:?}) in assign LHS.",t.value, t.kind)))
        }
    }
    //
    t = next_t!(ts,false);
    // Check for hierarchical access
    while t.kind == TokenKind::Dot {
        s.push('.');
        t = next_t!(ts,false);
        if t.kind!=TokenKind::Ident {
            return Err(SvError::syntax(t, "after dot in assign, expecting identifier".to_string()));
        }
        s.push_str(&t.value);
        t = next_t!(ts,false);
    }
    // Check for optional range
    if t.kind==TokenKind::SquareLeft {
        let r = parse_range(ts)?;
        s.push_str(&r);
        t = next_t!(ts,false);
    }
    node.attr.insert("lhs".to_string(), s);
    if t.kind!=TokenKind::OpEq {
        return Err(SvError::syntax(t, "in assign, expecting =".to_string()));
    }
    let s = parse_expr(ts,ExprCntxt::Stmt)?;
    ts.flush(0); // Parse expression let the last character in the buffer -> this was a ;
    node.attr.insert("rhs".to_string(), s);
    // println!("[parse_assign_c] {}", node);
    Ok(node)
}

pub fn parse_assign_bnb(ts : &mut TokenStream) -> Result<AstNode, SvError> {

    let mut t = next_t!(ts,false);
    let mut node;
    let mut s;
    loop {
        match t.kind {
            TokenKind::Ident => {
                s = t.value;
                break;
            }
            // TODO: support drive/delay
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Unexpected {} ({:?}) in assign LHS.",t.value, t.kind)))
        }
    }
    //
    t = next_t!(ts,false);
    // Check for hierarchical access
    while t.kind == TokenKind::Dot {
        s.push('.');
        t = next_t!(ts,false);
        if t.kind!=TokenKind::Ident {
            return Err(SvError::syntax(t, "after dot in assign, expecting identifier".to_string()));
        }
        s.push_str(&t.value);
        t = next_t!(ts,false);
    }
    // Check for optional range
    if t.kind==TokenKind::SquareLeft {
        let r = parse_range(ts)?;
        s.push_str(r.as_ref());
        t = next_t!(ts,false);
    }

    match t.kind {
        TokenKind::OpEq  => node = AstNode::new(AstNodeKind::AssignB),
        TokenKind::OpLTE => node = AstNode::new(AstNodeKind::AssignNB),
        _ => return Err(SvError::syntax(t, "in assign, expecting = or <=".to_string()))
    }
    node.attr.insert("lhs".to_string(), s);
    s = parse_expr(ts,ExprCntxt::Stmt)?;
    ts.flush(0); // Parse expression let the last character in the buffer -> this was a ;
    // println!("[parse_assign_bnb] {}", s.clone());
    node.attr.insert("rhs".to_string(), s);
    Ok(node)
}

// Parse an instance
pub fn parse_instance(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut node;
    // First token is the module type
    ts.rewind(0);
    // ts.display_status();
    let mut t = next_t!(ts,false);
    node = AstNode::new(AstNodeKind::Instance(t.value));
    t = next_t!(ts,true);
    if t.kind==TokenKind::Hash {
        ts.flush(0); // Cons ume the hash token
        parse_port_connection(ts,&mut node,true)?;
    } else {
        ts.rewind(0);
    }
    // Instances can be a list
    loop {
        t = next_t!(ts,false);
        if t.kind!=TokenKind::Ident {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting instance name, got {} {:?}",t.value, t.kind)));
        }
        // TODO: maybe differentiate instance type and instance name ?
        let mut node_i = AstNode::new(AstNodeKind::Instance(t.value));
        t = next_t!(ts,true);
        // TODO: Test for array of instance
        if t.kind==TokenKind::SquareLeft {
            unimplemented!("Instance array are not supported yet");
        }
        ts.rewind(0);;
        parse_port_connection(ts,&mut node_i,false)?;
        t = next_t!(ts,false);
        match t.kind {
            TokenKind::Comma => node.child.push(node_i),
            TokenKind::SemiColon => {
                node.child.push(node_i);
                break;
            },
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Expecting , or ; at end of instantiation. Got {} ({:?})",t.value, t.kind)))

        }
        if t.kind!=TokenKind::Ident {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting instance name, got {} {:?}",t.value, t.kind)));
        }
    }
    // println!("[Instance] {}",node);
    Ok(node)
}

/// Parse an always block
pub fn parse_always(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let t0 = next_t!(ts,false);
    let mut node = AstNode::new(AstNodeKind::Process(t0.value.clone()));
    let mut is_block = false;
    let mut t = next_t!(ts,true);

    // println!("[parse_always] Node {}\nFirst Token {}",node, t);
    if t.kind == TokenKind::At {
        ts.flush(0);
        match t0.kind {
            TokenKind::KwAlwaysL |
            TokenKind::KwAlwaysC => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                        format!("Sensitivity list not supported for {} process",t0.value))),
            // Parse the sensitivity list
            _ => {
                let node_s = parse_sensitivity(ts)?;
                node.child.push(node_s);
            }
        }

        t = next_t!(ts,true);
        // println!("[parse_always] Token post sensitivity list: {}", t);
    }
    if t.kind == TokenKind::KwBegin {
        is_block = true;
        parse_label(ts,&mut node,"block".to_string())?;
    }
    // Loop on statement, if/else / case
    parse_stmt(ts,&mut node, is_block)?;

    Ok(node)
}

/// Parse sensitivity list
/// Suppose Token @ has been consumed
/// An empty sensitivity node corresponds to @(*) or @*
pub fn parse_sensitivity(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut node = AstNode::new(AstNodeKind::Sensitivity);
    // Check next character: open parenthesis or *
    let mut t = next_t!(ts,false);
    // println!("[parse_sensitivity] First Token {}", t);
    match t.kind {
        TokenKind::OpStar   |
        TokenKind::SensiAll => return Ok(node),
        TokenKind::ParenLeft => t = next_t!(ts,false),
        _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Unexpected {} {:?} for sensitivity list. Expecting *, (*) or ( event list )",t.value, t.kind))),
    }
    // Parse event list
    loop {
        // println!("[parse_sensitivity] First Token of event expression {}", t);
        match t.kind {
            TokenKind::Ident => node.child.push(AstNode::new(AstNodeKind::Event(t.value))),
            TokenKind::KwEdge => {
                let nt = next_t!(ts,false);
                if nt.kind!=TokenKind::Ident {
                    return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Expecting identifier after edge specifier, got {} {:?}",t.value, t.kind)));
                }
                let mut node_e = AstNode::new(AstNodeKind::Event(t.value));
                node_e.attr.insert("edge".to_string(),nt.value );
                node.child.push(node_e);
            },
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Unexpected {} {:?} inside sensitivity list.",t.value, t.kind))),
        }
        t = next_t!(ts,false);
        // TODO: support iff statement
        // Expecting closing parenthesis, comme, or kezword or
        // println!("[parse_sensitivity] event expression separator {}", t);
        match t.kind {
            TokenKind::ParenRight => break,
            TokenKind::KwOr  |
            TokenKind::Comma => {},
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Unexpected {} {:?} inside sensitivity list.",t.value, t.kind))),

        }
        t = next_t!(ts,false);
    }
    // println!("[parse_sensitivity] Node {}", node);
    Ok(node)
}

///
pub fn parse_stmt_or_block(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut is_block = false;
    let t = next_t!(ts,true);
    if t.kind == TokenKind::KwBegin {
        // println!("[parse_stmt_or_block] Parsing optional label");
        is_block = true;
        parse_label(ts,node,"block".to_string())?;
    }
    // Loop on statement, if/else / case
    parse_stmt(ts,node, is_block)?;
    Ok(())
}


/// Parse any statement in a process
pub fn parse_stmt(ts : &mut TokenStream, node: &mut AstNode, is_block: bool) -> Result<(), SvError> {
    ts.rewind(0);
    loop {
        let t = next_t!(ts,true);
        // println!("[parse_stmt] Token = {}", t);
        // ts.display_status();
        match t.kind {
            TokenKind::KwIf   => {
                ts.flush(0);
                parse_if_else(ts,node, "if".to_string(), false)?;
            }
            TokenKind::KwCase     |
            TokenKind::KwPriority |
            TokenKind::KwUnique   |
            TokenKind::KwUnique0   => {
                ts.rewind(0);
                parse_case(ts,node)?;
            }
            TokenKind::Ident  => {
                ts.rewind(0);
                node.child.push(parse_assign_bnb(ts)?)
            },
            TokenKind::KwEnd  => {
                ts.flush(0);
                break;
            },
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Unexpected {} ({:?}) in statement",t.value, t.kind)))
        }
        // Stop parsing if not in a block
        if ! is_block {break;}
    }
    Ok(())
}

/// Parse If/Else if/Else statements
/// Suppose first IF has been consumed
pub fn parse_if_else(ts : &mut TokenStream, node: &mut AstNode, cond: String, is_gen: bool) -> Result<(), SvError> {
    let mut t = next_t!(ts,false);
    // println!("[parse_if_else] First Token = {}", t);
    // Parse IF condition
    if t.kind!=TokenKind::ParenLeft {
        return Err(SvError::syntax(t,"if. Expecting (".to_string()));
    }
    let mut node_if = AstNode::new(AstNodeKind::Branch);
    node_if.attr.insert("kind".to_string(),cond);
    let s = parse_expr(ts,ExprCntxt::Port)?;
    // println!("[parse_if_else] Expr = {}", s);
    ts.flush(0); // No need to check last token, with this context parse expr only go out on close parenthesis
    node_if.attr.insert("expr".to_string(), s);
    // Check for begin
    let mut is_block = false;
    t = next_t!(ts,true);
    if t.kind == TokenKind::KwBegin {
        is_block = true;
        parse_label(ts,&mut node_if,"block".to_string())?;
    }
    // Loop on statement, if/else / case
    if is_gen {
        parse_module_body(ts,&mut node_if, if is_block {ModuleCntxt::IfBlock} else {ModuleCntxt::IfStmt})?;
    } else {
        parse_stmt(ts,&mut node_if, is_block)?;
    }
    node.child.push(node_if);

    // Check for else if/else statement
    loop {
        t = next_t!(ts,true);
        // println!("[parse_if_else] Else Token ? {}", t);
        if t.kind == TokenKind::KwElse {
            ts.flush(0);
            t = next_t!(ts,true);
            // println!("[parse_if_else] If Token ? {}", t);
            if t.kind == TokenKind::KwIf {
                ts.flush(0);
                parse_if_else(ts,node,"else if".to_string(), is_gen)?;
            } else {
                let mut node_else = AstNode::new(AstNodeKind::Branch);
                node_else.attr.insert("kind".to_string(),"else".to_string());
                is_block = t.kind == TokenKind::KwBegin;
                // println!("[parse_if_else] Else token : is_block {}, is_gen {}", is_block, is_gen);
                if is_block {
                    parse_label(ts,&mut node_else,"block".to_string())?;
                }
                if is_gen {
                    ts.rewind(0);
                    parse_module_body(ts,&mut node_else, if is_block {ModuleCntxt::IfBlock} else {ModuleCntxt::IfStmt})?;
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

/// Parse case statement
pub fn parse_case(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let mut t = next_t!(ts,false);
    let mut node_c = AstNode::new(AstNodeKind::Case);
    // println!("[parse_case] First Token {}", t);
    if t.kind==TokenKind::KwPriority || t.kind==TokenKind::KwUnique || t.kind==TokenKind::KwUnique0 {
        node_c.attr.insert("prio".to_string(),t.value);
        t = next_t!(ts,false);
    }
    if t.kind!=TokenKind::KwCase {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                    format!("Unexpected {} {:?} in case statement.",t.value, t.kind)));
    }
    node_c.attr.insert("kind".to_string(),t.value);
    // Parse case expression
    t = next_t!(ts,false);
    if t.kind!=TokenKind::ParenLeft {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                    format!("Expecting open parenthesis after if , got {} {:?}",t.value, t.kind)));
    }
    let s = parse_expr(ts,ExprCntxt::Port)?;
    ts.flush(0); // consume closing parenthesis
    // println!("[parse_case] case expr {}", s.clone());
    node_c.attr.insert("expr".to_string(),s);
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
        // println!("[parse_case] case item {}", t);
        let mut node_i =  AstNode::new(AstNodeKind::CaseItem);
        match t.kind {
            TokenKind::OpPlus   |
            TokenKind::OpMinus  |
            TokenKind::Integer  |
            TokenKind::Ident    |
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
                // println!("[parse_case] case item value {}", s);
                node_i.attr.insert("item".to_string(),s);
                ts.flush(0); // every character until the colon should be consumed
            }
            TokenKind::KwDefault => {
                let nt = next_t!(ts,true);
                // Check for colon after keyword default
                if nt.kind!=TokenKind::Colon {
                    return Err(SvError::new(SvErrorKind::Syntax, nt.pos,
                                format!("Unexpected {} {:?} in default case item",nt.value, nt.kind)));
                }
                ts.flush(0);
                node_i.attr.insert("item".to_string(),"default".to_string());
            }
            TokenKind::KwEndcase => break,
            // TODO : support tagged keyword for case-matches
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Unexpected {} ({:?}) in case entry",t.value, t.kind)))
        }
        // Parse statement
        parse_stmt_or_block(ts,&mut node_i)?;
        // println!("[parse_case] case item node {}", node_i);
        node_c.child.push(node_i);
    }
    ts.flush(0);
    node.child.push(node_c);
    Ok(())
}

pub fn parse_for(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush(0);
    let mut t = next_t!(ts,false);
    if t.kind!=TokenKind::ParenLeft {
        return Err(SvError::syntax(t,"for. Expecting (".to_string()));
    }
    let mut node_for = AstNode::new(AstNodeKind::LoopFor);
    // Parse init part : end on ;
    let s = parse_expr(ts,ExprCntxt::Stmt)?;
    node_for.attr.insert("init".to_string(), s);
    ts.flush(0); // clear semi-colon
    // Parse test part : end on ;
    let s = parse_expr(ts,ExprCntxt::Stmt)?;
    node_for.attr.insert("test".to_string(), s);
    ts.flush(0); // clear semi-colon
    // Parse incr part : end on )
    let s = parse_expr(ts,ExprCntxt::Port)?;
    node_for.attr.insert("incr".to_string(), s);
    ts.flush(0); // Clear parenthesis
    // TODO: analyze each statement to make sure those are valid
    // Check for begin
    let mut cntxt_body = ModuleCntxt::ForStmt;
    t = next_t!(ts,true);
    if t.kind == TokenKind::KwBegin {
        cntxt_body = ModuleCntxt::ForBlock;
        parse_label(ts,&mut node_for,"block".to_string())?;
    }
    // Parse content of for loop as if inside a module body
    parse_module_body(ts,&mut node_for,cntxt_body)?;
    node.child.push(node_for);
    Ok(())
}
