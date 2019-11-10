// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::collections::{HashMap, HashSet};

use crate::ast::Ast;
use crate::ast::astnode::{AstNode,AstNodeKind};

use crate::comp::comp_obj::{CompObj,ObjDef};
use crate::comp::prototype::{DefMethod,DefMacro,MacroPort,Port,PortDir,SignalType};

#[derive(Debug, Clone)]
pub struct CompLib {
    pub name   : String,
    pub objects: HashMap<String, CompObj>
}

impl CompLib {

    // Create a library containing definition of all object compiled
    // Try to fix any missing reference, analyze hierarchical access, ...
    pub fn new(name: String, ast_list: Vec<Ast>, ast_inc: HashMap<String,Ast>) -> CompLib {
        let mut lib = CompLib {name, objects:HashMap::new()};
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
            // println!("[{}] Linking ...", name);
            let mut import_hdr = o.import_hdr.clone();
            import_hdr.push("!".to_owned());
            let mut import_body = o.import_body.clone();
            // import_body.extend(&import_hdr);
            import_body.append(&mut import_hdr.clone());
            lib.fix_unref(o,&mut missing_scope,&mut import_hdr,&mut import_body);
            lib.check_call(o,&mut missing_scope,&mut import_body);
            // Add current scope to the import
            import_body.push(name.clone());
            // Check definition
            for v in o.definition.values() {
                match v {
                    ObjDef::Class(def) => {
                        // println!("[{}] Should check unresolved in {:?}", name,def);
                        lib.fix_unref(def,&mut missing_scope,&mut import_hdr,&mut import_body);
                        lib.check_call(def,&mut missing_scope,&mut import_body);
                    }
                    // Check instance paramters
                    ObjDef::Instance => {}
                    _ => {}
                }
            }
        }
        lib
    }

    pub fn get_import_obj(&self, imports: &mut Vec<String>, name: &str, missing: &mut HashSet<String>) -> Option<&ObjDef> {
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

    pub fn find_obj<'a>(&'a self, obj_top:&'a CompObj, name:&str, node: &'a AstNode, missing: &'a mut HashSet<String>,imports: &'a mut Vec<String>) -> Option<&'a ObjDef> {
        // Check if scoped
        if node.has_scope() {
            let scope_name = &node.child[0].attr["name"];
            // Check if already flagged as missing
            if missing.contains(scope_name) {return None;}
            // Check if the scope is a package
            if self.objects.contains_key(scope_name) {
                if node.child[0].has_scope() {
                    if let Some(ObjDef::Class(cc)) = self.objects[scope_name].definition.get(&node.child[0].child[0].attr["name"]) {
                        return cc.definition.get(name);
                    } else {
                        println!("[{}] Unable to find sub-class {}::{}", obj_top.name, scope_name,node.child[0].child[0].attr["name"]);
                        return None;
                    }
                } else {
                    return self.objects[scope_name].definition.get(name);
                }
            }
            // Try to find the scope as part of classes in imported packages
            else if let Some(ObjDef::Class(c)) = self.get_import_obj(imports,scope_name,missing) {
                if node.child[0].has_scope() {
                    if let Some(ObjDef::Class(cc)) = c.definition.get(&node.child[0].child[0].attr["name"]) {
                        return cc.definition.get(name);
                    } else {
                        // Hack until macro expansion is working properly
                        if node.child[0].child[0].attr["name"] == "type_id" {
                            if let Some(ObjDef::Class(cc)) = self.objects["uvm_pkg"].definition.get("uvm_reg_field") {
                                if let Some(ObjDef::Class(ccc)) = cc.definition.get("type_id") {
                                    return ccc.definition.get(name);
                                }
                            }
                        } else {
                            println!("[{}] Unable to find sub-class {}::{}", obj_top.name, scope_name,node.child[0].child[0].attr["name"]);
                            return None;
                        }
                    }
                } else {
                    return c.definition.get(name);
                }
            }
            // Scoped not found -> Flag it to avoid future useless search
            else {
                println!("[{}] Unable to find scope {}", obj_top.name, scope_name);
                missing.insert(scope_name.clone());
                return None
            }
            // println!("[{}] Unsolved ref {}::{} ", obj_top.name, scope_name, name);
            // return None;
        }
        // Check in current context
        if obj_top.definition.contains_key(name) {
            return obj_top.definition.get(name);
        }
        // Check in base class
        let mut o = obj_top;
        while let Some(base) = &o.base_class {
            if let Some(ObjDef::Class(bc)) = self.get_import_obj(imports,&base,missing) {
                // println!("[{}] Searching {} in {}", obj_top.name, name, bc.name);
                if bc.definition.contains_key(name) {
                    // println!("[{}] Found {} in {}", obj_top.name,name,bc.name);
                    return bc.definition.get(name)
                }
                o = bc;
            } else {break;}
        }
        // Not scoped : checked amongst imported package
        self.get_import_obj(imports,&name,missing)
    }

    pub fn fix_unref(&self, o: &CompObj,missing_scope: &mut HashSet<String>,import_hdr: &mut Vec<String>,import_body:&mut Vec<String>) {
        for (name,node) in &o.unref {
            match node.kind {
                AstNodeKind::Header => {
                    if self.find_obj(&o,name,&node,missing_scope,import_hdr).is_some() {
                        continue;
                    }
                }
                AstNodeKind::Port        |
                AstNodeKind::Extends     |
                AstNodeKind::Declaration |
                AstNodeKind::Identifier  |
                AstNodeKind::Type        => {
                    if self.find_obj(&o,name,&node,missing_scope,import_body).is_some() {
                        continue;
                    }
                }
                _ => {println!("[{}] CompLib | Skipping {}",o.name,node);}
            }
            // If we reached this point it means the reference was not found anywhere
            println!("[{}] Unsolved ref {} ({})", o.name, name, node.kind);
        }
    }

    // Check call to function/task/instance/...
    pub fn check_call(&self, o: &CompObj,missing_scope: &mut HashSet<String>,imports: &mut Vec<String>) {
        for c in &o.call {
            match c.kind {
                AstNodeKind::MethodCall => {
                    if let Some(name) = c.attr.get("name") {
                        if name=="randomize" || name=="srandom" {
                            continue;
                        }
                        if let Some(obj) = self.find_obj(&o,name,&c,missing_scope,imports) {
                            if let ObjDef::Method(d) = obj {
                                let mut ports = d.ports.clone();
                                for n in &c.child {
                                    match n.kind {
                                        AstNodeKind::Ports => {
                                            // println!("[{}] Call to Port {:?}", o.name, d.name);
                                            for p in &n.child {
                                                if ports.len() == 0 {
                                                    println!("[{}] Too many arguments in call to {}: {:?}", o.name, d.name, p);
                                                    break;
                                                }
                                                // When unamed, port are taken in order
                                                if p.attr["name"] == "" {
                                                    ports.remove(0);
                                                } else {
                                                    if let Some(i) = ports.iter().position(|x| x.name == p.attr["name"]) {
                                                        // println!("[{}] Calling {} with argument name {} found at index {} of {}", o.name, d.name, p.attr["name"], i, ports.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                                        ports.remove(i);
                                                    }
                                                    else {
                                                        println!("[{}] Calling {} with unknown argument name {} in {}", o.name, d.name, p.attr["name"], ports.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                                    }
                                                }
                                            }
                                        }
                                        _ => {} // Ignore all other node
                                    }
                                }
                                if ports.len() > 0 {
                                    // Check if remaining ports are optional or not
                                    let ma :Vec<_> = ports.iter().filter(|p| p.default.is_none()).collect();
                                    if ma.len() > 0 {
                                        println!("[{}] Missing {} arguments in call to {}: {}", o.name, ports.len(), d.name, ma.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                    }
                                }
                            } else {
                                println!("[{}] {} is not a method : {:?}", o.name, name, obj);
                            }
                        } else {
                            println!("[{}] Unsolved call to {} ({})", o.name, name, c.kind);
                        }
                    }
                    else {
                        println!("[{}] check_call | No name in {}", o.name, c);
                    }
                }
                AstNodeKind::MacroCall => {
                    if let Some(name) = c.attr.get("name") {
                        if let Some(obj) = self.find_obj(&o,name,&c,missing_scope,imports) {
                            if let ObjDef::Macro(d) = obj {
                                let mut ports = d.ports.clone();
                                for n in &c.child {
                                    if ports.len() == 0 {
                                        println!("[{}] Too many arguments in call to {}: {}", o.name, d.name, n);
                                        break;
                                    }
                                    ports.remove(0);
                                }
                                if ports.len() > 0 {
                                    // Check if remaining ports are optional or not
                                    let ma :Vec<_> = ports.iter().filter(|p| p.is_opt==false).collect();
                                    if ma.len() > 0 {
                                        println!("[{}] Missing {} arguments in call to {}: {}", o.name, ports.len(), d.name, ma.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                    }
                                }
                            } else {
                                println!("[{}] {} is not a macro : {:?}", o.name, name, obj);
                            }
                        } else {
                            println!("[{}] Unsolved call to macro {} ({})", o.name, name, c.kind);
                        }
                    }
                    else {
                        println!("[{}] check_call | No name in {}", o.name, c);
                    }
                }
                AstNodeKind::Instances => {
                    // println!("[{}] Checking instance of {:?}", o.name,c.attr);
                    if let Some(t) = c.attr.get("type") {
                        if let Some(obj) = self.objects.get(t) {
                            if let Some(ObjDef::Module(d)) = obj.definition.get("!") {
                                let mut ports = d.ports.clone();
                                let mut params = d.params.clone();
                                for n in &c.child {
                                    match n.kind {
                                        AstNodeKind::Instance => {
                                            for nc in &n.child {
                                                match nc.kind {
                                                    AstNodeKind::Port => {
                                                        // println!("[{}] Instance {} of {}, port {:?}", o.name, n.attr["name"],c.attr["type"],nc.attr);
                                                        if ports.len() == 0 {
                                                            println!("[{}] Too many arguments in call to {}: {:?}", o.name, d.name, nc);
                                                            break;
                                                        }
                                                        match nc.attr["name"].as_ref() {
                                                            // When unamed, port are taken in order
                                                            "" => {ports.remove(0);}
                                                            // Implicit connection
                                                            ".*" => {
                                                                // Checked that signal with same name of ports are defined
                                                                for p in &ports {
                                                                    if o.definition.contains_key(&p.name) {
                                                                        // Type checking
                                                                    } else {
                                                                        println!("[{}] Missing signal for implicit connection to {} in {}", o.name, p, n.attr["name"])
                                                                    }

                                                                }
                                                                ports.clear();
                                                            }
                                                            // Nameed connection
                                                            _ => {
                                                                if let Some(i) = ports.iter().position(|x| x.name == nc.attr["name"]) {
                                                                    // println!("[{}] Calling {} with argument name {} found at index {} of {}", o.name, d.name, nc.attr["name"], i, ports.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                                                    ports.remove(i);
                                                                }
                                                                else {
                                                                    println!("[{}] Calling {} with unknown argument name {} in {}", o.name, d.name, nc.attr["name"], ports.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => println!("[{}] Instance {} of {}, skipping {}", o.name, n.attr["name"],c.attr["type"],nc.kind)
                                                    // _ => {} // Ignore all other node
                                                }
                                            }
                                        }
                                        AstNodeKind::Params => {
                                            for nc in &n.child {
                                                match nc.kind {
                                                    AstNodeKind::Param => {
                                                        // println!("[{}] Instance of {:?}, param {:?}", o.name, c.attr["type"],nc.attr);
                                                        if params.len() == 0 {
                                                            println!("[{}] Too many arguments in parameters of {}: ({:?})", o.name, d.name, nc);
                                                            break;
                                                        }
                                                        match nc.attr["name"].as_ref() {
                                                            // When unamed, port are taken in order
                                                            "" => {params.remove(0);}
                                                            // Nameed connection
                                                            _ => {
                                                                if let Some(i) = params.iter().position(|x| x.name == nc.attr["name"]) {
                                                                    // println!("[{}] Calling {} with argument name {} found at index {} of {}", o.name, d.name, nc.attr["name"], i, params.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                                                    params.remove(i);
                                                                }
                                                                else {
                                                                    println!("[{}] Calling {} with unknown parameter name {} in {}", o.name, d.name, nc.attr["name"], params.iter().fold(String::new(), |acc, x| format!("{}{:?},", acc,x)));
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => println!("[{}] Instance of {:?}, skipping {:?}", o.name, c.attr["type"],nc.kind)
                                                    // _ => {} // Ignore all other node
                                                }
                                            }
                                            // TODO: check parameters as well
                                        }
                                        _ => println!("[{}] Instances of {}, skipping {}", o.name, c.attr["type"],n.kind)
                                        // _ => {} // Ignore all other node
                                    }
                                }
                                if ports.len() > 0 {
                                    // Check if remaining ports are optional or not
                                    let ma :Vec<_> = ports.iter().filter(|p| p.default.is_none()).collect();
                                    if ma.len() > 0 {
                                        println!("[{}] Missing {} arguments in call to {}: {}", o.name, ports.len(), d.name, ma.iter().fold(String::new(), |acc, x| format!("{}{},", acc,x)));
                                    }
                                }
                            } else {
                                println!("[{}] Unable to get module definition for {} in {:?}", o.name, t,obj.definition);
                            }
                        } else {
                            println!("[{}] Unsolved instance of {}", o.name, t);
                        }
                    } else {
                        println!("[{}] check_call | Instance with no type !\n {}", o.name, c);
                    }
                }
                _ => println!("[{}] check_call | Skipping {}", o.name, c.kind)
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
        o.definition.insert("self".to_owned(),ObjDef::Method(DefMethod::new("self".to_owned(),false)));
        self.objects.insert("process".to_owned(),o);
    }

}

// Temporary uvm definition object
// Need to put in place incremental compilation and load pre-compiled uvm lib
fn get_uvm_lib() -> CompObj {
    let mut o = CompObj::new("uvm_pkg".to_owned());
    // Class
    o.definition.insert("uvm_phase".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_verbosity".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_analysis_export".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_analysis_port".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_comparer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_object".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_object_wrapper".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_objection".to_owned(),ObjDef::Type);
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
    o.definition.insert("uvm_event".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_object_wrapper".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_objection".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_printer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_adapter".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_addr_t".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_bus_op".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_data_t".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_item".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_reg_map".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_table_printer".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_top".to_owned(),ObjDef::Type);
    // Function / Macro
    let mut m = DefMacro::new("`uvm_info".to_owned());
    m.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"VERBOSITY".to_owned(),is_opt: false});
    o.definition.insert("`uvm_info".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_warning".to_owned());
    m.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    o.definition.insert("`uvm_warning".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_error".to_owned());
    m.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    o.definition.insert("`uvm_error".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_fatal".to_owned());
    m.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    o.definition.insert("`uvm_fatal".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_component_utils".to_owned());
    m.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    o.definition.insert("`uvm_component_utils".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_object_utils".to_owned());
    m.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    o.definition.insert("`uvm_object_utils".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_object_param_utils".to_owned());
    m.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    o.definition.insert("`uvm_object_param_utils".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_create".to_owned());
    m.ports.push(MacroPort{name:"SEQ_OR_ITEM".to_owned(),is_opt: false});
    o.definition.insert("`uvm_create".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_create_on".to_owned());
    m.ports.push(MacroPort{name:"SEQ_OR_ITEM".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"SEQR".to_owned(),is_opt: false});
    o.definition.insert("`uvm_create_on".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_send".to_owned());
    m.ports.push(MacroPort{name:"SEQ_OR_ITEM".to_owned(),is_opt: false});
    o.definition.insert("`uvm_send".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_declare_p_sequencer".to_owned());
    m.ports.push(MacroPort{name:"SEQUENCER".to_owned(),is_opt: false});
    o.definition.insert("`uvm_declare_p_sequencer".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_component_utils_begin".to_owned());
    m.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    o.definition.insert("`uvm_component_utils_begin".to_owned(),ObjDef::Macro(m));
    m = DefMacro::new("`uvm_component_utils_begin".to_owned());
    m.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"ARG".to_owned(),is_opt: false});
    m.ports.push(MacroPort{name:"FLAG".to_owned(),is_opt: false});
    o.definition.insert("`uvm_field_enum".to_owned(),ObjDef::Macro(m));
    o.definition.insert("`uvm_component_utils_end".to_owned(),ObjDef::Macro(DefMacro::new("`uvm_component_utils_end".to_owned())));
    let mut m = DefMethod::new("uvm_report_fatal".to_owned(),false);
    m.ports.push(Port{name:"id".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"message".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"verbosity".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("UVM_NONE".to_owned())});
    m.ports.push(Port{name:"file".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: Some("".to_owned())});
    m.ports.push(Port{name:"line".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("".to_owned())});
    o.definition.insert("uvm_report_fatal".to_owned(),ObjDef::Method(m.clone()));
    o.definition.insert("uvm_report_error".to_owned(),ObjDef::Method(m.clone()));
    o.definition.insert("uvm_report_warning".to_owned(),ObjDef::Method(m.clone()));
    o.definition.insert("uvm_report_info".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("run_test".to_owned(),false);
    m.ports.push(Port{name:"test_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: Some("".to_owned())});
    o.definition.insert("run_test".to_owned(),ObjDef::Method(m));
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
    let mut o_ = CompObj::new("uvm_component".to_owned());
    o_.definition.insert("m_name".to_owned(),ObjDef::Type);
    o_.definition.insert("type_name".to_owned(),ObjDef::Type);
    o_.definition.insert("m_current_phase".to_owned(),ObjDef::Type);
    o_.definition.insert("get_parent".to_owned(),ObjDef::Method(DefMethod::new("get_parent".to_owned(),false)));
    o_.definition.insert("get_full_name".to_owned(),ObjDef::Method(DefMethod::new("get_full_name".to_owned(),false)));
    o_.definition.insert("get_name".to_owned(),ObjDef::Method(DefMethod::new("get_name".to_owned(),false)));
    o_.definition.insert("get_type_name".to_owned(),ObjDef::Method(DefMethod::new("get_type_name".to_owned(),false)));
    m = DefMethod::new("create_component".to_owned(),false);
    m.ports.push(Port{name:"requested_type_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    o_.definition.insert("create_component".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("create_object".to_owned(),false);
    m.ports.push(Port{name:"requested_type_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: Some("".to_owned())});
    o_.definition.insert("create_object".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("set_inst_override".to_owned(),false);
    m.ports.push(Port{name:"relative_inst_path".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"original_type_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"override_type_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    o_.definition.insert("set_inst_override".to_owned(),ObjDef::Method(m));
    o_.definition.insert("get_report_verbosity_level".to_owned(),ObjDef::Method(DefMethod::new("get_report_verbosity_level".to_owned(),false)));
    o.definition.insert("uvm_component".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_test".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o.definition.insert("uvm_test".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_env".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o.definition.insert("uvm_env".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_driver".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o_.definition.insert("req".to_owned(),ObjDef::Type);
    o_.definition.insert("rsp".to_owned(),ObjDef::Type);
    o_.definition.insert("seq_item_port".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_driver".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_monitor".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o.definition.insert("uvm_monitor".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_sequencer".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o.definition.insert("uvm_sequencer".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_sequence".to_owned());
    o_.base_class = Some("uvm_sequence_item".to_owned());
    o_.definition.insert("req".to_owned(),ObjDef::Type);
    o_.definition.insert("rsp".to_owned(),ObjDef::Type);
    m = DefMethod::new("get_response".to_owned(),false);
    m.ports.push(Port{name:"response".to_owned(),dir:PortDir::Input,kind:SignalType::new("RSP".to_owned()),default: None});
    m.ports.push(Port{name:"transaction_id".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("-1".to_owned())});
    o_.definition.insert("get_response".to_owned(),ObjDef::Method(m));
    o.definition.insert("uvm_sequence".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_sequence_item".to_owned());
    o_.definition.insert("m_parent_sequence".to_owned(),ObjDef::Type); // Part of uvm_sequence item
    o_.definition.insert("m_sequencer".to_owned(),ObjDef::Type);
    o_.definition.insert("p_sequencer".to_owned(),ObjDef::Type);
    o_.definition.insert("get_starting_phase".to_owned(),ObjDef::Method(DefMethod::new("get_starting_phase".to_owned(),false)));
    m = DefMethod::new("start".to_owned(),false);
    m.ports.push(Port{name:"sequencer".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_sequencer_base".to_owned()),default: None});
    m.ports.push(Port{name:"parent_sequence".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_sequence_base".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"this_priority".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("-1".to_owned())});
    m.ports.push(Port{name:"call_pre_post".to_owned(),dir:PortDir::Input,kind:SignalType::new("bit".to_owned()),default: Some("1".to_owned())});
    o_.definition.insert("start".to_owned(),ObjDef::Method(m));
    o_.definition.insert("get_sequencer".to_owned(),ObjDef::Method(DefMethod::new("get_sequencer".to_owned(),false)));
    o_.definition.insert("get_full_name".to_owned(),ObjDef::Method(DefMethod::new("get_full_name".to_owned(),false)));
    o_.definition.insert("get_sequence_id".to_owned(),ObjDef::Method(DefMethod::new("get_sequence_id".to_owned(),false)));
    o.definition.insert("uvm_sequence_item".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_agent".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o_.definition.insert("is_active".to_owned(),ObjDef::Type);
    o.definition.insert("uvm_agent".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_reg_block".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o_.definition.insert("default_map".to_owned(),ObjDef::Type);
    m = DefMethod::new("create_map".to_owned(),false);
    m.ports.push(Port{name:"name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"base_addr".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg_addr_t".to_owned()),default: None});
    m.ports.push(Port{name:"n_bytes".to_owned(),dir:PortDir::Input,kind:SignalType::new("int unsigned".to_owned()),default: None});
    m.ports.push(Port{name:"endian".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_endianness_e".to_owned()),default: None});
    m.ports.push(Port{name:"byte_addressing".to_owned(),dir:PortDir::Input,kind:SignalType::new("bit".to_owned()),default: Some("1".to_owned())});
    o_.definition.insert("create_map".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("configure".to_owned(),false);
    m.ports.push(Port{name:"parent".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg_block".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"hdl_path".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: Some("".to_owned())});
    o_.definition.insert("configure".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("find_block".to_owned(),false);
    m.ports.push(Port{name:"name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"root".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg_block".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"accessor".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_object".to_owned()),default: Some("null".to_owned())});
    o_.definition.insert("find_block".to_owned(),ObjDef::Method(m));
    o_.definition.insert("lock_model".to_owned(),ObjDef::Method(DefMethod::new("lock_model".to_owned(),false)));
    o_.definition.insert("reset".to_owned(),ObjDef::Method(DefMethod::new("reset".to_owned(),false)));
    o.definition.insert("uvm_reg_block".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_reg_predictor".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    o_.definition.insert("bus_in".to_owned(),ObjDef::Type);
    o_.definition.insert("map".to_owned(),ObjDef::Type);
    o_.definition.insert("adapter".to_owned(),ObjDef::Type);
    o_.definition.insert("reg_ap".to_owned(),ObjDef::Type);
    o_.definition.insert("get_full_name".to_owned(),ObjDef::Method(DefMethod::new("get_full_name".to_owned(),false)));
    o.definition.insert("uvm_reg_predictor".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_reg_sequence".to_owned());
    o_.base_class = Some("uvm_sequence".to_owned());
    o_.definition.insert("m_verbosity".to_owned(),ObjDef::Type); // Not true, but just to avoid error until we know how to follow properly the inheritance tree
    m = DefMethod::new("write_reg".to_owned(),false);
    m.ports.push(Port{name:"rg".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg".to_owned()),default: None});
    m.ports.push(Port{name:"status".to_owned(),dir:PortDir::Output,kind:SignalType::new("uvm_status_e".to_owned()),default: None});
    m.ports.push(Port{name:"value".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg_data_t".to_owned()),default: None});
    m.ports.push(Port{name:"path".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_path_e".to_owned()),default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(Port{name:"map".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg_map".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"prior".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("-1".to_owned())});
    m.ports.push(Port{name:"extension".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_object".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"fname".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: Some("".to_owned())});
    m.ports.push(Port{name:"lineno".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("0".to_owned())});
    o_.definition.insert("write_reg".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("read_reg".to_owned(),false);
    m.ports.push(Port{name:"rg".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg".to_owned()),default: None});
    m.ports.push(Port{name:"status".to_owned(),dir:PortDir::Output,kind:SignalType::new("uvm_status_e".to_owned()),default: None});
    m.ports.push(Port{name:"value".to_owned(),dir:PortDir::Output,kind:SignalType::new("uvm_reg_data_t".to_owned()),default: None});
    m.ports.push(Port{name:"path".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_path_e".to_owned()),default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(Port{name:"map".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg_map".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"prior".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("-1".to_owned())});
    m.ports.push(Port{name:"extension".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_object".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"fname".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: Some("".to_owned())});
    m.ports.push(Port{name:"lineno".to_owned(),dir:PortDir::Input,kind:SignalType::new("int".to_owned()),default: Some("0".to_owned())});
    o_.definition.insert("read_reg".to_owned(),ObjDef::Method(m));
    o.definition.insert("uvm_reg_sequence".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_config_db".to_owned());
    let mut m = DefMethod::new("get".to_owned(),false);
    m.ports.push(Port{name:"cntxt".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_component".to_owned()),default: None});
    m.ports.push(Port{name:"inst_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"field_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"value".to_owned(),dir:PortDir::Inout,kind:SignalType::new("T".to_owned()),default: None});
    o_.definition.insert("get".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("set".to_owned(),false);
    m.ports.push(Port{name:"cntxt".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_component".to_owned()),default: None});
    m.ports.push(Port{name:"inst_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"field_name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"value".to_owned(),dir:PortDir::Input,kind:SignalType::new("T".to_owned()),default: None});
    o_.definition.insert("set".to_owned(),ObjDef::Method(m));
    o.definition.insert("uvm_config_db".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_report_server".to_owned());
    o_.definition.insert("get_server".to_owned(),ObjDef::Method(DefMethod::new("get_server".to_owned(),false)));
    o.definition.insert("uvm_report_server".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_factory".to_owned());
    o_.definition.insert("get".to_owned(),ObjDef::Method(DefMethod::new("get".to_owned(),false)));
    o.definition.insert("uvm_factory".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_reg".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    m = DefMethod::new("include_coverage".to_owned(),false);
    m.ports.push(Port{name:"scope".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"models".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_reg_cvr_t".to_owned()),default: None});
    m.ports.push(Port{name:"accessor".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_object".to_owned()),default: Some("null".to_owned())});
    o_.definition.insert("include_coverage".to_owned(),ObjDef::Method(m));
    o.definition.insert("uvm_reg".to_owned(),ObjDef::Class(Box::new(o_)));
    o_ = CompObj::new("uvm_reg_field".to_owned());
    o_.base_class = Some("uvm_component".to_owned());
    let mut ot = CompObj::new("type_id".to_owned());
    m = DefMethod::new("create".to_owned(),false);
    m.ports.push(Port{name:"name".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: None});
    m.ports.push(Port{name:"parent".to_owned(),dir:PortDir::Input,kind:SignalType::new("uvm_component".to_owned()),default: Some("null".to_owned())});
    m.ports.push(Port{name:"contxt".to_owned(),dir:PortDir::Input,kind:SignalType::new("string".to_owned()),default: Some("".to_owned())});
    ot.definition.insert("create".to_owned(),ObjDef::Method(m));
    o_.definition.insert("type_id".to_owned(),ObjDef::Class(Box::new(ot)));
    o.definition.insert("uvm_reg_field".to_owned(),ObjDef::Class(Box::new(o_)));

    o
}