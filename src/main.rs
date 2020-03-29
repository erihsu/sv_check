// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

mod lex;
mod ast;
mod error;
mod comp;
mod project;

// #[macro_use]
#[allow(unused_imports)]
use std::{
    path::PathBuf,
    io::{BufWriter, Write},
    process,
};

extern crate structopt;
use structopt::StructOpt;
use structopt::clap::{App, AppSettings};


use project::Project;

use comp::comp_lib::CompLib;
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
    //
    let mut proj;
    if !args.files.is_empty() {
        proj = Project::from_list(args.files, args.incdir).unwrap_or_else(|e| exit!("{:?} ",e));
    }
    // Sourcelist file -> parse it
    else if let Some(srclist) = args.srclist  {
        proj = Project::from_srcfile(srclist, args.incdir).unwrap_or_else(|e| exit!("{:?} ",e));
    }
    // No file or source list provided -> display help message
    else {
        App::new("myprog").setting(AppSettings::ArgRequiredElseHelp);
        return;
    }

    proj.compile_all();

    // Debug : save AST
    // let fw = File::create("C:/tmp/sv_parser.log").unwrap();
    // let mut w = BufWriter::new(&fw);
    // write!(&mut w, "{:#?}", proj.ast_list).unwrap();

    // Analyze ASTs
    let _lib = CompLib::new("my_lib".to_owned(),proj.ast_list, proj.ast_inc);
    // Debug : save Lib
    // let fw = File::create("E:/tmp/sv_check_lib.log").unwrap();
    // let mut w = BufWriter::new(&fw);
    // write!(&mut w, "{:#?}", _lib).unwrap();
}
