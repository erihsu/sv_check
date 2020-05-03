// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use std::path::PathBuf;
use std::collections::HashMap;

use crate::ast::astnode::{AstNode, AstNodeKind};
use crate::lex::{source::path_display};
// use crate::lex::{token::Token, position::Position};
use crate::error::SvError;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum MsgID {
    ErrFile           , // File not found
    ErrToken          , // Illegal Token
    ErrSyntax         , // Illegal syntax
    ErrNotFound       , // Identifier undeclared
    ErrImplicit       , // Implicit connect undeclared
    ErrArgMiss        , // Port/Argument missing in instance/method
    ErrArgExtra       , // Too many argument in instance/method
    WarnUnused        , // Unused token
    InfoStatus        , // Compile/Link status
    DbgSkip           , // Skipping analysis of some AstNode
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Severity {Debug, Info, Warning, Error}


#[derive(Debug, Clone)]
pub struct Reporter {
    /// Path of the logfile
    logfile: Option<PathBuf>,
    /// Severity level for stdout
    stdout_level: Severity,
    /// Severity level for stdout
    id_level: HashMap<MsgID,Severity>,
    /// Full path of the file being parsed/compile
    pub filename: PathBuf,
    /// List of previous message in current file to avoid spamming same issue multiple
    prev_msg : HashMap<MsgID,Vec<String>>,
}

#[allow(dead_code)]
impl Reporter {

    pub fn new(logfile: Option<PathBuf>, level: Severity) -> Reporter {
        let mut id_level = HashMap::new();
        id_level.insert(MsgID::ErrFile      , Severity::Error);
        id_level.insert(MsgID::ErrToken     , Severity::Error);
        id_level.insert(MsgID::ErrSyntax    , Severity::Error);
        id_level.insert(MsgID::ErrNotFound  , Severity::Error);
        id_level.insert(MsgID::ErrImplicit  , Severity::Error);
        id_level.insert(MsgID::ErrArgMiss   , Severity::Error);
        id_level.insert(MsgID::WarnUnused   , Severity::Warning);
        id_level.insert(MsgID::InfoStatus   , Severity::Info);
        id_level.insert(MsgID::DbgSkip      , Severity::Debug);
        Reporter {
            logfile, stdout_level: level, id_level,
            filename: PathBuf::new(),
            prev_msg: HashMap::new()}
    }

    // Set the filename begin analyzed
    pub fn set_filename(&mut self, name: &PathBuf) {
        self.filename = name.clone();
        self.prev_msg.clear();
    }

    pub fn get_severity_str(&self, id: &MsgID) -> String {
        match self.id_level.get(id) {
            Some(Severity::Debug)   => "[DEBUG]  ".to_string(),
            Some(Severity::Info)    => "[INFO]   ".to_string(),
            Some(Severity::Warning) => "[WARNING]".to_string(),
            _                       => "[ERROR]  ".to_string(),
        }
    }

    pub fn msg(&self, id: MsgID, node: &AstNode, cntxt: &str) {
        let str_sev = self.get_severity_str(&id);
        let str_fn = path_display(&self.filename);
        let str_body =
            match id {
                MsgID::ErrToken      => format!("Unable to parse token {}.", cntxt),
                MsgID::ErrSyntax     => format!("Unexpected {} in {}.", node.kind, cntxt),
                MsgID::ErrNotFound   => format!("{} {} not found ", node.kind, cntxt),
                MsgID::ErrArgMiss    => format!("Missing port in instance of {} : {}", node.attr["type"], cntxt),
                MsgID::ErrImplicit   => format!("Implicit connection to port {} of {} not found.", cntxt, node.attr["type"]),
                MsgID::ErrArgExtra   => {
                    match node.kind {
                        AstNodeKind::Instance => format!("Too many ports in instance of {} : expecting {}", node.attr["type"], cntxt),
                        AstNodeKind::MacroCall |
                        AstNodeKind::MethodCall => format!("Too many arguments in call to {} : expecting {}", node.attr["name"], cntxt),
                        _ => format!("Too many arguments : expecting {}", cntxt)
                    }
                }
                MsgID::WarnUnused    => format!("Unused {}", "".to_string()),
                MsgID::DbgSkip       => format!("Skipping {} : {}", cntxt, node),
                _ => cntxt.to_string(),
            };
        // print the message to a file and/or stdout (TODO)
        println!("{} {}:{} | {}", str_sev, str_fn ,node.pos, str_body);
    }

    // Message from error
    pub fn msg_e(&self, error : SvError) {
        let str_sev  = self.get_severity_str(&MsgID::ErrSyntax);
        let str_fn   = path_display(&self.filename);
        println!("{} {}{}", str_sev, str_fn ,error);
    }

    // Basic message (no mode/token)
    pub fn msg_s(&self, id: MsgID, cntxt: &str) {
        let str_sev = self.get_severity_str(&id);
        let str_body =
            match id {
                MsgID::ErrFile     => format!("File {} not found.", cntxt),
                MsgID::InfoStatus  => format!("{} {:?}", cntxt, path_display(&self.filename)),
                _ => cntxt.to_string(),
            };
        // print the message to a file and/or stdout (TODO)
        println!("{} {}", str_sev, str_body);
    }
}