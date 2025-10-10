use super::inistialize_machine::Machine;
use super::opcode_implementation::{AddressingMode, Opcode};
use crate::disassembler::disassembler;
use crate::loader::loader;
use crate::predefined::common::{
    AddressFlags, Command, DisAssembledToken, OBJECTPROGRAM, ObjectRecord,
};
use crate::predefined::opcode::reverse_optab;
use crate::predefined::registers::reverse_register_map;
use crate::tui::tui::Tui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

pub struct Simulator {
    pub machine: Machine,
    pub breakpoints: Vec<u32>,
    pub running: bool,
    pub instructions: Vec<DisAssembledToken>,
    pub program_start: u32,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            machine: Machine::new(),
            breakpoints: Vec::new(),
            running: false,
            program_start: 0,
            instructions: Vec::new(),
        }
    }

    pub fn load_program(&mut self, buffer: String) {
        loader::loader(buffer);
        self.find_program_start();

        self.machine.reg_pc = self.program_start;
        self.instructions = disassembler::disassemble();
        println!("Loaded {} instructions", self.instructions.len());
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            // Check for breakpoints
            if self.breakpoints.contains(&self.machine.reg_pc) {
                println!("Breakpoint hit at address: {:06X}", self.machine.reg_pc);
                self.running = false;
                break;
            }

            if !self.fetch_decode_execute() {
                self.running = false;
            }
        }
    }
    fn find_program_start(&mut self) {
        let object_program = OBJECTPROGRAM.lock().unwrap();
        for record in object_program.iter() {
            match record {
                ObjectRecord::Header { start, .. } => {
                    self.program_start = *start;
                    break;
                }
                _ => {}
            }
        }

        if self.program_start == 0 && !self.instructions.is_empty() {
            self.program_start = self.instructions[0].locctr;
        }
    }

    pub fn step(&mut self) -> bool {
        self.fetch_decode_execute()
    }

    pub fn fetch_decode_execute(&mut self) -> bool {
        if let Some(instr) = self.find_instruction_at_pc(self.machine.reg_pc).cloned() {
            println!(
                "Executing at {:06X}: {}",
                self.machine.reg_pc,
                self.format_instruction(&instr)
            );
            self.execute_instruction(&instr);
            true
        } else {
            println!("No instruction found at PC: {:06X}", self.machine.reg_pc);
            false
        }
    }

    fn find_instruction_at_pc(&self, pc: u32) -> Option<&DisAssembledToken> {
        self.instructions.iter().find(|instr| instr.locctr == pc)
    }

    fn execute_instruction(&mut self, token: &DisAssembledToken) {
        match &token.command {
            Command::Instruction(instr) => {
                let opcode_byte = instr.opcode.code;
                let format = instr.opcode.format;

                // Convert opcode byte to Opcode enum
                if let Some(opcode) = self.byte_to_opcode(opcode_byte) {
                    match format {
                        1 => {
                            // Format 1: No operand
                            opcode.execute(&mut self.machine, 0, AddressingMode::Direct);
                            self.machine.reg_pc += 1;
                        }
                        2 => {
                            // Format 2: Register operations
                            let operand = self.get_format2_operand(token);
                            opcode.execute(&mut self.machine, operand, AddressingMode::Direct);
                            self.machine.reg_pc += 2;
                        }
                        3 => {
                            // Format 3: Memory operations with 12-bit displacement
                            let (operand, mode) = self.get_format3_operand(token);
                            opcode.execute(&mut self.machine, operand, mode);
                            self.machine.reg_pc += 3;
                        }
                        4 => {
                            // Format 4: Extended format with 20-bit address
                            let (operand, mode) = self.get_format4_operand(token);
                            opcode.execute(&mut self.machine, operand, mode);
                            self.machine.reg_pc += 4;
                        }
                        _ => {
                            println!("Unknown instruction format: {}", format);
                            self.machine.reg_pc += 1;
                        }
                    }
                } else {
                    println!("Unknown opcode: 0x{:02X}", opcode_byte);
                    self.machine.reg_pc += 1;
                }
            }
            Command::Directive(_) => {
                self.machine.reg_pc += 1;
            }
        }
    }

    fn byte_to_opcode(&self, byte: u8) -> Option<Opcode> {
        let reverse_table = reverse_optab();
        if let Some((name, _)) = reverse_table.get(&byte) {
            self.name_to_opcode(name)
        } else {
            None
        }
    }

    fn name_to_opcode(&self, name: &str) -> Option<Opcode> {
        match name {
            "LDA" => Some(Opcode::LDA),
            "LDX" => Some(Opcode::LDX),
            "LDL" => Some(Opcode::LDL),
            "LDB" => Some(Opcode::LDB),
            "LDS" => Some(Opcode::LDS),
            "LDT" => Some(Opcode::LDT),
            "LDF" => Some(Opcode::LDF),
            "LDCH" => Some(Opcode::LDCH),
            "STA" => Some(Opcode::STA),
            "STX" => Some(Opcode::STX),
            "STL" => Some(Opcode::STL),
            "STB" => Some(Opcode::STB),
            "STS" => Some(Opcode::STS),
            "STT" => Some(Opcode::STT),
            "STF" => Some(Opcode::STF),
            "STI" => Some(Opcode::STI),
            "STCH" => Some(Opcode::STCH),
            "STSW" => Some(Opcode::STSW),
            "ADD" => Some(Opcode::ADD),
            "ADDF" => Some(Opcode::ADDF),
            "SUB" => Some(Opcode::SUB),
            "SUBF" => Some(Opcode::SUBF),
            "MUL" => Some(Opcode::MUL),
            "MULF" => Some(Opcode::MULF),
            "DIV" => Some(Opcode::DIV),
            "DIVF" => Some(Opcode::DIVF),
            "COMP" => Some(Opcode::COMP),
            "COMPF" => Some(Opcode::COMPF),
            "COMPR" => Some(Opcode::COMPR),
            "ADDR" => Some(Opcode::ADDR),
            "SUBR" => Some(Opcode::SUBR),
            "MULR" => Some(Opcode::MULR),
            "DIVR" => Some(Opcode::DIVR),
            "RMO" => Some(Opcode::RMO),
            "CLEAR" => Some(Opcode::CLEAR),
            "TIXR" => Some(Opcode::TIXR),
            "SHIFTL" => Some(Opcode::SHIFTL),
            "SHIFTR" => Some(Opcode::SHIFTR),
            "J" => Some(Opcode::J),
            "JEQ" => Some(Opcode::JEQ),
            "JGT" => Some(Opcode::JGT),
            "JLT" => Some(Opcode::JLT),
            "JSUB" => Some(Opcode::JSUB),
            "RSUB" => Some(Opcode::RSUB),
            "TIX" => Some(Opcode::TIX),
            "RD" => Some(Opcode::RD),
            "WD" => Some(Opcode::WD),
            "TD" => Some(Opcode::TD),
            "SIO" => Some(Opcode::SIO),
            "TIO" => Some(Opcode::TIO),
            "HIO" => Some(Opcode::HIO),
            "FIX" => Some(Opcode::FIX),
            "FLOAT" => Some(Opcode::FLOAT),
            "NORM" => Some(Opcode::NORM),
            "SSK" => Some(Opcode::SSK),
            "LPS" => Some(Opcode::LPS),
            "SVC" => Some(Opcode::SVC),
            _ => None,
        }
    }

    fn get_format2_operand(&self, token: &DisAssembledToken) -> u32 {
        if let Some(reg) = &token.reg {
            let register_map = reverse_register_map();
            let r1 = self.register_name_to_code(&reg.r1).unwrap_or(0);
            let r2 = self.register_name_to_code(&reg.r2).unwrap_or(0);
            ((r1 as u32) << 4) | (r2 as u32)
        } else {
            0
        }
    }

    fn get_format3_operand(&self, token: &DisAssembledToken) -> (u32, AddressingMode) {
        if let (Some(flags), Some(displacement)) = (&token.flags, &token.address) {
            let effective_address = self.calculate_effective_address(*displacement, flags);
            let mode = self.determine_addressing_mode(flags);
            (effective_address, mode)
        } else {
            (0, AddressingMode::Direct)
        }
    }

    fn get_format4_operand(&self, token: &DisAssembledToken) -> (u32, AddressingMode) {
        if let (Some(flags), Some(address)) = (&token.flags, &token.address) {
            let mode = self.determine_addressing_mode(flags);
            (*address, mode)
        } else {
            (0, AddressingMode::Direct)
        }
    }

    fn calculate_effective_address(&self, displacement: u32, flags: &AddressFlags) -> u32 {
        if flags.p {
            // PC-relative addressing
            self.machine.reg_pc.wrapping_add(displacement)
        } else if flags.b {
            // Base-relative addressing
            self.machine.reg_b.wrapping_add(displacement)
        } else {
            // Direct addressing
            displacement
        }
    }

    fn determine_addressing_mode(&self, flags: &AddressFlags) -> AddressingMode {
        if flags.i && !flags.n {
            AddressingMode::Immediate
        } else if !flags.i && flags.n {
            AddressingMode::Indirect
        } else if flags.x {
            AddressingMode::Indexed
        } else {
            AddressingMode::Direct
        }
    }

    fn register_name_to_code(&self, name: &str) -> Option<u8> {
        match name {
            "A" => Some(0),
            "X" => Some(1),
            "L" => Some(2),
            "B" => Some(3),
            "S" => Some(4),
            "T" => Some(5),
            "F" => Some(6),
            "PC" => Some(8),
            "SW" => Some(9),
            _ => None,
        }
    }

    fn format_instruction(&self, token: &DisAssembledToken) -> String {
        match &token.command {
            Command::Instruction(instr) => {
                let mut result = instr.instr.clone();

                match instr.opcode.format {
                    2 => {
                        if let Some(reg) = &token.reg {
                            result.push_str(&format!(" {},{}", reg.r1, reg.r2));
                        }
                    }
                    3 | 4 => {
                        if let (Some(flags), Some(address)) = (&token.flags, &token.address) {
                            if flags.i && !flags.n {
                                result.push_str(&format!(" #{}", address));
                            } else if !flags.i && flags.n {
                                result.push_str(&format!(" @{:06X}", address));
                            } else {
                                result.push_str(&format!(" {:06X}", address));
                            }

                            if flags.x {
                                result.push_str(",X");
                            }
                        }
                    }
                    _ => {}
                }

                result
            }
            Command::Directive(dir) => dir.clone(),
        }
    }

    pub fn reset(&mut self) {
        self.machine = Machine::new();
        self.running = false;
    }

    pub fn add_breakpoint(&mut self, address: u32) {
        if !self.breakpoints.contains(&address) {
            self.breakpoints.push(address);
            println!("Breakpoint added at {:06X}", address);
        }
    }

    pub fn remove_breakpoint(&mut self, address: u32) {
        if let Some(pos) = self.breakpoints.iter().position(|&x| x == address) {
            self.breakpoints.remove(pos);
            println!("Breakpoint removed from {:06X}", address);
        }
    }

    pub fn get_disassembly(&self) -> &Vec<DisAssembledToken> {
        &self.instructions
    }

    pub fn print_state(&self) {
        println!("\n=== MACHINE STATE ===");
        println!(
            "A: {:06X}  X: {:06X}  L: {:06X}",
            self.machine.reg_a, self.machine.reg_x, self.machine.reg_l
        );
        println!(
            "B: {:06X}  S: {:06X}  T: {:06X}",
            self.machine.reg_b, self.machine.reg_s, self.machine.reg_t
        );
        println!(
            "F: {:.2}     PC: {:06X}  SW: {:06X}",
            self.machine.reg_f, self.machine.reg_pc, self.machine.reg_sw
        );
        println!("CC: {}     Running: {}", self.machine.cc, self.running);
    }
}

// Main simulator function for compatibility
pub fn simulator(buffer: String) {
    let mut sim = Simulator::new();
    sim.load_program(buffer);

    println!("Starting simulation...");
    sim.print_state();

    // Run the simulation
    sim.run();

    println!("Simulation completed.");
    sim.print_state();
}

pub fn calling_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut tui = Tui::new();
    let mut sim = Simulator::new();

    // Main event loop
    loop {
        // Draw UI
        terminal.draw(|f| tui.draw(f))?;

        // Update TUI with simulator state
        tui.update_registers(
            sim.machine.reg_a,
            sim.machine.reg_x,
            sim.machine.reg_l,
            sim.machine.reg_pc,
            sim.machine.reg_sw,
        );

        let mem_slice = &sim.machine.memory[0..256]; // Show first 256 bytes
        tui.update_memory(0, mem_slice);

        // Convert instructions to the format expected by TUI
        let disassembly: Vec<(u32, String, String)> = sim
            .instructions
            .iter()
            .map(|instr| {
                (
                    instr.locctr,
                    sim.format_instruction(instr),
                    String::new(), // Add any additional info here if needed
                )
            })
            .collect();
        tui.update_disassembly(disassembly);

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => {
                        sim.step();
                    }
                    KeyCode::Char('r') => {
                        sim.run();
                    }
                    KeyCode::Char('b') => {
                        sim.add_breakpoint(sim.machine.reg_pc);
                    }
                    KeyCode::Char('l') => {
                        // Example: Load a program
                        // You might want to implement proper file loading here
                        if let Ok(buffer) = std::fs::read_to_string("program.obj") {
                            sim.load_program(buffer);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
