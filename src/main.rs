use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

mod assembler;
mod disassembler;
mod loader;
mod predefined;
mod simulator;
mod tui;

use tui::tui::Tui;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut tui = Tui::new();

    let args: Vec<String> = env::args().collect();
    let file_path: String = args[1].clone();
    let mut file = File::open(file_path).expect("Failed to open the file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    // lexer::tokenize(&buffer);
    // pass2asm::pass2asm(&buffer);
    // loader::loader::loader(buffer);
    // simulator::sim::simulator(buffer);
    let mut sim = simulator::sim::Simulator::new();
    sim.load_program(buffer);
    // sim.add_breakpoint(0x1000);
    sim.step();

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|f| tui.draw(f))?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    // Add other key handlers here
                    _ => {}
                }
            }
        }

        // Update state here
        // Example:
        // tui.update_registers(0x1234, 0x5678, 0, 0x1000, 0);
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
