use std::env;
use std::fs::File;
use std::io::{self, Read};

mod assembler;
use assembler::pass2asm;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path: String = args[1].clone();
    let mut file = File::open(file_path).expect("Failed to open the file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    // lexer::tokenize(&buffer);
    pass2asm::pass2asm(&buffer);
    // println!("{buffer}");
    Ok(())
}
