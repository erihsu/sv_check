// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

// use crate::ast::Ast;

// use std::{fs,path,io, mem, str, iter};
// use std::collections::{HashMap};
// use crate::ast::astnode::{AstNode,AstNodeKind};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PortDir {
    Input,
    Output,
    Inout,
    Ref,
    Modport(String),
}

#[derive(Debug, Clone)]
pub struct VectorType {
    pub width : String,
    pub signed : bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SignalType {
    Integer(bool)     , // Integer type (int, shortint, longint, integer) can be signed/unsigned
    Vector(VectorType), // logic/bit vector
    Standard(String)  , // One of the standard type: byte, time, real, shortreal, realtime, event, process, ...
    User(String)      ,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub name  : String,
    pub dir   : PortDir,
    pub kind  : SignalType,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name  : String,
    pub kind  : SignalType,
}

#[derive(Debug, Clone)]
pub struct DefModule {
    pub name   : String,
    pub params : Vec<Param>,
    pub ports  : Vec<Port>,
}

#[derive(Debug, Clone)]
pub struct DefFunc {
    pub name   : String,
    pub ports  : Vec<Port>,
    pub ret    : SignalType,
}

#[derive(Debug, Clone)]
pub struct DefTask {
    pub name   : String,
    pub ports  : Vec<Port>,
}