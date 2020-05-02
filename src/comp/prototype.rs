// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::HashMap;
use std::fmt;

use crate::ast::astnode::{AstNode,AstNodeKind};
use crate::comp::def_type::{DefType,TypeUser};
use crate::comp::comp_obj::*;

// --------------
// Port direction
#[derive(Debug, Clone, PartialEq)]
pub enum PortDir {
    Input,
    Output,
    Inout,
    Ref,
    Param,
    Modport(String),
}

pub fn str_to_dir(s: &str) -> PortDir {
    match s {
        "input"     =>  PortDir::Input,
        "output"    =>  PortDir::Output,
        "inout"     =>  PortDir::Inout,
        "ref"       =>  PortDir::Ref,
        "parameter" =>  PortDir::Param,
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
            PortDir::Param      => write!(f,"parameter"),
            PortDir::Modport(s) => write!(f,"{}",s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SvArrayKind {Fixed(u32), Dynamic, Queue, Dict(String)}

impl fmt::Display for SvArrayKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SvArrayKind::Fixed(i) => write!(f,"[{}]",i),
            SvArrayKind::Dynamic  => write!(f,"[]"),
            SvArrayKind::Queue    => write!(f,"[$]"),
            SvArrayKind::Dict(s)  => write!(f,"[{}]",s),
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
    pub unpacked : Vec<SvArrayKind>,
    pub idx   : i16,
    pub default : Option<String>,
}

impl DefPort {
    pub fn new(node: &AstNode, dir: &mut PortDir, idx: &mut i16) -> DefPort {
        let d =
            if node.kind == AstNodeKind::Param {
                PortDir::Param
            }
            else if let Some(mp) = node.attr.get("modport") {
                str_to_dir(mp)
            } else {
                node.attr.get("dir").map(|x| *dir = str_to_dir(x));
                dir.clone()
            };
        *idx += 1;
        DefPort{
            name: "".to_string(),
            dir : d,
            // is_param : ,
            kind: DefType::from(node),
            unpacked : Vec::new(),
            // unpacked : node.attr.get("unpacked").map_or(None,|x| Some(x.clone())),
            idx : idx.clone(),
            default: None
        }
    }

    // Update a port definition with the name, unpacked dimension and default value (if any)
    pub fn updt(&mut self, idx: &mut i16, node : &AstNode)  {
        let mut allow_slice = true;
        self.name = node.attr["name"].clone();
        self.idx = idx.clone();
        *idx += 1;
        for nc in &node.child {
            match nc.kind {
                AstNodeKind::Slice if allow_slice => self.unpacked.push(parse_dim(nc)),
                AstNodeKind::Value      => {self.default = Some(nc.attr["value"].clone()); allow_slice = false; }
                AstNodeKind::Identifier => {self.default = Some(nc.attr["name"].clone());  allow_slice = false; }
                AstNodeKind::Type       => {self.default = Some(nc.attr["type"].clone());  allow_slice = false; }
                // TODO !!!
                AstNodeKind::Slice      |
                AstNodeKind::Concat     |
                AstNodeKind::StructInit |
                AstNodeKind::Branch     |
                AstNodeKind::SystemTask |
                AstNodeKind::Expr       => {self.default = Some("".to_owned());  allow_slice = false; }
                _ => {
                    allow_slice = false;
                    println!("[DefPort] Port {} | Skipping {:?} : {:?}",self.name, nc.kind, nc.attr);
                }
            }
        }
    }
}

pub fn parse_dim(node : &AstNode,) -> SvArrayKind {
    if node.child.len() == 0 {SvArrayKind::Dynamic}
    else if node.child.len() > 1 {SvArrayKind::Fixed(0)}
    // else if node.attr.contains_key("range") {self.unpacked.push(SvArrayKind::Fixed(0));}
    else {
        match node.child[0].kind {
            AstNodeKind::Type => SvArrayKind::Dict(node.child[0].attr["type"].clone()),
            AstNodeKind::Identifier => {
                // TODO: determine if the identifier is a user-type or a constant (Default to constant for the moment)
                SvArrayKind::Fixed(0)
            }
            AstNodeKind::Value => {
                // TODO: try to parse the value as int
                SvArrayKind::Fixed(0)
            }
            AstNodeKind::Expr => {
                if node.child[0].attr.get("value") == Some(&"$".to_string()) {
                    SvArrayKind::Queue
                } else {
                    // println!("[DefPort] Port {} | Slice childs = {:?}", self.name, node.child);
                    // TODO: expression parser to extract the actual value
                    SvArrayKind::Fixed(0)
                }
            }
            _ => {
                println!("[parse_dim] Slice with attr {:?} | child kind = {}", node.child[0].attr, node.child[0].kind);
                SvArrayKind::Fixed(0)
            }
        }
    }

}

impl fmt::Display for DefPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {}{:?}{}",
            self.dir,
            self.kind,
            self.name,
            self.unpacked,
            // if let Some(s) = &self.unpacked {format!(" {}", s)} else {"".to_owned()},
            if let Some(s) = &self.default {format!(" = {}", s)} else {"".to_owned()},
        )
    }
}



pub fn param_value(node: &AstNode) -> String {
    let mut s = "".to_owned();
    for npc in &node.child {
        match npc.kind {
            AstNodeKind::Type |
            AstNodeKind::Identifier => s.push_str(&npc.attr["name"].clone()),
            AstNodeKind::Value => s.push_str(&npc.attr["value"].clone()),
            // TODO
            AstNodeKind::Slice => {},
            AstNodeKind::Concat => {},
            AstNodeKind::StructInit => {},
            AstNodeKind::Expr => {},
            AstNodeKind::Branch => {},
            AstNodeKind::SystemTask => {},
            _ => {
                println!("[ParamValue] Skipping param value {:?}",npc);
            }
        }
    }
    s
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
                        for npc in &np.child {
                            if npc.kind==AstNodeKind::Identifier {
                                let mut pc = p.clone();
                                pc.updt(&mut prev_idx,npc);
                                d.ports.push(pc);
                            }
                        }
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
        // println!("[Method] {:?}", d);
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
    pub name : String,
    pub defs : HashMap<String,ObjDef>,
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
// Block definition
// TODO: Add some field for loop/if info
#[derive(Debug, Clone)]
pub struct DefBlock {
    pub name : String,
    pub defs : HashMap<String,ObjDef>,
}

impl DefBlock {
    pub fn new(name: String) -> DefBlock {
        DefBlock {
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
    pub params : HashMap<String,DefPort>,
    pub ports  : HashMap<String,ObjDef>,
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
    pub unpacked : Vec<SvArrayKind>,
    pub access   : Access,
}

impl DefMember {
    pub fn new(node: &AstNode) -> DefMember {
        // if node.attr.contains_key("name") {println!("[DefMember] Found a name instead of a child !\n{:#?}", node);}
        // println!("[DefMember] {:#?}", node);
        DefMember{
            name     : if node.attr.contains_key("name") {node.attr["name"].clone()} else {"".to_string()},
            kind     : DefType::from(node),
            is_const : node.kind==AstNodeKind::Param, // TODO
            unpacked : Vec::new(),
            access   : Access::Public // TODO
        }
    }

    // Update a port definition with the name, unpacked dimension and default value (if any)
    pub fn updt(&mut self, node : &AstNode)  {
        // println!("[DefMember] Member from {} ", node);
        self.name = node.attr["name"].clone();
        for nc in &node.child {
            match nc.kind {
                AstNodeKind::Slice => self.unpacked.push(parse_dim(nc)),
                _ => break
            }
        }
        // if self.name=="m_data" {println!("[DefMember] Member {:?} ", self);}
        // println!("[DefMember] Member {:?} ", self);
    }
}



// Class
#[derive(Debug, Clone)]
pub struct DefClass {
    pub name    : String,
    pub base    : Option<TypeUser>,
    pub params  : HashMap<String,ObjDef>,
    pub defs    : HashMap<String,ObjDef>,
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
