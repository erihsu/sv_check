# Overview
SV Checker is a tool dedicated to development in SystemVerilog.

The long term goals is to offer a full-featured language server:
 - compilation check
 - linting
 - reformatting
 - refactoring
 - code completion

The idea is also that some features should be accessible in command line (like compilation/linting),
and being compatible with the SublimeText SystemVerilog plugin to extend its existing features.

## Current status
 - [x] Parsing of basic RTL file
 - [ ] Parsing of basic verification file (UVM)
 - [ ] Basic linting: port connection, signal declaration, fields, ...
 - [ ] Advanced linting
 - [ ] Basic LSP implementation
 - [ ] Reformatting
 - [ ] Refactoring


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

The source list file uses the same format as the one used by standard EDA tools like DC


## LSP setup

Not available