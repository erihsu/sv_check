// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::ast::Ast;

// use std::{fs,path,io, mem, str, iter};
use std::collections::{HashMap};
use crate::ast::astnode::{AstNode,AstNodeKind};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ObjDef {
    Signal,
    Instance,
    Class,
    Function,
    Type,
}

/// Structure holding compiled information about module/class/package/...
#[derive(Debug, Clone)]
pub struct WorkObj {
    ///
    pub name : String,
    ///
    pub definition   : HashMap<String, ObjDef>,
    pub declaration  : HashMap<String, ObjDef>,
    pub base_class   : Option<String>,
    pub import_hdr   : Vec<String>,
    pub import_body  : Vec<String>,
    pub unsolved_ref : HashMap<String, AstNodeKind>,
    pub call         : Vec<AstNode>,
    pub tmp_decl     : Vec<HashMap<String, ObjDef>>,
}


impl WorkObj {

    #[allow(dead_code)]
    pub fn new(name: String) -> WorkObj {
        WorkObj {
            name         : name,
            base_class   : None,
            definition   : HashMap::new(),
            declaration  : HashMap::new(),
            tmp_decl     : Vec::new(),
            import_hdr   : Vec::new(),
            import_body  : Vec::new(),
            call         : Vec::new(),
            unsolved_ref : HashMap::new()
        }
    }

    //
    #[allow(dead_code,unused_variables)]
    pub fn from_ast(ast: Ast, lib: &mut HashMap<String, WorkObj>)  {
        for node in ast.tree.child {
            match node.kind {
                AstNodeKind::Directive => {},
                AstNodeKind::Interface |
                AstNodeKind::Module => {
                    let mut o = WorkObj::new(node.attr["name"].clone());
                    // println!("Compiling {:?}", node.attr["name"]);
                    for node_m in node.child {
                        // println!(" - {:?}", node_m.kind);
                        match node_m.kind {
                            AstNodeKind::Header => {
                                for n in node_m.child {
                                    match n.kind {
                                        AstNodeKind::Port => {
                                            o.declaration.insert(n.attr["name"].clone(),ObjDef::Signal);
                                            o.check_type(&n);
                                        }
                                        AstNodeKind::Import => {
                                            if n.attr.contains_key("dpi") {
                                                println!("[WorkObj Skipping DPI import : {}", n);
                                            } else {
                                                for nc in n.child {
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
                                            o.declaration.insert(n.attr["name"].clone(),ObjDef::Signal);
                                        }
                                        _ => {println!("[WorkObj] Header: Skipping {:?}", n.kind);}
                                    }
                                }
                            }
                            _ => o.parse_body(&node_m)
                        }
                    }
                    // println!("[ObjWork] {:?}", o);
                    lib.insert(o.name.clone(),o);
                }
                AstNodeKind::Class     |
                AstNodeKind::Package   => {
                    let mut o = WorkObj::new(node.attr["name"].clone());
                    // println!("Compiling {:?}", node.attr["name"]);
                    o.parse_body(&node);
                    lib.insert(o.name.clone(),o);
                }
                _ => {println!("[WorkObj] Top: Skipping {:?}", node.kind);}
            }
        }
    }

    pub fn parse_body(&mut self, node: &AstNode) {
        let mut has_hdr = false;
        for n in &node.child {
            // println!("[WorkObj] next node = {}", n.kind);
            match n.kind {
                AstNodeKind::Directive => {},
                // Header in a body: Loop definition
                AstNodeKind::Header => {
                    if has_hdr {
                        println!("[WorkObj] Too much header !: {:?}", n);
                    } else {
                        self.tmp_decl.push(HashMap::new());
                    }
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Declaration => {
                                self.tmp_decl.last_mut().unwrap().insert(nc.attr["name"].clone(),ObjDef::Signal);
                                self.check_type(&nc);
                            }
                            _ => self.search_ident(&nc),
                        }
                    }
                    has_hdr = true;
                },
                AstNodeKind::Param => {
                    // TODO: CHeck array size
                    self.declaration.insert(n.attr["name"].clone(),ObjDef::Signal);
                }
                AstNodeKind::Port => self.add_decl(&n),
                AstNodeKind::Typedef => {
                    self.definition.insert(n.attr["name"].clone(),ObjDef::Type);
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::EnumIdent   => { self.declaration.insert(nc.attr["name"].clone(),ObjDef::Signal);},
                            AstNodeKind::Declaration => {
                                // println!("Typedef : {}", nc);
                            },
                            _ => println!("[WorkObj] Typedef: Skipping {}", nc.kind),
                        }
                    }
                    // self.declaration.insert(n.attr["name"].clone(),ObjDef::Signal);
                }
                AstNodeKind::Import => {
                    if n.attr.contains_key("dpi") {
                        println!("[WorkObj Skipping DPI import : {}", n);
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
                AstNodeKind::Declaration => self.add_decl(&n),
                AstNodeKind::VIntf => {
                    for nc in &n.child {
                        self.add_decl(&nc);
                    }
                }
                // TODO: assign might need a special check function to handle left/right type check
                AstNodeKind::Assign => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Identifier => self.parse_ident(&nc),
                            _ => self.search_ident(&nc),
                            // _ => {println!("[WorkObj] Assign: Skipping {}", nc.kind);}
                        }
                    }
                }
                AstNodeKind::Process => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Sensitivity => self.check_sensitivity(&nc),
                            AstNodeKind::Assign => self.search_ident(&nc),
                            _ => self.search_ident(&nc),
                        }
                    }
                }
                AstNodeKind::Instances => {
                    self.call.push(n.clone());
                    // println!("[WorkObj] Instances = {}", n);
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Params => {
                                self.search_ident(&nc)
                            }
                            AstNodeKind::Instance => {
                                self.declaration.insert(nc.attr["name"].clone(),ObjDef::Instance);
                                self.search_ident(&nc);
                            }
                            _ => println!("[WorkObj] Instances: Skipping = {}", n.kind)
                        }
                    }
                }
                AstNodeKind::Branch  |
                AstNodeKind::LoopFor => self.parse_body(&n),
                AstNodeKind::Operation => self.search_ident(&n),
                AstNodeKind::Task |
                AstNodeKind::Function => {
                    // TODO: add prototype
                    self.definition.insert(n.attr["name"].clone(),ObjDef::Function);
                    self.tmp_decl.push(HashMap::new());
                    // The function name is also a variable using the return type
                    if n.kind == AstNodeKind::Function {
                        self.tmp_decl.last_mut().unwrap().insert(n.attr["name"].clone(),ObjDef::Signal);
                    }
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Ports => {
                                // println!("Ports child {}", nc);
                                for np in &nc.child {
                                    // println!("Ports child {}", np);
                                    // TODO: update definition of the function to include port name and type
                                    self.tmp_decl.last_mut().unwrap().insert(np.attr["name"].clone(),ObjDef::Signal);
                                    self.check_type(&np);
                                }
                            }
                            AstNodeKind::Declaration => {
                                self.tmp_decl.last_mut().unwrap().insert(nc.attr["name"].clone(),ObjDef::Signal);
                                self.check_type(&nc);
                            }
                            AstNodeKind::Assign => self.search_ident(&nc),
                            AstNodeKind::Branch => self.search_ident(&nc),
                            AstNodeKind::Case   => self.search_ident(&nc),
                            AstNodeKind::Identifier => self.parse_ident(&nc),
                            AstNodeKind::Fork       => {
                                for ncc in &nc.child {
                                    self.parse_block(&ncc);
                                }
                            }
                            AstNodeKind::Loop       |
                            AstNodeKind::LoopFor    => self.parse_block(&nc),
                            AstNodeKind::MethodCall => self.parse_call(&nc),
                            AstNodeKind::MacroCall => self.parse_call(&nc),
                            // _ => self.search_ident(&nc),
                            _ => {println!("[WorkObj] Function: Skipping {:?}", nc.kind);}
                        }
                    }
                    self.tmp_decl.pop();
                }
                AstNodeKind::MacroCall => self.parse_call(&n),
                AstNodeKind::Modport => {
                    self.add_decl(&n);
                    // TODO: add list of signal direction for the modport
                },
                _ => {println!("[WorkObj] Body: Skipping {}", n);}
            }
        }
        // Remove declaration of variable done in header
        if has_hdr {
            self.tmp_decl.pop();
        }

    }

    pub fn parse_block(&mut self, node: &AstNode) {
        self.tmp_decl.push(HashMap::new());
        for nc in &node.child {
            match nc.kind {
                AstNodeKind::Header => {
                    for ncc in &nc.child {
                        match ncc.kind {
                            AstNodeKind::Declaration => {
                                self.tmp_decl.last_mut().unwrap().insert(ncc.attr["name"].clone(),ObjDef::Signal);
                                self.check_type(&ncc);
                            }
                            _ => self.search_ident(&ncc),
                        }
                    }
                }
                AstNodeKind::Identifier => self.parse_ident(&nc),
                AstNodeKind::Branch     => self.search_ident(&nc),
                AstNodeKind::Operation  => self.search_ident(&nc),
                AstNodeKind::Assign     => self.search_ident(&nc),
                AstNodeKind::Fork       => {
                    for ncc in &nc.child {
                        self.parse_block(&ncc);
                    }
                }
                AstNodeKind::Loop       |
                AstNodeKind::LoopFor    => self.parse_block(&nc),
                AstNodeKind::MethodCall => self.parse_call(&nc),
                AstNodeKind::MacroCall  => self.parse_call(&nc),
                _ => {println!("[WorkObj] Block: Skipping {:?}", nc.kind);}
            }
        }
        self.tmp_decl.pop();
    }

    pub fn parse_call(&mut self, node: &AstNode) {
        // Function call parameters need to be checked later, so add the full node to the call vector
        self.call.push(node.clone());
        // Check ident in parameter
        self.search_ident(&node);
    }

    pub fn add_decl(&mut self, node: &AstNode) {
        if self.declaration.contains_key(&node.attr["name"]) {
            // TODO: add to the message queue instead of printing
            println!("[WorkObj] Redeclation of {}",node.attr["name"]);
        } else {
            self.declaration.insert(node.attr["name"].clone(),ObjDef::Signal);
            self.check_type(&node);
        }
    }

    pub fn search_ident(&mut self, node: &AstNode) {
        // println!("[WorkObj] Searching: {}",node);
        for n in &node.child {
            match n.kind {
                AstNodeKind::Identifier => self.parse_ident(&n),
                AstNodeKind::MethodCall => self.parse_call(&n),
                AstNodeKind::LoopFor    => self.parse_block(&n),
                _ => self.search_ident(&n),
            }
        }
    }

    pub fn parse_ident(&mut self, node: &AstNode) {
        match node.attr["name"].as_ref() {
            "super" => {/*TODO: automatically add child to a list of undeclared to be cheked later */},
            "this" => {
                let nc = &node.child[0];
                match nc.kind {
                    AstNodeKind::MethodCall => self.parse_call(&nc),
                    AstNodeKind::MacroCall => self.parse_call(&nc),
                    AstNodeKind::Identifier => self.parse_ident(&nc),
                    _ => println!("Skipping 'this' child : {}", nc.kind),
                }
            }
            _ => {
                if self.name != node.attr["name"] {
                    if !self.declaration.contains_key(&node.attr["name"]) {
                        let mut found = false;
                        for d in &self.tmp_decl {
                            if d.contains_key(&node.attr["name"]) {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            if !self.unsolved_ref.contains_key(&node.attr["name"]) {
                                // println!("Unknown identifer {}", node.attr["name"]);
                                self.unsolved_ref.insert(node.attr["name"].clone(),node.kind.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_type(&mut self, node: &AstNode) {
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
            "void"     => {},
            // Non default SystemVerilog type ? -> check if it was defined
            _ => {
                if !self.definition.contains_key(&node.attr["type"]) {
                    if !self.unsolved_ref.contains_key(&node.attr["type"]) {
                        // println!("Unknown type {}", node.attr["type"]);
                        self.unsolved_ref.insert(node.attr["type"].clone(),node.kind.clone());
                    }
                }
            }
        }
    }

    // Expect a list of event
    pub fn check_sensitivity(&mut self, node: &AstNode) {
        for ne in &node.child {
            // TODO: also check name versus posedge/negedge
            self.parse_ident(&ne.child[0]);
        }
    }

}


