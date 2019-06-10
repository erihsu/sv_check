use std::collections::HashMap;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AstNodeKind {
    Root, // first node of a tree
    // Top Level Nodes
    Module(String),
    Class(String),
    Interface(String),
    Package,
    Program(String),
    Udp(String),
    Bind(String),
    Config(String),
    //
    Header,
    Body,
    //
    Port(String),
    Param(String),
    Import,
    AssignC, AssignB, AssignNB,
    Process(String),
    // Sensitivity list: contains child of Event(signal_name), each with an optional attribute of edge
    Sensitivity,
    Event(String),
    Branch, Case, CaseItem, LoopFor,
    Instance(String),
    Nettype(String),
    Signal(String),
    Genvar(String),
    Typedef,
    Struct, Union,
    Enum, EnumIdent,
    Function,
    Task,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AstNode {
    pub kind  : AstNodeKind,
    pub child : Vec<AstNode>,
    pub attr  : HashMap<String, String>
}

impl AstNode {
    pub fn new(k: AstNodeKind) -> AstNode {
        AstNode {
            kind : k,
            child : Vec::new(),
            attr : HashMap::new()
        }
    }

    pub fn to_string_lvl(&self, lvl:usize) -> String {
        let mut s = format!("{:width$}{} :","",self.kind,width=lvl*2);
        for (k,v) in &self.attr {
            s.push_str(format!(" {}={},",k,v).as_ref());
        }
        s.pop();
        for c in &self.child {
            s.push('\n');
            s.push_str(&c.to_string_lvl(lvl+1));
        }
        return s;
    }
}


impl fmt::Display for AstNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}",self)
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.to_string_lvl(0))
    }
}
