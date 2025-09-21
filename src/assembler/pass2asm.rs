use crate::assembler::directive;

use super::parser::Command;
use super::parser::ParsedToken;
use super::pass1asm::{LabeledParsedLines, SymbolTable, pass1asm};
pub enum ObjectRecord {
    Header {
        name: String,
        start: u32,
        length: u32,
    },
    Text {
        start: u32,
        length: u8,
        objcodes: Vec<String>,
    },
    Modification {
        address: u32,
        length: u8,
    },
    End {
        start: u32,
    },
}

pub struct ObjectProgram {
    pub records: Vec<ObjectRecord>,
}

pub fn pass2asm(buffer: &str) {
    let (labeled_parsed_lines, len, start_addr, symbol_table): (
        Vec<LabeledParsedLines>,
        u32,
        u32,
        Vec<SymbolTable>,
    ) = pass1asm(buffer);
    for lines in labeled_parsed_lines.iter() {
        match &lines.parsedtoken.command {
            Command::Directive(directive) => match directive.to_uppercase().as_str() {
                "START" => {
                    let prog_name = lines.parsedtoken.label.clone();
                    header_record(prog_name, len, start_addr);
                }
                _ => {
                    print!("not correct directive");
                }
            },
            Command::Instruction(instr) => {
                let format = instr.opcode.format.clone();
                let opcode = instr.opcode.code.clone();
                let mut obj_code = String::new();
                match &format {
                    1 => {
                        obj_code = object_code1(opcode);
                    }
                    2 => {
                        let operand1 = lines.parsedtoken.operand1.clone();
                        let operand2 = lines.parsedtoken.operand2.clone();

                        object_code2(
                            opcode,
                            &operand1.unwrap_or_else(|| String::from("no operand 1")),
                            &operand2.unwrap_or_else(|| String::from("no operand 2")),
                        );
                    }
                    3 => object_code3(
                        opcode,
                        &lines.parsedtoken.operand1.clone(),
                        &lines.parsedtoken.operand2.clone(),
                    ),
                    4 => object_code3(
                        opcode,
                        &lines.parsedtoken.operand1.clone(),
                        &lines.parsedtoken.operand2.clone(),
                    ),
                    _ => {
                        println!(" Invalid format found to make object code");
                    }
                }
            }
        }
    }
}

fn header_record(prog_name: Option<String>, len: u32, starting_addr: u32) -> ObjectRecord {
    ObjectRecord::Header {
        name: prog_name.unwrap_or_else(|| String::from("DEFAULT")),
        start: starting_addr,
        length: len,
    }
}

// fomat 1
//  1 byte == 8bit opcode

// format 2
// 2 byte == 8bit opcode + 4bit R1 + 4bit R2

//format 3
// 3 byte == 6bit opcode + 6bit flag + 12bit disp

//format 4
// 4 byte == 6bit opcode + 6bit flag + 20bit disp

// flag =
// n i x b p e  -- addressing modes to check
// indirect , immediate, indexed, base relative, pc relative, format

pub fn object_code1(opcode: u8) -> String {
    opcode.to_string()
}
pub fn object_code2(opcode: u8, operand1: &String, operand2: &String) {}
pub fn object_code3(opcode: u8, operand1: &Option<String>, operand2: &Option<String>) {}
pub fn object_code4(opcode: u8, operand1: &Option<String>, operand2: &Option<String>) {}
