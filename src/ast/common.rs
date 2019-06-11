
use crate::token::TokenKind;
use crate::tokenizer::TokenStream;
use crate::ast::astnode::{AstNode, AstNodeKind};
use crate::error::{SvErrorKind, SvError};

macro_rules! next_t {
    ($ts:expr, $peek:expr) => {{
        $ts.next_non_comment($peek).unwrap_or(Err(SvError::eof()))?
    }};
}

#[allow(dead_code)]
#[derive(PartialEq,Debug)]
pub enum ExprCntxt {
    PortList, Port,
    StmtList, Stmt,
    FieldList,
}

/// Parse an import statement (suppose previous token was the import keyword !)
pub fn parse_import(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.flush(0);
    let mut s="".to_string();
    loop {
        let mut t = next_t!(ts,false);
        if t.kind!=TokenKind::Ident {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Expecting package identifier, got {} ({:?})",t.value, t.kind)));
        }
        s.push_str(&t.value);
        t = next_t!(ts,false);
        if t.kind!=TokenKind::Scope {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Expecting scope operator, got {} ({:?})",t.value, t.kind)));
        }
        s.push_str("::");
        t = next_t!(ts,false);
        if t.kind!=TokenKind::Ident && t.kind!=TokenKind::OpStar {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Expecting identifier or * in package import, got {} ({:?})",t.value, t.kind)));
        }
        s.push_str(&t.value);
        t = next_t!(ts,false);
        match t.kind {
            TokenKind::SemiColon => break,
            TokenKind::Comma => {},
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Expecting , or ; after package import, got {} ({:?})",t.value, t.kind)))
        }
        s.push_str(", ");
    }
    let mut n = AstNode::new(AstNodeKind::Import);
    n.attr.insert("pkg".to_string(),s);
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
            node.attr.insert("kind".to_string(), format!("{:?}",t.kind ) );
            ts.flush(0);
        },
        _ => {}
    }

    // Optional data type
    parse_data_type(ts,&mut node)?;
    t = next_t!(ts,false);
    // Parameter name
    if t.kind != TokenKind::Ident {
        return Err(SvError::syntax(t, "param declaration, expecting identifier".to_string()));
    }
    node.attr.insert("name".to_string(), ".*".to_string());
    // Optional Unpacked dimension : [x][y:z]
    t = next_t!(ts,false);
    if t.kind == TokenKind::SquareLeft {
        ts.flush(0);
        node.attr.insert("unpacked".to_string(), parse_range(ts)?);
        t = next_t!(ts,false);
    }

    // Default value i.e. "= expr"
    if t.kind != TokenKind::OpEq {
        return Err(SvError::syntax(t, "param declaration, expecting =".to_string()));
    }
    let cntxt = if is_body {ExprCntxt::StmtList} else {ExprCntxt::PortList};
    let s = parse_expr(ts,cntxt)?;
    node.attr.insert("default".to_string(), s);
    // println!("{}", node);
    Ok(node)
}

/// Parse a list of signal declaration
pub fn parse_signal_decl_list(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    ts.rewind(0);
    // ts.display_status();
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
            _ => return Err(SvError::syntax(nt, "in signal declaration, expecting , or ;".to_string()))
        }
    }
    // println!("[parse_signal_decl_list] {}", node);
    ts.rewind(0); // put back any token we have not used
    // ts.display_status();
    Ok(())
}

/// Parse a signal declaration
pub fn parse_signal_decl(ts : &mut TokenStream, has_type: bool) -> Result<AstNode, SvError> {
    let mut t;
    let mut node = AstNode::new(AstNodeKind::Root);
    if has_type {
        // Parse potential net type
        parse_net_type(ts,&mut node)?;
        // Parse data type
        parse_data_type(ts,&mut node)?;
    }
    // Signal name
    t = next_t!(ts,false);
    if t.kind != TokenKind::Ident {
        return Err(SvError::syntax(t, "signal declaration, expecting identifier".to_string()))
    }
    node.kind = AstNodeKind::Signal(t.value);
    // Optional Unpacked dimension : [x][y:z]
    t = next_t!(ts,true);
    if t.kind == TokenKind::SquareLeft {
        ts.flush(0);
        node.attr.insert("unpacked".to_string(), parse_range(ts)?);
        t = next_t!(ts,true);
    }
    // Optional Default value i.e. "= expr"
    if t.kind == TokenKind::OpEq {
        ts.flush(0);
        let s = parse_expr(ts,ExprCntxt::StmtList)?;
        node.attr.insert("init".to_string(), s);
    }
    Ok(node)
}

/// Tentatively parse a net type
pub fn parse_net_type(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let mut t = next_t!(ts,true);
    // println!("[parse_net_type] {}", t);
    if t.kind==TokenKind::KwNetType || t.kind==TokenKind::KwSupply {
        node.attr.insert("nettype".to_string(),t.value);
        ts.flush(0);
        t = next_t!(ts,true);
        // println!("[parse_net_type] next ? {}", t);
        // Check for optional strength
        if t.kind==TokenKind::ParenLeft {
            ts.flush(0);
            parse_strength(ts,node)?;
            t = next_t!(ts,true);
        }
        // Check for optional vector info
        if t.kind==TokenKind::KwVector{
            node.attr.insert("vector".to_string(),t.value);
            ts.flush(0);
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
            node.attr.insert("charge".to_string(),t.value);
        }
        TokenKind::KwDrive | TokenKind::KwSupply => {
            let mut s = t.value;
            t = next_t!(ts,false);
            if t.kind!=TokenKind::Comma {
                return Err(SvError::syntax(t, "drive strength declaration, expecting ,".to_string()))
            }
            s.push(',');
            t = next_t!(ts,false);
            if t.kind!=TokenKind::KwDrive && t.kind!=TokenKind::KwSupply {
                return Err(SvError::syntax(t, "drive strength declaration, expecting ,".to_string()))
            }
            s.push_str(&t.value);
            node.attr.insert("drivee".to_string(),s);
            // TODO: Check combination are actually valid
        }
        _ => return Err(SvError::syntax(t, "strength declaration, expecting drive or charge".to_string()))
    }
    // Done, expecting closing parenthesis
    t = next_t!(ts,false);
    if t.kind!=TokenKind::ParenRight {
        return Err(SvError::syntax(t, "strength declaration, expecting )".to_string()))
    }
    Ok(())
}

/// Parse a data type
pub fn parse_data_type(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
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
        TokenKind::TypeIntVector => {ts.flush(0); get_next = true; }
        TokenKind::TypeIntAtom   => {ts.flush(0); get_next = true; has_width=false}
        TokenKind::TypeReal    |
        TokenKind::TypeString  |
        TokenKind::TypeCHandle |
        TokenKind::TypeEvent   => {ts.flush(0); get_next = true; has_width=false; has_signing=false}
        // Ident -> check next word, could be a user type
        TokenKind::Ident => {
            has_signing = false;
            has_width   = false;
            let nt = next_t!(ts,true);
            // println!("[parse_data_type] Ident followed by {}", t);
            match nt.kind {
                // Scope operator => custom type
                TokenKind::Scope => {
                    let nnt = next_t!(ts,true);
                    if nnt.kind != TokenKind::Ident {
                        return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) as modport",nnt.value, nnt.kind)))
                    }
                    s = format!("{}::{}",s,nnt.value);
                    get_next=true;
                    ts.flush(0);
                }
                // Another ident : t is the type and nt is the port/signal name
                // -> consume first character and put back the one we read
                TokenKind::Ident => {get_next=true; ts.flush(1); ts.rewind(1);}
                // Comma/parenthesis/Equal/semicolon -> t was the port name
                TokenKind::Comma |
                TokenKind::SemiColon |
                TokenKind::OpEq |
                TokenKind::ParenRight => ts.rewind(1),
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in data type.",nt.value, nt.kind)))
            }
        }
        // Sign/Range start (ignore handling now, will be done after)
        TokenKind::KwSigning |
        TokenKind::SquareLeft => {}
        // Any token not listed here is an error
        _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos, format!("Unexpected {} ({:?}) in data type. Expecting type or identifier",t.value, t.kind)))
    }
    // println!("[parse_data_type] ->get next = {} : {}", get_next, t );
    if get_next {
        node.attr.insert("type".to_string(), s);
        t = next_t!(ts,true);
    }
    // println!("[parse_data_type] -> has_sign={} : {}", has_signing, t );
    //
    if has_signing  && t.kind == TokenKind::KwSigning {
        node.attr.insert("signing".to_string(), t.value);
        ts.flush(0); // TODO: maybe a simple pop !
        t = next_t!(ts,true);
    }
    // println!("[parse_data_type] -> has_width={} : {}", has_width, t );
    if has_width  && t.kind == TokenKind::SquareLeft {
        let ws = parse_range(ts)?;
        // Add packed dimension to the port attribute and retrieve the next token
        node.attr.insert("packed".to_string(), ws);
    }
    //
    ts.rewind(1); // Put back last token we did not used
    // ts.display_status();
    Ok(())
}

/// Parse a range
pub fn parse_range(ts : &mut TokenStream) -> Result<String,SvError> {
    let mut s = "[".to_string();
    let mut cnt_s = 1;
    let mut cnt_p = 0;
    let mut prev_tk = TokenKind::SquareLeft;
    loop {
        let t = next_t!(ts,true);
        // println!("[parse_range]  {} (cnt s={}, p={})", t,cnt_s,cnt_p );
        if cnt_s==0 && t.kind != TokenKind::SquareLeft {
            if cnt_p > 0 {
                return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced parenthesis".to_string()));
            }
            break;
        }
        match t.kind {
            TokenKind::SquareLeft  => cnt_s += 1,
            TokenKind::SquareRight => cnt_s -= 1,
            TokenKind::ParenLeft   => cnt_p += 1,
            TokenKind::ParenRight  => {
                if cnt_p == 0 {return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced parenthesis".to_string()));}
                cnt_p -= 1;
            }
            TokenKind::SemiColon => return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unexpected semi-colon in range definition".to_string())),
            TokenKind::Ident => {
                if prev_tk==TokenKind::Ident {
                    return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Invalid range definition".to_string()));
                }
            },
            _ => {}
        }
        prev_tk = t.kind;
        s.push_str(&t.value);
        ts.flush(0); // Token consumed, can flush it
    }
    Ok(s)
}
pub fn parse_ident_list(ts : &mut TokenStream) -> Result<String,SvError> {
    let mut s = "".to_string();
    let mut expect_ident = true;
    loop {
        let t = next_t!(ts,false);
        match t.kind {
            TokenKind::Ident if expect_ident => {
                s.push_str(&t.value);
                expect_ident = false;
            }
            TokenKind::Comma if !expect_ident => {
                s.push_str(&t.value);
                expect_ident = true;
            }
            TokenKind::SemiColon if !expect_ident => break, // Semi colon indicate end of statement, stop the loop
            _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                    format!("Unexpected {} ({:?}) in ident list, expecting identifier/comma/semicolon",t.value, t.kind)))
        }
    }
    Ok(s)
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
            node_e.attr.insert("type".to_string(),s);
        }
        TokenKind::TypeIntVector => {
            let mut s = t.value;
            // Check for optional signing info
            t = next_t!(ts,true);
            // ts.display_status();
            if t.kind == TokenKind::KwSigning {
                s.push_str(&t.value);
                t = next_t!(ts,true);
            }
            // Check for optional dimension
            if t.kind == TokenKind::SquareLeft {
                ts.flush(0);
                s.push_str(&parse_range(ts)?);
                t = next_t!(ts,false);
            }
            node_e.attr.insert("type".to_string(),s);
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
        node_id.attr.insert("name".to_string(), t.value);
        t = next_t!(ts,false);
        // Optional range
        if t.kind == TokenKind::SquareLeft {
            // node_id.attr.insert("range".to_string(), s);
            ts.flush(0);
            t = next_t!(ts,true);
            // unimplemented!();
        }
        // Optional value
        if t.kind == TokenKind::OpEq {
            ts.flush(0);
            let s = parse_expr(ts,ExprCntxt::FieldList)?;
            node_id.attr.insert("init".to_string(), s);
            t = next_t!(ts,false);
        }
        node_e.child.push(node_id);
        // Expect , or }
        match t.kind {
            TokenKind::Comma => {},
            TokenKind::CurlyRight => break,
            _ => return Err(SvError::syntax(t, "enum. Expecting , }".to_string()))
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
            t = next_t!(ts,false);
            if t.kind == TokenKind::KwTagged {

            } else {
                ts.rewind(1);
            }
        },
        _ => return Err(SvError::syntax(t, "struct. Expecting struct or union".to_string()))
    }
    t = next_t!(ts,false);
    // Optional packed keyword
    if t.kind==TokenKind::KwPacked {
        node.attr.insert("packed".to_string(),"".to_string());
        t = next_t!(ts,false);
        // Optional signing
        if t.kind==TokenKind::KwSigning {
            node.attr.insert("signing".to_string(), t.value);
            t = next_t!(ts,false);
        }
    }
    if t.kind!=TokenKind::CurlyLeft {
        return Err(SvError::syntax(t, "struct. Expecting {".to_string()));
    }
    // Loop on type declaration until closing curly brace
    loop {
        t = next_t!(ts,true);
        match t.kind {
            TokenKind::Ident         |
            TokenKind::TypeIntAtom   |
            TokenKind::TypeIntVector |
            TokenKind::TypeReal      |
            TokenKind::TypeString    |
            TokenKind::TypeCHandle   |
            TokenKind::TypeEvent     => parse_signal_decl_list(ts,&mut node)?,
            // anonymous enum
            TokenKind::KwEnum => {
                let mut node_e = parse_enum(ts)?;
                let s = parse_ident_list(ts)?;
                node_e.attr.insert("name".to_string(),s);
                node.child.push(node_e);
            }
            // anonymous struct/union
            TokenKind::KwStruct |
            TokenKind::KwUnion  => {
                let mut node_s = parse_struct(ts)?;
                let s = parse_ident_list(ts)?;
                node_s.attr.insert("name".to_string(),s);
                node.child.push(node_s);
            }
            // End of struct declaration
            TokenKind::CurlyRight => break,
            _ => return Err(SvError::syntax(t, "struct. Expecting data type".to_string())),
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
        TokenKind::Ident         |
        TokenKind::TypeIntAtom   |
        TokenKind::TypeIntVector |
        TokenKind::TypeReal      |
        TokenKind::TypeString    |
        TokenKind::TypeCHandle   |
        TokenKind::TypeEvent     => {
            node_type = AstNode::new(AstNodeKind::Typedef);
            parse_data_type(ts,&mut node_type)?;
        }
        _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                            format!("Unexpected {} ({:?}) in typedef declaration",t.value, t.kind)))
    }
    // Parse type name, followed by semi-colon
    t = next_t!(ts,false);
    if t.kind!=TokenKind::Ident {
        return Err(SvError::syntax(t, "typedef enum. Expecting identifier".to_string()));
    }
    node_type.kind = AstNodeKind::Typedef;
    node_type.attr.insert("name".to_string(),t.value);
    node.child.push(node_type);
    t = next_t!(ts,false);
    if t.kind!=TokenKind::SemiColon {
        return Err(SvError::syntax(t, "package header. Expecting ;".to_string()));
    }
    Ok(())
}


/// Parse an expression
pub fn parse_expr(ts : &mut TokenStream, cntxt: ExprCntxt) -> Result<String, SvError> {
    let mut prev_tk = TokenKind::ParenLeft;
    let mut s = "".to_string();
    let mut cnt_c = 0;
    let mut cnt_s = 0;
    let mut cnt_p = 0;
    let mut t;
    loop {
        t = next_t!(ts,true);
        // println!("[parse_expr] Token = {}, cnt c={}, s={}, p={} (cntxt={:?})", t,cnt_c,cnt_s,cnt_p, cntxt);
        match t.kind {
            // Body => end on semi-colon
            TokenKind::SemiColon if cntxt==ExprCntxt::StmtList || cntxt==ExprCntxt::Stmt => break,
            TokenKind::SemiColon if cntxt!=ExprCntxt::StmtList && cntxt!=ExprCntxt::Stmt  => return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Unexpected semi-colon.".to_string())),
            // End on comma (if not inside curly braces)
            TokenKind::Comma if cnt_c==0 => {
                if cntxt==ExprCntxt::Stmt || cntxt==ExprCntxt::Port  {
                    return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Unexpected comma.".to_string()));
                }
                break;
            }
            // Count parenthesis/bbraces to check if it is balanced
            TokenKind::TickCurly   |
            TokenKind::CurlyLeft   => cnt_c += 1,
            TokenKind::CurlyRight  => {
                if cnt_c == 0 {
                    if cntxt==ExprCntxt::FieldList {
                        break;
                    } else {
                        return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Missing left curly braces".to_string()));
                    }
                }
                cnt_c -= 1
            }
            TokenKind::SquareLeft  => cnt_s += 1,
            TokenKind::SquareRight => {
                if cnt_s == 0 {
                    return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Missing left square braces".to_string()));
                }
                cnt_s -= 1
            }
            TokenKind::ParenLeft   => cnt_p += 1,
            TokenKind::ParenRight  => {
                if cnt_p == 0 {
                    if cntxt==ExprCntxt::PortList || cntxt==ExprCntxt::Port {
                        break;
                    }
                    else {
                        return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Unbalanced parenthesis".to_string()));
                    }
                }
                cnt_p -= 1
            }
            //
            TokenKind::Ident => {
                if prev_tk==TokenKind::Ident {
                    return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Missing operator/semi-colon/ ".to_string()));
                }
            },
            // TODO : white list of allowed token to detect error like keyword and such
            _ => {}
        }
        ts.flush(0);
        s.push_str(&t.value);
        prev_tk = t.kind;
    }
    // Final checks before returning the expression
    if cnt_c != 0 {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Unbalanced curly braces".to_string()));
    }
    if cnt_p != 0 {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Unbalanced parenthesis".to_string()));
    }
    if cnt_s != 0 {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos, "[EXPR] Unbalanced square brace".to_string()));
    }
    // TODO : add more check like ternary operator
    // println!("[parse_expr] {} (last token = {:?})", s,t.kind);
    Ok(s)
}

/// Parse port/parameter connection
/// Stream should start at open parenthesis and will be consumed until the closing parenthesis included
pub fn parse_port_connection(ts : &mut TokenStream, node: &mut AstNode, is_param: bool) -> Result<(), SvError> {
    let mut t = next_t!(ts,false);
    // Allow simple list until a named connection is found
    let mut allow_list = true;
    // Allow dot star if we are not in a parameter port connection
    // Also prevent dot star once we found one
    let mut allow_dot_star = !is_param;
    let mut cnt = 0;
    if t.kind!=TokenKind::ParenLeft {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Expecting open parenthesis".to_string()));
    }
    loop {
        t = next_t!(ts,true);
        // println!("[parse_port_connection] Token {}", t);
        match t.kind {
            TokenKind::Dot => {
                allow_list = false;
                ts.flush(0); // Consume the dot
                let mut nt = next_t!(ts,false);
                if nt.kind!=TokenKind::Ident {
                    return Err(SvError::new(SvErrorKind::Syntax, nt.pos, "Expecting port name".to_string()));
                }
                let mut node_p = AstNode::new( if is_param {AstNodeKind::Param} else {AstNodeKind::Port});
                node_p.attr.insert("name".to_string(), nt.value);
                nt = next_t!(ts,true);
                match nt.kind {
                    TokenKind::ParenLeft => {
                        // ts.flush(0); // Consume the do
                        let s = parse_expr(ts,ExprCntxt::Port)?;
                        ts.flush(0); // clear up last token (close parenthesis)
                        node_p.attr.insert("bind".to_string(), s);
                        node_p.attr.insert("pos".to_string(), format!("{}",cnt));
                        node.child.push(node_p);
                        cnt += 1;
                    }
                    // Implicit named
                    TokenKind::Comma |
                    TokenKind::ParenRight => {
                        node_p.attr.insert("pos".to_string(), format!("{}",cnt));
                        node.child.push(node_p);
                        cnt += 1;
                        ts.rewind(0);
                    }
                    _ => return Err(SvError::new(SvErrorKind::Syntax, nt.pos, "Expecting open parenthesis".to_string()))
                }
            },
            TokenKind::DotStar if allow_dot_star => {
                allow_dot_star = false;
                node.child.push(AstNode::new(AstNodeKind::Port));
                node.attr.insert("name".to_string(), ".*".to_string());
            },
            // Comma -> the list continue
            TokenKind::Comma => ts.flush(0),
            // End of port connection
            TokenKind::ParenRight => break,
            //
            _ => {
                // ordered connection
                if allow_list {
                    ts.rewind(0);
                    let s = parse_expr(ts,ExprCntxt::PortList)?;
                    let mut node_p = AstNode::new( if is_param {AstNodeKind::Param} else {AstNodeKind::Port});
                    node_p.attr.insert("name".to_string(), "".to_string());
                    node_p.attr.insert("bind".to_string(), s);
                    node_p.attr.insert("pos".to_string(), format!("{}",cnt));
                    node.child.push(node_p);
                    cnt += 1;
                    ts.rewind(0); //
                } else {
                    return Err(SvError::syntax(t, "port connection".to_string()));
                }
            }
        }
    }
    ts.flush(0);
    Ok(())
}

/// Parse the optional label after a begin keyword, and update
pub fn parse_label(ts : &mut TokenStream, node: &mut AstNode, attr_name: String) -> Result<bool, SvError> {
    ts.flush(0); // Consume the begin keyword
    let mut t = next_t!(ts,true);
    // println!("[parse_label] Token = : {}", t);
    // Check for named block
    if t.kind == TokenKind::Colon {
        ts.flush(0);
        t = next_t!(ts,false);
        if t.kind!=TokenKind::Ident {
            return Err(SvError::syntax(t, "block name".to_string()))
        }
        node.attr.insert(attr_name, t.value);
        return Ok(true)
    } else {
        ts.rewind(0);
        node.attr.insert(attr_name, "".to_string());
        return Ok(false);
    }
}

/// Parse Macro/Directive
pub fn parse_macro(ts : &mut TokenStream, node: &mut AstNode) -> Result<(), SvError> {
    let mut node_m = AstNode::new(AstNodeKind::Macro);
    ts.rewind(1);
    let mut t = next_t!(ts,true);
    node_m.attr.insert("name".to_string(),t.value.clone());
    match t.value.as_ref() {
        // Directive with no parameters
        "`else" | "`endif"| "`undefineall"| "`resetall"| "`celldefine"| "`endcelldefine" => ts.flush(0),
        // Directive with one parameter
        "`ifndef" | "`ifdef" | "`elsif" | "`undef" => {
            t = next_t!(ts,true);
            if t.kind!=TokenKind::Ident {
                return Err(SvError::syntax(t, "ifdef directive".to_string()))
            }
            node_m.attr.insert("param".to_string(), t.value);
            ts.flush(0);
        }
        // Include directive : `include <file> , `include "file" or `include `mymacro
        // "`include" => {

        // }
        // `default_nettype wire | tri | tri0 | tri1 | wand | triand | wor | trior | trireg | uwire | none
        _ => return Err(SvError::syntax(t, "macro".to_string()))
    }
    // println!("[parse_macro] {}", node_m);
    // ts.display_status();
    node.child.push(node_m);
    Ok(())
}