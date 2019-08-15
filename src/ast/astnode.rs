// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::HashMap;
use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum AstNodeKind {
    Root, // first node of a tree
    Module, Ports, Port, Params, Param,
    Class, Extends, Implements, Function, Task,
    Constraint, Covergroup,
    Interface, Modport, Clocking,
    Package,
    Program,
    Udp,
    Config,
    //
    Header,
    Body,
    //
    Identifier,
    Import,
    Assign,
    Statement, Block,
    Process,
    Sensitivity,
    Event, EventCtrl,
    Fork,
    Wait,
    Branch, Case, CaseItem, LoopFor,Loop,
    Instances,Instance,Bind,
    Nettype,
    Declaration, MethodCall, SystemTask,
    Expr, ExprGroup, Operation,
    New, Args, Slice, Value, Return,
    Assert,
    VIntf,
    Genvar(String),
    Directive, MacroCall, Timescale,
    Type, Typedef, Scope,
    Struct, Union, StructInit, Concat, Replication,
    Enum, EnumIdent,
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
