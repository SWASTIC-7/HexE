use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
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
            .map(|(addr, instr, marker)| {
                let line = format!("{} {:06X}  {}", marker, addr, instr);
                if marker == ">" {
                    ListItem::new(line).style(
                        Style::default()
                            .fg(Color::Rgb(255, 200, 0))
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(line).style(Style::default().fg(Color::Rgb(200, 200, 200)))
                }
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Disassembly")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }
}
