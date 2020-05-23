use crate::lex::token::{Token,TokenKind::*};

use crate::lex::position::Position;
use crate::ast::{Ast,MacroDef};


pub fn get_uvm_macro() -> Box<Ast> {
    let mut ast = Ast::new(std::path::PathBuf::from("uvm_macro.svh"));
    let mut body = Vec::with_capacity(1024);

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_enabled".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "VERBOSITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_INFO".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_info".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "MSG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "VERBOSITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__FILE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__LINE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_info".to_string(), Some(
        MacroDef {
            ports: [("ID".to_string(),[].to_vec()), ("MSG".to_string(),[].to_vec()), ("VERBOSITY".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_enabled".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_WARNING".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_warning".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "MSG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__FILE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__LINE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_warning".to_string(), Some(
        MacroDef {
            ports: [("ID".to_string(),[].to_vec()), ("MSG".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_enabled".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_ERROR".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_error".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "MSG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__FILE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__LINE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_error".to_string(), Some(
        MacroDef {
            ports: [("ID".to_string(),[].to_vec()), ("MSG".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_enabled".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_FATAL".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_fatal".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "MSG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__FILE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__LINE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_fatal".to_string(), Some(
        MacroDef {
            ports: [("ID".to_string(),[].to_vec()), ("MSG".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: Ident,value: "SEQUENCER".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "p_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwVirtual,value: "virtual".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeVoid,value: "void".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "m_set_p_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwSuper,value: "super".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "m_set_p_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$cast".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "p_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "m_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_fatal".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "DCLPSQ".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$sformatf".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "%m %s Error casting p_sequencer, please verify that this sequence/sequence item is intended to execute on this type of sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_full_name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_declare_p_sequencer".to_string(), Some(
        MacroDef {
            ports: [("SEQUENCER".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`uvm_object_utils_begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_object_utils_end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_object_utils".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "create".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeString,value: "string".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CompDir,value: "`ifdef".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_OBJECT_DO_NOT_NEED_CONSTRUCTOR".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwNew,value: "new".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpDiff,value: "!=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "set_name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CompDir,value: "`else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq2,value: "==".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwNew,value: "new".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwElse,value: "else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwNew,value: "new".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CompDir,value: "`endif".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`m_uvm_object_create_func".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwConst,value: "const".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwStatic,value: "static".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeString,value: "string".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwVirtual,value: "virtual".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeString,value: "string".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_type_name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`m_uvm_get_type_name_func".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwTypedef,value: "typedef".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object_registry".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Hash,value: "#".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "S".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwStatic,value: "static".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_type".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwVirtual,value: "virtual".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object_wrapper".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_object_type".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`m_uvm_object_registry_internal".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec()), ("S".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`m_uvm_object_registry_internal".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`m_uvm_object_create_func".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`m_uvm_get_type_name_func".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_field_utils_begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_object_utils_begin".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`uvm_create_on".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "m_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_create".to_string(), Some(
        MacroDef {
            ports: [("SEQ_OR_ITEM".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object_wrapper".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "w_".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "w_".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_type".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$cast".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "create_item".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "w_".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQR".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_create_on".to_string(), Some(
        MacroDef {
            ports: [("SEQ_OR_ITEM".to_string(),[].to_vec()), ("SEQR".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`uvm_send_pri".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpMinus,value: "-".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_send".to_string(), Some(
        MacroDef {
            ports: [("SEQ_OR_ITEM".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_sequence_base".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$cast".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "start_item".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "PRIORITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "finish_item".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "PRIORITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwElse,value: "else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "start".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwThis,value: "this".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "PRIORITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "0".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_send_pri".to_string(), Some(
        MacroDef {
            ports: [("SEQ_OR_ITEM".to_string(),[].to_vec()), ("PRIORITY".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();
    body.push(Token {kind: MacroCall,value: "`uvm_rand_send_pri_with".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpMinus,value: "-".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "CONSTRAINTS".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_rand_send_with".to_string(), Some(
        MacroDef {
            ports: [("SEQ_OR_ITEM".to_string(),[].to_vec()), ("CONSTRAINTS".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`uvm_rand_send_pri_with".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpMinus,value: "-".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyLeft,value: "{".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyRight,value: "}".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_rand_send".to_string(), Some(
        MacroDef {
            ports: [("SEQ_OR_ITEM".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_sequence_base".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$cast".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "start_item".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "PRIORITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwElse,value: "else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "set_item_context".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwThis,value: "this".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq2,value: "==".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwNull,value: "null".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpLogicOr,value: "||".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "do_not_randomize".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpLogicAnd,value: "&&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "randomize".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwWith,value: "with".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "CONSTRAINTS".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_warning".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "RNDFLD".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "Randomization failed in uvm_rand_send_with action".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$cast".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "finish_item".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "SEQ_OR_ITEM".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "PRIORITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwElse,value: "else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "start".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__seq".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_sequencer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwThis,value: "this".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "PRIORITY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "0".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_rand_send_pri_with".to_string(), Some(
        MacroDef {
            ports: [("SEQ_OR_ITEM".to_string(),[].to_vec()), ("PRIORITY".to_string(),[].to_vec()), ("CONSTRAINTS".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`m_uvm_component_registry_internal".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`m_uvm_get_type_name_func".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_component_utils".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeVoid,value: "void".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_field_automation".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeIntAtom,value: "int".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "what__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeString,value: "string".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "str__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "local_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwTypedef,value: "typedef".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "___local_type____".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: TypeString,value: "string".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "string_aa_key".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__current_scopes".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SquareLeft,value: "[".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dollar,value: "$".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SquareRight,value: "]".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "what__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwInside,value: "inside".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyLeft,value: "{".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_SETINT".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_SETSTR".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_SETOBJ".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyRight,value: "}".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "m_do_cycle_check".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwThis,value: "this".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwElse,value: "else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__current_scopes".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "m_uvm_cycle_scopes".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwSuper,value: "super".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_field_automation".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "what__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "str__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpDiff,value: "!=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwNull,value: "null".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$cast".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "local_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "tmp_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_field_utils_begin".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`m_uvm_object_registry_internal".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`m_uvm_object_create_func".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`m_uvm_get_type_name_func".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_field_utils_begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_object_utils_begin".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_object_utils_end".to_string(), Some(MacroDef {ports: Vec::new(),body: body.clone()}));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`uvm_object_utils_begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_object_utils_end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_object_utils".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwTypedef,value: "typedef".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_component_registry".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Hash,value: "#".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "S".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwStatic,value: "static".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_type".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwVirtual,value: "virtual".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object_wrapper".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_object_type".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`m_uvm_component_registry_internal".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec()), ("S".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`uvm_object_param_utils_begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_object_utils_end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_object_param_utils".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`m_uvm_object_registry_param".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`m_uvm_object_create_func".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_field_utils_begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_object_param_utils_begin".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwTypedef,value: "typedef".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object_registry".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Hash,value: "#".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwStatic,value: "static".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_type".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwVirtual,value: "virtual".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwFunction,value: "function".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object_wrapper".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_object_type".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "type_id".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`m_uvm_object_registry_param".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: MacroCall,value: "`uvm_component_utils".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`uvm_field_utils_begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_component_utils_begin".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndFunction,value: "endfunction".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_component_utils_end".to_string(), Some(
        MacroDef {
            ports: Vec::new(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NORECORD".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "recorder".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "record_string".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "STR".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`m_uvm_record_string".to_string(), Some(
        MacroDef {
            ports: [("ARG".to_string(),[].to_vec()), ("STR".to_string(),[].to_vec()), ("FLAG".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwCase,value: "case".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "what__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_CHECK_FIELDS".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "do_field_check".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwThis,value: "this".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_COPY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "local_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq2,value: "==".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwNull,value: "null".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NOCOPY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "local_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_COMPARE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "local_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq2,value: "==".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwNull,value: "null".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NOCOMPARE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpDiff2,value: "!==".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "local_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "scope".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "set_arg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$swrite".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "stringv".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "lhs = %0s : rhs = %0s".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "local_data__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "comparer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "print_msg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "stringv".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "comparer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "result".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpLogicAnd,value: "&&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "comparer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "show_max".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpLTE,value: "<=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "comparer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "result".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwReturn,value: "return".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_PACK".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NOPACK".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "packer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "pack_field".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$bits".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_UNPACK".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NOPACK".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Casting,value: "T\'".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "packer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "unpack_field_int".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$bits".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_RECORD".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`m_uvm_record_string".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_PRINT".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpBang,value: "!".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NOPRINT".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "printer".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "print_generic".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$bits".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_SETINT".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "scope".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "set_arg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_is_match".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "str__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "scope".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_READONLY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_warning".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "RDONLY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$sformatf".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "Readonly argument match %s is ignored".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_full_scope_arg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwElse,value: "else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "print_matches".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_info".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "STRMTC".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyLeft,value: "{".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "set_int()".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: ": Matched string ".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "str__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: " to field ".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_full_scope_arg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyRight,value: "}".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_LOW".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Casting,value: "T\'".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "bitstream".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "status".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_SETSTR".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Colon,value: ":".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "scope".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "set_arg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_is_match".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "str__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "scope".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "FLAG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpAnd,value: "&".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_READONLY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_warning".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "RDONLY".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SystemTask,value: "$sformatf".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "Readonly argument match %s is ignored".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_full_scope_arg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwElse,value: "else".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "print_matches".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_info".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "STRMTC".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyLeft,value: "{".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "set_str()".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: ": Matched string ".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "str__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: " to field ".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "get_full_scope_arg".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: CurlyRight,value: "}".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_LOW".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Casting,value: "void\'".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_enum_wrapper".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Hash,value: "#".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "T".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "from_name".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_object".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Scope,value: "::".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "stringv".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ARG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "__m_uvm_status_container".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "status".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: OpEq,value: "=".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEndcase,value: "endcase".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_field_enum".to_string(), Some(
        MacroDef {
            ports: [("T".to_string(),[].to_vec()), ("ARG".to_string(),[].to_vec()), ("FLAG".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    body.push(Token {kind: KwBegin,value: "begin".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwIf,value: "if".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "RO".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_enabled".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_ERROR".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "RO".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Dot,value: ".".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "uvm_report_error".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenLeft,value: "(".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "ID".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "MSG".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Ident,value: "UVM_NONE".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__FILE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: MacroCall,value: "`__LINE__".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Str,value: "".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Comma,value: ",".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: Integer,value: "1".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: ParenRight,value: ")".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: SemiColon,value: ";".to_string(),pos: Position {line: 1, col: 1}});
    body.push(Token {kind: KwEnd,value: "end".to_string(),pos: Position {line: 1, col: 1}});
    ast.defines.insert("`uvm_error_context".to_string(), Some(
        MacroDef {
            ports: [("ID".to_string(),[].to_vec()), ("MSG".to_string(),[].to_vec()), ("RO".to_string(),[].to_vec())].to_vec(),
            body: body.clone()}
    ));
    body.clear();

    Box::new(ast)
}
