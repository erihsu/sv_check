// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::fmt;

use crate::ast::astnode::{AstNode,AstNodeKind};

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

pub fn str_to_dir(s: &String) -> PortDir {
    match s.as_ref() {
        "input" =>  PortDir::Input,
        "output" =>  PortDir::Output,
        "inout" =>  PortDir::Inout,
        "ref" =>  PortDir::Ref,
        _ => PortDir::Modport(s.clone()),
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

// ------------
// Signal type

#[derive(Debug, Clone)]
pub struct SignalType {
    pub name : String,
    pub scope : Option<String>,
    pub packed : Option<String>,
    pub unpacked : Option<String>,
    pub signed : bool,
    // TODO: packed/unpacked dimension
}

impl SignalType {
    pub fn new(name: String) -> SignalType {
        SignalType {
            name : name,
            scope : None,
            packed : None,
            unpacked : None,
            signed : false,
        }
    }
}

impl From<&AstNode> for SignalType {
    fn from(node: &AstNode) -> Self {
        let mut st = SignalType::new(node.attr.get("type").unwrap_or(&"".to_owned()).to_owned());
        // println!("[SignalType] {:?}", node);
        node.attr.get("packed"  ).map(|a| st.packed   = Some(a.clone()));
        node.attr.get("unpacked").map(|a| st.unpacked = Some(a.clone()));
        node.attr.get("signing" ).map(|a| st.signed   = a=="signed");
        node.child.get(0).map(|c|
            if c.kind==AstNodeKind::Scope{st.scope = Some(c.attr["name"].clone());}
        );
        st
    }
}

impl fmt::Display for SignalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}",
            if self.signed {"signed "} else {""},
            if let Some(s) = &self.scope {format!("{}::", s)} else {"".to_owned()},
            self.name,
            if let Some(s) = &self.packed {format!(" {}", s)} else {"".to_owned()},
        )
    }
}

// -----------
// Port
#[derive(Debug, Clone)]
pub struct Port {
    pub name  : String,
    pub dir   : PortDir,
    pub kind  : SignalType,
}

impl Port {
    pub fn new(node: &AstNode, prev_dir: PortDir) -> Port {
        Port{
            name: node.attr["name"].clone(),
            dir : node.attr.get("dir").map_or_else(|| prev_dir.clone(),|d| str_to_dir(d)),
            kind: SignalType::from(node)
        }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}{}",
            self.dir,
            self.kind,
            self.name,
            if let Some(s) = &self.kind.unpacked {format!(" {}", s)} else {"".to_owned()},
        )
    }
}

// -----------
// Parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name  : String,
    pub kind  : SignalType,
}

// ------------------
// Module definition
#[derive(Debug, Clone)]
pub struct DefModule {
    pub name   : String,
    pub params : Vec<Param>,
    pub ports  : Vec<Port>,
}

// ------------------
// Function definition
#[derive(Debug, Clone)]
pub struct DefMethod {
    pub name   : String,
    pub ports  : Vec<Port>,
    pub ret    : Option<SignalType>,
    pub is_task: bool
}

impl DefMethod {
    pub fn new(name: String, is_task: bool) -> DefMethod {
        DefMethod {
            name:name,
            ports:Vec::new(),
            ret:None,
            is_task:is_task
        }
    }
}

impl fmt::Display for DefMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}{}\n",
            if self.is_task {"Task"} else {"Function"},
            self.name,
            if let Some(s) = &self.ret {format!(" -> {}", s)} else {"".to_owned()},
        )?;
        for p in &self.ports {
            write!(f,"\t{}\n",p)?;
        }
        Ok(())
    }
}


// ------------------
// Function definition
#[derive(Debug, Clone)]
pub struct DefMacro {
    pub name   : String,
    pub ports  : Vec<String>
}

impl DefMacro {
    pub fn new(name: String) -> DefMacro {
        DefMacro {name:name, ports:Vec::new()}
    }
}

impl fmt::Display for DefMacro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Macro {}\n",self.name)?;
        for p in &self.ports {
            write!(f,"\t{}\n",p)?;
        }
        Ok(())
    }
}
