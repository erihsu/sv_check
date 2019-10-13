# Overview
SV Checker is a tool dedicated to development in SystemVerilog.

It should provide compilation & elaboration error/warning, as well as advanced linting.

The end goal is that this crate could be the basis of an LSP implementation (language server protocol)
 and also be usable directly with the Sublime Text SystemVerilog plugin.

## Current status
 - [x] Parsing of basic RTL file
 - [x] Parsing of basic verification file (UVM)
 - [ ] Basic linting: port connection, signal declaration, fields, ...
 - [ ] Advanced linting
 - [ ] Incremental compilation
 - [ ] Configurable error system (lower severity of messages, min severity in display, ...)


# Usage

## Installation

From source :
 - Clone this repositories
 - install the rust toolchain : https://www.rust-lang.org/tools/install
 - run ```cargo build --release```

## Command-line option

There is currently two way to use the checker :

 - simply put all the file to be compiled as arguments, e.g. ```sv_check.exe file1.sv file2.sv```
 - Use a source list file containing all the file to be compiled and pass it with -f : ```sv_check.exe -f my_project.srclist```

The source list file uses the dot-f format, used by standard EDA tools like DC.

