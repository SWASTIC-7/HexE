pub use super::disassembly;
pub use super::memory;
pub use super::registers;
pub use super::tabs;

use crate::predefined::common::{ObjectRecord, SymbolTable};
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
    tabs: tabs::TabsWidget,
    buttons: Vec<&'static str>,
    focused_button: usize,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            registers: registers::RegistersWidget::new(),
            disassembly: disassembly::DisassemblyWidget::new(),
            memory: memory::MemoryWidget::new(65536), // 64K memory
            tabs: tabs::TabsWidget::new(),
            buttons: vec!["Step", "Run", "Reset"],
            focused_button: 0,
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        // Set background color
        f.render_widget(
            Block::default().style(Style::default().bg(Color::Rgb(25, 25, 35))),
            f.area(),
        );

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
        self.tabs.render(f, right_layout[0]);
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
                    .bg(Color::Rgb(255, 200, 0))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::Rgb(200, 200, 200))
                    .bg(Color::Rgb(50, 50, 60))
            };

            let button = Paragraph::new(Span::styled(button_text, style))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan)),
                )
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

    pub fn update_memory(&mut self, start_address: usize, _data: &[u8]) {
        // Update max address and auto-focus on first load
        self.memory.update_max_address();
        self.memory.start_address = start_address;
    }

    pub fn scroll_memory_up(&mut self) {
        self.memory.scroll_up();
    }

    pub fn scroll_memory_down(&mut self) {
        self.memory.scroll_down();
    }

    pub fn auto_focus_memory(&mut self) {
        self.memory.auto_focus();
    }

    pub fn update_disassembly(&mut self, instructions: Vec<(u32, String, String)>) {
        self.disassembly.instructions = instructions;
    }

    pub fn update_object_program(&mut self, object_program: Vec<ObjectRecord>) {
        self.tabs.object_program = object_program;
    }

    pub fn update_symbol_table(&mut self, symbol_table: Vec<SymbolTable>) {
        self.tabs.symbol_table = symbol_table;
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

    pub fn next_tab(&mut self) {
        self.tabs.next_tab();
    }

    pub fn previous_tab(&mut self) {
        self.tabs.previous_tab();
    }

    pub fn get_focused_button(&self) -> usize {
        self.focused_button
    }
}
