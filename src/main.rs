use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
mod assembler;
mod disassembler;
mod error;
mod loader;
mod predefined;
mod simulator;
mod tui;
use assembler::pass2asm;
use error::{log_error, log_info};
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
    log_info("HexE Simulator started");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        log_error("Usage: cargo run -- <filename>");
        eprintln!("Usage: cargo run -- <filename>");
        return Ok(());
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
            log_info("Assembling source file");
            pass2asm::pass2asm(&buffer);
            simulator::sim::calling_tui().unwrap_or_else(|e| {
                log_error(&format!("TUI error: {}", e));
                eprintln!("Error: {}", e);
            });
        }
        "txt" => {
            log_info("Loading object file");
            loader::loader::loader(buffer.clone());
            simulator::sim::calling_tui().unwrap_or_else(|e| {
                log_error(&format!("TUI error: {}", e));
                eprintln!("Error: {}", e);
            });
        }
        _ => {
            log_error(&format!("Unsupported file extension: {}", ext));
            eprintln!("Error: Unsupported file type. Use .asm or .txt files.");
        }
    }

    let mut sim = simulator::sim::Simulator::new();
    sim.load_program();
    // sim.add_breakpoint(0x1000);
    sim.step();

    if let Err(e) = calling_tui() {
        eprintln!("Error occurred in calling_tui: {}", e);
        std::process::exit(1);
    }

    log_info("HexE Simulator finished");

    Ok(())
}
