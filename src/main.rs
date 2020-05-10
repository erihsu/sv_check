// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

mod error;
mod reporter;
mod lex;
mod ast;
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
    /// Only parse file, no elaboration/type check/...
    #[structopt( long = "parse_only")]
    parse_only: bool,
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
    if args.parse_only {
        return;
    }
    // Debug : save AST
    // let fw = File::create("C:/tmp/sv_parser.log").unwrap();
    // let mut w = BufWriter::new(&fw);
    // write!(&mut w, "{:#?}", proj.ast_list).unwrap();

    // Analyze ASTs
    proj.elaborate();
}
