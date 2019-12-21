// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

use crate::ast::Ast;
#[allow(unused_imports)]
use crate::ast::astnode::{AstNode,AstNodeKind};

use crate::comp::comp_obj::{ObjDef};
use crate::comp::prototype::*;
use crate::comp::def_type::{DefType,TypeVIntf,TYPE_INT};
use crate::comp::lib_uvm::get_uvm_lib;

type LinkCntxt = (AstNodeKind,String);

#[derive(Debug, Clone)]
pub struct CompLib {
    pub name   : String,
    pub objects: HashMap<String, ObjDef>,
    cntxt : Vec<LinkCntxt>
}

// Structure containing local information for a block:
// namely import dan signal/type definition
#[derive(Debug, Clone)]
pub struct LocalInfo {
    pub imports: Vec<String>,
    pub defs   : Vec<HashMap<String,ObjDef>>,
    pub obj    : Option<ObjDef>,
}

impl LocalInfo {

    pub fn add_def(&mut self, name: String, d: ObjDef) {
        // Check it was not already defined
        // for i in &self.defs {
        //     if i.contains_key(&name) {
        //         println!("[Linking] Redefinition of {} : \n  Prev = {:?}\n  New  = {:?}", name, i[&name], d);
        //         return;
        //     }
        // }
        // Insert new def
        self.defs.last_mut().unwrap().insert(name,d);
    }
}

#[allow(unused_mut)]
impl CompLib {

    // Create a library containing definition of all object compiled
    // Try to fix any missing reference, analyze hierarchical access, ...
    pub fn new(name: String, ast_list: Vec<Ast>, ast_inc: HashMap<String,Ast>) -> CompLib {
        let mut lib = CompLib {name, objects:HashMap::new(), cntxt:Vec::new()};
        // let mut missing_scope : HashSet<String> = HashSet::new();

        // Create a top object for type/localparam definition without scope
        lib.add_std_obj(); // Add definition for all standard lib classes
        lib.objects.insert("uvm_pkg".to_owned(),get_uvm_lib());

        // Extract object definition from all ASTs
        for ast in &ast_list {
            ObjDef::from_ast(&ast, &ast_inc, &mut lib.objects);
        }

        // Second pass : check types and signals are defined, module instance are correct ...
        for ast in ast_list {
            let mut li = LocalInfo{imports: Vec::new(),defs: Vec::new(), obj: None};
            lib.check_ast(&ast.tree, &ast_inc, &mut li, false);
        }

        lib
    }

    pub fn check_ast(&mut self, node: &AstNode, ast_inc: & HashMap<String,Ast>, li: &mut LocalInfo, new_cntxt: bool) {
        let mut port_dir = PortDir::Input; // Default port direction to input
        let mut port_idx = -1 as i16;
        if new_cntxt {
            li.defs.push(HashMap::new());
        }
        for nc in &node.child {
            match nc.kind {
                // Blocks
                AstNodeKind::Interface |
                AstNodeKind::Module    |
                AstNodeKind::Package   |
                AstNodeKind::Class     => {
                    // TODO: scope might be best describe as a vector of string ...
                    let mut scope = None;
                    if let Some((_,v)) = self.cntxt.get(0) {
                        scope = Some(v.clone());
                    }
                    if let Some(x) = self.find_def(&nc.attr["name"],scope.as_ref(),li,false,false) {
                        li.obj = Some(x.clone());
                    } else {
                        li.obj = None;
                    }
                    self.cntxt.push((nc.kind.clone(), nc.attr["name"].clone()));
                    // println!("[Linking] {} {}", nc.kind, nc.attr["name"]);
                    self.check_ast(&nc, &ast_inc, li,true);
                    self.cntxt.pop();
                }
                AstNodeKind::Process   |
                AstNodeKind::Branch    |
                AstNodeKind::Fork      |
                AstNodeKind::Block     |
                AstNodeKind::Case      => {
                    self.check_ast(&nc, &ast_inc, li,true);
                }
                // AstNodeKind::Fork => {
                //     println!("[Linking] {:?} | Checking Fork \n{:#?}", self.cntxt, nc);
                //     self.check_ast(&nc, &ast_inc, li,true);
                // }
                AstNodeKind::LoopFor => {
                    // TODO: add loop variable to the stack
                    self.check_ast(&nc, &ast_inc, li,true);
                }
                AstNodeKind::Loop => {
                    // TODO: add loop variable to the stack
                    li.defs.push(HashMap::new());
                    // Extract indices from the foreach
                    if nc.attr.get("kind")==Some(&"foreach".to_string()) {
                        // println!("[Linking] {:?} | Loop Foreach\n {}", self.cntxt, nc.child[0]);
                        let mut ncc = &nc.child[0]; // Foreach loop always have at least one child
                        loop {
                            // Check if no child: should never happen ...
                            if ncc.child.is_empty() {break;}
                            // When found the slice extract all identifier
                            if ncc.kind == AstNodeKind::Slice {
                                // println!("[Linking] {:?} | Loop Foreach\n {}", self.cntxt, ncc);
                                for x in &ncc.child {
                                    if x.kind != AstNodeKind::Identifier {
                                        println!("[Linking] {:?} | Unable to extract foreach variables in: {}", self.cntxt, ncc);
                                        break;
                                    }
                                    let mb = DefMember{
                                        name: x.attr["name"].clone(),
                                        kind: TYPE_INT,
                                        unpacked : None, is_const: false, access: Access::Public};
                                    li.add_def(mb.name.clone(),ObjDef::Member(mb));
                                }
                                break;
                            }
                            ncc = &ncc.child[0];
                        }
                    }
                    self.check_ast(&nc, &ast_inc, li,false);
                    li.defs.pop();
                }
                // Sub-part
                AstNodeKind::Ports  |
                AstNodeKind::Params |
                AstNodeKind::Header |
                AstNodeKind::Body   => {
                    self.check_ast(&nc, &ast_inc, li,false);
                }
                // Include node
                AstNodeKind::Directive => {
                    nc.attr.get("include").map(
                        |i| ast_inc.get(i).map_or_else(
                            || if i!="uvm_macros.svh" {println!("Include {} not found", i)},
                            |a| self.check_ast(&a.tree,ast_inc,li,false)
                        )
                    );
                },                  // Update local info
                AstNodeKind::Import => {
                    // Import DPI function/task
                    if nc.attr.contains_key("dpi") {
                        if nc.attr["kind"]=="import" {
                            if nc.child.len() == 1 {
                                let m = DefMethod::from(&nc.child[0]);
                                li.add_def(m.name.clone(),ObjDef::Method(m));
                            } else {
                                println!("[Linking] {:?} | Skipping DPI import : {:?}", self.cntxt, nc);
                            }
                        }
                    }
                    // Import package
                    else {
                        for ncc in &nc.child {
                            // Check package name is known
                            if let Some(ObjDef::Package(_)) = self.objects.get(&ncc.attr["pkg_name"]) {
                                if ncc.attr["name"] == "*" {
                                    li.imports.push(ncc.attr["pkg_name"].clone());
                                } else {
                                    // TODO : check that the name exist in the context
                                    //        decide how this should be handled in the local info:
                                    //        maybe copy the def from the import ?
                                    println!("[Linking] {:?} | Skipping Import {:?}", self.cntxt, ncc.attr);
                                }
                            } else {
                                println!("[Linking] {:?} | Import Package {} not found", self.cntxt, ncc.attr["pkg_name"]);
                            }
                        }
                    }
                }
                AstNodeKind::Declaration => {
                    let m = DefMember::new(nc);
                    // Check type was defined
                    if nc.child.get(0).map(|x| x.kind==AstNodeKind::Enum) == Some(true) {
                        self.add_enum_def(&nc.child[0],li);
                    }
                    self.check_type(nc,li);
                    // add_def ensure it was not already defined
                    li.add_def(m.name.clone(),ObjDef::Member(m));
                }
                AstNodeKind::Type if self.cntxt.last().unwrap().0 == AstNodeKind::Function => {
                    let t = DefType::from(nc);
                    let m = DefMember{
                        name: self.cntxt.last().unwrap().1.clone(),
                        kind : t, is_const: false, unpacked: None, access: Access::Local};
                    if let DefType::User(k) = &m.kind {
                        if self.find_def(&k.name,k.scope.as_ref(),li,false,false).is_none() {
                            println!("[Linking] {:?} | Type {:?} undeclared", self.cntxt, k.name);
                        }
                    }
                    // println!("[Linking] {:?} | Method {} has return type {:?}", self.cntxt, m.name,m.kind);
                    li.add_def(m.name.clone(),ObjDef::Member(m));
                }
                AstNodeKind::VIntf => {
                    let t = DefType::VIntf(TypeVIntf::from(nc));
                    for ncc in &nc.child {
                        match ncc.kind {
                            AstNodeKind::Identifier => {
                                let m = DefMember{
                                    name : ncc.attr["name"].clone(),
                                    kind : t.clone(),
                                    is_const : false,
                                    unpacked : ncc.attr.get("unpacked").map_or(None,|x| Some(x.clone())),
                                    access   : Access::Public // TODO
                                };
                                li.add_def(m.name.clone(),ObjDef::Member(m));
                            }
                            AstNodeKind::Params => {}
                            _ => println!("[Linking] {:?} | VIntf Skipping {}",self.cntxt, ncc.kind),
                        }
                    }
                }
                AstNodeKind::Typedef => {
                    if let Some(c) = nc.child.get(0) {
                        let d = DefType::from(c);
                        match &d {
                            // Add Enum value if any
                            DefType::Enum(te) => {
                                // println!("[Linking] Typedef enum {:?}", te);
                                for tev in te {
                                    li.add_def(tev.clone(),ObjDef::EnumValue(nc.attr["name"].clone()));
                                }
                                li.add_def(nc.attr["name"].clone(),ObjDef::Type(d));
                            }
                            // Expand forward declaration
                            DefType::None => {
                                // println!("[Linking] {:?} | Forward declaration for {}", self.cntxt, nc.attr["name"]);
                                let mut scope = None;
                                if let Some((_,v)) = self.cntxt.get(0) {
                                    scope = Some(v.clone());
                                }
                                if let Some(fd) = self.find_def(&nc.attr["name"],scope.as_ref(),li,false,true) {
                                    // println!("[Linking] {:?} | Forward declaration for {} = {:?}", self.cntxt, nc.attr["name"],fd);
                                    let fdc = fd.clone();
                                    li.add_def(nc.attr["name"].clone(),fdc);
                                } else {
                                    println!("[Linking] {:?} | Type {:?} undeclared", self.cntxt, nc.attr["name"]);
                                }
                            }
                            _ => li.add_def(nc.attr["name"].clone(),ObjDef::Type(d))
                        }
                        // Add typedef definition
                        ;
                    }
                }
                AstNodeKind::Enum => self.add_enum_def(nc,li),
                AstNodeKind::Port => {
                    let mut p = DefPort::new(nc,&mut port_dir,&mut port_idx);
                    li.add_def(p.name.clone(),ObjDef::Port(p));
                    // Handle Ansi Port, check type
                    // if self.ports.contains_key(&p.name) {
                    //     p.idx = self.ports[&p.name].idx;
                    //     self.ports.insert(p.name.clone(),p);
                    // } else {
                    //     println!("[{:?}] Port {} definition without declaration", self.name,p.name);
                    // }
                }
                AstNodeKind::Param => {
                    let p = DefParam::new(nc,&mut port_idx); // Index is actually irrelevant here so reuse the ame as port
                    li.add_def(p.name.clone(),ObjDef::Param(p));
                }
                AstNodeKind::Task |
                AstNodeKind::Function => {
                    // let m = DefMethod::from(nc);
                    // li.add_def(m.name.clone(),ObjDef::Method(m));
                    // Get return type if any and auto-declare
                    // Check content of the method
                    // println!("[Linking] {:?} | Parsing childs of methods {:?} :\n{:?}", self.cntxt, nc.attr,nc.child);
                    self.cntxt.push((nc.kind.clone(), nc.attr["name"].clone()));
                    self.check_ast(&nc, &ast_inc, li,true);
                    self.cntxt.pop();
                }
                // Interface defintion
                AstNodeKind::Modport => {
                    let d : DefModport = nc.child.iter().map(|x| x.attr["name"].clone()).collect();
                    li.add_def(nc.attr["name"].clone(),ObjDef::Modport(d));
                    // TODO: add check
                }
                // Handle Non-Ansi port declaration
                AstNodeKind::Clocking => {
                    // TODO: update once parsing actually extract all part of the clocking block
                    let d : DefClocking = nc.child.iter()
                                            .filter(|x| x.kind==AstNodeKind::Identifier)
                                            .map(|x| x.attr["name"].clone())
                                            .collect();
                    // println!("[{:?}] Clocking {:?} : {:?} \n\t{:#?}", self.name, nc.attr["name"], d,n);
                    li.add_def(nc.attr["name"].clone(),ObjDef::Clocking(d));
                    // TODO: add check
                }
                AstNodeKind::Covergroup => {
                    let mut d = DefCovergroup::new(nc.attr["name"].clone());
                    li.add_def(nc.attr["name"].clone(),ObjDef::Covergroup(d));
                    // TODO: add check
                }
                //-----------------------------
                // Check
                AstNodeKind::Extends  => {
                    // println!("[Linking] {:?} | Extendind {:?}",self.cntxt, nc.attr);
                    self.check_type(nc,li);
                }
                AstNodeKind::Identifier => {
                    self.check_ident(nc,&li);
                }
                AstNodeKind::Event => self.check_ident(nc,&li),
                AstNodeKind::Instances => {
                    self.check_inst(nc,li);
                }
                AstNodeKind::MethodCall => self.check_call(nc,li),
                AstNodeKind::MacroCall  => self.check_macro(&nc,li),
                AstNodeKind::CaseItem   => {
                    // Check case item itself: need to update the AST
                    self.check_ast(&nc, &ast_inc, li,false);
                }
                // Assignement:
                // - Check every variable has been declared
                // - Check type compatibility
                AstNodeKind::Assign => {
                    self.search_ident(&nc,&li);
                },
                AstNodeKind::Assert      |
                AstNodeKind::Concat      |
                AstNodeKind::Expr        |
                AstNodeKind::ExprGroup   |
                AstNodeKind::Operation   |
                AstNodeKind::Return      |
                AstNodeKind::Sensitivity |
                AstNodeKind::Statement   |
                AstNodeKind::SystemTask  |
                AstNodeKind::Wait        => {
                    self.search_ident(&nc,&li);
                },
                // TODO :
                AstNodeKind::Define     => {}
                AstNodeKind::Constraint => {}
                // Whitelist
                AstNodeKind::Value  |
                AstNodeKind::Timescale  => {}
                _ => {println!("[Linking] {:?} | Skipping {:?} ({} childs) : {:?}", self.cntxt, nc.kind, nc.child.len(), nc.attr);}
            }
        }
        if new_cntxt {
            li.defs.pop();
        }
    }

    // Search for identifier in all children
    pub fn add_enum_def(&self, node: &AstNode, li: &mut LocalInfo) {
        let enum_type = DefType::from(node);
        // println!("[{:?}] enum type = {}", self.name,node);
        for nc in &node.child {
            match nc.kind {
                AstNodeKind::EnumIdent  => {
                    li.add_def(nc.attr["name"].clone(),ObjDef::EnumValue("".to_owned()));
                },
                AstNodeKind::Identifier => {
                    let m = DefMember{
                        name     : nc.attr["name"].clone(),
                        kind     : enum_type.clone(),
                        unpacked : nc.attr.get("unpacked").map_or(None,|x| Some(x.clone())),
                        is_const : false,
                        access   : Access::Public
                    };
                    // println!("[{:?}] Adding enum : {:?}", self.name,m);
                    li.add_def(m.name.clone(),ObjDef::Member(m));
                }
                _ => println!("[Linking] {:?} | Enum: Skipping {}",self.cntxt, nc.kind),
            }
        }
    }

    // Search for identifier in all children
    pub fn search_ident(&self, node: &AstNode, li: &LocalInfo) {
        // println!("[CompObj] {} | Searching: {}",self.name,node);
        for n in &node.child {
            match n.kind {
                AstNodeKind::Identifier => self.check_ident(&n,li),
                _ => if n.child.len()>0 {self.search_ident(&n,li)},
            }
        }
    }


    // TODO: evaluate a cache version of find_def
    // Find a definition from a string
    pub fn find_def<'a>(&'a self, name: &String, scope: Option<&String>,li: &'a LocalInfo, check_base: bool, check_obj: bool) -> Option<&'a ObjDef> {
        // if name == "type_name" {println!("[Linking] {:?} | searching for {} : scope = {:?}, check_base={}, check_obj={}",self.cntxt,name,scope,check_base,check_obj);}
        if let Some(scope_name) = scope {
            if let Some(ObjDef::Package(di)) = &self.objects.get(scope_name) {
                return di.defs.get(name);
            }
            match self.find_def(scope_name,None,li,false,check_obj) {
                Some(ObjDef::Class(cd)) => {
                    if let Some(bd) = cd.defs.get(name) {
                        return Some(bd);
                    }
                    if check_base {
                        return self.find_def_in_base(cd,name,li)
                    }
                    return None;
                }
                Some(di) => {println!("[Linking] {:?} | Ignoring scope definition: {:?}",self.cntxt,di);return None;}
                _ => {return None;}
            }
        }
        // Check local definition
        if check_obj {
            // TODO: write some traits or whatever to get easy access to the defs
            match &li.obj {
                Some(ObjDef::Class(d)) => {
                    if d.defs.contains_key(name) {
                        return Some(&d.defs[name]);
                    }
                }
                Some(ObjDef::Module(d)) => {
                    if d.defs.contains_key(name) {
                        return Some(&d.defs[name]);
                    }
                }
                Some(ObjDef::Package(d)) => {
                    if d.defs.contains_key(name) {
                        return Some(&d.defs[name]);
                    }
                }
                _ => {}
            }
        }
        for i in &li.defs {
            if i.contains_key(name) {
                return Some(&i[name]);
            }
        }
        // Check in current link context
        if self.cntxt.len()>1 {
            for (_k,n) in &self.cntxt {
                match self.objects.get(n) {
                    Some(ObjDef::Package(di)) => {
                        if di.defs.contains_key(name) {
                            return di.defs.get(name);
                        }
                    }
                    // TODO: support Class ?
                    _ => {}
                }
            }
        }
        // TODO: Check in base class if any
        if check_base {
            if let Some(ObjDef::Class(cd)) = &li.obj {
                if let Some(bd) = self.find_def_in_base(cd,name,li) {
                    return Some(bd);
                }
            }
        }
        // Check in imports
        for i in &li.imports {
            if let ObjDef::Package(di) = &self.objects[i] {
                if di.defs.contains_key(name) {
                    return Some(&di.defs[name]);
                }
            }
        }
        // Last try: Check top
        self.objects.get(name)
    }

    pub fn find_def_in_base<'a>(&'a self, cd: &DefClass, name: &String, li: &'a LocalInfo) -> Option<&'a ObjDef> {
        let mut o = cd;
        // println!("[Linking] {:?} | Class {:?} -> looking for base",self.cntxt,cd.name);
        while let Some(bct) = &o.base {
            // if name=="type_name" {println!("[Linking] Class {} looking for {} in base class {}", o.name, name, bct.name);}
            match self.find_def(&bct.name,None,li,false,true) {
                Some(ObjDef::Class(bcd)) =>  {
                    // if name=="type_name" {println!("[Linking] Class {} looking for {} in base class {} \n{:#?}", o.name, name, bcd.name, bcd);}
                    // println!("[Linking] {:?} | Class {:?} has base {:?}",self.cntxt,cd.name,bct.name);
                    if bcd.defs.contains_key(name) {
                        return Some(&bcd.defs[name]);
                    }
                    o = &bcd;
                }
                Some(bcd) => {
                    println!("[Linking] {:?} | Class {:?} -> Base class {} is not a class : {:?}",self.cntxt,cd.name,bct.name,bcd);
                    break;
                }
                None => {
                    println!("[Linking] {:?} | Class {:?} -> Base class {} not found",self.cntxt,cd.name,bct.name);
                    break;
                }
            }
        }
        None
    }


    // Find definition of a node Identifier, checking the scope
    pub fn find_ident_def<'a>(&'a self, node: &AstNode, li: &'a LocalInfo, check_obj: bool) -> Option<&'a ObjDef> {
        let name = &node.attr["name"];
        let scope = if node.has_scope() {Some(&node.child[0].attr["name"])} else {None};
        return self.find_def(name,scope,li,true,check_obj);
    }

    // Check a node identifier is properly defined and check any hierarchical access to it.
    pub fn check_ident(&self, node: &AstNode, li: &LocalInfo) {
        match node.attr["name"].as_ref() {
            "this"  => {}
            "super" => {}
            _ => match self.find_ident_def(node,li,false) {
                Some(ObjDef::Module(_d)) => {
                    // Check for hierarchical access
                    if node.child.len() != 0 {
                    }
                }
                Some(ObjDef::Class(_d)) => {
                    // Check for hierarchical access
                    if node.child.len() != 0 {
                    }
                }
                Some(ObjDef::Covergroup(_d)) => {
                    // Check for hierarchical access
                    if node.child.len() != 0 {
                    }
                }
                Some(_d) => {
                    // Check for hierarchical access
                    if node.child.len() != 0 {
                        // let t = d.get_type();
                        // match t {
                        //     DefType::User(x) => {
                        //         // println!("[Linking] {:?} | Checking Node {:?} with type {:?}", self.cntxt, node.attr, x);
                        //         let td = self.find_def(&x.name,x.scope.as_ref(),li,false);
                        //         match td {
                        //             // Structure : Allow access to field and potentially slice if array of struct
                        //             Some(ObjDef::Type(DefType::Struct(_td))) => {}//println!("[Linking] {:?} | Identifier {} is a struct named {}", self.cntxt, node.attr["name"], x.name),
                        //             // Enum type: allow function first/last/next...
                        //             Some(ObjDef::Type(DefType::Enum(_td))) => {}
                        //             // Typdedef of usertype -> need to fully solve the type
                        //             Some(ObjDef::Type(DefType::User(_td))) => {}
                        //             Some(ObjDef::Param(_td)) => {}
                        //             // Interface/module : allow access to signal/instance/clocking block/ ...
                        //             Some(ObjDef::Module(_td)) => {} //println!("[Linking] {:?} | Identifier {} is a module/interface named {}", self.cntxt, node.attr["name"], x.name),
                        //             // Class
                        //             Some(ObjDef::Class(_td)) => {}
                        //             Some(_x) => println!("[Linking] {:?} | Identifier {} with user type {} = {:?}", self.cntxt, node.attr["name"], x.name, _x),
                        //             _ => println!("[Linking] {:?} | No definition for user type {:?}", self.cntxt, x.name)
                        //         }
                        //     }
                        //     // Virtual interface: allow access to fields of the virtual interface
                        //     DefType::VIntf(_x) => {}
                        //     // Should only expect slice child
                        //     DefType::IntVector(_x) => {}
                        //     DefType::IntAtom(_x) => {}
                        //     // Need differentiation between the different primary type
                        //     DefType::Primary(_x) => {}
                        //     DefType::None => {}
                        //     _ => println!("[Linking] {:?} | Identifier {} found with type {:?}\n\tDefinition = {:?}\n\tChilds = {:?}", self.cntxt, node.attr["name"],t,d,node.child)
                        // }
                    }
                }
                None => {
                    println!("[Linking] {:?} | Identifier {:?} undeclared", self.cntxt, node.attr["name"]);
                }
            }
        }
    }

    // Analyse a module/interface instance
    pub fn check_type(&self, node: &AstNode, li: &LocalInfo) {
        if node.attr.contains_key("type") {
            // println!("[Linking] {:?} | Checking type in {:?}", self.cntxt, node.attr);
            let name = &node.attr["type"];
            let scope = if node.has_scope() {Some(&node.child[0].attr["name"])} else {None};
            match name.as_ref() {
                "logic" | "bit" | "reg" => {},
                "genvar" | "process" | "struct" | "enum"=> {},
                "byte" | "shortint" | "int" | "longint" | "integer" | "time" => {},
                "shortreal" | "real" | "realtime" | "string" | "void" | "chandle" | "event" => {},
                _ => {
                    if self.find_def(name,scope,li,false,false).is_none() {
                        println!("[Linking] {:?} | Type {:?} undeclared", self.cntxt, node.attr["type"]);
                    }
                }
            }
        }
    }

    // Analyse a module/interface instance
    pub fn check_inst(&self, node: &AstNode, li: &mut LocalInfo) {
        // Paranoid check : to be removed
        if !node.attr.contains_key("type") {
            println!("[Linking] Instance with no type: {:?}", node.attr);
            return;
        }
        // Instance module should appear as a top object
        match self.objects.get(&node.attr["type"]) {
            Some(ObjDef::Module(d)) => {
                // println!("[Linking] Instance type {:?}\n\tDefinition = {:?}", node.attr["type"],d);
                let mut ports : Vec<DefPort> = d.ports.values().cloned().collect();
                let mut params : Vec<DefParam> = d.params.values().cloned().collect();
                for n in &node.child {
                    match n.kind {
                        AstNodeKind::Instance => {
                            for nc in &n.child {
                                match nc.kind {
                                    AstNodeKind::Port => {
                                        // println!("[{}] Instance {} of {}, port {:?} = {:?}", self.cntxt, n.attr["name"],c.attr["type"],nc.attr, nc.child);
                                        if ports.len() == 0 {
                                            println!("[Linking] {:?} |  Too many ports in instance {} of {}",self.cntxt,nc.attr["name"], node.attr["type"]);
                                            break;
                                        }
                                        match nc.attr["name"].as_ref() {
                                            // When unamed, port are taken in order
                                            "" => {ports.remove(0);}
                                            // Implicit connection
                                            ".*" => {
                                                // Checked that signal with same name of ports are defined
                                                for p in &ports {
                                                    if let Some(_d) = self.find_def(&p.name,None,li,false,false) {
                                                        // Type checking
                                                    } else {
                                                        println!("[Linking] {:?} | Missing signal for implicit connection to {} in {}", self.cntxt, p, n.attr["name"])
                                                    }

                                                }
                                                ports.clear();
                                            }
                                            // Named connection
                                            _ => {
                                                if let Some(i) = ports.iter().position(|x| x.name == nc.attr["name"]) {
                                                    // println!("[{}] Calling {} with argument name {} found at index {} of {}", self.cntxt, d.name, nc.attr["name"], i, ports.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                                    ports.remove(i);
                                                }
                                                else {
                                                    println!("[Linking] {:?} | Unknown port name {} in instance {} of {}", self.cntxt, nc.attr["name"], n.attr["name"], d.name );
                                                }
                                            }
                                        }
                                        // Check identifiers use in binding are OK
                                        // TODO: check type / direction as well
                                        self.search_ident(&nc,li);
                                    }
                                    _ => println!("[Linking] {:?} | Instance {} of {}, skipping {}", self.cntxt, n.attr["name"],nc.attr["type"],nc.kind)
                                    // _ => {} // Ignore all other node
                                }
                            }
                            li.add_def(n.attr["name"].clone(),ObjDef::Module(d.clone()));
                        }
                        AstNodeKind::Params => {
                            for nc in &n.child {
                                match nc.kind {
                                    AstNodeKind::Param => {
                                        // println!("[{}] Instance of {:?}, param {:?}", self.cntxt, c.attr["type"],nc.attr);
                                        if params.len() == 0 {
                                            println!("[Linking] {:?} | Too many arguments in parameters of {}: ({:?})", self.cntxt, d.name, nc);
                                            break;
                                        }
                                        match nc.attr["name"].as_ref() {
                                            // When unamed, port are taken in order
                                            "" => {params.remove(0);}
                                            // Nameed connection
                                            _ => {
                                                if let Some(i) = params.iter().position(|x| x.name == nc.attr["name"]) {
                                                    // println!("[{}] Calling {} with argument name {} found at index {} of {}", self.cntxt, d.name, nc.attr["name"], i, params.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                                    params.remove(i);
                                                }
                                                else {
                                                    println!("[Linking] {:?} | Calling {} with unknown parameter name {} in {}", self.cntxt, d.name, nc.attr["name"], params.iter().fold(String::new(), |acc, x| format!("{}{:?},", acc,x)));
                                                }
                                            }
                                        }
                                    }
                                    _ => println!("[Linking] {:?} | Instance of {:?}, skipping {:?}", self.cntxt,node.attr["type"],nc.kind)
                                    // _ => {} // Ignore all other node
                                }
                                // TODO: check parameters as well
                            }
                        }
                        _ => println!("[Linking] {:?} | Instances of {}, skipping {}", self.cntxt, node.attr["type"],n.kind)
                        // _ => {} // Ignore all other node
                    }
                }
                if ports.len() > 0 {
                    // Check if remaining ports are optional or not
                    let ma :Vec<_> = ports.iter().filter(|p| p.default.is_none()).collect();
                    if ma.len() > 0 {
                        println!("[Linking] {:?} | Missing {} arguments in call to {}: {}", self.cntxt, ports.len(), d.name, ma.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                    }
                }
            }
            _ => {
                println!("[Linking] Instance Type {:?} undeclared", node.attr["type"]);
            }
        }
    }

    // Analyze a function/task call
    pub fn check_call(&self, node: &AstNode, li: &LocalInfo) {

        // Check for standard defined method
        if let Some(name) = node.attr.get("name") {
            if name=="randomize" || name=="srandom" {
                return;
            }
        }
        match self.find_ident_def(node,li,true) {
            Some(ObjDef::Method(d)) => {
                let mut ports = d.ports.clone();
                for n in &node.child {
                    match n.kind {
                        AstNodeKind::Ports => {
                            // println!("[Linking] {:?} | Call to Port {:?}", self.cntxt, d.name);
                            for p in &n.child {
                                if ports.len() == 0 {
                                    println!("[Linking] {:?} | Too many arguments in call to {}: {:?}", self.cntxt, d.name, p);
                                    break;
                                }
                                // When unamed, port are taken in order
                                if p.attr["name"] == "" {
                                    ports.remove(0);
                                } else {
                                    if let Some(i) = ports.iter().position(|x| x.name == p.attr["name"]) {
                                        // println!("[Linking] {:?} | Calling {} with argument name {} found at index {} of {}", self.cntxt, d.name, p.attr["name"], i, ports.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                        ports.remove(i);
                                    }
                                    else {
                                        println!("[Linking] {:?} | Calling {} with unknown argument name {} in {}", self.cntxt, d.name, p.attr["name"], ports.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                    }
                                }
                            }
                        }
                        _ => {} // Ignore all other node
                    }
                }
                if ports.len() > 0 {
                    // Check if remaining ports are optional or not
                    let ma :Vec<_> = ports.iter().filter(|p| p.default.is_none()).collect();
                    if ma.len() > 0 {
                        println!("[Linking] {:?} | Missing {} arguments in call to {}: {}", self.cntxt, ports.len(), d.name, ma.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                    }
                }
            }
            Some(_) => println!("[Linking] {:?} | {:?} is not a method ", self.cntxt, node.attr.get("name")),
            None    => println!("[Linking] {:?} | Unsolved call to {:?} ({:?})", self.cntxt, node.attr.get("name"), node.attr)
        }
    }

    // Analyze a function/task call
    pub fn check_macro(&self, node: &AstNode, li: &LocalInfo) {
        // println!("[Linking] {:?} | Checking Macro call {} ", self.cntxt, node);
        match self.find_ident_def(node,li,true) {
            Some(ObjDef::Macro(d)) => {
                // println!("[Linking] {:?} | Found macro {:?}", self.cntxt, d)
                let mut ports = d.ports.clone();
                for nc in &node.child {
                    if ports.len() == 0 {
                        println!("[Linking] {:?} | Too many arguments in call to {}: {}", self.cntxt, d.name, nc);
                        break;
                    }
                    ports.remove(0);
                }
                if ports.len() > 0 {
                    // Check if remaining ports are optional or not
                    let ma :Vec<_> = ports.iter().filter(|p| p.is_opt==false).collect();
                    if ma.len() > 0 {
                        println!("[Linking] {:?} | Missing {} arguments in call to {}: {}", self.cntxt, ports.len(), d.name, ma.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                    }
                }
            }
            Some(_) => println!("[Linking] {:?} | {:?} is not a macro ", self.cntxt, node.attr.get("name")),
            None => println!("[Linking] {:?} | Unsolved macro {:?}", self.cntxt, node.attr.get("name"))
        }
    }

}
