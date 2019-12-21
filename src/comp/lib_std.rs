// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::comp::prototype::*;
use crate::comp::comp_obj::{ObjDef};
use crate::comp::comp_lib::{CompLib};
// use crate::comp::def_type::{DefType,TypePrimary,TypeUser};

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
        self.objects.insert(o.name.clone(),ObjDef::Class(o));

        //
    }
}
