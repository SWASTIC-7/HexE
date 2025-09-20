use super::parser::Command;
use super::parser::parser;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SymbolTable {
    pub label: String,
    pub address: i32,
}

pub fn pass1asm(buffer: &str) {
    let parsed_lines = parser(buffer);
    let mut symbol_table: Vec<SymbolTable> = Vec::new();
    let mut locctr: i32 = 0x9999999;
    for lines in parsed_lines.iter() {
        match &lines.command {
            Command::Instruction(instr) => {
                if instr.instr.to_uppercase() == "START" {
                    let operand: Option<String> = lines.operand1.clone();
                    let num: Option<i32> = operand.as_ref().and_then(|s| s.parse::<i32>().ok());
                    if let Some(value) = num {
                        locctr = value;
                        if let Some(label) = lines.label.clone() {
                            symbol_table.push({
                                SymbolTable {
                                    label: label,
                                    address: locctr,
                                }
                            });
                        }
                    } else {
                        locctr = 0x00;
                    }
                } else if locctr != 0x9999999 {
                    let format = instr.opcode.format;
                    match format {
                        1 => {
                            locctr += 1;
                        }
                        2 => {
                            locctr += 2;
                        }
                        3 => {
                            locctr += 3;
                        }
                        4 => {
                            locctr += 4;
                        }
                        _ => {
                            println!("Error updating locctr");
                        }
                    }
                }
            }
            Command::Directive(directive) => {
                if locctr != 0x9999999 {
                    match directive.to_ascii_uppercase().as_str() {
                        "WORD" => {
                            locctr += 3;
                        }
                        "RESW" => {
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<i32> =
                                operand.as_ref().and_then(|s| s.parse::<i32>().ok());
                            if let Some(value) = num {
                                locctr += 3 * value;
                            }
                        }
                        "RESB" => {
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<i32> =
                                operand.as_ref().and_then(|s| s.parse::<i32>().ok());
                            if let Some(value) = num {
                                locctr += value;
                            }
                        }
                        "BYTE" => {
                            let operand: Option<String> = lines.operand1.clone();
                            let len: Option<i32> = operand.as_ref().map(|s| s.len() as i32);
                            if let Some(value) = len {
                                locctr += (value - 3);
                            }
                        }
                        _ => {
                            println!("some other directive");
                        }
                    }
                }
            }
        }
    }
}
