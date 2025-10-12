use super::inistialize_machine::Machine;
use super::opcode_implementation::{AddressingMode, Opcode};
use super::{name_to_opcode, register_name_to_code};
use crate::disassembler::disassembler;
use crate::predefined::common::{
    AddressFlags, Command, DisAssembledToken, OBJECTPROGRAM, ObjectRecord, SYMBOLTABLE,
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

    pub fn load_program(&mut self) {
        self.instructions = disassembler::disassemble();

        // Get starting address from first instruction's locctr
        if !self.instructions.is_empty() {
            self.program_start = self.instructions[0].locctr;
        } else {
            // Fallback to header record if no instructions
            self.find_program_start_from_header();
        }

        self.machine.reg_pc = self.program_start;
        println!("Program starts at: {:06X}", self.program_start);
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

    fn find_program_start_from_header(&mut self) {
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
    }

    pub fn step(&mut self) -> bool {
        self.fetch_decode_execute()
    }

    pub fn fetch_decode_execute(&mut self) -> bool {
        if let Some(instr) = self.find_instruction_at_pc(self.machine.reg_pc).cloned() {
            // println!(
            //     "Executing at {:06X}: {}",
            //     self.machine.reg_pc,
            //     self.format_instruction(&instr)
            // );
            self.execute_instruction(&instr);
            true
        } else {
            // println!("No instruction found at PC: {:06X}", self.machine.reg_pc);
            false
        }
    }

    fn find_instruction_at_pc(&self, pc: u32) -> Option<&DisAssembledToken> {
        self.instructions.iter().find(|instr| instr.locctr == pc)
    }

    pub fn execute_instruction(&mut self, token: &DisAssembledToken) {
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
            self::name_to_opcode(name)
        } else {
            None
        }
    }

    fn get_format2_operand(&self, token: &DisAssembledToken) -> u32 {
        if let Some(reg) = &token.reg {
            let _register_map = reverse_register_map();
            let r1 = self::register_name_to_code(&reg.r1).unwrap_or(0);
            let r2 = self::register_name_to_code(&reg.r2).unwrap_or(0);
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
    sim.load_program();

    println!("Starting simulation...");
    sim.print_state();

    // Run the simulation
    sim.run();
    sim.remove_breakpoint(0x1000); // Example usage
    sim.get_disassembly(); // Example usage
    sim.print_state(); // Example usage

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

    // Load program first to initialize simulator state
    sim.load_program();

    // Load object program and symbol table from global state
    let object_program = {
        let obj_prog = OBJECTPROGRAM.lock().unwrap();
        obj_prog.clone()
    };
    tui.update_object_program(object_program);

    let symbol_table = SYMBOLTABLE.lock().unwrap().clone();
    tui.update_symbol_table(symbol_table);

    // Auto-focus memory on the object code location
    tui.auto_focus_memory();

    // Main event loop
    loop {
        // Update TUI with current simulator state before drawing
        tui.update_registers(
            sim.machine.reg_a,
            sim.machine.reg_x,
            sim.machine.reg_l,
            sim.machine.reg_pc,
            sim.machine.reg_sw,
        );

        // Memory widget now reads directly from OBJECTPROGRAM, so we don't need to update it
        // Just call update_memory with empty data to trigger render
        tui.update_memory(0, &[]);

        // Update disassembly with current instructions
        let disassembly: Vec<(u32, String, String)> = sim
            .instructions
            .iter()
            .map(|instr| {
                let is_current = instr.locctr == sim.machine.reg_pc;
                let marker = if is_current { ">" } else { " " };
                (
                    instr.locctr,
                    sim.format_instruction(instr),
                    marker.to_string(),
                )
            })
            .collect();
        tui.update_disassembly(disassembly);

        // Draw UI
        terminal.draw(|f| tui.draw(f))?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => {
                        tui.move_focus_left();
                    }
                    KeyCode::Right => {
                        tui.move_focus_right();
                    }
                    KeyCode::Up => {
                        tui.scroll_memory_up();
                    }
                    KeyCode::Down => {
                        tui.scroll_memory_down();
                    }
                    KeyCode::Tab => {
                        tui.next_tab();
                    }
                    KeyCode::BackTab => {
                        tui.previous_tab();
                    }
                    KeyCode::Enter => {
                        match tui.get_focused_button() {
                            0 => {
                                // Step button
                                sim.step();
                            }
                            1 => {
                                // Run button
                                sim.run();
                            }
                            2 => {
                                // Reset button
                                sim.reset();
                                sim.load_program();
                                tui.auto_focus_memory();
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char('s') => {
                        sim.step();
                    }
                    KeyCode::Char('r') => {
                        sim.run();
                    }
                    KeyCode::Char('b') => {
                        sim.add_breakpoint(sim.machine.reg_pc);
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
