use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Table},
};

pub struct MemoryWidget {
    pub memory: Vec<u8>,
    pub start_address: usize,
}

impl MemoryWidget {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: vec![0; memory_size],
            start_address: 0,
        }
    }

    pub fn render(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let mut rows = Vec::new();
        for chunk in self.memory.chunks(16) {
            let addr = format!("{:05X}", self.start_address + rows.len() * 16);
            let hex = chunk
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");
            rows.push(ratatui::widgets::Row::new(vec![addr, hex]));
        }

        let memory_table = Table::new(rows)
            .widths(&[Constraint::Length(10), Constraint::Length(50)])
            .block(Block::default().title("Memory").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        f.render_widget(memory_table, area);
    }
}
