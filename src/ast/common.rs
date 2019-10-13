// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::lex::token::TokenKind;
use crate::lex::token_stream::TokenStream;
use crate::ast::astnode::{AstNode, AstNodeKind};
use crate::ast::class::{parse_func};
use crate::error::{SvErrorKind, SvError};

macro_rules! next_t {
    ($ts:expr, $peek:expr) => {{
        $ts.next_non_comment($peek).unwrap_or(Err(SvError::eof()))?
    }};
}

macro_rules! expect_t {
    ($ts:expr, $cntxt:expr, $($exp_tk:expr),+) => {{
        let t = next_t!($ts,false);
        let mut m = false;
        // let mut s = "".to_owned();
        $(
            m |= t.kind == $exp_tk;
        )+
        if !m {
            return Err(SvError::syntax(t,  format!("{}. Expecting {:?}", $cntxt.to_string(), ($($exp_tk),+) ) ));
        }
        t
    }};
}

macro_rules! loop_args_break_cont {
    ($ts:expr, $cntxt:expr, $tk:ident) => {{
        let t = next_t!($ts,false);
        match t.kind {
            // Comma -> the port list continue
            TokenKind::Comma => {},
            // Right parenthesis, port list is done
            TokenKind::$tk => break,
            // Any other token is an error
            _ => return Err(SvError::syntax(t, format!("{}. Expecting , or {}", $cntxt,TokenKind::$tk)))
        }
    }}
}

#[allow(dead_code)]
#[derive(PartialEq,Clone,Debug)]
pub enum ExprCntxt {
    ArgList, Arg, ExprGroup,
    StmtList, Stmt,
    FieldList, Sensitivity,
    BracketMsb, BracketLsb,
    Question
}

/// Parse an import/export statement
pub fn parse_import(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(1);
    let mut n = AstNode::new(AstNodeKind::Import);
    let mut t = expect_t!(ts,"import/export",TokenKind::KwImport,TokenKind::KwExport);
    n.attr.insert("kind".to_owned(),t.value);
    t = next_t!(ts,true);
    match t.kind {
        TokenKind::Ident => {
            ts.rewind(1);
            loop {
                let mut ni = AstNode::new(AstNodeKind::Identifier);
                t = expect_t!(ts,"import",TokenKind::Ident);
                ni.attr.insert("pkg_name".to_owned(),t.value);
                expect_t!(ts,"import",TokenKind::Scope);
                t = expect_t!(ts,"import",TokenKind::Ident,TokenKind::OpStar);
                ni.attr.insert("name".to_owned(),t.value);
                n.child.push(ni);
                t = next_t!(ts,false);
                match t.kind {
                    TokenKind::SemiColon => break,
                    TokenKind::Comma => {},
                    _ => return Err(SvError::syntax(t, "package import. Expecting , or ;".to_owned()))
                }
            }
        }
        TokenKind::Str => {
            if t.value!="DPI-C" && t.value!="DPI"  {
                return Err(SvError::syntax(t, "import DPI. Expecting DPI-C, DPI or package identifier".to_owned()))
            }
            n.attr.insert("dpi".to_owned(),t.value);
            ts.flush(1);
            t = next_t!(ts,true);
            if t.kind==TokenKind::KwPure || t.kind==TokenKind::KwContext {
                ts.flush(1);
                n.attr.insert("property".to_owned(),t.value);
            } else {ts.rewind(1);}
            parse_func(ts, &mut n, false, true)?;
        }
        _ => return Err(SvError::syntax(t, "import. Expecting DPI-C or package identifier".to_owned()))
    }
    node.child.push(n);
    Ok(())
}

/// Parse a param/localparam declaration
pub fn parse_param_decl(ts : &mut TokenStream, is_body: bool) -> Result<AstNode, SvError> {
    let mut t = next_t!(ts,true);
    let mut node = AstNode::new(AstNodeKind::Param);
    // optionnal keyword param/localparam
    match t.kind {
        TokenKind::KwParam | TokenKind::KwLParam  => {
            node.attr.insert("kind".to_owned(), format!("{:?}",t.kind ) );
            ts.flush(0);
        },
        _ => {}
    }

    // Optional data type
    parse_data_type(ts,&mut node, 2)?;
    t = next_t!(ts,false);
    // Parameter name
    if t.kind != TokenKind::Ident {
        return Err(SvError::syntax(t, "param declaration, expecting identifier".to_owned()));
    }
    node.attr.insert("name".to_owned(), t.value);
    // Optional Unpacked dimension : [x][y:z]
    t = next_t!(ts,true);
    if t.kind == TokenKind::SquareLeft {
        ts.flush(0);
        node.attr.insert("unpacked".to_owned(), parse_range(ts)?);
        t = next_t!(ts,true);
    }

    // Optional default value i.e. "= expr"
    if t.kind != TokenKind::OpEq {
        ts.rewind(1);
        return Ok(node);
    } else {
        ts.flush(1);
    }
    let cntxt = if is_body {ExprCntxt::StmtList} else {ExprCntxt::ArgList};
    node.child.push(parse_expr(ts,cntxt,false)?);
    // println!("{}", node);
    // ts.display_status("param_decl");
    Ok(node)
}

/// Parse a port declaration
pub fn parse_port_decl(ts : &mut TokenStream, allow_void : bool) -> Result<AstNode, SvError> {
    let mut node = AstNode::new(AstNodeKind::Port);
    let mut type_found = false;
    let mut t = next_t!(ts,true);
    // println!("[parse_port_decl] First token = {:?}", t);
    if t.kind == TokenKind::KwConst {
        node.attr.insert("const".to_owned(), "".to_owned());
        t = next_t!(ts,true);
    }
    // direction/interface
    match t.kind {
        TokenKind::KwInput | TokenKind::KwOutput | TokenKind::KwInout | TokenKind::KwRef => {
            node.attr.insert("dir".to_owned(), t.value);
            ts.flush(0);
        }
        // Interface / User-defined type
        TokenKind::Ident => {
            type_found = true;
            // Check if mod port is available
            let nt = next_t!(ts,true);
            // println!("[parse_port_decl] Second token = {:?}", nt);
            match nt.kind {
                // Dot : t is the interface type and token is the modport (expect identifier)
                TokenKind::Dot => {
                    let nnt = next_t!(ts,true);
                    if nnt.kind != TokenKind::Ident {
                        return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) for port type",nnt.value, nnt.kind)))
                    }
                    node.attr.insert("intf".to_owned(), t.value);
                    node.attr.insert("modport".to_owned(), nnt.value);
                    ts.flush(0);
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
                _ =>  return Err(SvError::syntax(nt, "port declaration. Expecting identifer, comma, ), #".to_owned()))
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
                    node.attr.insert("intf".to_owned(), t.value);
                    node.attr.insert("modport".to_owned(), nnt.value);
                    ts.flush(0);
                }
                // Another ident : No modport, nt is the port name, rewind it
                TokenKind::Ident => ts.rewind(1),
                // Any other token is an error
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in port declaration.",nt.value, nt.kind)))
            }
        }
        TokenKind::ParenRight => {return Ok(node);},
        // Handle Ident
        _ => {}
    }
    if ! type_found {
        // Optional net type
        parse_net_type(ts,&mut node)?;
        // Optional data type
        parse_data_type(ts,&mut node, if allow_void {1} else {0})?;
    }
    // Port name
    t = expect_t!(ts,"port declaration",TokenKind::Ident);
    node.attr.insert("name".to_owned(), t.value);
    // Optional Unpacked dimension : [x][y:z]
    t = next_t!(ts,true);
    // println!("[parse_port_decl] After port name token = {:?}", t);
    if t.kind == TokenKind::SquareLeft {
        ts.flush(0);
        node.attr.insert("unpacked".to_owned(), parse_range(ts)?);
        t = next_t!(ts,true);
    }
    // Optional Default value i.e. "= expr"
    if t.kind == TokenKind::OpEq {
        ts.flush(1);
        node.child.push(parse_expr(ts,ExprCntxt::ArgList,false)?);
    } else {
        ts.rewind(1);
    }
    // println!("port_decl: {:?}", node);
    // ts.display_status("port_decl");
    Ok(node)
}

/// Parse a list of signal declaration
pub fn parse_signal_decl_list(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    // ts.display_status("");
    let mut is_first = true;
    loop {
        let node_sig = parse_signal_decl(ts,is_first)?;
        // println!("[parse_signal_decl_list] {}", node_sig);
        node.child.push(node_sig);
        let nt = next_t!(ts,false);
        // println!("[parse_signal_decl_list] Next token : {}", nt);
        match nt.kind {
            TokenKind::Comma => is_first = false, // Comma indicate a list -> continue
            TokenKind::SemiColon => break, // Semi colon indicate end of statement, stop the loop
            _ => return Err(SvError::syntax(nt, "signal declaration, expecting , or ;".to_owned()))
        }
    }
    // println!("[parse_signal_decl_list] {}", node);
    ts.rewind(0); // put back any token we have not used
    // ts.display_status("");
    Ok(())
}

/// Parse a signal declaration
pub fn parse_signal_decl(ts : &mut TokenStream, has_type: bool) -> Result<AstNode, SvError> {
    let mut node = AstNode::new(AstNodeKind::Declaration);
    if has_type {
        // Parse potential net type
        parse_net_type(ts,&mut node)?;
        // Parse data type
        parse_data_type(ts,&mut node, 1)?;
    }
    parse_var_decl_name(ts, &mut node)?;
    Ok(node)
}

pub fn parse_var_decl_name(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    // Signal name
    let mut t = next_t!(ts,false);
    if t.kind != TokenKind::Ident {
        return Err(SvError::syntax(t, "signal declaration, expecting identifier".to_owned()))
    }
    node.attr.insert("name".to_owned(), t.value);
    // Optional Unpacked dimension : [x][y:z]
    t = next_t!(ts,true);
    if t.kind == TokenKind::SquareLeft {
        ts.flush(1);
        node.attr.insert("unpacked".to_owned(), parse_range(ts)?);
        t = next_t!(ts,true);
    }
    // Optional Default value i.e. "= expr"
    if t.kind == TokenKind::OpEq {
        ts.flush(1);
        node.child.push(parse_expr(ts,ExprCntxt::StmtList,false)?);
    }
    ts.rewind(0);
    Ok(())
}

/// Tentatively parse a net type
pub fn parse_net_type(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let mut t = next_t!(ts,true);
    // println!("[parse_net_type] {}", t);
    if t.kind == TokenKind::KwConst {
        node.attr.insert("const".to_owned(),"".to_owned());
        ts.flush(1);
        t = next_t!(ts,true);
    }
    if t.kind==TokenKind::KwNetType || t.kind==TokenKind::KwSupply || t.kind==TokenKind::KwVar {
        let allow_strength = t.kind!=TokenKind::KwVar;
        node.attr.insert("nettype".to_owned(),t.value);
        ts.flush(1);
        t = next_t!(ts,true);
        // println!("[parse_net_type] next ? {}", t);
        // Check for optional strength
        if t.kind==TokenKind::ParenLeft && allow_strength {
            ts.flush(1);
            parse_strength(ts,node)?;
            t = next_t!(ts,true);
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
    let mut t = next_t!(ts,false);
    match t.kind {
        TokenKind::KwCharge => {
            node.attr.insert("charge".to_owned(),t.value);
        }
        TokenKind::KwDrive | TokenKind::KwSupply => {
            let mut s = t.value;
            t = next_t!(ts,false);
            if t.kind!=TokenKind::Comma {
                return Err(SvError::syntax(t, "drive strength declaration, expecting ,".to_owned()))
            }
            s.push(',');
            t = next_t!(ts,false);
            if t.kind!=TokenKind::KwDrive && t.kind!=TokenKind::KwSupply {
                return Err(SvError::syntax(t, "drive strength declaration, expecting ,".to_owned()))
            }
            s.push_str(&t.value);
            node.attr.insert("drivee".to_owned(),s);
            // TODO: Check combination are actually valid
        }
        _ => return Err(SvError::syntax(t, "strength declaration, expecting drive or charge".to_owned()))
    }
    // Done, expecting closing parenthesis
    t = next_t!(ts,false);
    if t.kind!=TokenKind::ParenRight {
        return Err(SvError::syntax(t, "strength declaration, expecting )".to_owned()))
    }
    Ok(())
}

/// Parse optional scope
#[allow(dead_code)]
pub fn parse_opt_scope(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    let t_ident = next_t!(ts,true);
    if t_ident.kind != TokenKind::Ident {
        ts.rewind(1);
        return Ok(());
    }
    let t_scope = next_t!(ts,true);
    if t_scope.kind != TokenKind::Scope {
        ts.rewind(2);
        return Ok(());
    }
    let mut n = AstNode::new(AstNodeKind::Scope);
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
    let mut t = next_t!(ts,true);
    let mut s = t.value.clone();
    // println!("[parse_data_type] First Token = {}", t);
    // First word of a data type
    match t.kind {
        // Integer vector type -> has signing and packed dimension
        TokenKind::KwReg         |
        TokenKind::TypeIntVector => {ts.flush(1); get_next = true; }
        TokenKind::TypeVoid   if (allowed_flag & 1)!=0 => {ts.flush(1); get_next = true; }
        TokenKind::KwType     if (allowed_flag & 2)!=0 => {ts.flush(1); get_next = true; }
        TokenKind::TypeGenvar if (allowed_flag & 4)!=0 => {ts.flush(1); get_next = true; }
        TokenKind::TypeIntAtom => {ts.flush(1); get_next = true; has_width=false}
        TokenKind::TypeReal    |
        TokenKind::TypeString  |
        TokenKind::TypeCHandle |
        TokenKind::TypeEvent   => {ts.flush(1); get_next = true; has_width=false; has_signing=false}
        TokenKind::KwEnum => {
            has_signing = false;
            has_width   = false;
            node.attr.insert("type".to_owned(), "enum".to_owned());
            ts.flush(1);
            node.child.push(parse_enum(ts)?);
        }
        // Ident -> check next word, could be a user type
        TokenKind::Macro |
        TokenKind::Ident => {
            has_signing = false;
            has_width   = false;
            let nt = next_t!(ts,true);
            // println!("[parse_data_type] Ident followed by {}", nt);
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
                TokenKind::SquareLeft => {get_next=true; ts.flush(1); ts.rewind(1); has_width = true;}
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
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({}) in data type.",nt.value, nt.kind)))
            }
        }
        // Sign/Slice start (ignore handling now, will be done after)
        TokenKind::KwSigning |
        TokenKind::SquareLeft => {}
        // Any token not listed here is an error
        _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in data type. Expecting type or identifier",t.value, t.kind)))
    }
    // println!("[parse_data_type] ->get next = {} : {}", get_next, t );
    if get_next {
        node.attr.insert("type".to_owned(), s);
        t = next_t!(ts,true);
    }
    // println!("[parse_data_type] -> has_sign={} : {}", has_signing, t );
    //
    if has_signing  && t.kind == TokenKind::KwSigning {
        node.attr.insert("signing".to_owned(), t.value);
        ts.flush(1);
        t = next_t!(ts,true);
    }
    // println!("[parse_data_type] -> has_width={} : {}", has_width, t );
    if has_width  && t.kind == TokenKind::SquareLeft {
        ts.flush(1);
        let ws = parse_range(ts)?;
        // Add packed dimension to the port attribute and retrieve the next token
        node.attr.insert("packed".to_owned(), ws);
    }
    //
    ts.rewind(1); // Put back last token we did not used
    // println!("[parse_data_type] -> {}", node);
    // ts.display_status("Post parse_data_type");
    Ok(())
}

/// Parse a range
pub fn parse_range(ts : &mut TokenStream) -> Result<String,SvError> {
    let mut s = "[".to_owned();
    let mut cnt_s = 1;
    let mut cnt_p = 0;
    let mut prev_tk = TokenKind::SquareLeft;
    loop {
        let t = next_t!(ts,true);
        // println!("[parse_range]  {} (cnt s={}, p={})", t,cnt_s,cnt_p );
        if cnt_s==0 && t.kind != TokenKind::SquareLeft {
            if cnt_p > 0 {
                return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced parenthesis".to_owned()));
            }
            break;
        }
        match t.kind {
            TokenKind::SquareLeft  => cnt_s += 1,
            TokenKind::SquareRight => cnt_s -= 1,
            TokenKind::ParenLeft   => cnt_p += 1,
            TokenKind::ParenRight  => {
                if cnt_p == 0 {break;}
                cnt_p -= 1;
            }
            TokenKind::SemiColon => return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unexpected semi-colon in range definition".to_owned())),
            TokenKind::Ident => {
                if prev_tk==TokenKind::Ident {
                    return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Invalid range definition".to_owned()));
                }
            },
            _ => {}
        }
        prev_tk = t.kind;
        s.push_str(&t.value);
        ts.flush(0); // Token consumed, can flush it
    }
    ts.rewind(0);
    // ts.display_status("parse_range");
    Ok(s)
}

pub fn parse_opt_slice(ts : &mut TokenStream, node: &mut AstNode, allow_range: bool) -> Result<(),SvError> {
    let mut t;
    loop {
        t = next_t!(ts,true);
        if t.kind != TokenKind::SquareLeft {break;} else {ts.flush(1);}
        let mut n = AstNode::new(AstNodeKind::Slice);
        n.child.push(parse_expr(ts,ExprCntxt::BracketMsb,false)?);
        // Check if range
        if allow_range {
            // The expression parser ends either on : or ]
            t = next_t!(ts,false);
            if t.kind == TokenKind::Colon || t.kind == TokenKind::OpRange {
                n.attr.insert("range".to_owned(),t.value);
                n.child.push(parse_expr(ts,ExprCntxt::BracketLsb,false)?);
            }
            ts.flush(1);
        } else {
            expect_t!(ts,"size",TokenKind::SquareRight);
        }
        node.child.push(n);
    }
    ts.rewind(0);
    // ts.display_status("parse_opt_slice");
    Ok(())
}

// Parse identifier with potential hierarchy and range selection
pub fn parse_ident_hier(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    // ts.display_status("parse_ident_hier: start");
    let mut node = AstNode::new(AstNodeKind::Identifier);
    parse_opt_scope(ts,&mut node)?;
    let mut t = expect_t!(ts,"identifier", TokenKind::Ident, TokenKind::KwThis);
    node.attr.insert("name".to_owned(),t.value);
    parse_opt_slice(ts,&mut node,true)?;
    t = next_t!(ts,true);
    if t.kind == TokenKind::Dot {
        ts.flush(1);
        node.child.push(parse_ident_hier(ts)?);
    } else {
        ts.rewind(1);
    }
    // println!("[parse_ident_hier] {}", node);
    // ts.display_status("parse_ident_hier: done");
    return Ok(node);
}

pub fn parse_ident_list(ts : &mut TokenStream, node: &mut AstNode) -> Result<(),SvError> {
    let mut expect_ident = true;
    loop {
        let t = next_t!(ts,false);
        match t.kind {
            TokenKind::Ident if expect_ident => {
                let mut n = AstNode::new(AstNodeKind::Identifier);
                n.attr.insert("name".to_owned(),t.value);
                parse_opt_init_value(ts,&mut n)?;
                node.child.push(n);
                expect_ident = false;
            }
            TokenKind::Comma if !expect_ident => {
                expect_ident = true;
            }
            TokenKind::SemiColon if !expect_ident => break, // Semi colon indicate end of statement, stop the loop
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                    format!("Unexpected {} ({:?}) in ident list, expecting identifier/comma/semicolon",t.value, t.kind)))
        }
    }
    Ok(())
}

pub fn parse_opt_init_value(ts : &mut TokenStream, node: &mut AstNode) -> Result<(),SvError> {
    let t = next_t!(ts,true);
    if t.kind != TokenKind::OpEq {
        ts.rewind(1);
    } else {
        node.child.push(parse_expr(ts,ExprCntxt::StmtList,false)?);
    }
    Ok(())
}

/// Parse an enum declaration
pub fn parse_enum(ts : &mut TokenStream) -> Result<AstNode,SvError> {
    let mut node_e = AstNode::new(AstNodeKind::Enum);
    let mut t = next_t!(ts,true);
    // Optionnal data type
    match t.kind {
        TokenKind::TypeIntAtom => {
            let mut s = t.value;
            // Check for optional signing info
            t = next_t!(ts,true);
            if t.kind == TokenKind::KwSigning {
                s.push_str(&t.value);
                t = next_t!(ts,true);
            }
            node_e.attr.insert("type".to_owned(),s);
        }
        TokenKind::TypeIntVector => {
            node_e.attr.insert("type".to_owned(),t.value);
            // Check for optional signing info
            t = next_t!(ts,true);
            // ts.display_status("");
            if t.kind == TokenKind::KwSigning {
                node_e.attr.insert("signing".to_owned(), t.value);
                t = next_t!(ts,true);
            }
            // Check for optional dimension
            if t.kind == TokenKind::SquareLeft {
                let ws = parse_range(ts)?;
                // Add packed dimension to the port attribute and retrieve the next token
                node_e.attr.insert("packed".to_owned(), ws);
                t = next_t!(ts,true);
            }
        }
        TokenKind::Ident => {
            ts.flush(0);
            node_e.attr.insert("type".to_owned(),t.value);
            t = next_t!(ts,false);
        }
        TokenKind::CurlyLeft => {}
        _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in enum", t.value, t.kind)))
    }
    if t.kind != TokenKind::CurlyLeft {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in enum", t.value, t.kind)))
    }
    ts.flush(0); // Consume all character up to the opening curly brace
    loop {
        // Capture enum identifier
        t = next_t!(ts,false);
        let mut node_id = AstNode::new(AstNodeKind::EnumIdent);
        node_id.attr.insert("name".to_owned(), t.value);
        t = next_t!(ts,false);
        // Optional range
        if t.kind == TokenKind::SquareLeft {
            // node_id.attr.insert("range".to_owned(), s);
            t = next_t!(ts,false);
        }
        // Optional value
        if t.kind == TokenKind::OpEq {
            node_id.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
            t = next_t!(ts,false);
        }
        node_e.child.push(node_id);
        // Expect , or }
        match t.kind {
            TokenKind::Comma => {},
            TokenKind::CurlyRight => break,
            _ => return Err(SvError::syntax(t, "enum. Expecting , }".to_owned()))
        }
    }
    Ok(node_e)
}

/// Parse a struct declaration
pub fn parse_struct(ts : &mut TokenStream) -> Result<AstNode,SvError> {
    // TODO: handle union
    let mut t = next_t!(ts,false);
    let mut node;
    match t.kind {
        TokenKind::KwStruct => node = AstNode::new(AstNodeKind::Struct),
        TokenKind::KwUnion => {
            node = AstNode::new(AstNodeKind::Union);
            t = next_t!(ts,true);
            if t.kind == TokenKind::KwTagged {
                node.attr.insert("tagged".to_owned(),"".to_owned());
                ts.flush(1);
            } else {
                ts.rewind(1);
            }
        },
        _ => return Err(SvError::syntax(t, "struct. Expecting struct or union".to_owned()))
    }
    t = next_t!(ts,true);
    // Optional packed keyword
    if t.kind==TokenKind::KwPacked {
        ts.flush(0);
        node.attr.insert("packed".to_owned(),"".to_owned());
        t = next_t!(ts,true);
        // Optional signing
        if t.kind==TokenKind::KwSigning {
            ts.flush(0);
            node.attr.insert("signing".to_owned(), t.value);
            t = next_t!(ts,true);
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
        _ => return Err(SvError::syntax(t, "struct. Expecting {".to_owned()))
    }
    // Loop on type declaration until closing curly brace
    loop {
        t = next_t!(ts,true);
        match t.kind {
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
                let mut node_e = parse_enum(ts)?;
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
            _ => return Err(SvError::syntax(t, "struct. Expecting data type".to_owned())),
        }
        //
    }
    // TODO: check for unpacked dimension in packed struct
    // println!("{}", node);
    ts.flush(0);
    Ok(node)
}

/// Parse typedef
pub fn parse_typedef(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush(0);
    let mut t = next_t!(ts,true);
    let mut node_type;
    match t.kind {
        TokenKind::KwEnum => node_type = parse_enum(ts)?,
        TokenKind::KwStruct |
        TokenKind::KwUnion  => node_type = parse_struct(ts)?,
        TokenKind::KwReg         |
        TokenKind::Ident         |
        TokenKind::TypeIntAtom   |
        TokenKind::TypeIntVector |
        TokenKind::TypeReal      |
        TokenKind::TypeString    |
        TokenKind::TypeCHandle   |
        TokenKind::TypeEvent     => {
            node_type = AstNode::new(AstNodeKind::Typedef);
            parse_data_type(ts,&mut node_type, 0)?;
        }
        TokenKind::KwClass => {
            ts.flush(1);
            node_type = AstNode::new(AstNodeKind::Typedef);
            node_type.attr.insert("kind".to_owned(),t.value);
        }
        _ => return Err(SvError::syntax(t, "typedef declaration, expecting type/enum/struct".to_owned()))
    }
    // Parse type name
    t = next_t!(ts,false);
    if t.kind!=TokenKind::Ident && t.kind!=TokenKind::Macro {
        return Err(SvError::syntax(t, "typedef enum. Expecting identifier".to_owned()));
    }
    node_type.kind = AstNodeKind::Typedef;
    node_type.attr.insert("name".to_owned(),t.value);
    // Optional unpacked dimension
    t = next_t!(ts,false);
    if t.kind == TokenKind::SquareLeft {
        let ws = parse_range(ts)?;
        node_type.attr.insert("unpacked".to_owned(), ws);
        t = next_t!(ts,false);
    }
    // Expec semi-colon
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "package header. Expecting ;".to_owned()));
    }
    node.child.push(node_type);
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
        let t = next_t!(ts,true);
        // println!("[parse_port_connection] Token {}", t);
        match t.kind {
            TokenKind::Dot => {
                allow_list = false;
                ts.flush(0); // Consume the dot
                let mut nt = expect_t!(ts,"port name",TokenKind::Ident);
                let mut node_p = AstNode::new( if is_param {AstNodeKind::Param} else {AstNodeKind::Port});
                node_p.attr.insert("name".to_owned(), nt.value);
                nt = next_t!(ts,true);
                match nt.kind {
                    TokenKind::ParenLeft => {
                        ts.flush(0); // Consume the (
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
                    _ => return Err(SvError::new(SvErrorKind::Syntax, nt.pos, "Expecting open parenthesis".to_owned()))
                }
            },
            TokenKind::DotStar if allow_dot_star => {
                ts.flush(0); // Consume the (
                allow_dot_star = false;
                node.child.push(AstNode::new(AstNodeKind::Port));
                node.attr.insert("name".to_owned(), ".*".to_owned());
            },
            TokenKind::ParenRight if is_first => break,
            //
            _ => {
                // ordered connection
                if allow_list {
                    ts.rewind(0);
                    let mut node_p = AstNode::new( if is_param {AstNodeKind::Param} else {AstNodeKind::Port});
                    node_p.attr.insert("name".to_owned(), "".to_owned());
                    node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                    node_p.child.push(parse_expr(ts,ExprCntxt::ArgList,is_param)?);
                    node.child.push(node_p);
                    cnt += 1;
                } else {
                    return Err(SvError::syntax(t, "port connection".to_owned()));
                }
            }
        }
        is_first = false;
        loop_args_break_cont!(ts,"port connection",ParenRight);
    }
    // println!("parse_port_connection: {}", node);
    ts.flush(0);
    Ok(())
}


macro_rules! parse_opt_params {
    ($ts:expr, $node:expr, $t:expr) => {
        if $t.kind==TokenKind::Hash {
            $ts.flush(0); // Consume the hash token
            let mut node_p = AstNode::new(AstNodeKind::Params);
            parse_port_connection($ts,&mut node_p,true)?;
            $node.child.push(node_p);
            $t = next_t!($ts,true);
        }
    };
    ($ts:expr, $node:expr) => {
        let t = next_t!($ts,true);
        if t.kind==TokenKind::Hash {
            $ts.flush(0); // Consume the hash token
            let mut node_p = AstNode::new(AstNodeKind::Params);
            parse_port_connection($ts,&mut node_p,true)?;
            $node.child.push(node_p);
        } else {
            $ts.rewind(1);
        }
    };
}

/// Parse
pub fn parse_has_begin(ts : &mut TokenStream, node: &mut AstNode) -> Result<bool, SvError> {
    let mut is_block = false;
    let t = next_t!(ts,true);
    if t.kind == TokenKind::KwBegin {
        is_block = true;
        ts.flush(1);
        parse_label(ts,node,"block".to_owned())?;
    } else {
        ts.rewind(1);
    }
    Ok(is_block)
}

/// Parse the optional label after a begin keyword, and update
pub fn parse_label(ts : &mut TokenStream, node: &mut AstNode, attr_name: String) -> Result<bool, SvError> {
    ts.flush(1); // Consume the begin keyword
    let mut t = next_t!(ts,true);
    // println!("[parse_label] Token = : {}", t);
    // Check for named block
    if t.kind == TokenKind::Colon {
        ts.flush(1);
        t = next_t!(ts,false);
        if t.kind!=TokenKind::Ident {
            return Err(SvError::syntax(t, "block name".to_owned()))
        }
        node.attr.insert(attr_name, t.value);
        return Ok(true)
    } else {
        ts.rewind(1);
        node.attr.insert(attr_name, "".to_owned());
        return Ok(false);
    }
}

pub fn check_label(ts : &mut TokenStream, name: &str) -> Result<(), SvError> {
    if let Some(Ok(mut t)) = ts.next_non_comment(true) {
        // println!("[parse_label] Token = : {}", t);
        // Check for named block
        if t.kind == TokenKind::Colon {
            ts.flush(0);
            t = next_t!(ts,false);
            if t.kind!=TokenKind::Ident && t.kind!=TokenKind::KwNew {
                return Err(SvError::syntax(t, "label".to_owned()));
            } else if t.value != name {
                return Err(SvError::syntax(t, format!("label name. Expecting {}",name)));
            }
        } else {
            ts.rewind(0);
        }
    }
    Ok(())
}

pub fn parse_delay (ts : &mut TokenStream) -> Result<AstNode, SvError> {
    ts.rewind(0);
    let mut t = next_t!(ts,false);
    let mut node = AstNode::new(AstNodeKind::Wait);
    node.attr.insert("kind".to_owned(),t.value);
    t = next_t!(ts,true);
    match t.kind {
        TokenKind::Integer | TokenKind::Real => {
            let mut nv = AstNode::new(AstNodeKind::Value);
            nv.attr.insert("value".to_owned(), t.value);
            ts.flush(1); // consume number
            // Optional time unit
            t = next_t!(ts,true);
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
        _ => return Err(SvError::syntax(t, "wait statement. Expecting integer/real".to_owned()))
    }
    Ok(node)
}

/// Parse Macro/Directive
pub fn parse_macro(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut node_m = AstNode::new(AstNodeKind::Directive);
    ts.rewind(0);
    let mut t = next_t!(ts,false);
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
            t = next_t!(ts,true);
            if t.kind!=TokenKind::Ident {
                return Err(SvError::syntax(t, "ifdef directive".to_owned()))
            }
            node_m.attr.insert("param".to_owned(), t.value);
            ts.flush(0);
        }
        "`begin_keywords" => {
            t = expect_t!(ts,"type",TokenKind::Str);
            node_m.attr.insert("version".to_owned(),t.value);
        }
        // Expect pull0 or pull1
        "`unconnected_drive" => {
            t = expect_t!(ts,"type",TokenKind::KwDrive);
            if t.value != "pull0" && t.value != "pull1" {
                return Err(SvError::syntax(t, "Invalid unconnected drive, Expecting pull0/1 !".to_owned()));
            }
            node_m.attr.insert("drive".to_owned(),t.value);
        }
        // Include directive : `include <file> , `include "file" or `include `mymacro
        "`include" => {
            t = next_t!(ts,true);
            match t.kind {
                TokenKind::Macro => {node_m.attr.insert("include".to_owned(),t.value);},
                TokenKind::Str => {
                    ts.add_inc(&t.value);
                    node_m.attr.insert("include".to_owned(),t.value);
                },
                TokenKind::OpLT => {
                    t = next_t!(ts,true);
                    if t.kind!=TokenKind::Ident {
                        return Err(SvError::syntax(t, "include directive".to_owned()));
                    }
                    ts.add_inc(&t.value);
                    node_m.attr.insert("include".to_owned(),t.value);
                    t = next_t!(ts,true);
                    if t.kind!=TokenKind::OpGT {
                        return Err(SvError::syntax(t, "include directive".to_owned()));
                    }
                }
                _ => return Err(SvError::syntax(t, "include directive".to_owned()))
            }
            ts.flush(0);
        }
        // Define directive : first token is the name, followed by optional argument and then the content is all token until EOL
        "`define" => {
            t = next_t!(ts,true);
            if t.kind!=TokenKind::Ident {
                return Err(SvError::syntax(t, "ifdef directive".to_owned()))
            }
            node_m.attr.insert("name".to_owned(),t.value);
            let mut line_num = t.pos.line;
            ts.flush(0);
            t = next_t!(ts,true);
            if t.pos.line != line_num {
                ts.rewind(0);
            } else {
                ts.flush(0);
                if t.kind == TokenKind::ParenLeft {
                    t = next_t!(ts,true);
                    if t.kind == TokenKind::Ident {
                        loop {
                            t = next_t!(ts,false);
                            match t.kind {
                                TokenKind::Ident => {
                                    let mut node_p = AstNode::new(AstNodeKind::Param);
                                    node_p.attr.insert("name".to_owned(),t.value);
                                    // Optional Default value i.e. "= expr"
                                    t = next_t!(ts,true);
                                    if t.kind == TokenKind::OpEq {
                                        ts.flush(1);
                                        node_p.child.push(parse_expr(ts,ExprCntxt::ArgList,false)?);
                                    } else {ts.rewind(1);}
                                    node_m.child.push(node_p);
                                    loop_args_break_cont!(ts,"macro arguments",ParenRight);
                                }
                                TokenKind::LineCont => line_num += 1,
                                _ =>  return Err(SvError::syntax(t,"define. Expecting port name/expression".to_owned())),
                            }
                        }
                    } else {ts.rewind(0);}
                }
                let mut content = "".to_owned();
                loop {
                    // t = next_t!(ts,true);
                    if let Some(x) = ts.next_non_comment(true) {
                        match x {
                            Ok(t) => {
                                // println!("[parse_macro] Define {} content: next = {}. Current line = {} vs {}", node_m.attr["name"], t, line_num, t.pos.line);
                                if t.pos.line != line_num {
                                    node_m.attr.insert("content".to_owned(),content);
                                    ts.rewind(0);
                                    // println!("[parse_macro] Define content = {:?}", node_m);
                                    break;
                                } else if t.kind == TokenKind::LineCont {
                                    content.push('\n');
                                    line_num += 1;
                                } else {
                                    // TODO add each token as a child to ease string interpolation
                                    // Maybe also need to properly handle space characters ...
                                    // content.push(' ');
                                    content.push_str(&t.value);
                                }
                                ts.flush(0);

                            }
                            Err(t) => return Err(t),
                        }
                    }
                    // Reach end of file
                    else {
                        node_m.attr.insert("content".to_owned(),content);
                        break;
                    }
                }
            }
        }
        "`pragma" => {
            t = expect_t!(ts,"type",TokenKind::Ident);
            let line = t.pos.line;
            node_m.attr.insert("pragma_name".to_owned(), t.value);
            // Silently consume any token on current line
            loop {
                t = next_t!(ts,true);
                if t.pos.line != line {break;}
            }
            ts.flush_keep(1); // Cleanup everything except last token
        }
        "`default_nettype" => {
            t = next_t!(ts,true);
            if t.kind!=TokenKind::KwNetType && (t.kind!=TokenKind::Ident || t.value != "none")  {
                return Err(SvError::syntax(t,"default_nettype. Expecting net type (wire/tri/...) or none".to_owned()));
            }
            node_m.attr.insert("nettype".to_owned(),t.value);
            ts.flush(0);
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
            node_m.kind = AstNodeKind::MacroCall;
            t = next_t!(ts,true);
            if t.kind == TokenKind::ParenLeft {
                ts.flush(0);
                // Parse until closing parenthesis
                loop {
                    node_m.child.push(parse_expr(ts,ExprCntxt::ArgList,true)?);
                    t = next_t!(ts,false);
                    if t.kind == TokenKind::ParenRight {
                        break;
                    }
                }
            } else {
                ts.rewind(0);
            }
        }

    }
    // println!("[parse_macro] Done -> {}", node_m);
    // ts.display_status("[parse_macro]");
    node.child.push(node_m);
    Ok(())
}


#[allow(unused_assignments)]
/// Parse a virtual interface member declaration
pub fn parse_vintf(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    // Mandatory virtual keyword
    expect_t!(ts,"virtual interface",TokenKind::KwVirtual);
    let mut node_i = AstNode::new(AstNodeKind::VIntf);
    // Optional interface keyword
    let mut t = next_t!(ts,false);
    if t.kind==TokenKind::KwIntf {
        t = next_t!(ts,false);
        node_i.attr.insert("keyword".to_owned(),"explicit".to_owned());
    } else {
        node_i.attr.insert("keyword".to_owned(),"implicit".to_owned());
    }
    // Mandatory virtual interface type
    if t.kind!=TokenKind::Ident && t.kind!=TokenKind::Macro {
        return Err(SvError::syntax(t, "virtual interface. Expecting type identifier".to_owned()));
    }
    node_i.attr.insert("type".to_owned(),t.value);
    // Optional parameter
    parse_opt_params!(ts,node_i);
    loop {
        t = next_t!(ts,false);
        match t.kind {
            TokenKind::Ident => {
                let mut n = AstNode::new(AstNodeKind::Identifier);
                n.attr.insert("name".to_owned(),t.value);
                node_i.child.push(n);
                loop_args_break_cont!(ts,"virtual interface",SemiColon);
            }
            _ =>  return Err(SvError::syntax(t,"virtual interface. Expecting identifier".to_owned())),
        }
    }
    node.child.push(node_i);
    // println!("[parse_vintf] {}", node);
    Ok(())
}

pub fn parse_expr(ts : &mut TokenStream, cntxt: ExprCntxt, allow_type: bool) -> Result<AstNode, SvError> {
    let mut node_e = AstNode::new(AstNodeKind::Expr);
    let mut cnt_c = 0;
    let mut cnt_s = 0;
    let mut cnt_p = 0;
    let mut is_first = true;
    let mut allow_ident = true;
    let mut prev_kind = TokenKind::SemiColon;
    let mut t;
    // ts.display_status("parse_expr: start");
    loop {
        t = next_t!(ts,true);
        // println!("[parse_expr] Token = {}, cnt c={}, s={}, p={} (cntxt={:?}, first={}, allow_ident={})", t,cnt_c,cnt_s,cnt_p, cntxt, is_first, allow_ident);
        match t.kind {
            // Statement: end on semi-colon or comma: rewind it and end
            TokenKind::SemiColon if cntxt==ExprCntxt::StmtList || cntxt==ExprCntxt::Stmt => { ts.rewind(0); break; },
            TokenKind::SemiColon if cntxt!=ExprCntxt::StmtList && cntxt!=ExprCntxt::Stmt  => return Err(SvError::syntax(t, "expression".to_owned())),
            // End on comma (if not inside curly braces)
            TokenKind::Comma if cnt_c==0 => {
                if cntxt==ExprCntxt::Stmt || cntxt==ExprCntxt::Arg || cntxt==ExprCntxt::ExprGroup  {
                    return Err(SvError::syntax(t, "expression".to_owned()));
                }
                ts.rewind(1); // reset to comma token to be used by caller
                break;
            }
            // Count parenthesis/braces to check if it is balanced
            TokenKind::TickCurly if is_first  => {
                ts.flush(0);
                node_e.kind = AstNodeKind::StructInit;
                t = next_t!(ts,true);
                let mut is_struct = false;
                if t.kind == TokenKind::Ident || t.kind == TokenKind::Integer || t.kind == TokenKind::Str || t.kind == TokenKind::KwDefault {
                    t = next_t!(ts,true);
                    is_struct = t.kind == TokenKind::Colon;
                }
                ts.rewind(0);
                loop {
                    let mut s = "".to_owned();
                    if is_struct {
                        t = next_t!(ts,false);
                        if t.kind != TokenKind::Ident && t.kind != TokenKind::Integer && t.kind != TokenKind::Str && t.kind != TokenKind::KwDefault {
                            return Err(SvError::syntax(t, "struct init. Expecting identifier/integer/default".to_owned()));
                        }
                        s = t.value;
                        t = next_t!(ts,false);
                        if t.kind != TokenKind::Colon {
                            return Err(SvError::syntax(t, "struct init. Expecting colon".to_owned()));
                        }
                    }
                    let mut n = parse_expr(ts,ExprCntxt::FieldList,false)?;
                    if is_struct {
                        n.attr.insert("fieldName".to_owned(),s);
                    }
                    node_e.child.push(n);
                    loop_args_break_cont!(ts,"struct init",CurlyRight);
                }
            }
            TokenKind::CurlyLeft => {
                ts.flush(0);
                // Concatenation operator
                if is_first {
                    node_e.kind = AstNodeKind::Concat;
                    loop {
                        node_e.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                        loop_args_break_cont!(ts,"concatenation",CurlyRight);
                    }
                    // println!("{}", node_e);
                }
                // Check for replication operator
                else if (node_e.kind==AstNodeKind::Value || node_e.kind==AstNodeKind::Identifier || node_e.kind==AstNodeKind::ExprGroup) && cntxt == ExprCntxt::FieldList {
                    node_e.kind = AstNodeKind::Replication;
                    loop {
                        node_e.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                        loop_args_break_cont!(ts,"replication",CurlyRight);
                    }

                } else {
                    return Err(SvError::syntax(t, "expression".to_owned()));
                }
            }
            TokenKind::CurlyRight  => {
                if cnt_c == 0 {
                    if cntxt==ExprCntxt::FieldList {
                        ts.rewind(1); // reset to } token to be used by caller
                        break;
                    } else {
                        return Err(SvError::syntax(t, "expression".to_owned()));
                    }
                }
                cnt_c -= 1
            }
            TokenKind::SquareLeft  => cnt_s += 1,
            TokenKind::SquareRight => {
                if cnt_s == 0 {
                    if cntxt == ExprCntxt::BracketMsb || cntxt == ExprCntxt::BracketLsb {
                        ts.rewind(1);
                        break;
                    } else {
                        return Err(SvError::syntax(t, "expression".to_owned()));
                    }
                }
                cnt_s -= 1
            }
            TokenKind::OpRange if cntxt == ExprCntxt::BracketMsb => {ts.rewind(1);break;},
            TokenKind::Colon if cntxt == ExprCntxt::BracketMsb   => {ts.rewind(1);break;},
            TokenKind::Colon if cntxt == ExprCntxt::Question     => {ts.rewind(1);break;},
            TokenKind::ParenLeft   => {
                ts.flush(1); // Consume left parenthesis
                node_e.kind = AstNodeKind::ExprGroup;
                node_e.child.push(parse_expr(ts,ExprCntxt::ExprGroup,false)?);
                ts.flush(1); // Consume right parenthesis
            },
            TokenKind::ParenRight  => {
                if cnt_p == 0 {
                    match cntxt {
                        ExprCntxt::ArgList | ExprCntxt::Arg | ExprCntxt::ExprGroup | ExprCntxt::Sensitivity => {
                            ts.rewind(1);
                            break;
                        },
                        _ => return Err(SvError::syntax(t, "expression".to_owned()))
                    }
                }
                ts.flush(1);
                cnt_p -= 1
            }
            TokenKind::KwOr if cntxt==ExprCntxt::Sensitivity => {ts.rewind(1);break;},
            TokenKind::Casting => {
                node_e = AstNode::new(AstNodeKind::ExprGroup);
                node_e.attr.insert("casting".to_owned(),t.value);
                ts.flush(1); // Consume Casting operator
                expect_t!(ts,"casting expression",TokenKind::ParenLeft);
                node_e.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                ts.flush(1); // Consume right parenthesis
            }
            //
            TokenKind::KwNew if is_first => {
                node_e.kind = AstNodeKind::New;
                // Allow array
                ts.flush(1);
                // println!("[parse_expr] new followed by {}", t);
                parse_opt_slice(ts,&mut node_e,false)?;
                t = next_t!(ts,true);
                if t.kind == TokenKind::ParenLeft {
                    ts.flush(1);
                    t = next_t!(ts,true);
                    if t.kind != TokenKind::ParenRight {
                        ts.rewind(1);
                        loop {
                            node_e.child.push(parse_expr(ts,ExprCntxt::ArgList,false)?);
                            t = next_t!(ts,false);
                            match t.kind {
                                TokenKind::Comma => {},
                                TokenKind::ParenRight => break,
                                _ => return Err(SvError::syntax(t, "new arguments. Expecting , or )".to_owned()))
                            }
                        }
                    } else {
                        ts.flush(1);
                    }
                    t = next_t!(ts,true);
                }
                // Check that the statement will finish on next token and rewind
                match t.kind {
                    TokenKind::SemiColon  if cntxt==ExprCntxt::Stmt || cntxt==ExprCntxt::StmtList => {},
                    TokenKind::ParenRight if cntxt==ExprCntxt::Arg  || cntxt==ExprCntxt::ArgList  => {},
                    TokenKind::Comma      if cntxt==ExprCntxt::StmtList || cntxt==ExprCntxt::ArgList => {},
                    _ => return Err(SvError::syntax(t, "new statement.".to_owned()))
                }
                ts.rewind(0);
                allow_ident = false;
                // TODO : either check foollowing is a context terminator
            }
            //
            TokenKind::KwSuper |
            TokenKind::KwThis |
            TokenKind::Ident => {
                if ! allow_ident {
                    match t.value.as_ref() {
                        "fs" |"ps" |"ns" |"us" |"ms" | "s" => {
                            match prev_kind {
                                TokenKind::Integer | TokenKind::Real => {
                                    node_e.attr.insert("value".to_owned(), t.value);
                                    ts.flush(1);
                                }
                                _ => return Err(SvError::syntax(t, "expression. Unexpected identifier".to_owned()))
                            };
                        }
                        _ => return Err(SvError::syntax(t, "expression.".to_owned()))
                    }
                } else {
                    if node_e.kind == AstNodeKind::Expr {
                        node_e = parse_member_or_call(ts,true)?;
                    } else {
                        node_e.child.push(parse_member_or_call(ts,true)?);
                    }
                    allow_ident = false;
                }
            },
            TokenKind::Macro if allow_ident => {
                parse_macro(ts,&mut node_e)?;
                allow_ident = false;
            }
            TokenKind::Str     |
            TokenKind::Integer |
            TokenKind::KwNull  |
            TokenKind::Real     if allow_ident => {
                node_e.kind = AstNodeKind::Value;
                node_e.attr.insert("value".to_owned(), t.value);
                allow_ident = false;
                ts.flush(1);
            }
            TokenKind::Integer if prev_kind==TokenKind::Integer => {
                if t.value.starts_with('\'') {
                    node_e.attr.insert("value".to_owned(), format!("{}{}", node_e.attr["value"],t.value));
                    ts.flush(1);
                } else {
                    return Err(SvError::syntax(t, "expression".to_owned()))
                }
            }
            // Operator with one operand
            // TokenKind::OpBang => {
            TokenKind::OpBang if is_first => {
                allow_ident = false;
                ts.flush(1);
                node_e.kind = AstNodeKind::Operation;
                node_e.attr.insert("kind".to_owned(),t.value.clone());
                node_e.child.push(parse_expr(ts,cntxt.clone(),false)?);
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
            TokenKind::OpXnor      => {
                allow_ident = false;
                ts.flush(1);
                if is_first {
                    node_e.kind = AstNodeKind::Operation;
                } else {
                    let node_lhs = node_e;
                    node_e = AstNode::new(AstNodeKind::Operation);
                    node_e.child.push(node_lhs);
                }
                node_e.attr.insert("kind".to_owned(),t.value.clone());
                node_e.child.push(parse_expr(ts,cntxt.clone(),false)?);
                // println!("{}", node_e);
            },
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
            TokenKind::OpDiffQue  => {
                let node_lhs = node_e;
                node_e = AstNode::new(AstNodeKind::Operation);
                node_e.attr.insert("kind".to_owned(),t.value.clone());
                ts.flush(1);
                node_e.child.push(node_lhs);
                node_e.child.push(parse_expr(ts,cntxt.clone(),false)?);
                // println!("Two Operand : {}", node_e);
                allow_ident = false;
            },
            TokenKind::KwInside if !is_first => {
                let node_lhs = node_e;
                node_e = AstNode::new(AstNodeKind::Operation);
                node_e.attr.insert("kind".to_owned(),t.value.clone());
                ts.flush(1);
                node_e.child.push(node_lhs);
                expect_t!(ts,"inside expression",TokenKind::CurlyLeft);
                let mut node_rhs = AstNode::new(AstNodeKind::ExprGroup);
                loop {
                    node_rhs.child.push(parse_expr(ts,ExprCntxt::FieldList,false)?);
                    loop_args_break_cont!(ts,"inside expression",CurlyRight);
                }
                node_e.child.push(node_rhs);
                // println!("{}", node_e);
                allow_ident = false;
            }
            TokenKind::KwWith if !is_first => {
                t = next_t!(ts,true);
                // TODO : handle case of randomize with() {}
                match t.kind {
                    TokenKind::CurlyLeft => {
                        ts.rewind(0);
                        parse_constraint(ts,&mut node_e)?;
                    }
                    TokenKind::ParenLeft => {
                        ts.flush(2);
                        node_e.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                        ts.flush(1);
                    }
                    _ => return Err(SvError::syntax(t, "with constraint. Expecting ( or {".to_owned()))
                }
                allow_ident = false;
            }
            TokenKind::OpIncrDecr => {
                if is_first {
                    node_e.attr.insert("incr_decr".to_owned(),t.value);
                    node_e.attr.insert("op".to_owned(),"pre".to_owned());
                    allow_ident = true;
                }
                else {
                    node_e.attr.insert("incr_decr".to_owned(),t.value);
                    node_e.attr.insert("op".to_owned(),"post".to_owned());
                    allow_ident = false;
                }
                ts.flush(1);
            }
            // Composed assignement are allowed in expression when surrounded by parenthesis
            TokenKind::OpCompAss if cntxt==ExprCntxt::ExprGroup => {
                let node_lhs = node_e;
                node_e = AstNode::new(AstNodeKind::Operation);
                node_e.attr.insert("kind".to_owned(),t.value.clone());
                ts.flush(1);
                node_e.child.push(node_lhs);
                node_e.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                expect_t!(ts,"composed assignement expression",TokenKind::ParenRight);
                break;
            }
            //
            TokenKind::Que if !is_first => {
                let node_cond = node_e;
                node_e = AstNode::new(AstNodeKind::Branch);
                node_e.attr.insert("kind".to_owned(),"?".to_owned());
                node_e.child.push(node_cond);
                ts.flush(1);
                // Parse first expression
                node_e.child.push(parse_expr(ts,ExprCntxt::Question,false)?);
                ts.flush(1); // Consume the :
                // Parse second expression
                node_e.child.push(parse_expr(ts,cntxt.clone(),false)?);
                break;
            }
            //
            TokenKind::SystemTask => {
                node_e.child.push(parse_system_task(ts)?);
                allow_ident = false;
            }
            //
            TokenKind::TypeIntVector if allow_type => {
                node_e.kind = AstNodeKind::Type;
                node_e.attr.insert("type".to_owned(), t.value.clone());
                ts.flush(1);
                t = next_t!(ts,true);
                if t.kind == TokenKind::SquareLeft {
                    ts.flush(1);
                    node_e.attr.insert("unpacked".to_owned(), parse_range(ts)?);
                } else {
                    ts.rewind(1);
                }
                break; // next character should be , or ) : no need to consume it, will be checked by caller
            }
            TokenKind::TypeIntAtom |
            TokenKind::TypeReal    |
            TokenKind::TypeString  |
            TokenKind::TypeCHandle if allow_type => {
                node_e.kind = AstNodeKind::Type;
                node_e.attr.insert("type".to_owned(), t.value.clone());
                ts.flush(1);
                break; // next character should be , or ) : no need to consume it, will be checked by caller
            }
            // Allowed keywords
            _ => return Err(SvError::syntax(t, "expression".to_owned()))
        }
        is_first = false;
        prev_kind = t.kind;
    }
    // Final checks before returning the expression
    if cnt_c != 0 {
        return Err(SvError::syntax(t, "Unbalanced curly braces".to_owned()));
    }
    if cnt_p != 0 {
        return Err(SvError::syntax(t, "Unbalanced parenthesis".to_owned()));
    }
    if cnt_s != 0 {
        return Err(SvError::syntax(t, "Unbalanced square bracket".to_owned()));
    }
    // println!("[Expr] {}", node_e);
    // ts.display_status("parse_expr done");
    Ok(node_e)
}

pub fn parse_member_or_call(ts : &mut TokenStream, is_first: bool) -> Result<AstNode, SvError> {
    let mut n = AstNode::new(AstNodeKind::Identifier);
    // ts.display_status("parse_member_or_call: start");
    ts.rewind(0);
    let mut t = next_t!(ts,false);
    if t.kind!=TokenKind::Ident && t.kind!=TokenKind::KwNew {
        if (t.kind != TokenKind::KwThis && t.kind != TokenKind::KwSuper) || !is_first {
            return Err(SvError::syntax(t, "member. Expecting identifier".to_owned()));
        }
    }
    let name = t.value;
    // ts.display_status("parse_member_or_call");
    t = next_t!(ts,true);
    // println!("[parse_member_or_call] next token = {}", t);
    // Could be a static method call from parameterized class
    if t.kind == TokenKind::Hash && is_first {
        ts.flush(1);

        let mut ns = AstNode::new(AstNodeKind::Scope);

        parse_func_call(ts, &mut ns, true)?;

        t = next_t!(ts,true);
        // println!("[parse_member_or_call] static method call from parameterized class : next token = {}", t);
        match t.kind {
            TokenKind::Scope => {
                ns.attr.insert("name".to_owned(),name.clone());
                ts.flush(1);
                parse_opt_scope(ts,&mut ns)?;
                n.child.push(ns);
                t = expect_t!(ts,"type",TokenKind::Ident);
                n.attr.insert("name".to_owned(),t.value);
                t = next_t!(ts,true);
            }
            TokenKind::Comma | TokenKind::ParenRight => {
                n = ns;
                n.kind = AstNodeKind::Type;
                ts.rewind(1);
                return Ok(n);
            }
            TokenKind::Ident => {
                n = ns; // Might need to update some attr of ns ? Kind ?
                n.kind = AstNodeKind::Declaration;
                ts.rewind(1);
                parse_var_decl_name(ts, &mut n)?;
                loop {
                    t = next_t!(ts,true);
                    // println!("[parse_member_or_call] Next token : {}", t);
                    match t.kind {
                        TokenKind::Comma => ts.flush(1), // Comma indicate a list -> continue
                        TokenKind::SemiColon => break, // Semi colon indicate end of statement, stop the loop
                        _ => return Err(SvError::syntax(t, "signal declaration, expecting , or ;".to_owned()))
                    }
                    let mut node_m = AstNode::new(AstNodeKind::Declaration);
                    parse_var_decl_name(ts, &mut node_m)?;
                    n.child.push(node_m);
                }
                ts.rewind(0);
                // ts.display_status("parse_member_or_call: Member declaration");
                return Ok(n);
            }
            _ => return Err(SvError::syntax(t, "statement. Expecting scope/type".to_owned()))
        }

    }
    else if t.kind == TokenKind::Scope {
        ts.flush(1);
        let mut ns = AstNode::new(AstNodeKind::Scope);
        ns.attr.insert("name".to_owned(),name.clone());
        parse_opt_scope(ts,&mut ns)?;
        n.child.push(ns);
        t = expect_t!(ts,"type",TokenKind::Ident);
        n.attr.insert("name".to_owned(),t.value);
        t = next_t!(ts,true);
    } else {
        n.attr.insert("name".to_owned(),name);
    }
    // Check for function call
    if t.kind == TokenKind::ParenLeft {
        n.kind = AstNodeKind::MethodCall;
        ts.rewind(0);
        parse_func_call(ts, &mut n, false)?;
        t = next_t!(ts,true);
    }
    // ts.display_status("parse_member_or_call: post parse_func_call");
    // Check for array selection
    if t.kind == TokenKind::SquareLeft {
        ts.flush(1); // Consume token
        let s = parse_range(ts)?;
        n.attr.insert("range".to_owned(),s);
        t = next_t!(ts,true);
    }
    // Check for members
    if t.kind == TokenKind::Dot {
        ts.flush(1);
        n.child.push(parse_member_or_call(ts, false)?);
    }
    ts.rewind(1);
    // ts.display_status("parse_member_or_call: end");
    // println!("[parse_member_or_call] {}", n);
    Ok(n)
}

pub fn parse_system_task(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut n = AstNode::new(AstNodeKind::SystemTask);
    ts.rewind(0);
    let mut t = expect_t!(ts,"system task",TokenKind::SystemTask);
    let mut name = t.value.clone();
    // Handle special cases
    match t.value.as_ref() {
        "$test" | "$value" => {
            t = expect_t!(ts,"system task $plusargs",TokenKind::SystemTask);
            if t.value != "$plusargs" {
                return Err(SvError::syntax(t, "system task. Expecting $plusargs".to_owned()))
            }
            name.push_str(&t.value);
        }
        "$async" => {
            t = expect_t!(ts,"system task $plusargs",TokenKind::SystemTask);
            if t.value != "$and" && t.value != "$nand" && t.value != "$or" && t.value != "$nor" {
                return Err(SvError::syntax(t, "system task. Expecting $and/$nand/$or/$nor".to_owned()))
            }
            name.push_str(&t.value);
            t = expect_t!(ts,"system task $plusargs",TokenKind::SystemTask);
            if t.value != "$array" && t.value != "$plane" {
                return Err(SvError::syntax(t, "system task. Expecting $array/$plane".to_owned()))
            }
            name.push_str(&t.value);
        }
        _ => {},
    }

    t = next_t!(ts,true);
    if t.kind==TokenKind::ParenLeft {
        ts.flush(1);
        loop {
            n.child.push(parse_expr(ts,ExprCntxt::ArgList,false)?);
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
    let mut t = next_t!(ts,false);
    if t.kind!=TokenKind::ParenLeft {
        return Err(SvError::syntax(t,"function call. Expecting open parenthesis".to_owned()));
    }
    loop {
        t = next_t!(ts,true);
        // println!("[parse_func_call] Token {}", t);
        match t.kind {
            TokenKind::Dot => {
                allow_list = false;
                ts.flush(0); // Consume the dot
                let mut nt = expect_t!(ts,"function argument name",TokenKind::Ident);
                let mut node_p = AstNode::new( {AstNodeKind::Port});
                node_p.attr.insert("name".to_owned(), nt.value);
                nt = next_t!(ts,true);
                match nt.kind {
                    TokenKind::ParenLeft => {
                        ts.flush(1); // Consume the open parenthesis
                        node_p.child.push(parse_expr(ts,ExprCntxt::Arg,false)?);
                        ts.flush(1); // clear up last token (close parenthesis)
                        node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
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
                    _ => return Err(SvError::syntax(t,"function arg. Expecting open parenthesis".to_owned()))
                }
            },
            // Allow type for param
            TokenKind::TypeIntVector |
            TokenKind::TypeIntAtom if is_param => {
                ts.flush(1);
                let mut node_p = AstNode::new( {AstNodeKind::Param});
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("type".to_owned(), t.value);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                t = next_t!(ts,true);
                if t.kind == TokenKind::KwSigning {
                    node_p.attr.insert("signing".to_owned(), t.value);
                    ts.flush(1);
                } else {
                    ts.rewind(1);
                }
                node.child.push(node_p);
            }
            TokenKind::TypeReal    |
            TokenKind::TypeString  |
            TokenKind::TypeCHandle if is_param => {
                ts.flush(1);
                let mut node_p = AstNode::new( {AstNodeKind::Param});
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("type".to_owned(), t.value);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                node.child.push(node_p);
            }
            TokenKind::KwVirtual if is_param => {
                let mut s = t.value;
                ts.flush(1);
                t = next_t!(ts,false);
                if t.kind == TokenKind::KwIntf {
                    s.push_str(" interface");
                    t = next_t!(ts,false);
                }
                if t.kind!=TokenKind::Ident && t.kind!=TokenKind::Macro {
                    return Err(SvError::syntax(t,"function arg. Expecting port name".to_owned()));
                }
                s.push_str(&t.value);
                let mut node_p = AstNode::new( {AstNodeKind::Param});
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("value".to_owned(), s);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                parse_opt_params!(ts,node_p);
                // println!("Virtual interface param : {}", node_p);
                node.child.push(node_p);
            }
            TokenKind::KwNull => {
                ts.flush(1);
                let mut node_p = AstNode::new( {AstNodeKind::Port});
                node_p.attr.insert("name".to_owned(), "".to_owned());
                node_p.attr.insert("value".to_owned(), t.value);
                node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                node.child.push(node_p);
            }
            //
            _ => {
                // ordered connection
                if allow_list {
                    ts.rewind(0);
                    let mut node_p = AstNode::new( {AstNodeKind::Port});
                    node_p.child.push(parse_expr(ts,ExprCntxt::ArgList,false)?);
                    node_p.attr.insert("name".to_owned(), "".to_owned());
                    node_p.attr.insert("pos".to_owned(), format!("{}",cnt));
                    node.child.push(node_p);
                    cnt += 1;
                    ts.rewind(0); //
                } else {
                    return Err(SvError::syntax(t, "port connection".to_owned()));
                }
            }
        }
        loop_args_break_cont!(ts,"argument list",ParenRight);
    }
    // ts.display_status("parse_func_call");
    Ok(())
}

// Parse a time value
pub fn parse_time(ts : &mut TokenStream) -> Result<String,SvError> {
    let t1 = next_t!(ts,false);
    if t1.kind!=TokenKind::Integer && t1.kind!=TokenKind::Real {
        return Err(SvError::syntax(t1,"timescale. Expecting time value (integer or floating)".to_owned()));
    }
    let t2 = next_t!(ts,false);
    if t2.kind!=TokenKind::Ident {
        return Err(SvError::syntax(t2,"timescale. Expecting time unit".to_owned()));
    }
    match t2.value.as_ref() {
        "fs" |"ps" |"ns" |"us" |"ms" | "s" => {},
        _ => return Err(SvError::syntax(t2,"timescale. Expecting fs, ps, ns, ...".to_owned()))
    }
    Ok(format!("{}{}",t1.value,t2.value))
}

/// Parse constraint block
/// Temporary implementation : just get the name of the constraint
pub fn parse_constraint(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut n = AstNode::new(AstNodeKind::Constraint);
    let t = next_t!(ts,false);
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
    let mut n = AstNode::new(AstNodeKind::Covergroup);
    ts.flush(1); // Consume the covergroup word
    let t = expect_t!(ts,"covergroup",TokenKind::Ident);
    n.attr.insert("name".to_owned(),t.value);
    ts.skip_until(TokenKind::KwEndGroup)?;
    check_label(ts, &n.attr["name"])?;
    node.child.push(n);
    Ok(())
}