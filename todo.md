Reminder of special error/warning which needs to be caught by the tool eventually

# Error to capture

 [ ] $sformatf : all field present and compatible with their format
 [ ] Class : Detect access to field of unitialized variable
 [ ] Module instance : check missing port / incorrect name / type
 [ ] Port direction : check input is never assigned
 [ ] randc : check variable is not in a solve before statement
 [ ] clocking block : check all signal are part of the interface
 [ ] signal declaration : check re-declaration

# Warning to capture :

 [ ] signed/unsigned conversion
 [ ] not identical variable in the for loop init/test/increment
 [ ] mix blocking/non blocking in always block
 [ ] Unused port/signals
 [ ] badly formed always ff : if reset without else or similar stuff
