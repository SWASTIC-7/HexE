use std::fs::File;
use std::io::{self, Read};

mod assembler;
use assembler::parser;

fn main() -> io::Result<()> {
    let mut file = File::open("program.asm").expect("Failed to open the file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    // lexer::tokenize(&buffer);
    parser::parser(&buffer);
    // println!("{buffer}");
    Ok(())
}
