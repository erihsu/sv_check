// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::{HashMap};

use crate::ast::Ast;
use crate::ast::astnode::{AstNodeKind};

use crate::comp::comp_obj::CompObj;

#[derive(Debug, Clone)]
pub struct CompLib {
    pub name   : String,
    pub objects: HashMap<String, CompObj>,
}

impl CompLib {

    pub fn new(name: String, ast_list: Vec<Ast>) -> CompLib {
        let mut lib = CompLib {name:name, objects:HashMap::new()};
        // Extract object from all ASTs
        for ast in ast_list {
            CompObj::from_ast(ast, &mut lib.objects);
        }
        // Linking
        for (name,o) in &lib.objects {
            for (k,v) in &o.unsolved_ref {
                let mut found = false;
                match v {
                    AstNodeKind::Port => {
                        for pkg in &o.import_hdr {
                            if !lib.objects.contains_key(pkg) {
                                println!("Unable to find package {}", pkg);
                            } else {
                                found = lib.objects[pkg].definition.contains_key(k);
                                if found {
                                    // println!("[{}] Found ref {} in package ({})", name,k,pkg);
                                    break;
                                }
                            }
                        }
                    }
                    AstNodeKind::Declaration => {
                        for pkg in &o.import_body {
                            if ! &lib.objects.contains_key(pkg) {
                                println!("Unable to find package {}", pkg);
                            } else {
                                found = lib.objects[pkg].definition.contains_key(k);
                                if found {
                                    // println!("[{}] Found ref {} in package ({})", name,k,pkg);
                                    break;
                                }
                            }
                        }
                    }
                    _ => {}
                }
                if !found {
                    println!("[{}] Unsolved ref {} ({})", name,k,v);
                }
            }
        }
        return lib;
    }
}