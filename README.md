## A full-fledged Assembler, Linker, Loader and Simulator for SIC/XE machine 


- [x] Parse the code to valid instructions from assembly file
- [x] Get the valid opcode from the instructions
- [x] Assembler Pass 1 -- Symbol Table, Object program , relocation table
- [x] Assembler Pass 2
- [x] Step wise running the instruction
- [x] Floating point feature
- [x] All the addressing mode -- PC, BASE, Immediate, Indirect, Indexed
- [x] Supports all the Directive  
- [x] Simulator

## Feature
- [x] supports all the sic/xe instruction
- [x] supports all the sic/xe directive
- [x] supports literals
- [x] supports base 
- [x] provides tui simulator
- [x] debugger

## Usage
To run assembly language

``` Cargo run -- <file path>.asm ```

note file must be .asm extenstion
 
 To run Object Program

 ``` Cargo run -- <file_path>.txt ```

 note file must have .txt extenstion

 ## Project Status
- [x] Basic instruction parsing
- [x] Pass 1 assembler
- [x] Pass 2 assembler
- [x] TUI implementation
- [x] Basic simulation
- [x] Loader
- [x] Disassembler
- [ ] Full floating-point support
- [ ] Complete directive support
- [ ] Literal pool implementation
- [ ] Advanced debugging features