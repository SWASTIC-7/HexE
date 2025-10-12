use crate::predefined::common::{OBJECTPROGRAM, ObjectRecord};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

pub struct MemoryWidget {
    pub memory: Vec<u8>,
    pub start_address: usize,
    pub scroll_offset: u16,
    pub max_address: u32,
}

impl MemoryWidget {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: vec![0; memory_size],
            start_address: 0,
            scroll_offset: 0,
            max_address: 0,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        let max_scroll = (self.max_address / 16).saturating_sub(10);
        if (self.scroll_offset as u32) < max_scroll {
            self.scroll_offset += 1;
        }
    }

    pub fn scroll_to_address(&mut self, addr: u32) {
        // Scroll to show the address in the middle of the view
        let line_number = (addr / 16).saturating_sub(5);
        self.scroll_offset = line_number as u16;
    }

    pub fn render(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let mut lines = Vec::new();

        // Build complete memory map from OBJECTPROGRAM
        let end_addr = (self.max_address + 0x10000).max(0x10000);
        let mut memory_bytes: Vec<u8> = vec![0; end_addr as usize];

        if let Ok(obj_prog) = OBJECTPROGRAM.lock() {
            for record in obj_prog.iter() {
                if let ObjectRecord::Text {
                    start, objcodes, ..
                } = record
                {
                    let mut current_addr = *start;

                    for code in objcodes {
                        // Convert hex string to bytes
                        for i in (0..code.len()).step_by(2) {
                            if i + 1 < code.len() {
                                if let Ok(byte) = u8::from_str_radix(&code[i..i + 2], 16) {
                                    if (current_addr as usize) < memory_bytes.len() {
                                        memory_bytes[current_addr as usize] = byte;
                                    }
                                    current_addr += 1;
                                }
                            } else if i < code.len() {
                                // Handle odd-length hex strings
                                if let Ok(byte) = u8::from_str_radix(&code[i..i + 1], 16) {
                                    if (current_addr as usize) < memory_bytes.len() {
                                        memory_bytes[current_addr as usize] = byte;
                                    }
                                    current_addr += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Generate memory dump lines from 0x000000 to end_addr
        let end_addr_aligned = end_addr & !0xF; // Align to 16

        for addr in (0..=end_addr_aligned).step_by(16) {
            let line_str = self.format_memory_line(addr, &memory_bytes);
            lines.push(Line::from(line_str));
        }

        // Apply scroll offset
        let visible_lines: Vec<Line> = lines
            .into_iter()
            .skip(self.scroll_offset as usize)
            .take(area.height.saturating_sub(2) as usize)
            .collect();

        let paragraph = Paragraph::new(visible_lines)
            .block(
                Block::default()
                    .title("Memory Dump (↑↓ to scroll)")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::Cyan));

        f.render_widget(paragraph, area);

        // Render scrollbar
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let total_lines = (end_addr_aligned / 16) as usize;
        let visible_height = area.height.saturating_sub(2) as usize;
        let max_scroll = total_lines.saturating_sub(visible_height);

        let mut scrollbar_state =
            ScrollbarState::new(max_scroll).position(self.scroll_offset as usize);

        f.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }

    fn format_memory_line(&self, addr: u32, memory: &[u8]) -> String {
        let mut result = format!("{:07x}", addr);

        // Add 16 bytes in pairs
        for i in 0..16 {
            if i % 2 == 0 {
                result.push(' ');
            }

            let byte_addr = (addr + i) as usize;
            if byte_addr < memory.len() {
                result.push_str(&format!("{:02x}", memory[byte_addr]));
            } else {
                result.push_str("00");
            }
        }

        result
    }

    pub fn update_max_address(&mut self) {
        // Calculate max address from OBJECTPROGRAM
        let mut max_addr = 0u32;

        if let Ok(obj_prog) = OBJECTPROGRAM.lock() {
            for record in obj_prog.iter() {
                if let ObjectRecord::Text {
                    start, objcodes, ..
                } = record
                {
                    let mut current_addr = *start;
                    for code in objcodes {
                        current_addr += (code.len() / 2) as u32;
                    }
                    max_addr = max_addr.max(current_addr);
                }
            }
        }

        self.max_address = max_addr;
    }

    pub fn auto_focus(&mut self) {
        // Find first object code address and scroll to it
        if let Ok(obj_prog) = OBJECTPROGRAM.lock() {
            for record in obj_prog.iter() {
                if let ObjectRecord::Text { start, .. } = record {
                    self.scroll_to_address(*start);
                    return;
                }
            }
        }
    }
}
