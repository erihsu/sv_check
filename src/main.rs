// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

mod lex;
mod ast;
mod error;
mod comp;

// #[macro_use]
#[allow(unused_imports)]
use std::{
    path::PathBuf,
    collections::{HashSet,HashMap},
    io::{BufReader,BufWriter, BufRead, Write},
    fs::{File,metadata,DirEntry},
    process,
};

extern crate structopt;
use structopt::StructOpt;
use structopt::clap::{App, AppSettings};

use ast::Ast;
use comp::comp_lib::CompLib;
use lex::source::Source;
use lex::token_stream::TokenStream;

macro_rules! exit {
    ($str:expr, $($var:expr),+) => {{
        println!($str,($($var),+));
        process::exit(1)
    }};
}


#[derive(Debug, StructOpt)]
#[structopt(name = "sv_check", about = "SystemVerilog Checker")]
struct Cli {
    /// List of files to compile
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
    /// Source list containing the list of file to compile
    #[structopt(parse(from_os_str), short = "f", long = "filelist")]
    srclist: Option<PathBuf>,
    /// Include directories
    #[structopt(short = "I", long = "incdir")]
    incdir: Vec<PathBuf>,
}

fn main() {

    let args = Cli::from_args();
    let mut filelist : HashSet<PathBuf>;
    let mut incdir : HashSet<PathBuf>;
    let mut ast_list = Vec::new();
    let mut ast_inc : HashMap<String,Ast> = HashMap::new();
    filelist = HashSet::new();
    incdir = HashSet::new();
    //
    if args.files.len() > 0 {
        for f in args.files {
            let md = metadata(&f).unwrap_or_else(|_| exit!("File {:?} not found!",f));
            incdir.insert(f.clone());
            if md.is_dir() {
                let files = std::fs::read_dir(&f).unwrap_or_else(|_| exit!("Unable to read directory {:?} !",f));
                files
                    .filter_map(Result::ok)
                    .filter(|d| if let Some(e) = d.path().extension() { e == "v" || e == "sv" } else {false})
                    .for_each(|fd| {filelist.insert(fd.path());});

            } else {
                filelist.insert(f);
            }
        }
    }
    // Sourcelist file -> parse it
    else if let Some(srclist) = args.srclist  {

        let f = File::open(srclist.clone())
                    .unwrap_or_else(|_| exit!("File {:?} not found!",srclist));
        let mut src_path = PathBuf::from(srclist);
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
    // No file our source list provided -> display help message
    else {
        App::new("myprog").setting(AppSettings::ArgRequiredElseHelp);
        return;
    }
    for d in args.incdir {
        incdir.insert(d);
    }
    let mut inc_files : HashMap<String,PathBuf> = HashMap::new();

    for fname in filelist {
        // Ignore VHDL files from the source list
        if let Some(ext) = fname.extension() {
            if ext == "vhd" || ext == "vhdl" {continue;}
        }
        //
        // Build AST for all file from the source list
        let mut src = Source::from_file(fname.clone())
                    .unwrap_or_else(|_| exit!("File {:?} not found!",fname));
        let mut ts = TokenStream::new(&mut src);
        let mut ast = ast::Ast::new();
        match ast.build(&mut ts) {
            Err(e) => println!("[Error] {:?}, {}", ts.source.get_filename(), e),
            _ => {
                // println!("[Info] File {} compiled with success", fname.display())
                ast_list.push(ast);
            }
        }

        // Handle included files
        if ts.inc_files.len() > 0 {
            let cwd = fname.parent().unwrap();
            // println!("Current dir = {:?}, Include files : {:?}",cwd, ts.inc_files);
            for inc_name in ts.inc_files {
                let mut inc_path = PathBuf::new();
                for s in inc_name.to_string().split("/") {
                    inc_path.push(s);
                }
                let p = cwd.join(inc_path.clone());
                // println!("Looking for {:?}", p);
                if p.is_file() {
                    if let Ok(f_abs) = p.canonicalize() {
                        // println!(" -> Found in current directory {:?}", f_abs);
                        inc_files.insert(inc_name,f_abs);
                    }
                    // else {println!(" canonicalize failed on {:#?} !", p);}
                } else {
                    // let mut found = false;
                    for d in &incdir {
                        let mut f_raw = d.clone();
                        f_raw.push(inc_path.clone());
                        if let Ok(f_abs) = f_raw.canonicalize() {
                            // println!(" -> Found in {:?}", f_abs);
                            inc_files.insert(inc_name,f_abs);
                            // found = true;
                            break;
                        }
                        // else {println!("File {:?} not found", f_raw); }
                    }
                    // if !found {
                    //     println!("Unable to find {:?} in {:?}", inc_name, incdir);
                    // }
                }
            }
        }
    }

    // println!("{:#?}", inc_files);
    for (inc_name,fname) in inc_files {
        // Build AST for all file from the source list
        let mut src = Source::from_file(fname.clone())
                    .unwrap_or_else(|_| panic!("File {:?} not found!",fname));
        let mut ts = TokenStream::new(&mut src);
        let mut ast = ast::Ast::new();
        match ast.build(&mut ts) {
            Err(e) => println!("[Error] {:?}, {}", ts.source.get_filename(), e),
            _ => {
                // println!("[Info] File {:?} compiled with success", fname);
                ast_inc.insert(inc_name,ast);
            }
            // _ => println!("{}", ast.tree)
        }
    }

    // Debug : save AST
    // let fw = File::create("C:/tmp/sv_parser.log").unwrap();
    // let mut w = BufWriter::new(&fw);
    // write!(&mut w, "{:#?}", ast_list).unwrap();

    // Analyze ASTs
    let _lib = CompLib::new("my_lib".to_owned(),ast_list, ast_inc);

    // Debug : save Lib
    // let fw = File::create("E:/tmp/sv_check_lib.log").unwrap();
    // let mut w = BufWriter::new(&fw);
    // write!(&mut w, "{:#?}", _lib).unwrap();
}
