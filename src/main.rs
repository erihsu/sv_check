// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

mod lex;
mod ast;
mod error;
mod comp;

// #[macro_use]
extern crate structopt;
use std::path::PathBuf;
use std::collections::{HashSet};
use std::io::BufRead;
use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;

use comp::comp_lib::CompLib;
use lex::source::Source;
use lex::token_stream::TokenStream;


#[derive(Debug, StructOpt)]
#[structopt(name = "sv_check", about = "SystemVerilog Checker")]
struct Cli {
    /// source list containing the list of file to compile
    #[structopt(short = "f", long = "filelist", default_value = "")]
    srclist: PathBuf,
    /// List of files to compile
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {

    let args = Cli::from_args();
    let mut filelist : HashSet<PathBuf>;
    let mut incdir : HashSet<PathBuf>;
    let mut ast_list = Vec::new();
    filelist = HashSet::new();
    incdir = HashSet::new();
    if args.files.len() > 0 {
        for f in args.files {
            filelist.insert(f);
        }
    }
    else  {
        let f = File::open(args.srclist.clone())
                    .unwrap_or_else(|_| panic!("File {:?} not found!",args.srclist));
        let mut src_path = PathBuf::from(args.srclist);
        src_path.pop();
        let file = BufReader::new(&f);
        // TODO: use collect and filter to create the vector
        //       and also handle -f and --inc cases
        for (_num, line) in file.lines().enumerate() {
            if let Ok(l) = line {
                if l.len()==0 || l.starts_with("#") {
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
                        println!("Unable to resolve path {:?}", p);
                    }
                    continue;
                }
                // Add a file, using absolute path to avoid duplicate
                p.push(l);
                if let Ok(pc) = p.canonicalize() {
                    filelist.insert(pc);
                } else {
                    println!("Unable to resolve path {:?}", p);
                }
            }
        }
        // println!("Include dir : {:?}", incdir);
    }

    let mut inc_files : HashSet<PathBuf>;
    inc_files = HashSet::new();

    for fname in filelist {
        // Ignore VHDL files from the source list
        if let Some(ext) = fname.extension() {
            if ext == "vhd" || ext == "vhdl" {continue;}
        }
        // Build AST for all file from the source list
        let mut src = Source::from_file(fname.clone())
                    .unwrap_or_else(|_| panic!("File {:?} not found!",fname));
        let mut ts = TokenStream::new(&mut src);
        let mut ast = ast::Ast::new();
        match ast.build(&mut ts) {
            Err(e) => println!("[Error] {:?}, {}", fname, e),
            _ => println!("[Info] File {} compiled with success", fname.display())
            // _ => println!("{}", ast.tree)
        }
        ast_list.push(ast);
        // Handle included files
        if ts.inc_files.len() > 0 {
            let cwd = fname.parent().unwrap();
            // println!("Current dir = {:?}, Include files : {:?}",cwd, ts.inc_files);
            for inc_name in ts.inc_files {
                let p = cwd.join(PathBuf::from(inc_name.clone()));
                if p.is_file() {
                    inc_files.insert(p.canonicalize().unwrap());
                } else {
                    // let mut found = false;
                    for d in &incdir {
                        let mut f_raw = d.clone();
                        f_raw.push(inc_name.clone());
                        if let Ok(f_abs) = f_raw.canonicalize() {
                                inc_files.insert(f_abs);
                                // found = true;
                                break;
                        }
                    }
                    // if !found {
                    //     println!("Unable to find {:?} in {:?}", inc_name, incdir);
                    // }
                }
            }
        }
    }

    // println!("{:?}", inc_files);
    for fname in inc_files {
        // Build AST for all file from the source list
        let mut src = Source::from_file(fname.clone())
                    .unwrap_or_else(|_| panic!("File {:?} not found!",fname));
        let mut ts = TokenStream::new(&mut src);
        let mut ast = ast::Ast::new();
        match ast.build(&mut ts) {
            Err(e) => println!("[Error] {:?}, {}", fname, e),
            _ => println!("[Info] File {:?} compiled with success", fname)
            // _ => println!("{}", ast.tree)
        }
    }

    // Analyze ASTs
    let _lib = CompLib::new("my_lib".to_string(),ast_list);
}
