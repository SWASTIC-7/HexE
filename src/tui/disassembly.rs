use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub struct DisassemblyWidget {
    pub instructions: Vec<(u32, String, String)>, // (address, opcode, instruction)
}

impl DisassemblyWidget {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn render(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let items: Vec<ListItem> = self
            .instructions
            .iter()
            .map(|(addr, op, instr)| ListItem::new(format!("{:06X}  {:8}  {}", addr, op, instr)))
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Disassembly").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }
}
