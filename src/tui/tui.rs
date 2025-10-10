pub use super::disassembly;
pub use super::memory;
pub use super::registers;
// mod registers;
// mod disassembly;
// mod memory;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

pub struct Tui {
    registers: registers::RegistersWidget,
    disassembly: disassembly::DisassemblyWidget,
    memory: memory::MemoryWidget,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            registers: registers::RegistersWidget::new(),
            disassembly: disassembly::DisassemblyWidget::new(),
            memory: memory::MemoryWidget::new(65536), // 64K memory
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(f.size());

        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(0)])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        // Render components
        self.registers.render(f, left_layout[0]);
        self.disassembly.render(f, left_layout[1]);
        self.memory.render(f, right_layout[1]);
    }

    // Add methods to update component states
    pub fn update_registers(&mut self, a: u32, x: u32, l: u32, pc: u32, sw: u32) {
        self.registers.a = a;
        self.registers.x = x;
        self.registers.l = l;
        self.registers.pc = pc;
        self.registers.sw = sw;
    }

    pub fn update_memory(&mut self, address: usize, data: &[u8]) {
        self.memory.start_address = address;
        self.memory.memory[address..address + data.len()].copy_from_slice(data);
    }

    pub fn update_disassembly(&mut self, instructions: Vec<(u32, String, String)>) {
        self.disassembly.instructions = instructions;
    }
}
