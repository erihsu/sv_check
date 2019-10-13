// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::{HashMap, HashSet};

use crate::ast::Ast;
use crate::ast::astnode::{AstNode,AstNodeKind};

use crate::comp::comp_obj::{CompObj,ObjDef};

#[derive(Debug, Clone)]
pub struct CompLib {
    pub name   : String,
    pub objects: HashMap<String, CompObj>
}

impl CompLib {

    // Create a library containing definition of all object compiled
    // Try to fix any missing reference, analyze hierarchical access, ...
    pub fn new(name: String, ast_list: Vec<Ast>, ast_inc: HashMap<String,Ast>) -> CompLib {
        let mut lib = CompLib {name:name, objects:HashMap::new()};
        let mut missing_scope : HashSet<String> = HashSet::new();
        // Create a top object for type/localparam definition without scope
        lib.objects.insert("!".to_owned(),CompObj::new("!".to_owned()));
        lib.add_std_obj(); // Add definition for all standard lib classes
        // Extract object from all ASTs
        for ast in ast_list {
            CompObj::from_ast(&ast, &ast_inc, &mut lib.objects);
        }
        lib.objects.insert("uvm_pkg".to_owned(),get_uvm_lib());
        // Linking
        for (name,o) in &lib.objects {
            let mut import_hdr = o.import_hdr.clone();
            import_hdr.push("!".to_owned());
            let mut import_body = o.import_body.clone();
            // import_body.extend(&import_hdr);
            import_body.append(&mut import_hdr.clone());
            lib.fix_unref(name,o,&mut missing_scope,&mut import_hdr,&mut import_body);
            // Add current scope to the import
            import_body.push(name.clone());
            // Check definition
            for (k,v) in &o.definition {
                match v {
                    ObjDef::Class(def) => {
                        // println!("[{}] Should check unresolved in {:?}", name,def);
                        lib.fix_unref(k,def,&mut missing_scope,&mut import_hdr,&mut import_body);
                    }
                    _ => {}
                }
            }
        }
        return lib;
    }

    pub fn find_obj(&self, imports: &mut Vec<String>, name: &str, missing: &mut HashSet<String>) -> Option<&ObjDef> {
        for pkg in imports {
            // println!("[{}] Searching {} in {}  ",self.name,name,pkg);
            if missing.contains(pkg) {continue;}
            if !self.objects.contains_key(pkg) {
                // println!("[{}] Unable to find import package {}", self.name, pkg);
                missing.insert(pkg.clone());
            } else {
                // println!("[{}] Searching {} in definition of {} ",self.name,name,pkg);
                if self.objects[pkg].definition.contains_key(name) {
                    return Some(&self.objects[pkg].definition[name]);
                }
            }
        }

        None
    }

    pub fn find_scoped(&self, obj_name:&String, name:&String, node: &AstNode, missing: &mut HashSet<String>,imports: &mut Vec<String>) -> bool {
        let scope_name = &node.attr["name"];
        let mut found = false;
        // Check if already flagged as missing
        if missing.contains(scope_name) {return false;}
        // Check if the scope is a package
        if self.objects.contains_key(scope_name) {
            found = self.objects[scope_name].definition.contains_key(name);
        }
        // Try to find the scope as part of classes in imported packages
        else if let Some(ObjDef::Class(c)) = self.find_obj(imports,scope_name,missing) {
            found = c.definition.contains_key(name);
        }
        // Scoped not found -> Flag it to avoid future useless search
        else {
            println!("[{}] Unable to find scope {}", obj_name, scope_name);
            missing.insert(scope_name.clone());
        }
        if !found {println!("[{}] Unsolved ref {}::{} ", obj_name, scope_name, name);}
        return found;
    }

    pub fn fix_unref(&self, obj_name:&String, o: &CompObj,missing_scope: &mut HashSet<String>,import_hdr: &mut Vec<String>,import_body:&mut Vec<String>) {
        for (name,node) in &o.unref {
            let mut found = false;
            match node.kind {
                AstNodeKind::Header => {
                    // Check if scoped or if we need to search amongst all import
                    if node.child.len() > 0 {
                        if node.child[0].kind == AstNodeKind::Scope {
                            self.find_scoped(obj_name,name,&node.child[0],missing_scope,import_hdr);
                            continue;
                        }
                    }
                    if let Some(_) = self.find_obj(import_hdr,&name,missing_scope) {
                        continue;
                    }
                }
                AstNodeKind::Port        |
                AstNodeKind::Extends     |
                AstNodeKind::Declaration |
                AstNodeKind::Identifier  |
                AstNodeKind::Type        => {
                    if node.child.len() > 0 {
                        if node.child[0].kind == AstNodeKind::Scope {
                            self.find_scoped(obj_name,name,&node.child[0],missing_scope,import_body);
                            continue;
                        }
                    }
                    if let Some(_) = self.find_obj(import_body,&name,missing_scope) {
                        continue;
                    }
                }
                _ => {println!("[{}] Skipping {}",obj_name,node);}
            }
            // Check in base class
            let mut tmp = o;
            let mut has_base = true;
            while !found && has_base {
                has_base = false;
                if let Some(base) = &tmp.base_class {
                    // println!("[{}] Searching for {}. Base class of {:?} is {}", obj_name,name,tmp.name,base);
                    if let Some(ObjDef::Class(bc)) = self.find_obj(import_body,&base,missing_scope) {
                        has_base = true;
                        found = bc.definition.contains_key(name);
                        tmp = bc;
                        // if found {println!("[{}] Found {} in {}", obj_name,name,bc.name);}
                    }
                }
            }
            //
            if !found {
                println!("[{}] Unsolved ref {} ({})", obj_name, name, node.kind);
            }
        }
    }

    // Standard Lib
    // class process;
    //     typedef enum { FINISHED, RUNNING, WAITING, SUSPENDED, KILLED } state;
    //     static function process self();
    //     function state status();
    //     function void kill();
    //     task await();
    //     function void suspend();
    //     function void resume();
    //     function void srandom( int seed );
    //     function string get_randstate();
    //     function void set_randstate( string state );
    // endclass
    pub fn add_std_obj (&mut self)  {
        let mut o = CompObj::new("process".to_owned());
        o.definition.insert("FINISHED".to_owned(),ObjDef::Value);
        o.definition.insert("RUNNING".to_owned(),ObjDef::Value);
        o.definition.insert("WAITING".to_owned(),ObjDef::Value);
        o.definition.insert("SUSPENDED".to_owned(),ObjDef::Value);
        o.definition.insert("KILLED".to_owned(),ObjDef::Value);
        self.objects.insert("process".to_owned(),o);
    }

}

// Temporary uvm definition object
// Need to put in place incremental compilation and load pre-compiled uvm lib
fn get_uvm_lib() -> CompObj {
    let mut o = CompObj::new("uvm_pkg".to_owned());
    // Class
    o.definition.insert("uvm_phase".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_sequence_item".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_verbosity".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_analysis_export".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_analysis_port".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_comparer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_factory".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_object".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_object_wrapper".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_objection".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_block".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_bus_op".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_data_t".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_field".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_map".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_sequencer_base".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_status_e".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_tlm_analysis_fifo".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_active_passive_enum".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_analysis_port".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_coverage_model_e".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_default_comparer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_env".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_event".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_object".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_object_wrapper".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_objection".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_printer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_adapter".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_addr_t".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_bus_op".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_data_t".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_item".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_map".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_report_server".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_table_printer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_top".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_test".to_owned(),ObjDef::Type);
    // Enum
    o.definition.insert("UVM_NONE".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_FULL".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_ACTIVE".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_PASSIVE".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_DEBUG".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_HIGH".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_MEDIUM".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_ALL_ON".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_HEX".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_IS_OK".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_LOW".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_NO_COVERAGE".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_NOT_OK".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_READ".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_WRITE".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_LITTLE_ENDIAN".to_owned(),ObjDef::Value);
    o.definition.insert("UVM_NO_COVERAGE".to_owned(),ObjDef::Value);
    //
    let mut o_comp = CompObj::new("uvm_component".to_owned());
    o_comp.definition.insert("m_name".to_owned(),ObjDef::Type);
    o_comp.definition.insert("type_name".to_owned(),ObjDef::Type);
    o_comp.definition.insert("get_full_name".to_owned(),ObjDef::Type);
    o_comp.definition.insert("m_current_phase".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_component".to_owned(),ObjDef::Class(o_comp));
    let mut o_drv = CompObj::new("uvm_driver".to_owned());
    o_drv.base_class = Some("uvm_component".to_owned());
    o_drv.definition.insert("req".to_owned(),ObjDef::Type);
    o_drv.definition.insert("rsp".to_owned(),ObjDef::Type);
    o_drv.definition.insert("seq_item_port".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_driver".to_owned(),ObjDef::Class(o_drv));
    let mut o_mon = CompObj::new("uvm_monitor".to_owned());
    o_mon.base_class = Some("uvm_component".to_owned());
    o.definition.insert("uvm_monitor".to_owned(),ObjDef::Class(o_mon));
    let mut o_sqr = CompObj::new("uvm_sequencer".to_owned());
    o_sqr.base_class = Some("uvm_component".to_owned());
    o.definition.insert("uvm_sequencer".to_owned(),ObjDef::Class(o_sqr));
    let mut o_seq = CompObj::new("uvm_sequence".to_owned());
    o_seq.definition.insert("req".to_owned(),ObjDef::Type);
    o_seq.definition.insert("rsp".to_owned(),ObjDef::Type);
    o_seq.definition.insert("m_parent_sequence".to_owned(),ObjDef::Type); // Part of uvm_sequence item
    o_seq.definition.insert("m_sequencer".to_owned(),ObjDef::Type);
    o_seq.definition.insert("p_sequencer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_sequence".to_owned(),ObjDef::Class(o_seq));
    let mut o_agt = CompObj::new("uvm_agent".to_owned());
    o_agt.base_class = Some("uvm_component".to_owned());
    o_agt.definition.insert("is_active".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_agent".to_owned(),ObjDef::Class(o_agt));
    let mut o_regb = CompObj::new("uvm_reg_block".to_owned());
    o_regb.definition.insert("default_map".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_block".to_owned(),ObjDef::Class(o_regb));
    let mut o_regp = CompObj::new("uvm_reg_predictor".to_owned());
    o_regp.definition.insert("bus_in".to_owned(),ObjDef::Type);
    o_regp.definition.insert("map".to_owned(),ObjDef::Type);
    o_regp.definition.insert("adapter".to_owned(),ObjDef::Type);
    o_regp.definition.insert("reg_ap".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_predictor".to_owned(),ObjDef::Class(o_regp));
    let mut o_regs = CompObj::new("uvm_reg_sequence".to_owned());
    o_regs.base_class = Some("uvm_sequence".to_owned());
    o_regs.definition.insert("m_verbosity".to_owned(),ObjDef::Type); // Not true, but just to avoid error until we know how to follow properly the inheritance tree
    o.definition.insert("uvm_reg_sequence".to_owned(),ObjDef::Class(o_regs));

    return o;
}