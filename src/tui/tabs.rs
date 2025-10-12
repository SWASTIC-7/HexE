use crate::predefined::common::{ObjectRecord, SymbolTable};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Row, Table, Tabs},
};

pub struct TabsWidget {
    pub selected_tab: usize,
    pub object_program: Vec<ObjectRecord>,
    pub symbol_table: Vec<SymbolTable>,
}

impl TabsWidget {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
            object_program: Vec::new(),
            symbol_table: Vec::new(),
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 2;
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = if self.selected_tab == 0 { 1 } else { 0 };
    }

    pub fn render(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        // Render tabs
        let titles = vec!["Object Program", "Symbol Table"];
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Tables")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .select(self.selected_tab)
            .style(Style::default().fg(Color::Rgb(200, 200, 200)))
            .highlight_style(
                Style::default()
                    .fg(Color::Rgb(255, 200, 0))
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(tabs, chunks[0]);

        // Render content based on selected tab
        match self.selected_tab {
            0 => self.render_object_program(f, chunks[1]),
            1 => self.render_symbol_table(f, chunks[1]),
            _ => {}
        }
    }

    fn render_object_program(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let items: Vec<ListItem> = self
            .object_program
            .iter()
            .map(|record| {
                let text = match record {
                    ObjectRecord::Header {
                        name,
                        start,
                        length,
                    } => {
                        format!("H  {}  {:06X}  {:06X}", name, start, length)
                    }
                    ObjectRecord::Text {
                        start,
                        length,
                        objcodes,
                    } => {
                        let codes = objcodes.join(" ");
                        format!("T  {:06X}  {:02X}  {}", start, length, codes)
                    }
                    ObjectRecord::End { start } => {
                        format!("E  {:06X}", start)
                    }
                    ObjectRecord::Modification { address, length } => {
                        format!("M  {:06X}  {:02}", address, length)
                    }
                };
                ListItem::new(text)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Object Program Records")
                    .border_style(Color::Cyan),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    fn render_symbol_table(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let mut rows: Vec<Row> = Vec::new();

        // Add header row
        rows.push(
            Row::new(vec!["Label", "Address"]).style(Style::default().fg(Color::Rgb(255, 200, 0))),
        );

        for symbol in &self.symbol_table {
            rows.push(
                Row::new(vec![
                    symbol.label.clone(),
                    format!("{:06X}", symbol.address),
                ])
                .style(Style::default().fg(Color::White)),
            );
        }

        let widths = &[Constraint::Length(15), Constraint::Length(10)];

        let table = Table::new(rows, widths)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Symbol Table")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(table, area);
    }
}
