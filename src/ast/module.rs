
use crate::error::{SvErrorKind, SvError, };
use crate::token::{TokenKind};
use crate::tokenizer::TokenStream;
use crate::ast::astnode::*;

/// This function should be called after a keyword module/macromodule
pub fn parse_module_hdr(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    // First extract next token: can be lifetime or identifier.
    // let t = ts.next_non_comment(false);
    let mut name : String;
    let mut has_lifetime : Option<TokenKind> = None;
    let mut t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("module header".to_string())))?;

    if t.kind==TokenKind::KwStatic || t.kind==TokenKind::KwAutomatic {
        has_lifetime = Some(t.kind);
        t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("module header".to_string())))?;
    }
    match t.kind {
        TokenKind::Ident => {
            name = t.value.clone();
            t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("module header".to_string())))?;
        },
        _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        "Expecting module name or lifetime (static/automatic)".to_string()))
    }
    let mut node = AstNode::new(AstNodeKind::Module(name));
    // Add lifetime info
    if has_lifetime.is_some() {
        node.attr.insert("lifetime".to_string(),format!("{:?}", has_lifetime.unwrap()));
    }
    // Optional package import
    let mut s="".to_string();
    while t.kind == TokenKind::KwImport {
        loop {
            t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("import".to_string())))?;
            if t.kind!=TokenKind::Ident {
                return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Expecting package identifier, got {} ({:?})",t.value, t.kind)));
            }
            s=t.value;
            t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("import".to_string())))?;
            if t.kind!=TokenKind::Scope {
                return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Expecting scope operator, got {} ({:?})",t.value, t.kind)));
            }
            t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("import".to_string())))?;
            if t.kind!=TokenKind::Ident && t.kind!=TokenKind::OpStar {
                return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Expecting identifier or * in package import, got {} ({:?})",t.value, t.kind)));
            }
            s.push_str(&t.value);
            t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("import".to_string())))?;
            match t.kind {
                TokenKind::SemiColon => break,
                TokenKind::Comma => {},
                _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                                format!("Expecting , or ; after package import, got {} ({:?})",t.value, t.kind)))
            }
            s.push_str(&t.value);
        }
        t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("import".to_string())))?;
    }
    if s.len() > 0 {
        node.attr.insert("imports".to_string(),s);
    }
    // Optional parameter list
    if t.kind==TokenKind::Hash {
        t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;
        if t.kind!=TokenKind::ParenLeft {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting ( after #, got {} ({:?})",t.value, t.kind)));
        }
        loop {
            let node_port = parse_param_decl(ts)?;
            node.child.push(node_port);
            t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("param declaration".to_string())))?;
            match t.kind {
                // Comma -> the port list continue
                TokenKind::Comma => {},
                // Right parenthesis, port list is done, get next token
                TokenKind::ParenRight => {
                    t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("param declaration".to_string())))?;
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
            node.child.push(node_port);
            t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;
            match t.kind {
                // Comma -> the port list continue
                TokenKind::Comma => {},
                // Right parenthesis, port list is done, get next token
                TokenKind::ParenRight => {
                    t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;
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
    println!("{:?}", node);
    Ok(node)
}

/// Parse a port declaration
fn parse_param_decl(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("param declaration".to_string())))?;
    let mut node = AstNode::new(AstNodeKind::Root);
    // optionnal keyword param/localparam
    match t.kind {
        TokenKind::KwParam | TokenKind::KwLParam  => {ts.flush();},
        _ => {}
    }

    // Optional data type
    parse_data_type(ts,&mut node)?;
    t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("param declaration".to_string())))?;
    // Port name
    node.kind = AstNodeKind::Param(t.value);
    // Optional Unpacked dimension : [x][y:z]
    // Default value i.e. "= expr"
    t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("param declaration".to_string())))?;
    if t.kind != TokenKind::OpEq {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting ; in port declaration, got {}", t.value)))
    }
    let mut prev_tk = t.kind;
    let mut s = "".to_string();
    let mut cnt_c = 0;
    let mut cnt_s = 0;
    let mut cnt_p = 1; // Init to 1 to take intot account the openeing one for parameter declaration
    loop {
        t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("data type".to_string())))?;
        match t.kind {
            TokenKind::SemiColon   => return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unexpected semi-colon in parameter declaration".to_string())),
            TokenKind::CurlyLeft   => cnt_c += 1,
            TokenKind::CurlyRight  => {
                if cnt_c == 0 {return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced curly braces".to_string()));}
                cnt_c -= 1
            }
            TokenKind::SquareLeft  => cnt_s += 1,
            TokenKind::SquareRight => {
                if cnt_s == 0 {return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced square braces".to_string()));}
                cnt_s -= 1
            }
            TokenKind::ParenLeft   => cnt_p += 1,
            TokenKind::ParenRight  => {
                if cnt_p == 0 {return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced parenthesis".to_string()));}
                cnt_p -= 1
            }
            TokenKind::Ident => {
                if prev_tk==TokenKind::Ident {
                    return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Invalid packed dimension".to_string()));
                }
            },
            _ => {}
        }
        if t.kind == TokenKind::Comma && cnt_c==0 || t.kind == TokenKind::ParenRight && cnt_p==0 {
            if t.kind == TokenKind::Comma && cnt_p != 1 {
                return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced parenthesis".to_string()));
            }
            if cnt_s > 0 {
                return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced square brace".to_string()));
            }
            node.attr.insert("default".to_string(), s);
            break;
        }
        // Always flush buffer, we really just need to keep the last token read
        else {
            ts.flush()
        }
        s.push_str(&t.value);
        prev_tk = t.kind;
    }

    println!("{:?}", node);
    Ok(node)
        // _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
        //                 "Port declaration, expecting direction/interface.".to_string()))
}

/// Parse a port declaration
fn parse_port_decl(ts : &mut TokenStream) -> Result<AstNode, SvError> {
    let mut t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;
    let mut node = AstNode::new(AstNodeKind::Root);
    // direction/interface
    match t.kind {
        TokenKind::KwInput | TokenKind::KwOutput | TokenKind::KwInout | TokenKind::KwRef => {
            ts.flush();
            t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;
        },
        // Handle interface keyword
        // Handle Ident
        _ => {}
    }

    // Optional net type
    match t.kind {
        TokenKind::KwSupply | TokenKind::KwNetType => {ts.flush()},
        _ => {}
    }
    // Optional data type
    parse_data_type(ts,&mut node)?;
    // Port name
    t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;
    if t.kind != TokenKind::Ident {
        return Err(SvError::new(SvErrorKind::Syntax, t.pos,
                        format!("Expecting port identifier, got {} ({:?})", t.value, t.kind)))
    }
    node.kind = AstNodeKind::Port(t.value);
    // Optional Unpacked dimension : [x][y:z]
    t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;
    if t.kind == TokenKind::SquareLeft {
        ts.flush();
        node.attr.insert("unpacked".to_string(), parse_range(ts)?);
    }
    // Optional Default value i.e. "= expr"
    // t = ts.next_non_comment(false).unwrap_or(Err(SvError::eof("port declaration".to_string())))?;

    println!("{:?}", node);
    Ok(node)
        // _ => return Err(SvError::new(SvErrorKind::Syntax, t.pos,
        //                 "Port declaration, expecting direction/interface.".to_string()))
}

#[allow(unused_variables, unused_assignments)]
/// Parse a data type
fn parse_data_type(ts : &mut TokenStream, node : &mut AstNode) -> Result<(), SvError> {
    let mut has_signing = true;
    let mut has_width   = true;
    let mut get_next    = false;
    ts.rewind(); // Ensure we start analyzing data from
    let mut t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("data type".to_string())))?;;
    // First word of a data type
    match t.kind {
        // Integer vector type -> has signing and packed dimension
        TokenKind::TypeIntVector => {get_next = true;},
        TokenKind::TypeIntAtom   => {get_next = true; has_width=false},
        TokenKind::TypeReal    |
        TokenKind::TypeString  |
        TokenKind::TypeCHandle |
        TokenKind::TypeEvent   => {get_next = true; has_width=false; has_signing=false},
        _ => {},
    }
    if get_next {
        node.attr.insert("type".to_string(), t.value);
        get_next = false;
        ts.flush(); // TODO: maybe a simple pop !
        t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("data type".to_string())))?;
    }
    //
    if has_signing  && t.kind == TokenKind::KwSigning {
        node.attr.insert("signing".to_string(), t.value);
        ts.flush(); // TODO: maybe a simple pop !
        t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("data type".to_string())))?;
    }
    if has_width  && t.kind == TokenKind::SquareLeft {
        // TODO: make a function out of that, should be useful

        let mut cnt_s = 1;
        let mut cnt_p = 0;
        let mut s = "[".to_string();
        let mut prev_tk = t.kind;
        while cnt_s > 0 {
            t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("data type".to_string())))?;
            match t.kind {
                TokenKind::SquareLeft  => cnt_s += 1,
                TokenKind::SquareRight => cnt_s -= 1,
                TokenKind::ParenLeft   => cnt_p += 1,
                TokenKind::ParenRight  => cnt_p -= 1,
                TokenKind::SemiColon => return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unexpected semi-colon in packed dimension".to_string())),
                TokenKind::Ident => {
                    if prev_tk==TokenKind::Ident {
                        return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Invalid packed dimension".to_string()));
                    }
                },
                _ => {}
            }
            prev_tk = t.kind;
            s.push_str(&t.value);
        }
        if cnt_p > 0 {
            return Err(SvError::new(SvErrorKind::Syntax, t.pos, "Unbalanced parenthesis".to_string()));
        }
        // Add packed dimension to the port attribute and retrieve the next token
        node.attr.insert("packed".to_string(), s);
        ts.flush(); // TODO: maybe a simple pop !
    }
    //
    ts.rewind(); // Put back everything we read and did not used
    Ok(())
}

fn parse_range(ts : &mut TokenStream) -> Result<String,SvError> {
    let mut s = "[".to_string();
    let mut cnt_s = 1;
    let mut cnt_p = 0;
    let mut prev_tk = TokenKind::SquareLeft;
    loop {
        let t = ts.next_non_comment(true).unwrap_or(Err(SvError::eof("data type".to_string())))?;
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
        ts.flush(); // Token consumed, can flush it
    }
    Ok(s)
}