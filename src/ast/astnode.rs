use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AstNodeKind {
    Root, // first node of a tree
    // Top Level Nodes
    Module(String),
    Class(String),
    Interface(String),
    Package(String),
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
    Function(String),
    Task(String),
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
}