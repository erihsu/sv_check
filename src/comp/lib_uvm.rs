// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::comp::prototype::*;
use crate::comp::comp_obj::{ObjDef};
use crate::comp::def_type::{DefType,TypeUser,TypeStruct, TypePrimary, TypeIntVector, TYPE_INT, TYPE_UINT, TYPE_STR};

pub fn get_uvm_lib() -> ObjDef {
    let mut p = DefPackage::new("uvm_pkg".to_owned());
    // Top level methods
    let mut m = DefMethod::new("run_test".to_owned(),false);
    m.ports.push(DefPort{
        name:"test_name".to_owned(),
        dir:PortDir::Input,
        kind:TYPE_STR,
        idx: 0,unpacked: Vec::new(), default: None});
    p.defs.insert("run_test".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("uvm_report_fatal".to_owned(),false);
    m.ports.push(DefPort{name:"id".to_owned()          , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"message".to_owned()     , dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"verbosity".to_owned()   , dir:PortDir::Input, kind:TYPE_INT, idx: 2, unpacked: Vec::new(), default: Some("UVM_NONE".to_owned())});
    m.ports.push(DefPort{name:"file".to_owned()        , dir:PortDir::Input, kind:TYPE_STR, idx: 3, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"line".to_owned()        , dir:PortDir::Input, kind:TYPE_INT, idx: 4, unpacked: Vec::new(), default: Some("0".to_owned())});
    m.ports.push(DefPort{name:"context_name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 5, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"report_enabled_checked".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 6, unpacked: Vec::new(), default: Some("0".to_owned())});
    p.defs.insert("uvm_report_fatal".to_owned(),ObjDef::Method(m.clone()));
    m.name = "uvm_report_error".to_owned();
    p.defs.insert(m.name.clone(),ObjDef::Method(m.clone()));
    m.name = "uvm_report_warning".to_owned();
    p.defs.insert(m.name.clone(),ObjDef::Method(m.clone()));
    m.name = "uvm_report_info".to_owned();
    p.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("uvm_report_enabled".to_owned(),false);
    m.ports.push(DefPort{name:"verbosity".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"severity".to_owned() , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_severity".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("UVM_NONE".to_owned())});
    m.ports.push(DefPort{name:"id".to_owned()       , dir:PortDir::Input, kind:TYPE_STR, idx: 2, unpacked: Vec::new(), default: Some("".to_owned())});
    p.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("uvm_is_match".to_owned(),false);
    m.ports.push(DefPort{name:"expr".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"str".to_owned() , dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: None});
    p.defs.insert(m.name.clone(),ObjDef::Method(m));
    // Enum
    p.defs.insert("UVM_NONE".to_owned()         , ObjDef::EnumValue("UVM_NONE".to_owned()         ));
    p.defs.insert("UVM_FULL".to_owned()         , ObjDef::EnumValue("UVM_FULL".to_owned()         ));
    p.defs.insert("UVM_ACTIVE".to_owned()       , ObjDef::EnumValue("UVM_ACTIVE".to_owned()       ));
    p.defs.insert("UVM_PASSIVE".to_owned()      , ObjDef::EnumValue("UVM_PASSIVE".to_owned()      ));
    p.defs.insert("UVM_DEBUG".to_owned()        , ObjDef::EnumValue("UVM_DEBUG".to_owned()        ));
    p.defs.insert("UVM_HIGH".to_owned()         , ObjDef::EnumValue("UVM_HIGH".to_owned()         ));
    p.defs.insert("UVM_MEDIUM".to_owned()       , ObjDef::EnumValue("UVM_MEDIUM".to_owned()       ));
    p.defs.insert("UVM_ALL_ON".to_owned()       , ObjDef::EnumValue("UVM_ALL_ON".to_owned()       ));
    p.defs.insert("UVM_HEX".to_owned()          , ObjDef::EnumValue("UVM_HEX".to_owned()          ));
    p.defs.insert("UVM_IS_OK".to_owned()        , ObjDef::EnumValue("UVM_IS_OK".to_owned()        ));
    p.defs.insert("UVM_LOW".to_owned()          , ObjDef::EnumValue("UVM_LOW".to_owned()          ));
    p.defs.insert("UVM_NO_COVERAGE".to_owned()  , ObjDef::EnumValue("UVM_NO_COVERAGE".to_owned()  ));
    p.defs.insert("UVM_NOT_OK".to_owned()       , ObjDef::EnumValue("UVM_NOT_OK".to_owned()       ));
    p.defs.insert("UVM_READ".to_owned()         , ObjDef::EnumValue("UVM_READ".to_owned()         ));
    p.defs.insert("UVM_WRITE".to_owned()        , ObjDef::EnumValue("UVM_WRITE".to_owned()        ));
    p.defs.insert("UVM_LITTLE_ENDIAN".to_owned(), ObjDef::EnumValue("UVM_LITTLE_ENDIAN".to_owned()));
    p.defs.insert("UVM_NO_COVERAGE".to_owned()  , ObjDef::EnumValue("UVM_NO_COVERAGE".to_owned()  ));
    p.defs.insert("UVM_INFO".to_owned()   , ObjDef::EnumValue("UVM_INFO".to_owned()  ));
    p.defs.insert("UVM_WARNING".to_owned(), ObjDef::EnumValue("UVM_WARNING".to_owned()  ));
    p.defs.insert("UVM_ERROR".to_owned()  , ObjDef::EnumValue("UVM_ERROR".to_owned()  ));
    p.defs.insert("UVM_FATAL".to_owned()  , ObjDef::EnumValue("UVM_FATAL".to_owned()  ));
    // parameter value
    p.defs.insert("UVM_SETINT".to_owned(), ObjDef::EnumValue("UVM_SETINT".to_owned()));
    p.defs.insert("UVM_SETOBJ".to_owned(), ObjDef::EnumValue("UVM_SETOBJ".to_owned()));
    p.defs.insert("UVM_SETSTR".to_owned(), ObjDef::EnumValue("UVM_SETSTR".to_owned()));
    p.defs.insert("UVM_COPY".to_owned(), ObjDef::EnumValue("UVM_COPY".to_owned()));
    p.defs.insert("UVM_NOCOPY".to_owned(), ObjDef::EnumValue("UVM_NOCOPY".to_owned()));
    p.defs.insert("UVM_COMPARE".to_owned(), ObjDef::EnumValue("UVM_COMPARE".to_owned()));
    p.defs.insert("UVM_NOCOMPARE".to_owned(), ObjDef::EnumValue("UVM_NOCOMPARE".to_owned()));
    p.defs.insert("UVM_PRINT".to_owned(), ObjDef::EnumValue("UVM_PRINT".to_owned()));
    p.defs.insert("UVM_NOPRINT".to_owned(), ObjDef::EnumValue("UVM_NOPRINT".to_owned()));
    p.defs.insert("UVM_RECORD".to_owned(), ObjDef::EnumValue("UVM_RECORD".to_owned()));
    p.defs.insert("UVM_NORECORD".to_owned(), ObjDef::EnumValue("UVM_NORECORD".to_owned()));
    p.defs.insert("UVM_PACK".to_owned(), ObjDef::EnumValue("UVM_PACK".to_owned()));
    p.defs.insert("UVM_NOPACK".to_owned(), ObjDef::EnumValue("UVM_NOPACK".to_owned()));
    p.defs.insert("UVM_READONLY".to_owned(), ObjDef::EnumValue("UVM_READONLY".to_owned()));
    p.defs.insert("UVM_UNPACK".to_owned(), ObjDef::EnumValue("UVM_UNPACK".to_owned()));
    p.defs.insert("UVM_CHECK_FIELDS".to_owned(), ObjDef::EnumValue("UVM_CHECK_FIELDS".to_owned()));
    // Macro
    let mut mc = DefMacro::new("`uvm_info".to_owned());
    mc.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"VERBOSITY".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_warning".to_owned());
    mc.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_error".to_owned());
    mc.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_fatal".to_owned());
    mc.ports.push(MacroPort{name:"ID".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"MSG".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_component_utils".to_owned());
    mc.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_object_utils".to_owned());
    mc.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_object_param_utils".to_owned());
    mc.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_create".to_owned());
    mc.ports.push(MacroPort{name:"SEQ_OR_ITEM".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_create_on".to_owned());
    mc.ports.push(MacroPort{name:"SEQ_OR_ITEM".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"SEQR".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_send".to_owned());
    mc.ports.push(MacroPort{name:"SEQ_OR_ITEM".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_declare_p_sequencer".to_owned());
    mc.ports.push(MacroPort{name:"SEQUENCER".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_component_utils_begin".to_owned());
    mc.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    mc = DefMacro::new("`uvm_field_enum".to_owned());
    mc.ports.push(MacroPort{name:"T".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"ARG".to_owned(),is_opt: false});
    mc.ports.push(MacroPort{name:"FLAG".to_owned(),is_opt: false});
    p.defs.insert(mc.name.clone(),ObjDef::Macro(mc));
    p.defs.insert("`uvm_component_utils_end".to_owned(),ObjDef::Macro(DefMacro::new("`uvm_component_utils_end".to_owned())));

    //
    let mut o = DefClass::new("uvm_object".to_owned());
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set_name".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("print".to_owned(),false);
    m.ports.push(DefPort{name:"printer".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_printer".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("compare".to_owned(),false);
    m.ports.push(DefPort{name:"rhs".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"comparer".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_comparer".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_name".to_owned(),false);
    m.ret = Some(TYPE_STR);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_full_name".to_owned(),false);
    m.ret = Some(TYPE_STR);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("clone".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_object".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    let mb = DefMember{ name: "__m_uvm_status_container".to_owned(), kind: DefType::User(TypeUser::new("uvm_status_container".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("__m_uvm_field_automation".to_owned(),false);
    m.ports.push(DefPort{name:"tmp_data__".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"what__".to_owned()    , dir:PortDir::Input, kind:TYPE_INT                                             , idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"str__".to_owned()     , dir:PortDir::Input, kind:TYPE_STR                                             , idx: 2, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_component".to_owned());
    o.base = Some(TypeUser::new("uvm_object".to_owned())); // Not directly but no need to complexify yet
    let mut mb = DefMember{ name: "m_name".to_owned(), kind: TYPE_STR, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "type_name".to_owned(), kind: TYPE_STR, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "m_current_phase".to_owned(), kind: DefType::User(TypeUser::new("uvm_phase".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("set_name".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_parent".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_component".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("clone".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_object".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set_inst_override".to_owned(),false);
    m.ports.push(DefPort{name:"relative_inst_path".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"original_type_name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"override_type_name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 2, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("build_phase".to_owned(),false);
    m.ports.push(DefPort{name:"phase".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_phase".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("start_of_simulation_phase".to_owned(),false);
    m.ports.push(DefPort{name:"phase".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_phase".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("connect_phase".to_owned(),false);
    m.ports.push(DefPort{name:"phase".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_phase".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("end_of_elaboration_phase".to_owned(),false);
    m.ports.push(DefPort{name:"phase".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_phase".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("end_of_elaboration".to_owned(),false);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("check_phase".to_owned(),false);
    m.ports.push(DefPort{name:"phase".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_phase".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_top".to_owned()); // should be uvm root and uvm top a member of uvm_pkg
    mb = DefMember{ name: "enable_print_topology".to_owned(), kind: DefType::IntVector(TypeIntVector {name: "bit".to_owned(),packed: None, signed: false}), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("print_topology".to_owned(),false);
    m.ports.push(DefPort{name:"printer".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_printer".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_test".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_phase".to_owned());
    o.base = Some(TypeUser::new("uvm_object".to_owned()));
    m = DefMethod::new("get_objection".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_objection".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_objection_count".to_owned(),false);
    m.ports.push(DefPort{name:"obj".to_owned()        , dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ret = Some(TYPE_INT);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("is".to_owned(),false);
    m.ports.push(DefPort{name:"phase".to_owned() , dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_phase".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ret = Some(TYPE_INT);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("raise_objection".to_owned(),false);
    m.ports.push(DefPort{name:"obj".to_owned()        , dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"description".to_owned(), dir:PortDir::Input, kind: TYPE_STR, idx: 1, unpacked: Vec::new(), default: Some("".to_string())});
    m.ports.push(DefPort{name:"count".to_owned()      , dir:PortDir::Input, kind: TYPE_INT, idx: 2, unpacked: Vec::new(), default: Some("1".to_string())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("drop_objection".to_owned(),false);
    m.ports.push(DefPort{name:"obj".to_owned()        , dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"description".to_owned(), dir:PortDir::Input, kind: TYPE_STR, idx: 1, unpacked: Vec::new(), default: Some("".to_string())});
    m.ports.push(DefPort{name:"count".to_owned()      , dir:PortDir::Input, kind: TYPE_INT, idx: 2, unpacked: Vec::new(), default: Some("1".to_string())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_verbosity".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_driver".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    let mut cp = DefPort {name:"REQ".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type), default: Some("uvm_sequence_item".to_owned()), idx: 0, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    cp = DefPort {name:"RSP".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type),  default: Some("REQ".to_owned()), idx: 1, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    mb = DefMember{ name: "req".to_owned(), kind: DefType::User(TypeUser::new("REQ".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "rsp".to_owned(), kind: DefType::User(TypeUser::new("RSP".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "seq_item_port".to_owned(), kind: DefType::User(TypeUser::new("uvm_seq_item_pull_port".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_monitor".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequencer".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("env".to_string())});
    m.ports.push(DefPort{name:"parent".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequence".to_owned());
    o.base = Some(TypeUser::new("uvm_sequence_base".to_owned()));
    cp = DefPort {name:"REQ".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type),  default: Some("uvm_sequence_item".to_owned()), idx: 0, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    cp = DefPort {name:"RSP".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type),  default: Some("REQ".to_owned()), idx: 1, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("uvm_sequence".to_string())});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    mb = DefMember{ name: "req".to_owned(), kind: DefType::User(TypeUser::new("REQ".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "rsp".to_owned(), kind: DefType::User(TypeUser::new("RSP".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequence_base".to_owned());
    o.base = Some(TypeUser::new("uvm_sequence_item".to_owned()));
    cp = DefPort {name:"BASE".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type), default: Some("uvm_sequence".to_owned()), idx: 0, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    m = DefMethod::new("pre_start".to_owned(),false);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("start".to_owned(),false);
    m.ports.push(DefPort{name:"sequencer".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequencer_base".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent_sequence".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequence_base".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"this_priority".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 2, unpacked: Vec::new(), default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"call_pre_post".to_owned(), dir:PortDir::Input, kind:DefType::IntVector(TypeIntVector {name: "bit".to_owned(),packed: None, signed: false}), idx: 3, unpacked: Vec::new(), default: Some("1".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("start_item".to_owned(),false);
    m.ports.push(DefPort{name:"item".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequence_item".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"set_priority".to_owned()  , dir:PortDir::Input, kind:TYPE_INT, idx: 1, unpacked: Vec::new(), default: Some("-1".to_string())});
    m.ports.push(DefPort{name:"sequencer".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequencer_base".to_owned())), idx: 2, unpacked: Vec::new(), default: Some("null".to_string())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("finish_item".to_owned(),false);
    m.ports.push(DefPort{name:"item".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequence_item".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"set_priority".to_owned()  , dir:PortDir::Input, kind:TYPE_INT, idx: 1, unpacked: Vec::new(), default: Some("-1".to_string())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    mb = DefMember{ name: "do_not_randomize".to_owned(), kind: TYPE_INT, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequence_item".to_owned());
    // o.base = Some(TypeUser::new("uvm_transaction".to_owned()));
    o.base = Some(TypeUser::new("uvm_object".to_owned()));
    m = DefMethod::new("do_copy".to_owned(),false);
    m.ports.push(DefPort{name:"rhs".to_owned() , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("uvm_sequence_item".to_string())});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("set_sequence_id".to_owned(),false);
    m.ports.push(DefPort{name:"id".to_owned()  , dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    mb = DefMember{ name: "m_parent_sequence".to_owned(), kind: TYPE_STR, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "m_sequencer".to_owned(), kind: DefType::User(TypeUser::new("uvm_sequencer".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "p_sequencer".to_owned(), kind: DefType::User(TypeUser::new("uvm_sequencer".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("get_response".to_owned(),false);
    m.ports.push(DefPort{name:"response".to_owned()      , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"transaction_id".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 1, unpacked: Vec::new(), default: Some("-1".to_owned())});
    o.defs.insert("get_response".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("set_item_context".to_owned(),false);
    m.ports.push(DefPort{name:"parent_seq".to_owned()      , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequence_base".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"sequencer".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequencer_base".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_sequencer".to_owned(),false);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("m_set_p_sequencer".to_owned(),false);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set_sequencer".to_owned(),false);
    m.ports.push(DefPort{name:"sequencer".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequencer_base".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_sequence_id".to_owned(),false);
    m.ret = Some(TYPE_INT);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set_id_info".to_owned(),false);
    m.ports.push(DefPort{name:"item".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequence_item".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_agent".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    mb = DefMember{ name: "is_active".to_owned(), kind: DefType::User(TypeUser::new("uvm_active_passive_enum".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_env".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("env".to_string())});
    m.ports.push(DefPort{name:"parent".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_port_base".to_owned());
    m = DefMethod::new("get_next_item".to_owned(),true);
    m.ports.push(DefPort{ name:"t".to_owned(), dir:PortDir::Output, kind:DefType::User(TypeUser::new("REQ".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("item_done".to_owned(),false);
    m.ports.push(DefPort{ name:"item".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("RSP".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("null".to_string())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("connect".to_owned(),false);
    m.ports.push(DefPort{name:"provider".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("this_type".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("env".to_string())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("put".to_owned(),true);
    m.ports.push(DefPort{name:"t".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));

    o = DefClass::new("uvm_analysis_export".to_owned());
    o.base = Some(TypeUser::new("uvm_port_base".to_owned()));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_analysis_port".to_owned());
    o.base = Some(TypeUser::new("uvm_port_base".to_owned()));
    m = DefMethod::new("write".to_owned(),true);
    m.ports.push(DefPort{name:"t".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_comparer".to_owned());
    mb = DefMember{ name: "miscompares".to_owned(), kind: TYPE_STR, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "show_max".to_owned(), kind: TYPE_INT, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "result".to_owned(), kind: TYPE_INT, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_object_wrapper".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_objection".to_owned());
    m = DefMethod::new("clear".to_owned(),true);
    m.ports.push(DefPort{name:"obj".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("display_objections".to_owned(),true);
    m.ports.push(DefPort{name:"obj".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"show_header".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("bit".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("1".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequencer_base".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_status_e".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_tlm_analysis_fifo".to_owned());
    cp = DefPort {name:"T".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type),  default: Some("int".to_owned()), idx: 0, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    m = DefMethod::new("put".to_owned(),true);
    m.ports.push(DefPort{name:"t".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get".to_owned(),true);
    m.ports.push(DefPort{name:"t".to_owned()  , dir:PortDir::Output, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("peek".to_owned(),true);
    m.ports.push(DefPort{name:"t".to_owned()  , dir:PortDir::Output, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("used".to_owned(),true);
    m.ret = Some(TYPE_INT);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_active_passive_enum".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_coverage_model_e".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_default_comparer".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_event".to_owned());
    cp = DefPort {name:"T".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type), default: Some("uvm_object".to_owned()), idx: 0, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    m = DefMethod::new("trigger".to_owned(),true);
    m.ports.push(DefPort{name:"data".to_owned()   ,dir:PortDir::Input, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 0, unpacked:Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("wait_trigger_data".to_owned(),true);
    m.ports.push(DefPort{name:"data".to_owned()   ,dir:PortDir::Output, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 0, unpacked:Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_object_wrapper".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_report_server".to_owned());
    m = DefMethod::new("get_severity_count".to_owned(),false);
    m.ret = Some(TYPE_INT);
    m.ports.push(DefPort{name:"severity".to_owned()   ,dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_severity".to_owned())), idx: 0, unpacked:Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_printer".to_owned());
    mb = DefMember{ name: "knobs".to_owned(), kind: DefType::User(TypeUser::new("uvm_printer_knobs".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "m_string".to_owned(), kind: TYPE_STR, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_table_printer".to_owned());
    o.base = Some(TypeUser::new("uvm_printer".to_owned()));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    // UVM registers
    o = DefClass::new("uvm_reg".to_owned());
    o.base = Some(TypeUser::new("uvm_object".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()   ,dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked:Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"n_bits".to_owned()  ,dir:PortDir::Input, kind: TYPE_INT, idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"has_coverage".to_owned(),dir:PortDir::Input, kind: TYPE_INT, idx: 2, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("include_coverage".to_owned(),false);
    m.ports.push(DefPort{name:"scope".to_owned()   ,dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked:Vec::new(), default: None});
    m.ports.push(DefPort{name:"models".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_reg_cvr_t".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"accessor".to_owned(),dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_object".to_owned()))   , idx: 2, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("read".to_owned(),true);
    m.ports.push(DefPort{name:"status".to_owned()   , dir:PortDir::Output, kind: DefType::User(TypeUser::new("uvm_status_e".to_owned()))     , idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"value".to_owned()    , dir:PortDir::Output, kind: DefType::User(TypeUser::new("uvm_reg_data_t".to_owned()))   , idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"path".to_owned()     , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_path_e".to_owned()))       , idx: 2, unpacked: Vec::new(), default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(DefPort{name:"map".to_owned()      , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_reg_map".to_owned()))      , idx: 3, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"parent".to_owned()   , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_sequence_base".to_owned())), idx: 4, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"prior".to_owned()    , dir:PortDir::Input,  kind: TYPE_INT                                                    , idx: 5, unpacked: Vec::new(), default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"extension".to_owned(), dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_object".to_owned()))       , idx: 6, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"fname".to_owned()    , dir:PortDir::Input,  kind: TYPE_STR                                                    , idx: 7, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"lineno".to_owned()   , dir:PortDir::Input,  kind: TYPE_INT                                                    , idx: 8, unpacked: Vec::new(), default: Some("0".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("write".to_owned(),true);
    m.ports.push(DefPort{name:"status".to_owned()   , dir:PortDir::Output, kind: DefType::User(TypeUser::new("uvm_status_e".to_owned()))     , idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"value".to_owned()    , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_reg_data_t".to_owned()))   , idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"path".to_owned()     , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_path_e".to_owned()))       , idx: 2, unpacked: Vec::new(), default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(DefPort{name:"map".to_owned()      , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_reg_map".to_owned()))      , idx: 3, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"parent".to_owned()   , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_sequence_base".to_owned())), idx: 4, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"prior".to_owned()    , dir:PortDir::Input,  kind: TYPE_INT                                                    , idx: 5, unpacked: Vec::new(), default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"extension".to_owned(), dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_object".to_owned()))       , idx: 6, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"fname".to_owned()    , dir:PortDir::Input,  kind: TYPE_STR                                                    , idx: 7, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"lineno".to_owned()   , dir:PortDir::Input,  kind: TYPE_INT                                                    , idx: 8, unpacked: Vec::new(), default: Some("0".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("update".to_owned(),true);
    m.ports.push(DefPort{name:"status".to_owned()   , dir:PortDir::Output, kind: DefType::User(TypeUser::new("uvm_status_e".to_owned()))     , idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"path".to_owned()     , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_path_e".to_owned()))       , idx: 1, unpacked: Vec::new(), default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(DefPort{name:"map".to_owned()      , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_reg_map".to_owned()))      , idx: 2, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"parent".to_owned()   , dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_sequence_base".to_owned())), idx: 3, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"prior".to_owned()    , dir:PortDir::Input,  kind: TYPE_INT                                                    , idx: 4, unpacked: Vec::new(), default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"extension".to_owned(), dir:PortDir::Input,  kind: DefType::User(TypeUser::new("uvm_object".to_owned()))       , idx: 5, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"fname".to_owned()    , dir:PortDir::Input,  kind: TYPE_STR                                                    , idx: 6, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"lineno".to_owned()   , dir:PortDir::Input,  kind: TYPE_INT                                                    , idx: 7, unpacked: Vec::new(), default: Some("0".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("reset".to_owned(),false);
    m.ports.push(DefPort{name:"kind".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("HARD".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_reset".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_reg_data_t".to_owned())));
    m.ports.push(DefPort{name:"kind".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("HARD".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_fields".to_owned(),false);
    m.ports.push(DefPort{name:"fields".to_owned()     , dir:PortDir::Ref,   kind:DefType::User(TypeUser::new("uvm_reg_field".to_owned())), idx: 0, unpacked: vec![SvArrayKind::Queue], default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_field_by_name".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_reg_field".to_owned())));
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_n_bits".to_owned(),false);
    m.ret = Some(TYPE_UINT);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_block".to_owned());
    o.base = Some(TypeUser::new("uvm_object".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"has_coverage".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 1, unpacked: Vec::new(), default: Some("UVM_NO_COVERAGE".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("lock_model".to_owned(),false);
    o.defs.insert("lock_model".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("reset".to_owned(),false);
    m.ports.push(DefPort{name:"kind".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("HARD".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("configure".to_owned(),false);
    m.ports.push(DefPort{name:"parent".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_reg_block".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"hdl_path".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("find_blocks".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()     , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"blks".to_owned()     , dir:PortDir::Ref,   kind:DefType::User(TypeUser::new("uvm_reg_block".to_owned())), idx: 1, unpacked: vec![SvArrayKind::Queue], default: None});
    m.ports.push(DefPort{name:"root".to_owned()     , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_reg_block".to_owned())), idx: 2, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"accessor".to_owned() , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned()))   , idx: 3, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_maps".to_owned(),false);
    m.ports.push(DefPort{name:"maps".to_owned()     , dir:PortDir::Ref,   kind:DefType::User(TypeUser::new("uvm_reg_map".to_owned())), idx: 0, unpacked: vec![SvArrayKind::Queue], default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_registers".to_owned(),false);
    m.ports.push(DefPort{name:"regs".to_owned()     , dir:PortDir::Ref,   kind:DefType::User(TypeUser::new("uvm_reg".to_owned())), idx: 0, unpacked: vec![SvArrayKind::Queue], default: None});
    m.ports.push(DefPort{name:"hier".to_owned()     , dir:PortDir::Ref,   kind:DefType::User(TypeUser::new("uvm_hier_e".to_owned())), idx: 1, unpacked: vec![], default: Some("UVM_HIER".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    mb = DefMember{ name: "default_map".to_owned(), kind: DefType::User(TypeUser::new("uvm_reg_map".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("get_reg_by_name".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_reg".to_owned())));
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_map_by_name".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_reg_map".to_owned())));
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_field_by_name".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_reg_field".to_owned())));
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_parent".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_reg_block".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_predictor".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert("new".to_owned(),ObjDef::Method(m));
    mb = DefMember{ name: "reg_ap".to_owned(), kind: DefType::User(TypeUser::new("uvm_analysis_port".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "map".to_owned(), kind: DefType::User(TypeUser::new("uvm_reg_map".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "adapter".to_owned(), kind: DefType::User(TypeUser::new("uvm_reg_adapter".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_sequence".to_owned());
    o.base = Some(TypeUser::new("BASE".to_owned()));
    cp = DefPort {name:"BASE".to_owned(), dir: PortDir::Param,kind: DefType::Primary(TypePrimary::Type), default: Some("uvm_sequence".to_owned()), idx: 0, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    m = DefMethod::new("write_reg".to_owned(),false);
    m.ports.push(DefPort{name:"rg".to_owned()       , dir:PortDir::Input , kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"status".to_owned()   , dir:PortDir::Output, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"value".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 2, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"path".to_owned()     , dir:PortDir::Input , kind:TYPE_STR, idx: 3, unpacked: Vec::new(), default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(DefPort{name:"map".to_owned()      , dir:PortDir::Input , kind:TYPE_STR, idx: 4, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"prior".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 5, unpacked: Vec::new(), default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"extension".to_owned(), dir:PortDir::Input , kind:TYPE_STR, idx: 6, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"fname".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 7, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"lineno".to_owned()   , dir:PortDir::Input , kind:TYPE_STR, idx: 8, unpacked: Vec::new(), default: Some("0".to_owned())});
    o.defs.insert("write_reg".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("read_reg".to_owned(),false);
    m.ports.push(DefPort{name:"rg".to_owned()       , dir:PortDir::Input , kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"status".to_owned()   , dir:PortDir::Output, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"value".to_owned()    , dir:PortDir::Output, kind:TYPE_STR, idx: 2, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"path".to_owned()     , dir:PortDir::Input , kind:TYPE_STR, idx: 3, unpacked: Vec::new(), default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(DefPort{name:"map".to_owned()      , dir:PortDir::Input , kind:TYPE_STR, idx: 4, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"prior".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 5, unpacked: Vec::new(), default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"extension".to_owned(), dir:PortDir::Input , kind:TYPE_STR, idx: 6, unpacked: Vec::new(), default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"fname".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 7, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"lineno".to_owned()   , dir:PortDir::Input , kind:TYPE_STR, idx: 8, unpacked: Vec::new(), default: Some("0".to_owned())});
    o.defs.insert("read_reg".to_owned(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    let mut s = TypeStruct{is_packed: false, members: Vec::new()};
    s.members.push(ObjDef::Member(DefMember{ name: "kind".to_owned(), kind: DefType::User(TypeUser::new("uvm_access_e".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public}));
    s.members.push(ObjDef::Member(DefMember{ name: "addr".to_owned(), kind: DefType::User(TypeUser::new("uvm_reg_addr_t".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public}));
    s.members.push(ObjDef::Member(DefMember{ name: "data".to_owned(), kind: DefType::User(TypeUser::new("uvm_reg_data_t".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public}));
    s.members.push(ObjDef::Member(DefMember{ name: "n_bits".to_owned(), kind: TYPE_INT, unpacked : Vec::new(), is_const: false, access: Access::Public}));
    s.members.push(ObjDef::Member(DefMember{ name: "byte_en".to_owned(), kind: DefType::User(TypeUser::new("uvm_reg_byte_en_t".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public}));
    s.members.push(ObjDef::Member(DefMember{ name: "status".to_owned(), kind: DefType::User(TypeUser::new("uvm_status_e".to_owned())), unpacked : Vec::new(), is_const: false, access: Access::Public}));
    p.defs.insert("uvm_reg_bus_op".to_owned(),ObjDef::Type(DefType::Struct(s),Vec::new()));
    //
    o = DefClass::new("uvm_reg_data_t".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_field".to_owned());
    o.base = Some(TypeUser::new("uvm_object".to_owned()));
    m = DefMethod::new("get_lsb_pos".to_owned(),false);
    m.ret = Some(TYPE_UINT);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_n_bits".to_owned(),false);
    m.ret = Some(TYPE_UINT);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("has_reset".to_owned(),false);
    m.ret = Some(TYPE_UINT);
    m.ports.push(DefPort{name:"kind".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("HARD".to_owned())});
    m.ports.push(DefPort{name:"delete".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: Some("0".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("get_access".to_owned(),false);
    m.ret = Some(TYPE_STR);
    m.ports.push(DefPort{name:"map".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_reg_map".to_owned())), idx: 0, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_map".to_owned());
    o.base = Some(TypeUser::new("uvm_object".to_owned()));
    m = DefMethod::new("set_auto_predict".to_owned(),false);
    m.ports.push(DefPort{name:"on".to_owned()  , dir:PortDir::Input, kind:TYPE_UINT, idx: 0, unpacked: Vec::new(), default: Some("1".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set_sequencer".to_owned(),false);
    m.ports.push(DefPort{name:"sequencer".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_sequencer_base".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"adapter".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_reg_adapter".to_owned())), idx: 1, unpacked: Vec::new(), default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_adapter".to_owned());
    o.base = Some(TypeUser::new("uvm_object".to_owned()));
    m = DefMethod::new("new".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: Some("".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    mb = DefMember{ name: "supports_byte_enable".to_owned(), kind: TYPE_INT, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "provides_responses".to_owned(), kind: TYPE_INT, unpacked : Vec::new(), is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));

    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_addr_t".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_item".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_factory".to_owned());
    m = DefMethod::new("get".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_factory".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("create_component_by_name".to_owned(),false);
    m.ret = Some(DefType::User(TypeUser::new("uvm_component".to_owned())));
    m.ports.push(DefPort{name:"requested_type_name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent_inst_path".to_owned()   , dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"name".to_owned()               , dir:PortDir::Input, kind:TYPE_STR, idx: 2, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent".to_owned()             , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_component".to_owned())), idx: 3, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("create_object_by_name".to_owned(),false);
    m.ports.push(DefPort{name:"requested_type_name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"parent_inst_path".to_owned()   , dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ports.push(DefPort{name:"name".to_owned()               , dir:PortDir::Input, kind:TYPE_STR, idx: 2, unpacked: Vec::new(), default: Some("".to_owned())});
    m.ret = Some(DefType::User(TypeUser::new("uvm_object".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("print".to_owned(),false);
    m.ports.push(DefPort{name:"all_types".to_owned()  , dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: Some("1".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set_inst_override_by_name".to_owned(),false);
    m.ports.push(DefPort{name:"original_type_name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"override_type_name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"full_inst_path".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 2, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));

    // Config Database
    o = DefClass::new("uvm_config_db".to_owned());
    m = DefMethod::new("get".to_owned(),false);
    m.ports.push(DefPort{
        name:"cntxt".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("uvm_component".to_owned()),
        idx: 0,unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{
        name:"inst_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 1,unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{
        name:"field_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 2,unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{
        name:"value".to_owned(),
        dir:PortDir::Inout,
        kind:DefType::None,//new("T".to_owned()),
        idx: 3,unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set".to_owned(),false);
    m.ports.push(DefPort{
        name:"cntxt".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("uvm_component".to_owned()),
        idx: 0,unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{
        name:"inst_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 1,unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{
        name:"field_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 2,unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{
        name:"value".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("T".to_owned()),
        idx: 3,unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));

    // Ports
    o = DefClass::new("uvm_seq_item_pull_port".to_owned());
    o.base = Some(TypeUser::new("uvm_port_base".to_owned()));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));

    //
    o = DefClass::new("uvm_status_container".to_owned());
    m = DefMethod::new("m_do_cycle_check".to_owned(),false);
    m.ports.push(DefPort{name:"scope".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("do_field_check".to_owned(),false);
    m.ports.push(DefPort{name:"field".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"obj".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("uvm_object".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    mb = DefMember{ name: "m_uvm_cycle_scopes".to_owned(), kind: DefType::User(TypeUser::new("uvm_object".to_owned())), unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "scope".to_owned(), kind: DefType::User(TypeUser::new("uvm_scope_stack".to_owned())), unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "stringv".to_owned(), kind: TYPE_STR, unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "comparer".to_owned(), kind: DefType::User(TypeUser::new("uvm_comparer".to_owned())), unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "packer".to_owned(), kind: DefType::User(TypeUser::new("uvm_packer".to_owned())), unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "recorder".to_owned(), kind: DefType::User(TypeUser::new("uvm_recorder".to_owned())), unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "printer".to_owned(), kind: DefType::User(TypeUser::new("uvm_printer".to_owned())), unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "bitstream".to_owned(), kind: DefType::User(TypeUser::new("uvm_bitstream_t".to_owned())), unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "print_matches".to_owned(), kind: TYPE_INT, unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "status".to_owned(), kind: TYPE_INT, unpacked : vec![SvArrayKind::Queue], is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_enum_wrapper".to_owned());
    cp = DefPort {name:"T".to_owned(), dir:PortDir::Input, kind: DefType::Primary(TypePrimary::Type),  default: Some("uvm_active_passive_enum".to_owned()), idx: 0, unpacked: Vec::new()};
    o.params.insert(cp.name.clone(),ObjDef::Port(cp));
    m = DefMethod::new("from_name".to_owned(),false);
    m.ports.push(DefPort{name:"name".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
    m.ports.push(DefPort{name:"value".to_owned()  , dir:PortDir::Input, kind:DefType::User(TypeUser::new("T".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));

    //
    o = DefClass::new("uvm_coreservice_t".to_owned());
    m = DefMethod::new("get".to_owned(),true);
    m.ret = Some(DefType::User(TypeUser::new("uvm_coreservice_t".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    o = DefClass::new("uvm_root".to_owned());
    m = DefMethod::new("get".to_owned(),true);
    m.ret = Some(DefType::User(TypeUser::new("uvm_root".to_owned())));
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));

    ObjDef::Package(p)
}
