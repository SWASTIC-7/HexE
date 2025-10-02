use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};
use std::env;
use std::fs::File;
use std::io::{self, Read};
mod assembler;
mod disassembler;
mod loader;
mod predefined;
mod simulator;
mod tui;

fn main() -> io::Result<()> {
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
    //  tui::tui::list::list();
    Ok(())
}
