// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

mod position;
mod source;
mod token;
mod tokenizer;
mod ast;
mod error;

// #[macro_use]
extern crate structopt;
use std::path::PathBuf;
use std::io::BufRead;
use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;

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
    let mut filelist : Vec<PathBuf>;
    if args.files.len() > 0 {
        filelist = args.files;
    }
    else  {
        filelist = Vec::new();
        let f = File::open(args.srclist.clone())
                    .unwrap_or_else(|_| panic!("File {:?} not found!",args.srclist));
        let mut src_path = PathBuf::from(args.srclist);
        src_path.pop();
        let file = BufReader::new(&f);
        // TODO: use collect and filter to create the vector
        //       and also handle -f and --inc cases
        for (_num, line) in file.lines().enumerate() {
            if let Ok(l) = line {
                if l.len() >0 && !l.starts_with("-f") {
                    let mut p = src_path.clone();
                    p.push(l);
                    filelist.push(p)
                // } else {
                }
            }
        }
    }
    for fname in filelist {
        let mut src = source::Source::from_file(fname.clone())
                    .unwrap_or_else(|_| panic!("File {:?} not found!",fname));
        let mut ts = tokenizer::TokenStream::new(&mut src);
        let mut ast = ast::Ast::new();
        match ast.build(&mut ts) {
            Err(e) => println!("[Error] {:?}, {}", fname, e),
            _ => println!("[Info] File {:?} compiled with success", fname)
            // _ => println!("{}", ast.tree)
        }
    }
}
