// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::{HashMap};

use crate::ast::Ast;
use crate::ast::astnode::{AstNode,AstNodeKind};
use crate::comp::{
    comp_lib::CompLib,
    prototype::*,
    def_type::{DefType,TypeUser,TypeVIntf}
};
use crate::reporter::{REPORTER,/* Severity,*/ MsgID};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ObjDef {
    Module(DefModule), //
    Class(DefClass),
    Package(DefPackage),
    Block(DefBlock),
    Modport(DefModport),
    Clocking(DefClocking),
    Member(DefMember),
    Port(DefPort),
    Instance(String),
    EnumValue(String),
    Method(DefMethod),
    Macro(DefMacro),
    Type(DefType,Vec<SvArrayKind>),
    Covergroup(DefCovergroup),
}

pub type ObjDefParam<'a> = (&'a ObjDef, Option<HashMap<String,String>>);

impl ObjDef {

    //
    pub fn from_ast(ast: &Ast, ast_inc: & HashMap<String,Box<Ast>>, mut lib: &mut CompLib)  {
        for node in &ast.tree.child {
            // println!("[Compiling] Node {:?} ({:?}", node.kind, node.attr);
            match node.kind {
                AstNodeKind::Directive => {
                    if let Some(i) = node.attr.get("include") {
                        match ast_inc.get(i) {
                            Some(a) => {
                                rpt_push_fname!(&a.filename);
                                ObjDef::from_ast(a, &ast_inc, &mut lib);
                                rpt_pop_fname!();
                            }
                            _ => if i!="uvm_macros.svh" {rpt_s!(MsgID::ErrFile,i);}
                        }
                    }
                },
                AstNodeKind::MacroCall => {},
                AstNodeKind::Interface |
                AstNodeKind::Module => {
                    let mut prev_dir = PortDir::Input; // Default port direction to input
                    let mut idx_port = -1;
                    let mut idx_param = -1;
                    let mut d = DefModule::new(node.attr["name"].clone());
                    // println!("[Compiling] Module {}", node.attr["name"]);
                    for node_m in &node.child {
                        // println!(" - {:?}", node_m.kind);
                        match node_m.kind {
                            AstNodeKind::Header => {
                                for n in &node_m.child {
                                    // println!("[ObjDef] {} | Header: {:?}",d.name, n);
                                    match n.kind {
                                        AstNodeKind::Port => {
                                            let p = DefPort::new(n,&mut prev_dir,&mut idx_port);
                                            for nc in &n.child {
                                                if nc.kind==AstNodeKind::Identifier {
                                                    let mut pc = p.clone();
                                                    pc.updt(&mut idx_port,nc);
                                                    d.ports.insert(nc.attr["name"].clone(),ObjDef::Port(pc));
                                                }
                                            }
                                        }
                                        AstNodeKind::Param => {
                                            let p = DefPort::new(n, &mut PortDir::Param, &mut idx_param);
                                            for nc in &n.child {
                                                if nc.kind==AstNodeKind::Identifier {
                                                    let mut pc = p.clone();
                                                    pc.updt(&mut idx_port,nc);
                                                    d.params.insert(nc.attr["name"].clone(),ObjDef::Port(pc));
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => d.parse_body(&node_m,ast_inc, &mut lib.binds)
                        }
                    }
                    // println!("[ObjDef] {:?}", d);
                    lib.objects.insert(d.name.clone(),ObjDef::Module(d));
                }
                AstNodeKind::Package   => {
                    let mut d = DefPackage::new(node.attr["name"].clone());
                    // println!("[Compiling] Package {}", node.attr["name"]);
                    d.parse_body(&node,ast_inc);
                    lib.objects.insert(d.name.clone(),ObjDef::Package(d));
                }
                AstNodeKind::Class   => {
                    let mut d = DefClass::new(node.attr["name"].clone());
                    // println!("[Compiling] Class {}", node.attr["name"]);
                    d.parse_body(&node,ast_inc);
                    lib.objects.insert(d.name.clone(),ObjDef::Class(d));
                }
                AstNodeKind::Define => {
                    if !node.child.is_empty() {
                        // println!("[Compiling] Top define {}", node.attr["name"]);
                        let mut d = DefMacro::new(format!("`{}",node.attr["name"]));
                        for p in &node.child {
                            d.ports.push(MacroPort::new(p));
                        }
                        lib.objects.insert(d.name.clone(),ObjDef::Macro(d));
                    } else if node.attr.contains_key("content") {
                        // println!("[Compiling] Top define {:#?}", node.attr);
                    }
                }
                // Handle special case of type/localparams/define done out of context
                AstNodeKind::Param => {
                    let m = DefMember::new(node);
                    // if m.name != "" {self.defs.insert(m.name.clone(),ObjDef::Member(m.clone()));}
                    for nc in &node.child {
                        if nc.kind==AstNodeKind::Identifier {
                            let mut mc = m.clone();
                            mc.updt(nc);
                            lib.objects.insert(nc.attr["name"].clone(),ObjDef::Member(mc));
                        }
                    }
                }
                AstNodeKind::Typedef => {
                    if let Some(c) = node.child.get(0) {
                        let d = DefType::from(c);
                        // Add Enum value if any
                        if let DefType::Enum(te) = &d {
                            // println!("[CompLib] Typedef enum {:?}", te);
                            for tev in te {
                                lib.objects.insert(tev.clone(),ObjDef::EnumValue(node.attr["name"].clone()));
                            }
                        }
                        // Add typedef definition
                        lib.objects.insert(node.attr["name"].clone(),ObjDef::Type(d,Vec::new()));
                    }
                }
                AstNodeKind::Function => {
                    let m = DefMethod::from(node);
                    lib.objects.insert(m.name.clone(),ObjDef::Method(m));
                }
                // Temporay Whitelist
                AstNodeKind::Import => {}
                _ => rpt!(MsgID::DbgSkip,node,"Root (comp_obj)")
            }
        }
    }

    //
    #[allow(dead_code)]
    pub fn get_typename(&self) -> String {
        match self {
            ObjDef::Module(x)  => format!("module {}", x.name),
            ObjDef::Class(x)   => format!("class {}", x.name),
            ObjDef::Package(x) => format!("package {}", x.name),
            ObjDef::Type(x,_)  => format!("{}", x),
            ObjDef::Member(_x) => "member".to_owned(),
            ObjDef::Port(_x) => "member".to_owned(),
            _ => format!("{:?}", self)
        }
    }

    pub fn get_def(&self, name: &str) -> Option<&ObjDef> {
        match self {
            ObjDef::Class(d) => {
                if d.defs.contains_key(name) {
                    return Some(&d.defs[name]);
                }
                d.params.get(name)
            }
            ObjDef::Module(d) => {
                // if d.defs.contains_key(name) {
                //     return Some(&d.defs[name]);
                // }
                // d.params.get(name)
                d.defs.get(name)
            }
            ObjDef::Package(d) => d.defs.get(name),
            _ => None
        }
    }
}

impl DefModule {
    // Collect signals declaration, instance, type and function definition
    pub fn parse_body(&mut self, node: &AstNode, ast_inc: & HashMap<String,Box<Ast>>, binds  : &mut HashMap<String, Vec<String> >) {
        let mut prev_dir = PortDir::Input; // Default port direction to input
        let mut idx_port = -1 as i16;
        let mut idx_param = (self.params.len() as i16) - 1 ;
        for n in &node.child {
            // println!("[DefModule] {} | next node = {}",self.name, n.kind);
            match n.kind {
                AstNodeKind::Param => {
                    let p = DefPort::new(n,&mut PortDir::Param,&mut idx_param);
                    for nc in &n.child {
                        if nc.kind==AstNodeKind::Identifier {
                            let mut pc = p.clone();
                            pc.updt(&mut idx_port,nc);
                            self.params.insert(nc.attr["name"].clone(),ObjDef::Port(pc));
                        }
                    }
                }
                AstNodeKind::Directive => {
                    n.attr.get("include").map(
                        |i| ast_inc.get(i).map_or_else(
                            || if i!="uvm_macros.svh" {rpt_s!(MsgID::ErrFile,i);} ,
                            |a| {rpt_push_fname!(&a.filename); self.parse_body(&a.tree,ast_inc, binds); rpt_pop_fname!();}
                        )
                    );
                },
                // Handle Non-Ansi port declaration
                AstNodeKind::Port => {
                    let p = DefPort::new(n,&mut prev_dir,&mut idx_port);
                    for nc in &n.child {
                        if nc.kind==AstNodeKind::Identifier {
                            let mut pc = p.clone();
                            pc.updt(&mut idx_port,nc);
                            // Check the port was declared
                            if let Some(ObjDef::Port(pa)) = self.ports.get(&pc.name) {
                                pc.idx = pa.idx;
                                self.ports.insert(pc.name.clone(),ObjDef::Port(pc));
                            } else {
                                rpt!(MsgID::ErrNotFound, nc, &pc.name)
                            }
                        }
                    }
                }
                // Handle Non-Ansi port declaration
                AstNodeKind::Modport => {
                    let d : DefModport = n.child.iter().map(|x| x.attr["name"].clone()).collect();
                    self.defs.insert(n.attr["name"].clone(),ObjDef::Modport(d));
                }
                // Handle Non-Ansi port declaration
                AstNodeKind::Clocking => {
                    // TODO: update once parsing actually extract all part of the clocking block
                    let d : DefClocking = n.child.iter()
                                            .filter(|x| x.kind==AstNodeKind::Identifier)
                                            .map(|x| x.attr["name"].clone())
                                            .collect();
                    // println!("[{:?}] Clocking {:?} : {:?} \n\t{:#?}", self.name, n.attr["name"], d,n);
                    self.defs.insert(n.attr["name"].clone(),ObjDef::Clocking(d));
                }
                //
                AstNodeKind::Typedef => {
                    if let Some(c) = n.child.get(0) {
                        let d = DefType::from(c);
                        // Add Enum value if any
                        if let DefType::Enum(te) = &d {
                            // println!("[CompLib] Typedef enum {:?}", te);
                            for tev in te {
                                self.defs.insert(tev.clone(),ObjDef::EnumValue(n.attr["name"].clone()));
                            }
                        }
                        // Add typedef definition
                        self.defs.insert(n.attr["name"].clone(),ObjDef::Type(d,Vec::new()));
                    }
                }
                AstNodeKind::Enum => {
                    let enum_type = DefType::from(n);
                    // println!("[{:?}] enum type = {}", self.name,enum_type);
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Slice  => {}
                            AstNodeKind::EnumIdent  => {
                                self.defs.insert(nc.attr["name"].clone(),ObjDef::EnumValue("".to_owned()));
                            },
                            AstNodeKind::Identifier => {
                                let m = DefMember{
                                    name: nc.attr["name"].clone(),
                                    kind: enum_type.clone(),
                                    unpacked : Vec::new(),
                                    is_const: false,
                                    access: Access::Public
                                };
                                // println!("[{:?}] Adding enum : {:?}", self.name,m);
                                self.defs.insert(m.name.clone(),ObjDef::Member(m));
                            }
                            _ => rpt!(MsgID::DbgSkip,nc,"Enum declaration (comp_obj)")
                        }
                    }
                }
                AstNodeKind::Struct => {
                    let d = DefType::from(n);
                    let mut unpacked = Vec::new();
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Identifier => {
                                let m = DefMember{
                                    name: nc.attr["name"].clone(),
                                    kind: d.clone(),
                                    unpacked : unpacked.clone(),
                                    is_const: false,
                                    access: Access::Public
                                };
                                // println!("[{:?}] Adding enum : {:?}", self.name,m);
                                self.defs.insert(m.name.clone(),ObjDef::Member(m));
                            }
                            AstNodeKind::Slice  => {
                                unpacked.push(parse_dim(nc));
                            }
                            // Ignore declaration : was already used when getting the type
                            AstNodeKind::Declaration => {}
                            _ => rpt!(MsgID::DbgSkip,nc,"Struct declaration (comp_obj)")
                        }
                    }
                }
                AstNodeKind::Declaration => {
                    let m = DefMember::new(n);
                    if m.name != "" {self.defs.insert(m.name.clone(),ObjDef::Member(m.clone()));}
                    for nc in &n.child {
                        if nc.kind==AstNodeKind::Identifier {
                            let mut mc = m.clone();
                            mc.updt(nc);                            
                            self.defs.insert(nc.attr["name"].clone(),ObjDef::Member(mc));
                        }
                    }
                }
                AstNodeKind::Instances => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Params => {
                                // TODO: collect parameter info and add it to the instance definition
                            }
                            AstNodeKind::Instance => {
                                self.defs.insert(nc.attr["name"].clone(),ObjDef::Instance(n.attr["type"].clone()));
                            }
                            _ => rpt!(MsgID::DbgSkip,nc,"Instance (comp_obj)")
                        }
                    }
                }
                // Branch / For loop : check for instances only
                AstNodeKind::Branch  |
                AstNodeKind::LoopFor => {
                    // println!("[DefModule] {:?} | Branch : {:?}", self.name, n.attr);
                    let blk = self.get_block_inst(n,ast_inc, binds);
                    // TODO: check name conflict
                    self.defs.insert(blk.name.clone(),ObjDef::Block(blk));
                }
                //
                AstNodeKind::Task |
                AstNodeKind::Function => {
                    let m = DefMethod::from(n);
                    self.defs.insert(m.name.clone(),ObjDef::Method(m));
                }
                // Function definition in DPI import
                AstNodeKind::Import => {
                     if n.attr.contains_key("dpi") {
                        if n.attr["kind"]=="import" {
                            if n.child.len() == 1 {
                                let m = DefMethod::from(&n.child[0]);
                                self.defs.insert(m.name.clone(),ObjDef::Method(m));
                            } else {
                                rpt!(MsgID::DbgSkip,n,"DPI import (comp_obj)");
                            }
                        }
                    }
                }
                // Header in a body: Loop definition
                AstNodeKind::Header => {
                    // Check for instances ?
                    rpt!(MsgID::DbgSkip,n,"Header (comp_obj)");
                }
                AstNodeKind::Define  => {
                    // println!("[Compiling] Module define {}", n.attr["name"]);
                    let mut d = DefMacro::new(format!("`{}",n.attr["name"]));
                    for p in &n.child {
                        d.ports.push(MacroPort::new(p));
                    }
                    self.defs.insert(d.name.clone(),ObjDef::Macro(d));
                }
                // TODO
                AstNodeKind::Covergroup => {
                    let d = DefCovergroup::new(n.attr["name"].clone());
                    self.defs.insert(n.attr["name"].clone(),ObjDef::Covergroup(d));
                }
                AstNodeKind::SvaProperty => {}
                AstNodeKind::Bind => {
                    self.parse_bind(n, binds);
                    // println!("[DefModule] {} | Bind Skipping",self.name);
                    // println!("[DefModule] {} | Bind Skipping {}",self.name, n);
                }
                // Temporary: Whitelist node we can safely skip
                // To be removed and replaced by default once everything is working as intended
                AstNodeKind::Timescale |
                AstNodeKind::MacroCall |
                AstNodeKind::Primitive |
                AstNodeKind::Assign    |
                AstNodeKind::Assert    |
                AstNodeKind::SystemTask |
                AstNodeKind::Process   => {}
                //
                AstNodeKind::Class => {
                    let mut d = DefClass::new(n.attr["name"].clone());
                    d.parse_body(&n,ast_inc);
                    self.defs.insert(d.name.clone(),ObjDef::Class(d));
                }
                _ => rpt!(MsgID::DbgSkip,n,"Module top (comp_obj)")
            }
        }
    }

    // Extract info from a bind statement
    pub fn parse_bind(&mut self, node: &AstNode, binds  : &mut HashMap<String, Vec<String> >) {
        let mut path = Vec::new();
        path.push(self.name.clone());
        let mut t = "".to_string();
        for n in &node.child {
            match n.kind {
                AstNodeKind::Identifier => {
                    path.push(n.attr["name"].clone());
                    let mut nc = n;
                    while let Some(ncc) = nc.child.get(0) {
                        path.push(ncc.attr["name"].clone());
                        nc = ncc;
                    }
                }
                AstNodeKind::Instances => {
                    t = n.attr["type"].clone();
                }
                _ => rpt!(MsgID::DbgSkip,n,"Binding (comp_obj)")
            }
        }
        // println!("[DefModule] {} | Binding {} to {:?}",self.name, t, path);
        binds.insert(t,path);
    }

    // TODO : get info from the For loop and name from the branch/for loop
    pub fn get_block_inst(&mut self, node: &AstNode, ast_inc: & HashMap<String,Box<Ast>>, binds  : &mut HashMap<String, Vec<String> >) -> DefBlock {
        // println!("[DefModule] {} | get_block_inst on {:?}",self.name, node.attr);
        let blkname =
            if node.attr.contains_key("block") && node.attr["block"].len()>0 {node.attr["block"].clone()}
            else {format!("blk_{}_{}", if node.attr.contains_key("kind") {node.attr["kind"].clone()} else {"loop".to_string()} ,self.defs.len())};
        let mut blk = DefBlock::new(blkname);
        for n in &node.child {
            match n.kind {
                AstNodeKind::Instances => {
                    // println!("[get_block_inst] {:?} | Instance {:?}", self.name, n.attr);
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Params => {
                                // TODO: collect parameter info and add it to the instance definition
                            }
                            AstNodeKind::Instance => {
                                // println!("[get_block_inst] {:?} | Instance {}", self.name, nc.attr["name"]);
                                blk.defs.insert(nc.attr["name"].clone(),ObjDef::Instance(n.attr["type"].clone()));
                            }
                            _ => rpt!(MsgID::DbgSkip,n,"Instance (comp_obj)")
                        }
                    }
                }
                AstNodeKind::Branch  |
                AstNodeKind::LoopFor => {
                    let sub_blk = self.get_block_inst(n,ast_inc, binds);
                    // TODO: check name conflict
                    blk.defs.insert(sub_blk.name.clone(),ObjDef::Block(sub_blk));
                }
                AstNodeKind::Bind => self.parse_bind(n, binds),
                _ => {}
            }
        }
        // println!("[DefModule] {} | {:?}",self.name, blk);
        blk
    }
}


impl DefPackage {
    // Collect all definition
    pub fn parse_body(&mut self, node: &AstNode, ast_inc: & HashMap<String,Box<Ast>>) {
        for n in &node.child {
            // println!("[DefPackage] {} | next node = {}",self.name, n.kind);
            match n.kind {
                // Include directive
                AstNodeKind::Directive => {
                    n.attr.get("include").map(
                        |i| ast_inc.get(i).map_or_else(
                            || if i!="uvm_macros.svh" {rpt_s!(MsgID::ErrFile,i);},
                            |a| {rpt_push_fname!(&a.filename); self.parse_body(&a.tree,ast_inc); rpt_pop_fname!();}
                        )
                    );
                },
                AstNodeKind::Typedef => {
                    if let Some(c) = n.child.get(0) {
                        let d = DefType::from(c);
                        // Add Enum value if any
                        if let DefType::Enum(te) = &d {
                            // println!("[CompLib] Typedef enum {:?}", te);
                            for tev in te {
                                self.defs.insert(tev.clone(),ObjDef::EnumValue(n.attr["name"].clone()));
                            }
                        }
                        // Add typedef definition
                        self.defs.insert(n.attr["name"].clone(),ObjDef::Type(d,Vec::new()));
                    }
                }
                AstNodeKind::Enum => {
                    let enum_type = DefType::from(n);
                    // println!("[{:?}] enum type = {}", self.name,enum_type);
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Slice  => {}
                            AstNodeKind::EnumIdent  => {
                                self.defs.insert(nc.attr["name"].clone(),ObjDef::EnumValue("".to_owned()));
                            },
                            AstNodeKind::Identifier => {
                                let m = DefMember{
                                    name     : nc.attr["name"].clone(),
                                    kind     : enum_type.clone(),
                                    unpacked : Vec::new(),
                                    is_const : false,
                                    access   : Access::Public
                                };
                                // println!("[{:?}] Adding enum : {:?}", self.name,m);
                                self.defs.insert(m.name.clone(),ObjDef::Member(m));
                            }
                            _ => rpt!(MsgID::DbgSkip,nc,"Enum declaration (comp_obj)")
                        }
                    }
                }
                // AstNodeKind::Struct => {}
                AstNodeKind::Param => {
                    // println!("[DefPackage] {} : {:#?}",self.name, n);
                    let p = DefMember::new(n);
                    for nc in &n.child {
                        if nc.kind==AstNodeKind::Identifier {
                            let mut pc = p.clone();
                            pc.updt(nc);
                            self.defs.insert(nc.attr["name"].clone(),ObjDef::Member(pc));
                        }
                    }
                },
                AstNodeKind::Declaration => {
                    let m = DefMember::new(n);
                    if m.name != "" {self.defs.insert(m.name.clone(),ObjDef::Member(m.clone()));}
                    for nc in &n.child {
                        if nc.kind==AstNodeKind::Identifier {
                            let mut mc = m.clone();
                            mc.name = nc.attr["name"].clone();
                            mc.updt(nc);
                            self.defs.insert(nc.attr["name"].clone(),ObjDef::Member(mc));
                        }
                    }
                }
                AstNodeKind::Instances => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Params => {
                                // TODO: collect parameter info and add it to the instance definition
                            }
                            AstNodeKind::Instance => {
                                self.defs.insert(nc.attr["name"].clone(),ObjDef::Instance(nc.attr["name"].clone()));
                            }
                            _ => rpt!(MsgID::DbgSkip,nc,"Instance (comp_obj)")
                        }
                    }
                }
                // Branch / For loop : check for instances only
                // AstNodeKind::Branch  |
                // AstNodeKind::LoopFor => self.parse_body(&n,ast_inc),
                AstNodeKind::Task |
                AstNodeKind::Function => {
                    let m = DefMethod::from(n);
                    self.defs.insert(m.name.clone(),ObjDef::Method(m));
                }
                AstNodeKind::Import    => {
                    // println!("[CompObj] {} | DPI import {}",self.name, n);
                    if n.attr.contains_key("dpi") {
                        if n.attr["kind"]=="import" {
                            if n.child.len() == 1 {
                                let m = DefMethod::from(&n.child[0]);
                                self.defs.insert(m.name.clone(),ObjDef::Method(m));
                            } else {
                                rpt!(MsgID::DbgSkip,n,"DPI import (comp_obj)")
                            }
                        }
                    }
                }
                AstNodeKind::Define  => {
                    // println!("[Compiling] Package define {}", n.attr["name"]);
                    let mut d = DefMacro::new(format!("`{}",n.attr["name"]));
                    for p in &n.child {
                        d.ports.push(MacroPort::new(p));
                    }
                    self.defs.insert(d.name.clone(),ObjDef::Macro(d));
                }
                // Temporary: Whitelist node we can safely skip
                // To be removed and replaced by default once eveything is working as intended
                AstNodeKind::MacroCall => {}
                AstNodeKind::Timescale => {}
                AstNodeKind::SvaProperty => {}
                //
                AstNodeKind::Class => {
                    let mut d = DefClass::new(n.attr["name"].clone());
                    d.parse_body(&n,ast_inc);
                    self.defs.insert(d.name.clone(),ObjDef::Class(d));
                }
                _ => rpt!(MsgID::DbgSkip,n,"Package Top (comp_obj)")
            }
        }
        // println!("[DefPackage] {} : {:#?}", self.name, self.defs);
    }
}

impl DefClass {
    // Collect all definition
    pub fn parse_body(&mut self, node: &AstNode, ast_inc: & HashMap<String,Box<Ast>>) {
        for n in &node.child {
            // println!("[Compiling] Class {} | next node = {}",self.name, n.kind);
            match n.kind {
                AstNodeKind::Params => {
                    let mut idx_param = -1 as i16;
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Param => {
                                let p = DefPort::new(nc,&mut PortDir::Param,&mut idx_param);
                                for ncc in &nc.child {
                                    if ncc.kind==AstNodeKind::Identifier {
                                        let mut pc = p.clone();
                                        pc.updt(&mut idx_param,ncc);
                                        // rpt_s!(MsgID::DbgStatus,&format!("Port = {}", pc));
                                        self.params.insert(ncc.attr["name"].clone(),ObjDef::Port(pc));
                                    }
                                }
                            }
                            _ => rpt!(MsgID::DbgSkip,nc,"Class parameters (comp_obj)")
                        }
                    }
                }
                AstNodeKind::Extends  => self.base = Some(TypeUser::from(n)),
                AstNodeKind::Implements => {
                    // TODO
                },
                // Include directive
                AstNodeKind::Directive => {
                    n.attr.get("include").map(
                        |i| ast_inc.get(i).map_or_else(
                            || if i!="uvm_macros.svh" {rpt_s!(MsgID::ErrFile,i);},
                            |a| {rpt_push_fname!(&a.filename); self.parse_body(&a.tree,ast_inc); rpt_pop_fname!();}
                        )
                    );
                },
                AstNodeKind::Typedef => {
                    if let Some(c) = n.child.get(0) {
                        let d = DefType::from(c);
                        // Add Enum value if any
                        if let DefType::Enum(te) = &d {
                            // println!("[CompLib] Typedef enum {:?}", te);
                            for tev in te {
                                self.defs.insert(tev.clone(),ObjDef::EnumValue(n.attr["name"].clone()));
                            }
                        }
                        // Add typedef definition
                        self.defs.insert(n.attr["name"].clone(),ObjDef::Type(d,Vec::new()));
                    }
                }
                AstNodeKind::Enum => {
                    let enum_type = DefType::from(n);
                    // println!("[{:?}] enum type = {}", self.name,enum_type);
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Slice  => {}
                            AstNodeKind::EnumIdent  => {
                                self.defs.insert(nc.attr["name"].clone(),ObjDef::EnumValue("".to_owned()));
                            },
                            AstNodeKind::Identifier => {
                                let m = DefMember{
                                    name     : nc.attr["name"].clone(),
                                    kind     : enum_type.clone(),
                                    unpacked : Vec::new(),
                                    is_const : false,
                                    access   : Access::Public
                                };
                                // println!("[{:?}] Adding enum : {:?}", self.name,m);
                                self.defs.insert(m.name.clone(),ObjDef::Member(m));
                            }
                            _ => rpt!(MsgID::DbgSkip, nc, "Enum (comp_obj)")
                        }
                    }
                }
                // AstNodeKind::Struct => {}
                //     println!("[Compiling] Class {} : {:#?}",self.name, n);
                // },
                AstNodeKind::Param      |
                AstNodeKind::Declaration => {
                    let m = DefMember::new(n);
                    if m.name != "" {
                        self.defs.insert(m.name.clone(),ObjDef::Member(m.clone()));
                    }
                    for nc in &n.child {
                        if nc.kind==AstNodeKind::Identifier {
                            let mut mc = m.clone();
                            mc.name = nc.attr["name"].clone();
                            mc.updt(nc);
                            self.defs.insert(nc.attr["name"].clone(),ObjDef::Member(mc));
                        }
                    }
                }
                AstNodeKind::VIntf => {
                    let t = DefType::VIntf(TypeVIntf::from(n));
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Identifier => {
                                // if nc.attr["name"]=="vif" {println!("{:?}", n);}
                                let m = DefMember{
                                    name : nc.attr["name"].clone(),
                                    kind : t.clone(),
                                    is_const : false,
                                    unpacked : Vec::new(),
                                    access   : Access::Public // TODO
                                };
                                self.defs.insert(m.name.clone(),ObjDef::Member(m));
                            }
                            AstNodeKind::Params => {}
                            _ => rpt!(MsgID::DbgSkip, nc, "Virtual Interface (comp_obj)")
                        }
                    }
                }
                AstNodeKind::Instances => {
                    for nc in &n.child {
                        match nc.kind {
                            AstNodeKind::Params => {
                                // TODO: collect parameter info and add it to the instance definition
                            }
                            AstNodeKind::Instance => {
                                self.defs.insert(nc.attr["name"].clone(),ObjDef::Instance(nc.attr["name"].clone()));
                            }
                            _ => rpt!(MsgID::DbgSkip,nc,"Instance (comp_obj)")
                        }
                    }
                }
                // Branch / For loop : check for instances only
                // AstNodeKind::Branch  |
                // AstNodeKind::LoopFor => self.parse_body(&n,ast_inc),
                AstNodeKind::Task |
                AstNodeKind::Function => {
                    let m = DefMethod::from(n);
                    self.defs.insert(m.name.clone(),ObjDef::Method(m));
                }
                AstNodeKind::Define  => {
                    // println!("[Compiling] Class {} | define {}", self.name, n.attr["name"]);
                    let mut d = DefMacro::new(format!("`{}",n.attr["name"]));
                    for p in &n.child {
                        d.ports.push(MacroPort::new(p));
                    }
                    self.defs.insert(d.name.clone(),ObjDef::Macro(d));
                }
                // TODO
                AstNodeKind::Constraint => {}
                AstNodeKind::Covergroup => {}
                AstNodeKind::SvaProperty => {}
                // Temporary: Whitelist node we can safely skip
                // To be removed and replaced by default once eveything is working as intended
                AstNodeKind::MacroCall  |
                AstNodeKind::Timescale  |
                AstNodeKind::Import  => {}
                //
                AstNodeKind::Class => {
                    let mut d = DefClass::new(n.attr["name"].clone());
                    d.parse_body(&n,ast_inc);
                    self.defs.insert(d.name.clone(),ObjDef::Class(d));
                }
                _ => rpt!(MsgID::DbgSkip,n,"Class top (comp_obj)")
            }
        }
        // println!("[Compiling] Class {} : {:#?}", self.name, self.defs);
    }
}
