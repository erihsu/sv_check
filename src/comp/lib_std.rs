// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::comp::prototype::*;
use crate::comp::comp_obj::{ObjDef};
use crate::comp::comp_lib::{CompLib};
use crate::comp::def_type::{DefType,TypeIntVector, TypePrimary, TypeUser, TYPE_INT, TYPE_UINT, TYPE_BYTE, TYPE_STR};

impl CompLib {

    pub fn add_std_obj(&mut self) {
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
        let mut o = DefClass::new("process".to_owned());
        o.defs.insert("FINISHED".to_owned() , ObjDef::EnumValue("0".to_owned()));
        o.defs.insert("RUNNING".to_owned()  , ObjDef::EnumValue("1".to_owned()));
        o.defs.insert("WAITING".to_owned()  , ObjDef::EnumValue("2".to_owned()));
        o.defs.insert("SUSPENDED".to_owned(), ObjDef::EnumValue("3".to_owned()));
        o.defs.insert("KILLED".to_owned()   , ObjDef::EnumValue("4".to_owned()));
        o.defs.insert("kill".to_owned()     , ObjDef::Method(DefMethod::new("kill".to_owned()   ,false)));
        o.defs.insert("suspend".to_owned()  , ObjDef::Method(DefMethod::new("suspend".to_owned(),false)));
        o.defs.insert("resume".to_owned()   , ObjDef::Method(DefMethod::new("resume".to_owned() ,false)));
        o.defs.insert("await".to_owned()    , ObjDef::Method(DefMethod::new("resume".to_owned() ,true )));
        let mut m = DefMethod::new("srandom".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        m.ports.push(DefPort{name:"seed".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("status".to_owned(),false);
        m.ret = Some(DefType::User(TypeUser::new("state".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("self".to_owned(),false);
        m.ret = Some(DefType::User(TypeUser::new("process".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
        o = DefClass::new("string".to_owned());
        m = DefMethod::new("len".to_owned(),false); m.ret = Some(TYPE_INT); o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("putc".to_owned(),false);
        m.ports.push(DefPort{name:"i".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        m.ports.push(DefPort{name:"c".to_owned(), dir:PortDir::Input, kind:TYPE_BYTE, idx: 1, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("getc".to_owned(),false);
        m.ports.push(DefPort{name:"i".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        m.ret = Some(TYPE_BYTE);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("substr".to_owned(),false);
        m.ports.push(DefPort{name:"i".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        m.ports.push(DefPort{name:"j".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 1, unpacked: Vec::new(), default: None});
        m.ret = Some(TYPE_STR);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        o.defs.insert("tolower".to_owned(), ObjDef::Method(DefMethod::new("tolower".to_owned(),false)));
        o.defs.insert("toupper".to_owned(), ObjDef::Method(DefMethod::new("toupper".to_owned(),false)));
        // m = DefMethod::new("icompare".to_owned(),false);
        m = DefMethod::new("atoi".to_owned()  ,false); m.ret = Some(TYPE_INT); o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("atohex".to_owned(),false); m.ret = Some(TYPE_INT); o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("atooct".to_owned(),false); m.ret = Some(TYPE_INT); o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("atobin".to_owned(),false); m.ret = Some(TYPE_INT); o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("atoreal".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Real));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("itoa".to_owned()  ,false);
        m.ports.push(DefPort{name:"i".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("hextoa".to_owned(),false);
        m.ports.push(DefPort{name:"i".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("octtoa".to_owned(),false);
        m.ports.push(DefPort{name:"i".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("bintoa".to_owned(),false);
        m.ports.push(DefPort{name:"i".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("realtoa".to_owned(),false);
        m.ports.push(DefPort{name:"r".to_owned(), dir:PortDir::Input, kind:DefType::Primary(TypePrimary::Real), idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
        o = DefClass::new("enum".to_owned());
        m = DefMethod::new("first".to_owned(),false);
        m.ret = Some(DefType::User(TypeUser::new("enum".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("last".to_owned(),false);
        m.ret = Some(DefType::User(TypeUser::new("enum".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("next".to_owned(),false);
        m.ports.push(DefPort{name:"N".to_owned(), dir:PortDir::Input, kind:TYPE_UINT, idx: 0, unpacked: Vec::new(), default: Some("1".to_string())});
        m.ret = Some(DefType::User(TypeUser::new("enum".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("prev".to_owned(),false);
        m.ports.push(DefPort{name:"N".to_owned(), dir:PortDir::Input, kind:TYPE_UINT, idx: 0, unpacked: Vec::new(), default: Some("1".to_string())});
        m.ret = Some(DefType::User(TypeUser::new("enum".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("num".to_owned(),false);
        m.ret = Some(TYPE_INT);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("name".to_owned(),false);
        m.ret = Some(TYPE_STR);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));


        //
        o = DefClass::new("event".to_owned());
        m = DefMethod::new("triggered".to_owned(),false);
        m.ret = Some(DefType::IntVector(TypeIntVector {name   : "bit".to_owned(), packed : None,signed : false}));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
        o = DefClass::new("class".to_owned());
        m = DefMethod::new("srandom".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        m.ports.push(DefPort{name:"seed".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
        o = DefClass::new("!array!dyn".to_owned());
        m = DefMethod::new("size".to_owned(),false);
        m.ret = Some(TYPE_INT);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("delete".to_owned(),false);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
        o = DefClass::new("!array!dict".to_owned());
        m = DefMethod::new("num".to_owned(),false);
        m.ret = Some(TYPE_INT);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("size".to_owned(),false);
        m.ret = Some(TYPE_INT);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("delete".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: Some("-1".to_string())});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("exists".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("first".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Ref, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("last".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Ref, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("next".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Ref, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("prev".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Ref, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
        o = DefClass::new("!array!queue".to_owned());
        m = DefMethod::new("size".to_owned(),false);
        m.ret = Some(TYPE_INT);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("insert".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: None});
        m.ports.push(DefPort{name:"item".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("!element".to_owned())), idx: 1, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("delete".to_owned(),false);
        m.ports.push(DefPort{name:"index".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: Some("-1".to_string())});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("pop_front".to_owned(),false);
        m.ret = Some(DefType::User(TypeUser::new("!element".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("pop_back".to_owned(),false);
        m.ret = Some(DefType::User(TypeUser::new("!element".to_owned())));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("push_front".to_owned(),false);
        m.ports.push(DefPort{name:"item".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("!element".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("push_back".to_owned(),false);
        m.ports.push(DefPort{name:"item".to_owned(), dir:PortDir::Input, kind:DefType::User(TypeUser::new("!element".to_owned())), idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        // Generic array reduction method
        o = DefClass::new("!array".to_owned());
        m = DefMethod::new("size".to_owned(),false);
        m.ret = Some(TYPE_INT);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("sum".to_owned(),false);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("product".to_owned(),false);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("and".to_owned(),false);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("or".to_owned(),false);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("xor".to_owned(),false);
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
        o = DefClass::new("covergroup".to_owned());
        m = DefMethod::new("sample".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        // Temporary port definition with default value to support basic overload
        m.ports.push(DefPort{name:"_".to_owned(), dir:PortDir::Input, kind:TYPE_INT, idx: 0, unpacked: Vec::new(), default: Some("0".to_string())});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("start".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("stop".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("get_coverage".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("get_inst_coverage".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        m = DefMethod::new("set_inst_name".to_owned(),false);
        m.ret = Some(DefType::Primary(TypePrimary::Void));
        m.ports.push(DefPort{name:"name".to_owned(), dir:PortDir::Input, kind:TYPE_STR, idx: 0, unpacked: Vec::new(), default: None});
        o.defs.insert(m.name.clone(),ObjDef::Method(m));
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

    }
}
