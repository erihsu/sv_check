Reminder of special error/warning which needs to be caught by the tool eventually

# Error to capture

 [ ] $sformatf : all field present and compatible with their format
 [ ] Class : Detect access to field of unitialized variable
 [ ] Module instance : check missing port / incorrect name / type
 [ ] Port direction : check input is never assigned
 [ ] randc : check variable is not in a solve before statement
 [ ] clocking block : check all signal are part of the interface
 [x] signal declaration : check re-declaration
 [ ] label/instance name re-declaration
 [ ] non-continuous assignment of a wire
 [ ] Array assignement with non matching size
 [ ] Check hierachical access


# Warning to capture :

 [ ] signed/unsigned conversion
 [ ] not identical variable in the for loop init/test/increment
 [ ] mix blocking/non blocking in always block
 [ ] Unused port/signals
 [ ] badly formed always ff : if without else, missing case entry, ...

# Known Issues :
 - Non-Ansi C port declaration triggers re-declaration error

# Roadmap
 [x] v0.1.0 : Basic RTL parsing
 [x] v0.2.0 : Improved parser (class) and basic source list
 [x] v0.3.0 : Setup AST walker to collect missing reference, calls, ...
 [x] v0.4.0 : Complete identifier check (including going through base class)
 [x] v0.5.0 : Basic function/instance check (definition and number of parameters)
 [x] v0.6.0 : Check hierarchical access
 [ ] v0.6.5 : Improved pre-processor
 [ ] v0.7.0 : Setup basic messaging system
 [ ] v0.8.0 : Incremental compilation
 [ ] v0.8.5 : Pre-compiled UVM library
 [ ] v0.9.0 : Basic type check (function/instance)
 [ ] v1.0.0 : Pass a significant amount of test (TBD) from the SymbiFlow testsuite
 [ ] v1.1.0 : Handle binding
 [ ] v1.2.0 : Basic linting: unused port/signals, assign input
