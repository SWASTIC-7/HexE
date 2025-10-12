use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
};

pub struct RegistersWidget {
    pub a: u32,
    pub x: u32,
    pub l: u32,
    pub pc: u32,
    pub sw: u32,
}

impl RegistersWidget {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            l: 0,
            pc: 0,
            sw: 0,
        }
    }

    pub fn render(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let rows = vec![
            Row::new(vec!["A".to_string(), format!("{:06X}", self.a)]),
            Row::new(vec!["X".to_string(), format!("{:06X}", self.x)]),
            Row::new(vec!["L".to_string(), format!("{:06X}", self.l)]),
            Row::new(vec!["PC".to_string(), format!("{:06X}", self.pc)]),
            Row::new(vec!["SW".to_string(), format!("{:06X}", self.sw)]),
        ];

        let widths = vec![Constraint::Length(4), Constraint::Length(8)];

        let register_table = Table::new(rows, widths)
            .block(
                Block::default()
                    .title("CPU Registers")
                    .borders(Borders::ALL),
            )
            .header(Row::new(vec!["Reg", "Value"]).style(Style::default().fg(Color::Yellow)))
            .style(Style::default().fg(Color::White));

        f.render_widget(register_table, area);
    }
}
