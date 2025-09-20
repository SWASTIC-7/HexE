use std::fs::File;
use std::io::{self, Read};

mod assembler;
use assembler::lexer;

fn main() -> io::Result<()> {
    let mut file = File::open("program.asm").expect("Failed to open the file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    lexer::tokenize(&buffer);
    // println!("{buffer}");
    Ok(())
}
