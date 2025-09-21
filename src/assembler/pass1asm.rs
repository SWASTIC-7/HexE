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
    println!("Parsed {} lines", parsed_lines.len()); // Debug: check if parser returns data
    let mut symbol_table: Vec<SymbolTable> = Vec::new();
    let mut locctr: i32 = 0x9999999;
    let mut length = 0;
    let mut startaddr = 0x00;
    for lines in parsed_lines.iter() {
        match &lines.command {
            Command::Instruction(instr) => {
                if locctr != 0x9999999 {
                    let format = instr.opcode.format;
                    match format {
                        1 => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            locctr += 1;
                        }
                        2 => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            locctr += 2;
                        }
                        3 => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            locctr += 3;
                        }
                        4 => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            locctr += 4;
                        }
                        _ => {
                            println!("Error updating locctr");
                        }
                    }
                }
            }
            Command::Directive(directive) => {
                if locctr == 0x9999999 {
                    match directive.to_uppercase().as_str() {
                        "START" => {
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<i32> =
                                operand.as_ref().and_then(|s| s.parse::<i32>().ok());
                            if let Some(value) = num {
                                startaddr = value;
                                locctr = value;
                                if let Some(label) = lines.label.clone() {
                                    symbol_table.push({
                                        SymbolTable {
                                            label,
                                            address: locctr,
                                        }
                                    });
                                } else {
                                    locctr = 0x00;
                                }
                            }
                        }
                        _ => {
                            println!("Did not got start");
                        }
                    }
                }
                if locctr != 0x9999999 {
                    match directive.to_uppercase().as_str() {
                        "WORD" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            locctr += 3;
                        }
                        "RESW" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<i32> =
                                operand.as_ref().and_then(|s| s.parse::<i32>().ok());
                            if let Some(value) = num {
                                locctr += 3 * value;
                            }
                        }
                        "RESB" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<i32> =
                                operand.as_ref().and_then(|s| s.parse::<i32>().ok());
                            if let Some(value) = num {
                                locctr += value;
                            }
                        }
                        "BYTE" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push({
                                    SymbolTable {
                                        label,
                                        address: locctr,
                                    }
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let len: Option<i32> = operand.as_ref().map(|s| s.len() as i32);
                            if let Some(value) = len {
                                locctr += value - 3;
                            }
                        }
                        "END" => {
                            length = locctr - startaddr;
                            break;
                        }
                        _ => {
                            println!("some other directive");
                        }
                    }
                }
            }
        }
        println!("Processing line: {:?} with loccctr {:x?}", lines, locctr); // Debug: see what each line contains
    }

    for items in symbol_table {
        println!("{items:x?}");
    }
    println!("{length}");
}
