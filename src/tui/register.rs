use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Table},
};

pub struct RegisterWidget {
    pub registers: Vec<u32>,
}

impl RegisterWidget {
    pub fn new() -> Self {
        Self {
            registers: vec![0; 8], // Assuming 8 registers for SIC/XE
        }
    }

    pub fn render(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let mut rows = Vec::new();
        for (i, reg) in self.registers.iter().enumerate() {
            rows.push(ratatui::widgets::Row::new(vec![
                format!("R{}", i),
                format!("{:08X}", reg),
            ]));
        }

        let widths = vec![
            Constraint::Length(5),
            Constraint::Length(10)
        ];

        let register_table = Table::new(rows, widths)
            .block(Block::default().title("Registers").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        f.render_widget(register_table, area);
    }
}
