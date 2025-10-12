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

## TUI Simulator Controls

The TUI (Terminal User Interface) provides an interactive debugging environment for your SIC/XE programs.

### Keyboard Shortcuts

| Key | Action | Description |
|-----|--------|-------------|
| **q** | Quit | Exit the TUI simulator |
| **s** | Step | Execute a single instruction (step-by-step execution) |
| **r** | Run | Run the program until completion or breakpoint |
| **b** | Breakpoint | Add a breakpoint at the current PC address |
| **Tab** | Switch Tabs | Cycle between Object Program and Symbol Table tabs |
| **Shift+Tab** | Previous Tab | Go to the previous tab |
| **↑** | Scroll Up | Scroll memory dump upward |
| **↓** | Scroll Down | Scroll memory dump downward |
| **←** | Focus Left | Move focus to the previous control button |
| **→** | Focus Right | Move focus to the next control button |
| **Enter** | Activate | Execute the currently focused button (Step/Run/Reset) |


### UI Components

1. **CPU Registers** (Top-Left): Displays current values of A, X, L, PC, and SW registers
2. **Control Buttons** (Below Registers): Interactive buttons for Step/Run/Reset
3. **Disassembly** (Left Panel): Shows disassembled instructions with current PC marked by `>`
4. **Object Code/Symbol Table** (Top-Right): Tabbed view showing object program or symbol table
5. **Memory Dump** (Bottom-Right): Hexadecimal memory dump with scrolling support
6. **Reference Bar** (Bottom): Quick reference for keyboard shortcuts


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