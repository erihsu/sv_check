// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::comp::prototype::*;
use crate::comp::comp_obj::{ObjDef};
use crate::comp::def_type::{DefType,TypeUser, TYPE_INT, TYPE_STR};

pub fn get_uvm_lib() -> ObjDef {
    let mut p = DefPackage::new("uvm_pkg".to_owned());
    // Top level methods
    let mut m = DefMethod::new("run_test".to_owned(),false);
    m.ports.push(DefPort{
        name:"test_name".to_owned(),
        dir:PortDir::Input,
        kind:TYPE_STR,
        idx: 0,unpacked: None, default: None});
    p.defs.insert("run_test".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("uvm_report_fatal".to_owned(),false);
    m.ports.push(DefPort{name:"id".to_owned()       , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: None, default: None});
    m.ports.push(DefPort{name:"message".to_owned()  , dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: None, default: None});
    m.ports.push(DefPort{name:"verbosity".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 2, unpacked: None, default: Some("UVM_NONE".to_owned())});
    m.ports.push(DefPort{name:"file".to_owned()     , dir:PortDir::Input, kind:TYPE_STR, idx: 3, unpacked: None, default: Some("".to_owned())});
    m.ports.push(DefPort{name:"line".to_owned()     , dir:PortDir::Input, kind:TYPE_STR, idx: 4, unpacked: None, default: Some("".to_owned())});
    p.defs.insert("uvm_report_fatal".to_owned(),ObjDef::Method(m.clone()));
    m.name = "uvm_report_error".to_owned();
    p.defs.insert(m.name.clone(),ObjDef::Method(m.clone()));
    m.name = "uvm_report_warning".to_owned();
    p.defs.insert(m.name.clone(),ObjDef::Method(m.clone()));
    m.name = "uvm_report_info".to_owned();
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
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_component".to_owned());
    let mut mb = DefMember{ name: "m_name".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "type_name".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "m_current_phase".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("set_inst_override".to_owned(),false);
    m.ports.push(DefPort{name:"relative_inst_path".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: None, default: None});
    m.ports.push(DefPort{name:"original_type_name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 1, unpacked: None, default: None});
    m.ports.push(DefPort{name:"override_type_name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 2, unpacked: None, default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_top".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_test".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_phase".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_verbosity".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_driver".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    mb = DefMember{ name: "req".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "rsp".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "seq_item_port".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_monitor".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequencer".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequence".to_owned());
    o.base = Some(TypeUser::new("uvm_sequence_item".to_owned()));
    mb = DefMember{ name: "req".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "rsp".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequence_item".to_owned());
    mb = DefMember{ name: "m_parent_sequence".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "m_sequencer".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    mb = DefMember{ name: "p_sequencer".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("get_response".to_owned(),false);
    m.ports.push(DefPort{name:"response".to_owned()      , dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: None, default: None});
    m.ports.push(DefPort{name:"transaction_id".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 1, unpacked: None, default: Some("-1".to_owned())});
    o.defs.insert("get_response".to_owned(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_agent".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    mb = DefMember{ name: "is_active".to_owned(), kind: DefType::User(TypeUser::new("uvm_active_passive_enum".to_owned())), unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_env".to_owned());
    o.base = Some(TypeUser::new("uvm_component".to_owned()));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_analysis_export".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_analysis_port".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_comparer".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_object_wrapper".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_objection".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_sequencer_base".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_status_e".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_tlm_analysis_fifo".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_active_passive_enum".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_analysis_port".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_coverage_model_e".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_default_comparer".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_event".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_object_wrapper".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_objection".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_report_server".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_printer".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_table_printer".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    // UVM registers
    o = DefClass::new("uvm_reg".to_owned());
    m = DefMethod::new("include_coverage".to_owned(),false);
    m.ports.push(DefPort{name:"scope".to_owned()   ,dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked:None, default: None});
    m.ports.push(DefPort{name:"models".to_owned()  ,dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_reg_cvr_t".to_owned())), idx: 1, unpacked: None, default: None});
    m.ports.push(DefPort{name:"accessor".to_owned(),dir:PortDir::Input, kind: DefType::User(TypeUser::new("uvm_object".to_owned()))   , idx: 2, unpacked: None, default: Some("null".to_owned())});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_block".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_predictor".to_owned());
    mb = DefMember{ name: "reg_ap".to_owned(), kind: DefType::User(TypeUser::new("uvm_analysis_port".to_owned())), unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_sequence".to_owned());
    o.base = Some(TypeUser::new("uvm_sequence".to_owned()));
    // m_verbosity: Not true, but just to avoid error until we know how to follow properly the inheritance tree
    mb = DefMember{ name: "m_verbosity".to_owned(), kind: TYPE_STR, unpacked : None, is_const: false, access: Access::Public};
    o.defs.insert(mb.name.clone(),ObjDef::Member(mb));
    m = DefMethod::new("write_reg".to_owned(),false);
    m.ports.push(DefPort{name:"rg".to_owned()       , dir:PortDir::Input , kind:TYPE_STR, idx: 0, unpacked: None, default: None});
    m.ports.push(DefPort{name:"status".to_owned()   , dir:PortDir::Output, kind:TYPE_STR, idx: 1, unpacked: None, default: None});
    m.ports.push(DefPort{name:"value".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 2, unpacked: None, default: None});
    m.ports.push(DefPort{name:"path".to_owned()     , dir:PortDir::Input , kind:TYPE_STR, idx: 3, unpacked: None, default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(DefPort{name:"map".to_owned()      , dir:PortDir::Input , kind:TYPE_STR, idx: 4, unpacked: None, default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"prior".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 5, unpacked: None, default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"extension".to_owned(), dir:PortDir::Input , kind:TYPE_STR, idx: 6, unpacked: None, default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"fname".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 7, unpacked: None, default: Some("".to_owned())});
    m.ports.push(DefPort{name:"lineno".to_owned()   , dir:PortDir::Input , kind:TYPE_STR, idx: 8, unpacked: None, default: Some("0".to_owned())});
    o.defs.insert("write_reg".to_owned(),ObjDef::Method(m));
    m = DefMethod::new("read_reg".to_owned(),false);
    m.ports.push(DefPort{name:"rg".to_owned()       , dir:PortDir::Input , kind:TYPE_STR, idx: 0, unpacked: None, default: None});
    m.ports.push(DefPort{name:"status".to_owned()   , dir:PortDir::Output, kind:TYPE_STR, idx: 1, unpacked: None, default: None});
    m.ports.push(DefPort{name:"value".to_owned()    , dir:PortDir::Output, kind:TYPE_STR, idx: 2, unpacked: None, default: None});
    m.ports.push(DefPort{name:"path".to_owned()     , dir:PortDir::Input , kind:TYPE_STR, idx: 3, unpacked: None, default: Some("UVM_DEFAULT_PATH".to_owned())});
    m.ports.push(DefPort{name:"map".to_owned()      , dir:PortDir::Input , kind:TYPE_STR, idx: 4, unpacked: None, default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"prior".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 5, unpacked: None, default: Some("-1".to_owned())});
    m.ports.push(DefPort{name:"extension".to_owned(), dir:PortDir::Input , kind:TYPE_STR, idx: 6, unpacked: None, default: Some("null".to_owned())});
    m.ports.push(DefPort{name:"fname".to_owned()    , dir:PortDir::Input , kind:TYPE_STR, idx: 7, unpacked: None, default: Some("".to_owned())});
    m.ports.push(DefPort{name:"lineno".to_owned()   , dir:PortDir::Input , kind:TYPE_STR, idx: 8, unpacked: None, default: Some("0".to_owned())});
    o.defs.insert("read_reg".to_owned(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_bus_op".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_data_t".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_field".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_map".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_adapter".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_addr_t".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_bus_op".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_data_t".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_item".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_factory".to_owned());
    m = DefMethod::new("get".to_owned(),false);
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    //
    o = DefClass::new("uvm_reg_map".to_owned());
    p.defs.insert(o.name.clone(),ObjDef::Class(o));
    // Config Database
    o = DefClass::new("uvm_config_db".to_owned());
    m = DefMethod::new("get".to_owned(),false);
    m.ports.push(DefPort{
        name:"cntxt".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("uvm_component".to_owned()),
        idx: 0,unpacked: None, default: None});
    m.ports.push(DefPort{
        name:"inst_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 1,unpacked: None, default: None});
    m.ports.push(DefPort{
        name:"field_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 2,unpacked: None, default: None});
    m.ports.push(DefPort{
        name:"value".to_owned(),
        dir:PortDir::Inout,
        kind:DefType::None,//new("T".to_owned()),
        idx: 3,unpacked: None, default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    m = DefMethod::new("set".to_owned(),false);
    m.ports.push(DefPort{
        name:"cntxt".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("uvm_component".to_owned()),
        idx: 0,unpacked: None, default: None});
    m.ports.push(DefPort{
        name:"inst_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 1,unpacked: None, default: None});
    m.ports.push(DefPort{
        name:"field_name".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("string".to_owned()),
        idx: 2,unpacked: None, default: None});
    m.ports.push(DefPort{
        name:"value".to_owned(),
        dir:PortDir::Input,
        kind:DefType::None,//new("T".to_owned()),
        idx: 3,unpacked: None, default: None});
    o.defs.insert(m.name.clone(),ObjDef::Method(m));
    p.defs.insert(o.name.clone(),ObjDef::Class(o));

    ObjDef::Package(p)
}
