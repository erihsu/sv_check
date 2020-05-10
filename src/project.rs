// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::error::{SvError,SvErrorKind};
use crate::ast::{Ast,Defines,uvm_macro};

use std::{
    path::PathBuf,
    collections::{HashSet,HashMap},
    io::{BufReader, BufRead},
    fs::{File,metadata},
};

use crate::lex::{
    source::{Source,path_display},
    token::Token,
    // position::Position,
    token_stream::TokenStream};

use crate::comp::comp_lib::CompLib;

use crate::reporter::{REPORTER/*, Severity*/, MsgID};

pub struct Project {
    pub filelist : HashSet<PathBuf>,
    pub incdir : HashSet<PathBuf>,
    pub defines : Defines,
    pub ast_list : Vec<Ast>,
    pub ast_inc : HashMap<String,Box<Ast>>,
    pub cur_dir : PathBuf,
}

impl Project {

    // Create a project based on a vector of file/dir
    pub fn from_list(list: Vec<PathBuf>, incs: Vec<PathBuf>) -> Result<Project,std::io::Error> {
        // Create the include dir list
        let mut incdir = HashSet::new();
        for d in incs {
            incdir.insert(d);
        }
        // println!("[Project] Include Dir = {:?}", incdir);
        // Create the file list: if the input vec contains a dir, push all file from the fir in the list
        let mut filelist = HashSet::new();
        for f in list {
            let md = metadata(&f)?;
            if md.is_dir() {
                if !incdir.contains(&f) {incdir.insert(f.clone());}
                let files = std::fs::read_dir(&f)?;
                files
                    .filter_map(Result::ok)
                    .filter(|d| if let Some(e) = d.path().extension() { e == "v" || e == "sv" } else {false})
                    .for_each(|fd| {filelist.insert(fd.path());});
            } else {
                if let Some(d) = f.parent() {
                    if !incdir.contains(&d.to_path_buf()) {
                        incdir.insert(d.to_path_buf());
                    }
                }
                filelist.insert(f);
            }
        }
        // println!("[Project] Filelist = {:?}", filelist);
        // println!("[Project] Include Dir = {:?}", incdir);
        let mut ast_inc = HashMap::new();
        ast_inc.insert("uvm_macros.svh".to_string(),uvm_macro::get_uvm_macro());
        Ok(Project {
            filelist,
            incdir,
            defines: HashMap::new(),
            cur_dir: PathBuf::new(),
            ast_list: Vec::new(),
            ast_inc,
        })
    }

    // Create a project based on a source list (in .f format)
    #[allow(unused_mut)]
    pub fn from_srcfile(srclist: PathBuf, incs: Vec<PathBuf>) -> Result<Project,std::io::Error> {
        // Create the include dir list
        let mut incdir = HashSet::new();
        for d in incs {
            incdir.insert(d);
        }
        // Parse the source list to extract files, incdir, defines, ...
        let mut filelist = HashSet::new();
        let f = File::open(srclist.clone())?;
        let mut src_path = srclist;
        src_path.pop();
        let file = BufReader::new(&f);
        // TODO: use collect and filter to create the vector
        //       and also handle -f and --inc cases
        for (_num, line) in file.lines().enumerate() {
            if let Ok(l) = line {
                if l.is_empty() || l.starts_with('#') {
                    continue;
                }
                let mut p = src_path.clone();
                // Update include directory
                if l.starts_with("+incdir+") {
                    let d = l.trim_start_matches("+incdir+").trim();
                    p.push(d);
                    // incdir.insert(p);
                    if let Ok(pc) = p.canonicalize() {
                        incdir.insert(pc);
                    } else {
                        rpt_s!(MsgID::ErrFile,&path_display(&p));
                    }
                    continue;
                }
                // Add a file, using absolute path to avoid duplicate
                p.push(l);
                if let Ok(pc) = p.canonicalize() {
                    filelist.insert(pc);
                } else {
                    rpt_s!(MsgID::ErrFile,&path_display(&p));
                }
            }
        }
        let mut ast_inc = HashMap::new();
        ast_inc.insert("uvm_macros.svh".to_string(),uvm_macro::get_uvm_macro());
        Ok(Project {
            filelist,
            incdir,
            defines: HashMap::new(), // TODO: handle define from source list
            ast_list: Vec::new(),
            cur_dir: PathBuf::new(),
            ast_inc,
        })
    }

    // Compile all file from the project
    pub fn parse_file(&mut self, fname: PathBuf) -> Result<Ast,SvError> {
        rpt_set_fname!(&fname);
        // rpt_s!(MsgID::InfoStatus, "Parsing include");
        let mut src = Source::from_file(fname.clone())?;
        let mut ts = TokenStream::new(&mut src, self);
        let mut ast = Ast::new(fname);
        ast.build(&mut ts)?;
        Ok(ast)
    }

    // Compile all file from the project
    pub fn compile_all(&mut self) {
        // Parse
        for fname in self.filelist.clone() {
            // Ignore VHDL files from the source list
            if let Some(ext) = fname.extension() {
                if ext == "vhd" || ext == "vhdl" {continue;}
            }
            //
            if let Ok(mut src) = Source::from_file(fname.clone()) {
                rpt_set_fname!(&fname);
                // rpt_s!(MsgID::InfoStatus, "Parsing");
                self.defines.clear(); // TODO: reinit with project-wide define
                self.cur_dir = fname.clone();
                self.cur_dir.pop();
                let mut ts = TokenStream::new(&mut src, self);
                let mut ast = Ast::new(fname);
                match ast.build(&mut ts) {
                    Err(e) => rpt_e!(e),
                    _ => {
                        // rpt_info!("Compilation successfull");
                        self.ast_list.push(ast);
                    }
                }
            } else {
                rpt_s!(MsgID::ErrFile, &path_display(fname));
            }
        }
        // println!("[Info] Parsing Done");
        // Compile/link
    }

    // Compile all file from the project
    pub fn elaborate(&mut self) {
        let _lib = CompLib::new("my_lib".to_owned(),&self.ast_list, &self.ast_inc);
    }

    //
    pub fn compile_inc(&mut self, inc_name: String, token: Token) -> Result<(),SvError> {
        let mut inc_path = PathBuf::new();
        for s in inc_name.to_string().split('/') {
            inc_path.push(s);
        }
        let mut f : Option<PathBuf> = None;
        // Check if include is in local directory
        // TODO: Check should be done only if include is using "quotes" and not <angular_brackets>
        let mut f_raw = self.cur_dir.clone();
        f_raw.push(inc_path.clone());
        if let Ok(f_abs) = f_raw.canonicalize() {
            f = Some(f_abs.clone());
            // println!("Found {:?} in {:?}", inc_name, self.cur_dir);
        }
        // If not found yet, check if inside include directory list
        if f.is_none() {
            for d in &self.incdir {
                f_raw = d.clone();
                f_raw.push(inc_path.clone());
                if let Ok(f_abs) = f_raw.canonicalize() {
                    f = Some(f_abs.clone());
                    break;
                }
            }
        }
        // Parse file if found
        match f {
            Some(f_abs) => {
                // println!("Compiling include file {:?}", f_abs);
                match self.parse_file(f_abs.clone()) {
                    Ok(ast) => {self.ast_inc.insert(inc_name,Box::new(ast));}
                    Err(e) => rpt_e!(e)
                }
                Ok(())
            }
            None => Err(SvError::new(SvErrorKind::Include, token, "".to_string()))
            // None => Err(SvError::new(SvErrorKind::Include, token, format!("line {} -- Unable to find include file {:?}", pos, inc_name)))
        }
    }

}