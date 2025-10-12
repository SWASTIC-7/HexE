pub use super::disassembly;
pub use super::memory;
pub use super::registers;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

pub struct Tui {
    registers: registers::RegistersWidget,
    disassembly: disassembly::DisassemblyWidget,
    memory: memory::MemoryWidget,
    buttons: Vec<&'static str>,
    focused_button: usize,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            registers: registers::RegistersWidget::new(),
            disassembly: disassembly::DisassemblyWidget::new(),
            memory: memory::MemoryWidget::new(65536), // 64K memory
            buttons: vec!["Step", "Run", "Reset"],
            focused_button: 0,
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(f.area());

        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        // Render components
        self.registers.render(f, left_layout[0]);
        self.render_controls(f, left_layout[1]);
        self.disassembly.render(f, left_layout[2]);
        self.memory.render(f, right_layout[1]);
    }

    fn render_controls(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(area);

        for (i, &button_text) in self.buttons.iter().enumerate() {
            let style = if i == self.focused_button {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            };

            let button = Paragraph::new(Span::styled(button_text, style))
                .block(Block::default().borders(Borders::ALL))
                .alignment(ratatui::layout::Alignment::Center);

            f.render_widget(button, button_layout[i]);
        }
    }

    // Add methods to update component states
    pub fn update_registers(&mut self, a: u32, x: u32, l: u32, pc: u32, sw: u32) {
        self.registers.a = a;
        self.registers.x = x;
        self.registers.l = l;
        self.registers.pc = pc;
        self.registers.sw = sw;
    }

    pub fn update_memory(&mut self, start_address: usize, data: &[u8]) {
        self.memory.start_address = start_address;
        let len = data.len().min(self.memory.memory.len() - start_address);
        self.memory.memory[start_address..start_address + len].copy_from_slice(&data[..len]);
    }

    pub fn update_disassembly(&mut self, instructions: Vec<(u32, String, String)>) {
        self.disassembly.instructions = instructions;
    }

    pub fn move_focus_left(&mut self) {
        if self.focused_button > 0 {
            self.focused_button -= 1;
        }
    }

    pub fn move_focus_right(&mut self) {
        if self.focused_button + 1 < self.buttons.len() {
            self.focused_button += 1;
        }
    }

    pub fn get_focused_button(&self) -> usize {
        self.focused_button
    }
}
