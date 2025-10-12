use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
mod assembler;
mod disassembler;
mod loader;
mod predefined;
mod simulator;
mod tui;

use assembler::pass2asm;
use simulator::sim::calling_tui;
//when Assembly file is given
// Assembly Program (.asm)
//    ↓
// Assembler (Pass 1 + Pass 2)
//    ↓
// Object Program (.obj)
//    ↓
// Loader
//    ↓
// Memory Image
//    ↓
// Simulator (Disassemble + tui + Execution)

// when object program is passed
// Object Program (.obj)
//    ↓
// Loader
//    ↓
// Memory Image
//    ↓
// Simulator (Disassemble + tui +  Execution)

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let file_path: String = args[1].clone();
    let ext = Path::new(&file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let mut file = File::open(&file_path).expect("Failed to open the file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    match ext {
        "asm" => {
            println!("Running assembler on {}", file_path);
            pass2asm::pass2asm(&buffer);
        }
        "txt" => {
            println!("Running loader on {}", file_path);
            loader::loader::loader(buffer.clone());
        }
        _ => {
            eprintln!("Unsupported file extension: {}", ext);
            std::process::exit(1);
        }
    }
    // lexer::tokenize(&buffer);
    // pass2asm::pass2asm(&buffer);
    // loader::loader::loader(buffer);
    // simulator::sim::simulator(buffer);
    let mut sim = simulator::sim::Simulator::new();
    sim.load_program();
    // sim.add_breakpoint(0x1000);
    sim.step();
    sim.step();
    sim.step();
    if let Err(e) = calling_tui() {
        eprintln!("Error occurred in calling_tui: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
