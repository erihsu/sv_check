// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::fmt;

use crate::comp::comp_obj::{ObjDef};
use crate::ast::astnode::{AstNode,AstNodeKind};
use crate::comp::prototype::{DefMember, param_value};
use crate::reporter::{REPORTER, MsgID};

// ------------
// Signal type
#[derive(Debug, Clone)]
pub enum DefType {
    IntVector(TypeIntVector),
    IntAtom(TypeIntAtom),
    Primary(TypePrimary),
    Struct(TypeStruct),
    Enum(Vec<String>),
    VIntf(TypeVIntf),
    User(TypeUser),
    None
}

#[derive(Debug, Clone)]
pub struct TypeIntVector {
    pub name : String,
    pub packed : Option<String>,
    pub signed : bool,
}

/// byte, shortint, int, longint, integer, time
#[derive(Debug, Clone)]
pub enum IntAtomName {Byte, Shortint, Int, Longint, Integer, Time}

impl fmt::Display for IntAtomName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntAtomName::Byte     => write!(f, "byte"),
            IntAtomName::Shortint => write!(f, "shortint"),
            IntAtomName::Int      => write!(f, "int"),
            IntAtomName::Longint  => write!(f, "longint"),
            IntAtomName::Integer  => write!(f, "integer"),
            IntAtomName::Time     => write!(f, "time")
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeIntAtom {
    pub name : IntAtomName,
    pub signed : bool,
}

pub const TYPE_INT  : DefType = DefType::IntAtom(TypeIntAtom {name:IntAtomName::Int  , signed: true});
pub const TYPE_UINT : DefType = DefType::IntAtom(TypeIntAtom {name:IntAtomName::Int  , signed: false});
pub const TYPE_BYTE : DefType = DefType::IntAtom(TypeIntAtom {name:IntAtomName::Byte, signed: false});
pub const TYPE_STR  : DefType = DefType::Primary(TypePrimary::Str);

/// Standard defined type, non integer
#[derive(Debug, Clone)]
pub enum TypePrimary {Shortreal,Real,Realtime,Str,Void,CHandle,Event,Type}

impl fmt::Display for TypePrimary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypePrimary::Shortreal => write!(f, "shortreal"),
            TypePrimary::Real      => write!(f, "real"),
            TypePrimary::Realtime  => write!(f, "realtime"),
            TypePrimary::Str       => write!(f, "string"),
            TypePrimary::Void      => write!(f, "void"),
            TypePrimary::CHandle   => write!(f, "chandle"),
            TypePrimary::Event     => write!(f, "event"),
            TypePrimary::Type      => write!(f, "type")
        }
    }
}

// Structure/Union
#[derive(Debug, Clone)]
pub struct TypeStruct {
    pub is_packed : bool,
    pub members : Vec<ObjDef>,
}


// TODO: KeYVal is used to stored param default value: it should not be a string, but something to handle parameterized param
#[derive(Debug, Clone)]
pub struct KeyVal {pub key:String, pub val:String}
type VecKeyVal = Vec<KeyVal>;

impl From<&AstNode> for VecKeyVal {
    fn from(node: &AstNode) -> Self {
        let mut v = VecKeyVal::new();
        for np in &node.child {
            // println!("[VecKeyVal] {:?} : Params {:?}",node.attr, np);
            match np.kind {
                AstNodeKind::Param => {
                    v.push(KeyVal{key:np.attr["name"].clone(), val: param_value(np)});
                }
                _ => rpt!(MsgID::DbgSkip,np,"Params child")
            }
        }
        v
    }
}


// Virtual Interface type
#[derive(Debug, Clone)]
pub struct TypeVIntf {
    pub name   : String,
    pub params : VecKeyVal,
}

impl From<&AstNode> for TypeVIntf {
    fn from(node: &AstNode) -> Self {
        // println!("TypeVIntf {:?}", node);
        TypeVIntf {
            name   : node.attr["type"].to_owned(),
            params : node.child.iter().find(|x| x.kind==AstNodeKind::Params)
                                .map_or_else(||VecKeyVal::new(),|x| VecKeyVal::from(x))
        }
    }
}

// Enumerate type
#[derive(Debug, Clone)]
pub struct TypeUser {
    pub name   : String,
    pub scope  : Option<String>,
    pub packed : Option<String>,
    pub params : VecKeyVal,
}

impl TypeUser {
    pub fn new(name: String) -> TypeUser {
        TypeUser {name, scope: None, packed: None, params: VecKeyVal::new()}
    }
}

impl From<&AstNode> for TypeUser {
    fn from(node: &AstNode) -> Self {
        // if node.kind==AstNodeKind::Extends {println!("[TypeUser] {:#?}",node);}
        TypeUser {
            name   : node.attr["type"].to_owned(),
            packed : node.attr.get("packed").map_or(None,|x| Some(x.clone())),
            scope  : node.child.get(0)
                        .filter(|x| x.kind==AstNodeKind::Scope)
                        .map(|x| x.attr["name"].clone()),
            // scope  : None,
            params : node.child.iter().find(|x| x.kind==AstNodeKind::Params)
                                .map_or_else(||VecKeyVal::new(),|x| VecKeyVal::from(x))
        }
    }
}

impl From<&AstNode> for DefType {
    fn from(node: &AstNode) -> Self {
        // println!("[DefType] {:?}", node.kind);
        match node.kind {
            AstNodeKind::Enum =>
                DefType::Enum(node.child.iter()
                                .filter(|x| x.kind==AstNodeKind::EnumIdent)
                                .map(|x| x.attr["name"].clone())
                                .collect()
                ),
            AstNodeKind::Struct | AstNodeKind::Union => {
                let mut mv = Vec::new(); //Vec<ObjDef>
                for nc in &node.child {
                    if nc.kind==AstNodeKind::Declaration {
                        let m = DefMember::new(nc);
                        for ncc in &nc.child {
                            if ncc.kind==AstNodeKind::Identifier {
                                let mut mc = m.clone();
                                mc.name = ncc.attr["name"].clone();
                                mc.updt(ncc);
                                mv.push(ObjDef::Member(mc));
                            }
                        }
                    }
                }
                DefType::Struct(TypeStruct {
                    is_packed : node.child.iter().find(|x| x.kind==AstNodeKind::Slice).is_some(),
                    members : mv
                })
            }
            AstNodeKind::VIntf => DefType::VIntf(TypeVIntf::from(node)),
            _ => {
                if node.attr.contains_key("intf") {
                    return DefType::User(TypeUser::new(node.attr["intf"].clone()));
                }
                // println!("[DefType] {:?}", node.attr);
                match node.attr.get("type") {
                    // Implicit type
                    Some(t) => {
                        match t.as_ref() {
                            "bit" | "logic" | "reg" =>
                                DefType::IntVector(TypeIntVector {
                                    name   : t.to_owned(),
                                    packed : node.attr.get("packed").map_or(None,|x| Some(x.clone())),
                                    signed : node.attr.get("signing").map_or(false,|x| x=="signed"),
                                }),
                            // Default type is logic
                            "" =>
                                DefType::IntVector(TypeIntVector {
                                    name   : "logic".to_owned(),
                                    packed : node.attr.get("packed").map_or(None,|x| Some(x.clone())),
                                    signed : node.attr.get("signing").map_or(false,|x| x=="signed"),
                                }),
                            // Integer Atomic type
                            "byte"     => DefType::IntAtom(TypeIntAtom {name:IntAtomName::Byte    , signed : node.is_signed()}),
                            "shortint" => DefType::IntAtom(TypeIntAtom {name:IntAtomName::Shortint, signed : node.is_signed()}),
                            "int"      => DefType::IntAtom(TypeIntAtom {name:IntAtomName::Int     , signed : node.is_signed()}),
                            "longint"  => DefType::IntAtom(TypeIntAtom {name:IntAtomName::Longint , signed : node.is_signed()}),
                            "integer"  => DefType::IntAtom(TypeIntAtom {name:IntAtomName::Integer , signed : node.is_signed()}),
                            "time"     => DefType::IntAtom(TypeIntAtom {name:IntAtomName::Time    , signed : node.is_signed()}),
                            // Primary type
                            "shortreal" => DefType::Primary(TypePrimary::Shortreal),
                            "real"      => DefType::Primary(TypePrimary::Real),
                            "realtime"  => DefType::Primary(TypePrimary::Realtime),
                            "string"    => DefType::Primary(TypePrimary::Str),
                            "void"      => DefType::Primary(TypePrimary::Void),
                            "chandle"   => DefType::Primary(TypePrimary::CHandle),
                            "event"     => DefType::Primary(TypePrimary::Event),
                            "type"      => DefType::Primary(TypePrimary::Type),
                            // Forward declaration
                            "class"     => DefType::None,
                            // User type : TODO, check for interface
                            _ => DefType::User(TypeUser::from(node))
                        }
                    }
                    _ => DefType::IntVector(TypeIntVector {
                            name   : "logic".to_owned(),
                            packed : node.attr.get("unpacked").map_or(None,|x| Some(x.clone())),
                            signed : node.attr.get("signing").map_or(false,|x| x=="signed"),
                        })
                }
            }
        }
    }
}

impl fmt::Display for DefType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefType::IntVector(x) => {
                if x.packed.is_some() {write!(f,"{} [{}]",x.name, x.packed.as_ref().unwrap())}
                else {write!(f,"{}",x.name)}
            }
            DefType::IntAtom(x)   => write!(f,"{}",x.name),
            DefType::Primary(x)   => write!(f,"{}", x),
            DefType::Struct(_x)   => write!(f,"struct"),
            DefType::Enum(_x)     => write!(f,"enum"),
            DefType::VIntf(x)     => write!(f,"interface {}",x.name),
            DefType::User(x)      => {
                if x.packed.is_some() {write!(f,"typedef {} [{}]",x.name, x.packed.as_ref().unwrap())}
                else {write!(f,"typedef {}",x.name)}
            }
            _ =>  write!(f, "{:?}",self)
        }
    }
}
// impl fmt::Display for DefType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}{}{}{}{}",
//             if self.signed {"signed "} else {""},
//             if let Some(s) = &self.scope {format!("{}::", s)} else {"".to_owned()},
//             self.name,
//             if let Some(s) = &self.packed {format!(" {}", s)} else {"".to_owned()},
//             if self.enum_val.len()>0 {format!(" - Enum {:?}", self.enum_val)} else {"".to_owned()},
//         )
//     }
// }