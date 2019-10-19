// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::{HashMap};

use crate::ast::Ast;
use crate::ast::astnode::{AstNode,AstNodeKind};
use crate::comp::prototype::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ObjDef {
    Signal,
    Instance,
    Class(CompObj),
    Value,
    Method(DefMethod),
    Macro(DefMacro),
    Type,
}

/// Structure holding compiled information about module/class/package/...
#[derive(Debug, Clone)]
pub struct CompObj {
    ///
    pub name : String,
    ///
    pub definition   : HashMap<String, ObjDef>,
    pub base_class   : Option<String>,
    pub import_hdr   : Vec<String>,
    pub import_body  : Vec<String>,
    pub unref        : HashMap<String, AstNode>,
    pub call         : Vec<AstNode>,
    pub tmp_decl     : Vec<HashMap<String, ObjDef>>,
}


impl CompObj {

    #[allow(dead_code)]
    pub fn new(name: String) -> CompObj {
        CompObj {
            name        : name,
            base_class  : None,
            definition  : HashMap::new(),
            tmp_decl    : Vec::new(),
            import_hdr  : Vec::new(),
            import_body : Vec::new(),
            call        : Vec::new(),
            unref       : HashMap::new()
        }
    }

    //
    #[allow(dead_code,unused_variables)]
    // TODO: check performances when taking ownership of the AST:
    //       need copy in includes but avoid copy in all other cases ...
    pub fn from_ast(ast: &Ast, ast_inc: & HashMap<String,Ast>, mut lib: &mut HashMap<String, CompObj>)  {
        for node in &ast.tree.child {
            // println!("Compiling Node {:?} ({:?}", node.kind, node.attr);
            match node.kind {
                AstNodeKind::Directive => {
                    node.attr.get("include").map(
                        |i| ast_inc.get(i).map_or_else(
                            || if i!="uvm_macros.svh" {println!("Include {} not found", i)},
                            |a| CompObj::from_ast(a, &ast_inc, &mut lib)
                        )
                    );
                },
                AstNodeKind::MacroCall => {},
                AstNodeKind::Interface |
                AstNodeKind::Module => {
                    let mut o = CompObj::new(node.attr["name"].clone());
                    // println!("Compiling {:?}", node.attr["name"]);
                    for node_m in &node.child {
                        // println!(" - {:?}", node_m.kind);
                        match node_m.kind {
                            AstNodeKind::Header => {
                                for n in &node_m.child {
                                    // println!("[CompObj] {} | Header: {:?}",o.name, n);
                                    match n.kind {
                                        AstNodeKind::Port => {
                                            o.definition.insert(n.attr["name"].clone(),ObjDef::Signal);
                                            o.check_type(&n,true);
                                        }
                                        AstNodeKind::Import => {
                                            if n.attr.contains_key("dpi") {
                                                println!("[CompObj] {} | Skipping DPI import : {}",o.name, n);
                                            } else {
                                                for nc in &n.child {
                                                    if nc.attr["name"] == "*" {
                                                        o.import_hdr.push(nc.attr["pkg_name"].clone());
                                                    } else {
                                                        println!("Need to check sub-ref: {}", nc);
                                                    }
                                                }
                                            }
                                        }
                                        AstNodeKind::Param => {
                                            // TODO: CHeck array size
                                            o.definition.insert(n.attr["name"].clone(),ObjDef::Signal);
                                        }
                                        _ => {println!("[CompObj] {} | Header: Skipping {:?}",o.name, n.kind);}
                                    }
                                }
                            }
                            _ => o.parse_body(&node_m,ast_inc)
                        }
                    }
                    // println!("[ObjWork] {:?}", o);
                    lib.insert(o.name.clone(),o);
                }
                AstNodeKind::Class     |
                AstNodeKind::Package   => {
                    let mut o = CompObj::new(node.attr["name"].clone());
                    // println!("Compiling {:?}", node.attr["name"]);
                    o.parse_body(&node,ast_inc);
                    lib.insert(o.name.clone(),o);
                }
                AstNodeKind::Define => {
                    if !node.child.is_empty() {
                        // println!("[CompObj] Compiling define {}", node);
                        let mut d = DefMacro::new(format!("`{}",node.attr["name"]));
                        for p in &node.child {
                            d.ports.push(p.attr["name"].clone());
                        }
                        lib.get_mut("!").unwrap().definition.insert(d.name.clone(),ObjDef::Macro(d));
                    }
                }
                // Handle special case of type/localparams/define done out of context
                AstNodeKind::Param => {
                    lib.get_mut("!").unwrap().definition.insert(node.attr["name"].clone(),ObjDef::Signal);
                }
                AstNodeKind::Typedef => lib.get_mut("!").unwrap().add_type_def(&node),
                _ => {println!("[CompObj] Top: Skipping {:?}", node.kind);}
            }
        }
    }

    pub fn parse_body(&mut self, node: &AstNode, ast_inc: & HashMap<String,Ast>) {
        let mut has_hdr = false;
        for n in &node.child {
            // println!("[CompObj] {} | next node = {}",self.name, n.kind);
            match n.kind {
                AstNodeKind::Params => {
                    for nc in &n.child {
                        if nc.attr.get("type").unwrap_or(&"".to_owned()) == "type" {
                            // TODO: add info about default value: nc.child[0].attr["name"]
                            self.definition.insert(nc.attr["name"].clone(),ObjDef::Type);
                        } else {
                            // println!("Param Type : {:?}", nc);
                            self.definition.insert(nc.attr["name"].clone(),ObjDef::Signal);
                        }
                    }
                }
                AstNodeKind::Implements |
                AstNodeKind::Extends => {
                    // TODO: extract parameter if any !
                    self.check_type(&n,false);
                    self.base_class = Some(n.attr["type"].clone());
                }
                //
                AstNodeKind::Directive => {
                    n.attr.get("include").map(
                        |i| ast_inc.get(i).map_or_else(
                            || if i!="uvm_macros.svh" {println!("Include {} not found", i)},
                            |a| self.parse_body(&a.tree,ast_inc)
                        )
                    );
                },
                AstNodeKind::Timescale => {},
                // Class definition (can happen when inside a package)
                AstNodeKind::Class     => {
                    // println!("Compiling {:?}", n.attr);
                    let mut o = CompObj::new(n.attr["name"].clone());
                    o.parse_body(&n,ast_inc);
                    self.definition.insert(n.attr["name"].clone(),ObjDef::Class(o));
                }
                // Header in a body: Loop definition
                AstNodeKind::Header => {
                    if has_hdr {
                        println!("[CompObj] {} | Too much header !: {:?}",self.name, n);
                    } else {
                        self.tmp_decl.push(HashMap::new());
                    }
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Declaration => {
                                self.tmp_decl.last_mut().unwrap().insert(nc.attr["name"].clone(),ObjDef::Signal);
                                self.check_type(&nc,false);
                            }
                            _ => self.search_ident(&nc),
                        }
                    }
                    has_hdr = true;
                },
                AstNodeKind::Param => {
                    // TODO: CHeck array size
                    self.definition.insert(n.attr["name"].clone(),ObjDef::Signal);
                }
                AstNodeKind::Port => self.add_decl(&n,true),
                AstNodeKind::Constraint => self.add_decl(&n,true),
                AstNodeKind::Covergroup => {
                    self.definition.insert(n.attr["name"].clone(),ObjDef::Type);
                    // TODO: check covergroup info (once part of the AST)
                }
                AstNodeKind::Typedef => self.add_type_def(&n),
                AstNodeKind::Enum => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::EnumIdent  => { self.definition.insert(nc.attr["name"].clone(),ObjDef::Value);},
                            AstNodeKind::Identifier => self.add_decl(&nc,true),
                            _ => println!("[CompObj] {} | Typedef: Skipping {}",self.name, nc.kind),
                        }
                    }
                }
                AstNodeKind::Struct => {
                    for nc in &n.child {
                        match nc.kind {
                            // TODO: add definition of the structure for the fields
                            AstNodeKind::Declaration  => {},
                            AstNodeKind::Identifier => self.add_decl(&nc,true),
                            _ => println!("[CompObj] {} | Typedef: Skipping {}",self.name, nc.kind),
                        }
                    }
                }
                AstNodeKind::Import => {
                    // println!("{:?}", n);
                    if n.attr.contains_key("dpi") {
                        if n.attr["kind"]=="import" {
                            if n.child.len() == 1 {
                                self.parse_method_decl(&n.child[0]);
                            } else {
                                println!("[CompObj Skipping DPI import : {:?}", n);
                            }
                        }
                    } else {
                        for nc in &n.child {
                            if nc.attr["name"] == "*" {
                                self.import_body.push(nc.attr["pkg_name"].clone());
                            } else {
                                println!("Need to check sub-ref: {}", nc);
                            }
                        }
                    }
                }
                AstNodeKind::Declaration => {
                    self.add_decl(&n,true);
                    if n.child.len() > 0 {
                        if n.child[0].kind != AstNodeKind::Scope {
                            self.search_ident(&n);
                        }
                    }
                }
                AstNodeKind::VIntf => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Params => self.search_ident(&nc),
                            AstNodeKind::Identifier => self.add_decl(&nc,true),
                            _ => println!("[CompObj] {} | VIntf: Skipping {}",self.name, nc.kind),
                        }
                    }
                }
                // TODO: assign might need a special check function to handle left/right type check
                AstNodeKind::Assign => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Identifier => self.parse_ident(&nc),
                            _ => self.search_ident(&nc),
                            // _ => {println!("[CompObj] {} | Assign: Skipping {}",self.name, nc.kind);}
                        }
                    }
                }
                AstNodeKind::Process => self.parse_block(&n),
                AstNodeKind::Instances => {
                    self.call.push(n.clone());
                    // println!("[CompObj] {} | Instances = {}",self.name, n);
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Params => {
                                self.search_ident(&nc)
                            }
                            AstNodeKind::Instance => {
                                self.definition.insert(nc.attr["name"].clone(),ObjDef::Instance);
                                self.search_ident(&nc);
                            }
                            _ => println!("[CompObj] {} | Instances: Skipping = {}",self.name, n.kind)
                        }
                    }
                }
                AstNodeKind::Branch  |
                AstNodeKind::LoopFor => self.parse_body(&n,ast_inc),
                AstNodeKind::Operation => self.search_ident(&n),
                AstNodeKind::Clocking => self.search_ident(&n),
                AstNodeKind::Task |
                AstNodeKind::Function => self.parse_method_decl(&n),
                AstNodeKind::Define => {
                    if !n.child.is_empty() {
                        // println!("[CompObj] {} | Compiling define {}", self.name, n);
                        let mut d = DefMacro::new(format!("`{}",n.attr["name"]));
                        for p in &n.child {
                            d.ports.push(p.attr["name"].clone());
                        }
                         self.definition.insert(d.name.clone(),ObjDef::Macro(d));
                    }
                }
                AstNodeKind::MacroCall => self.parse_call(&n),
                AstNodeKind::Modport => {
                    self.add_decl(&n,true);
                    // TODO: add list of signal direction for the modport
                },
                AstNodeKind::Identifier => self.parse_ident(&n),
                _ => {println!("[CompObj] {} | Body: Skipping {}",self.name, n.kind);}
            }
        }
        // Remove definition of variable done in header
        if has_hdr {
            self.tmp_decl.pop();
        }

    }

    pub fn parse_block(&mut self, node: &AstNode) {
        self.tmp_decl.push(HashMap::new());
        // Handle variable definition of foreach loop
        if node.attr.get("kind").unwrap_or(&"".to_owned()) == "foreach" {
            let mut nc = &node.child[0]; // Foreach loop always have at least one child
            loop {
                if nc.kind == AstNodeKind::Slice || nc.child.len() == 0 {
                    break;
                }
                nc = &nc.child[0];
            }
            for ncc in &nc.child {
                // println!("Foreach slice definition: {}[{:#?}]", node.child[0].attr["name"],ncc.attr["name"]);
                self.tmp_decl.last_mut().unwrap().insert(ncc.attr["name"].clone(),ObjDef::Signal);
            }
        }
        // Check
        for nc in &node.child {
            match nc.kind {
                AstNodeKind::Header => {
                    for ncc in &nc.child {
                        match ncc.kind {
                            AstNodeKind::Declaration => {
                                self.tmp_decl.last_mut().unwrap().insert(ncc.attr["name"].clone(),ObjDef::Signal);
                                self.check_type(&ncc,false);
                            }
                            _ => self.search_ident(&ncc),
                        }
                    }
                }
                AstNodeKind::Declaration => {
                    self.tmp_decl.last_mut().unwrap().insert(nc.attr["name"].clone(),ObjDef::Signal);
                    self.check_type(&nc,false);
                }
                AstNodeKind::Sensitivity => self.check_sensitivity(&nc),
                AstNodeKind::Identifier => self.parse_ident(&nc),
                AstNodeKind::Branch     => self.parse_block(&nc),
                AstNodeKind::Case       => self.parse_block(&nc),
                AstNodeKind::CaseItem   => self.parse_block(&nc),
                AstNodeKind::Assign     |
                AstNodeKind::Concat     |
                AstNodeKind::Statement  => self.search_ident(&nc),
                AstNodeKind::Expr       |
                AstNodeKind::ExprGroup  |
                AstNodeKind::Operation  |
                AstNodeKind::Return     => self.search_ident(&nc),
                AstNodeKind::Fork       => {
                    for ncc in &nc.child {
                        self.parse_block(&ncc);
                    }
                }
                AstNodeKind::Loop       |
                AstNodeKind::LoopFor    => self.parse_block(&nc),
                AstNodeKind::SystemTask => self.check_system_task(&nc),
                AstNodeKind::MethodCall => self.parse_call(&nc),
                AstNodeKind::MacroCall  => self.parse_call(&nc),
                AstNodeKind::Assert     => self.parse_block(&nc),
                AstNodeKind::Wait       => self.parse_block(&nc),
                AstNodeKind::Event      => self.parse_ident(&nc),
                // Value : can be safely skipped (likely an argument of a repeat block)
                AstNodeKind::Value      => {},
                // TODO: handle `define case ?
                AstNodeKind::Directive  => {},
                _ => {println!("[CompObj] {} | Block: Skipping {:?}",self.name, nc.kind);}
            }
        }
        self.tmp_decl.pop();
    }

    pub fn parse_method_decl(&mut self, node: &AstNode) {
        let mut d = DefMethod::new(node.attr["name"].clone(),node.kind==AstNodeKind::Task);
        // Make distinction between function and task
        //
        self.tmp_decl.push(HashMap::new());
        // The function name is also a variable using the return type
        if node.kind == AstNodeKind::Function {
            self.tmp_decl.last_mut().unwrap().insert(node.attr["name"].clone(),ObjDef::Signal);
        }
        let mut prev_dir = PortDir::Input; // Default port direction to input
        for nc in &node.child {
            // println!("[Function] {}", nc);
            match nc.kind {
                AstNodeKind::Ports => {
                    for np in &nc.child {
                        // println!("Ports child {:?}", np.attr);
                        self.tmp_decl.last_mut().unwrap().insert(np.attr["name"].clone(),ObjDef::Signal);
                        self.check_type(&np,false);
                        // Update definition of the function to include port name and type
                        let p = Port::new(np,prev_dir);
                        prev_dir = p.dir.clone();
                        d.ports.push(p);
                    }
                }
                // Check return type
                AstNodeKind::Type  => {
                    // println!("[CompObj] {} | Function {} return {:?}", self.name, d.name, nc.attr);
                    d.ret = Some(SignalType::from(nc));
                    self.check_type(&nc,false);
                }
                // Variable definition
                AstNodeKind::Declaration => {
                    self.tmp_decl.last_mut().unwrap().insert(nc.attr["name"].clone(),ObjDef::Signal);
                    self.check_type(&nc,false);
                }
                AstNodeKind::Assign => self.search_ident(&nc),
                AstNodeKind::Branch => self.parse_block(&nc),
                AstNodeKind::Case       => self.parse_block(&nc),
                AstNodeKind::CaseItem   => self.parse_block(&nc),
                AstNodeKind::Identifier => self.parse_ident(&nc),
                AstNodeKind::Event      => self.parse_ident(&nc),
                AstNodeKind::Fork       => {
                    for ncc in &nc.child {
                        self.parse_block(&ncc);
                    }
                }
                AstNodeKind::Loop       |
                AstNodeKind::LoopFor    |
                AstNodeKind::Block      => self.parse_block(&nc),
                AstNodeKind::SystemTask => self.check_system_task(&nc),
                AstNodeKind::MethodCall => self.parse_call(&nc),
                AstNodeKind::MacroCall => self.parse_call(&nc),
                AstNodeKind::Wait   => self.parse_block(&nc),
                AstNodeKind::Assert => self.parse_block(&nc),
                AstNodeKind::Return => self.search_ident(&nc),
                // _ => self.search_ident(&nc),
                _ => {println!("[CompObj] {} | Function: Skipping {:?}",self.name, nc.kind);}
            }
        }
        self.tmp_decl.pop();
        // print!("[CompObj] {} | {}",self.name , d);
        self.definition.insert(node.attr["name"].clone(),ObjDef::Method(d));
    }

    pub fn parse_call(&mut self, node: &AstNode) {
        // println!("[CompObj] {} | parse_call on: \n{}",self.name, node);
        // Function call parameters need to be checked later, so add the full node to the call vector
        self.call.push(node.clone());
        // Check ident in parameter
        self.search_ident(&node);
    }

    pub fn add_type_def(&mut self, node: &AstNode) {
        self.definition.insert(node.attr["name"].clone(),ObjDef::Type);
        for nc in &node.child {
            match nc.kind {
                AstNodeKind::EnumIdent   => { self.definition.insert(nc.attr["name"].clone(),ObjDef::Value);},
                AstNodeKind::Declaration => {
                    // println!("Typedef : {}", nc);
                },
                AstNodeKind::Param => self.search_ident(&nc),
                _ => println!("[CompObj] {} | Typedef: Skipping {}",self.name, nc.kind),
            }
        }

    }

    pub fn add_decl(&mut self, node: &AstNode, do_check: bool) {
        if node.attr.contains_key("name") {
            if self.definition.contains_key(&node.attr["name"]) {
                // TODO: add to the message queue instead of printing
                println!("[CompObj] {} | Redeclaration of {}",self.name,node.attr["name"]);
            } else {
                self.definition.insert(node.attr["name"].clone(),ObjDef::Signal);
                if do_check {self.check_type(&node,false);}
            }
        }
        // else {println!("add_decl called,true on node with no name: {} ", node);}
    }

    pub fn search_ident(&mut self, node: &AstNode) {
        // println!("[CompObj] {} | Searching: {}",self.name,node);
        for n in &node.child {
            match n.kind {
                AstNodeKind::Identifier => self.parse_ident(&n),
                AstNodeKind::MethodCall => self.parse_call(&n),
                AstNodeKind::LoopFor    => self.parse_block(&n),
                AstNodeKind::CaseItem   |
                AstNodeKind::Block      => self.parse_block(&n),
                _ => self.search_ident(&n),
            }
        }
    }

    pub fn parse_ident(&mut self, node: &AstNode) {
        match node.attr["name"].as_ref() {
            "super" => {/*TODO: automatically add child to a list of undeclared to be cheked later */},
            "this" => if node.child.len()>0 {
                let nc = &node.child[0];
                match nc.kind {
                    AstNodeKind::MethodCall => self.parse_call(&nc),
                    AstNodeKind::MacroCall => self.parse_call(&nc),
                    AstNodeKind::Identifier => self.parse_ident(&nc),
                    _ => println!("[CompObj] {} | Skipping 'this' child : {}", self.name, nc.kind),
                }
            }
            _ => {
                if self.name != node.attr["name"] {
                    if !self.definition.contains_key(&node.attr["name"]) {
                        let mut found = false;
                        for d in &self.tmp_decl {
                            if d.contains_key(&node.attr["name"]) {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            if !self.unref.contains_key(&node.attr["name"]) {
                                // println!("Unknown identifer {}", node.attr["name"]);
                                self.unref.insert(node.attr["name"].clone(),node.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_type(&mut self, node: &AstNode, is_hdr: bool) {
        if !node.attr.contains_key("type") {
            return;
        }
        match node.attr["type"].as_ref() {
            // Ignore default type
            "genvar"   |
            "byte"     |
            "shortint" |
            "int"      |
            "longint"  |
            "integer"  |
            "time"     |
            "bit"      |
            "logic"    |
            "real"     |
            "reg"      |
            "realtime" |
            "shortreal"|
            "string"   |
            "chandle"  |
            "event"    |
            "process"  |
            "void"     => {},
            // Anonymous enumerate type -> add enum value to definition
            "enum" => {
                for nc in &node.child[0].child {
                    self.definition.insert(nc.attr["name"].clone(),ObjDef::Value);
                }
            }
            // Non default SystemVerilog type ? -> check if it was defined
            _ => {
                if !self.definition.contains_key(&node.attr["type"]) {
                    if !self.unref.contains_key(&node.attr["type"]) {
                        // println!("Unknown type {}", node);
                        let mut n = node.clone();
                        if is_hdr {n.kind = AstNodeKind::Header;}
                        self.unref.insert(node.attr["type"].clone(),n);
                    }
                }
            }
        }
    }

    pub fn check_system_task(&mut self, node: &AstNode) {
        // TODO: check number of element
        // println!("[CompObj] {} | SystemTask {}", self.name, node);
        self.search_ident(&node);
    }

    // Expect a list of event
    pub fn check_sensitivity(&mut self, node: &AstNode) {
        for ne in &node.child {
            // TODO: also check name versus posedge/negedge
            self.parse_ident(&ne.child[0]);
        }
    }

}


