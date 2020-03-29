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
    position::Position,
    token_stream::TokenStream};

// use crate::comp::comp_lib::CompLib;

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
        println!("[Project] Include Dir = {:?}", incdir);
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
        println!("[Project] Filelist = {:?}", filelist);
        println!("[Project] Include Dir = {:?}", incdir);
        let mut ast_inc = HashMap::new();
        ast_inc.insert("uvm_macros.svh".to_string(),uvm_macro::get_uvm_macro());
        Ok(Project {
            filelist,
            incdir,
            defines: HashMap::new(),
            cur_dir: PathBuf::new(),
            ast_list: Vec::new(),
            ast_inc})
    }

    // Create a project based on a source list (in .f format)
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
                        println!("[SourceList] Unable to resolve path {:?}", p);
                    }
                    continue;
                }
                // Add a file, using absolute path to avoid duplicate
                p.push(l);
                if let Ok(pc) = p.canonicalize() {
                    filelist.insert(pc);
                } else {
                    println!("[SourceList] Unable to resolve path {:?}", p);
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
            ast_inc})
    }

    // Compile all file from the project
    pub fn parse_file(&mut self, fname: PathBuf) -> Result<Ast,SvError> {
        let mut src = Source::from_file(fname)?;
        let mut ts = TokenStream::new(&mut src, self);
        // println!("[Info] Parsing file {:?}", ts.source.get_filename());
        let mut ast = Ast::new();
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
                // println!("[Info] Parsing file {:?}", ts.source.get_filename());
                self.defines.clear(); // TODO: reinit with project-wide define
                self.cur_dir = fname.clone();
                self.cur_dir.pop();
                let mut ts = TokenStream::new(&mut src, self);
                let mut ast = Ast::new();
                match ast.build(&mut ts) {
                    Err(e) => println!("[Error] {:?}, {}", ts.source.get_filename(), e),
                    _ => {
                        // println!("[Info] File {} compiled with success", fname.display())
                        self.ast_list.push(ast);
                    }
                }
            } else {
                println!("[Error] File {:?} not found", fname);
            }
        }
        println!("[Info] Parsing Done");
        // Compile/link
        // let _lib = CompLib::new("my_lib".to_owned(),self.ast_list, self.ast_inc);
    }

    //
    pub fn compile_inc(&mut self, inc_name: String, pos: Position) -> Result<(),SvError> {
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
                    Err(e) => {println!("[Error] {:?}, {}", path_display(f_abs), e);}
                }
                Ok(())
            }
            None => Err(SvError::new(SvErrorKind::Io, pos, format!("line {} -- Unable to find include file {:?}", pos, inc_name)))
        }
    }

}