// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::HashMap;
use std::fmt;

use crate::ast::astnode::{AstNode,AstNodeKind};
use crate::comp::def_type::{DefType,TypeUser};
use crate::comp::comp_obj::*;

// --------------
// Port direction
#[derive(Debug, Clone)]
pub enum PortDir {
    Input,
    Output,
    Inout,
    Ref,
    Modport(String),
}

pub fn str_to_dir(s: &str) -> PortDir {
    match s {
        "input"  =>  PortDir::Input,
        "output" =>  PortDir::Output,
        "inout"  =>  PortDir::Inout,
        "ref"    =>  PortDir::Ref,
        _ => PortDir::Modport(s.to_string()),
    }
}

impl fmt::Display for PortDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortDir::Input      => write!(f,"input"),
            PortDir::Output     => write!(f,"output"),
            PortDir::Inout      => write!(f,"inout"),
            PortDir::Ref        => write!(f,"ref"),
            PortDir::Modport(s) => write!(f,"{}",s),
        }
    }
}


// -----------
// Port
#[derive(Debug, Clone)]
pub struct DefPort {
    pub name  : String,
    pub dir   : PortDir,
    pub kind  : DefType,
    pub unpacked : Option<String>,
    pub idx   : i16,
    pub default : Option<String>,
}

impl DefPort {
    pub fn new(node: &AstNode, dir: &mut PortDir, idx: &mut i16) -> DefPort {
        let n = node.child.iter()
                    .find(|nc| nc.kind!=AstNodeKind::Scope)
                    .map(|x| format!("{}", x.kind));
        let d : PortDir;
        if let Some(mp) = node.attr.get("modport") {
            d = str_to_dir(mp);
        } else {
            node.attr.get("dir").map(|x| *dir = str_to_dir(x));
            d = dir.clone();
        }
        *idx += 1;
        DefPort{
            name: node.attr["name"].clone(),
            dir : d,
            kind: DefType::from(node),
            unpacked : node.attr.get("unpacked").map_or(None,|x| Some(x.clone())),
            idx : idx.clone(),
            default: n
        }
    }
}

impl fmt::Display for DefPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {}{}{}",
            self.dir,
            self.kind,
            self.name,
            if let Some(s) = &self.unpacked {format!(" {}", s)} else {"".to_owned()},
            if let Some(s) = &self.default {format!(" = {}", s)} else {"".to_owned()},
        )
    }
}


// -----------
// Parameter
#[derive(Debug, Clone)]
pub struct DefParam {
    pub name  : String,
    pub kind  : DefType,
    pub idx   : i16,
}

impl DefParam {
    pub fn new(node: &AstNode, idx: &mut i16) -> DefParam {
        *idx += 1;
        DefParam{
            name: node.attr["name"].clone(),
            kind: DefType::from(node),
            idx : idx.clone()
        }
    }
}

// ------------------
// Function definition
#[derive(Debug, Clone)]
pub struct DefMethod {
    pub name   : String,
    pub ports  : Vec<DefPort>,
    pub ret    : Option<DefType>,
    pub is_task: bool
}

impl DefMethod {
    pub fn new(name: String, is_task: bool) -> DefMethod {
        DefMethod {
            name,
            ports:Vec::new(),
            ret:None,
            is_task
        }
    }
}

impl From<&AstNode> for DefMethod {
    fn from(node: &AstNode) -> Self {
        let mut d = DefMethod::new(node.attr["name"].clone(),node.kind==AstNodeKind::Task);
        let mut prev_dir = PortDir::Input; // Default port direction to input
        let mut prev_idx = -1;
        for nc in &node.child {
            // println!("[Function] {}", nc);
            match nc.kind {
                // Add ports
                AstNodeKind::Ports => {
                    for np in &nc.child {
                        let p = DefPort::new(np,&mut prev_dir, &mut prev_idx);
                        d.ports.push(p);
                    }
                }
                // Add return type
                AstNodeKind::Type  => {
                    d.ret = Some(DefType::from(nc));
                }
                // Any other kind means we are in the body
                _ => {break;}
            }
        }
        d
    }
}

impl fmt::Display for DefMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}{}",
            if self.is_task {"Task"} else {"Function"},
            self.name,
            if let Some(s) = &self.ret {format!(" -> {:?}", s)} else {"".to_owned()},
        )?;
        for p in &self.ports {
            writeln!(f,"\t{}",p)?;
        }
        Ok(())
    }
}


// ------------------
// Macro definition
#[derive(Debug, Clone)]
pub struct MacroPort {
    pub name  : String,
    pub is_opt : bool,
}

#[allow(dead_code)]
impl MacroPort {
    pub fn new(node: &AstNode) -> MacroPort {
        MacroPort{
            name: node.attr["name"].clone(),
            is_opt: node.child.len()>0
        }
    }
}

impl fmt::Display for MacroPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}",
            self.name,
            if !self.is_opt {"?".to_owned()} else {"".to_owned()},
        )
    }
}

#[derive(Debug, Clone)]
pub struct DefMacro {
    pub name   : String,
    pub ports  : Vec<MacroPort>
}

#[allow(dead_code)]
impl DefMacro {
    pub fn new(name: String) -> DefMacro {
        DefMacro {name, ports:Vec::new()}
    }
}

impl fmt::Display for DefMacro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Macro {}",self.name)?;
        for p in &self.ports {
            writeln!(f,"\t{}",p.name)?;
        }
        Ok(())
    }
}


// ------------------
// Modport definition : temporary, might need more information than just name (like direction/task)
pub type DefModport = Vec<String>;

// ------------------
// Clocking definition : temporary, might need more information than just name (like direction)
pub type DefClocking = Vec<String>;

// ------------------
// Package definition
#[derive(Debug, Clone)]
pub struct DefPackage {
    pub name       : String,
    pub defs: HashMap<String,ObjDef>,
}

impl DefPackage {
    pub fn new(name: String) -> DefPackage {
        DefPackage {
            name,
            defs: HashMap::new(),
        }
    }
}

// ------------------
// Package definition
#[derive(Debug, Clone)]
pub struct DefCovergroup {
    pub name       : String,
    pub defs: HashMap<String,ObjDef>,
}

impl DefCovergroup {
    pub fn new(name: String) -> DefCovergroup {
        DefCovergroup {
            name,
            defs: HashMap::new(),
        }
    }
}

// ------------------
// Module definition
#[derive(Debug, Clone)]
pub struct DefModule {
    pub name   : String,
    pub params : HashMap<String,DefParam>,
    pub ports  : HashMap<String,DefPort>,
    pub defs   : HashMap<String,ObjDef>,
}

impl DefModule {
    pub fn new(name: String) -> DefModule {
        DefModule {
            name,
            params: HashMap::new(),
            ports : HashMap::new(),
            defs  : HashMap::new(),
        }
    }
}

// ------------------
// Class definition

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Access {Public, Protected, Local}

// Parameter
#[derive(Debug, Clone)]
pub struct DefMember {
    pub name     : String,
    pub kind     : DefType,
    pub is_const : bool,
    pub unpacked : Option<String>,
    pub access   : Access,
}

impl DefMember {
    pub fn new(node: &AstNode) -> DefMember {
        DefMember{
            name     : node.attr["name"].clone(),
            kind     : DefType::from(node),
            is_const : node.kind==AstNodeKind::Param, // TODO
            unpacked : node.attr.get("unpacked").map_or(None,|x| Some(x.clone())),
            access   : Access::Public // TODO
        }
    }
}



// Class
#[derive(Debug, Clone)]
pub struct DefClass {
    pub name      : String,
    pub base      : Option<TypeUser>,
    pub params    : HashMap<String,DefParam>,
    pub defs: HashMap<String,ObjDef>,
}

#[allow(dead_code)]
impl DefClass {
    pub fn new(name: String) -> DefClass {
        DefClass {
            name,
            base  : None,
            params: HashMap::new(),
            defs  : HashMap::new(),
        }
    }
}
