use super::parser::parser;
use crate::error::log_info;
use crate::predefined::common::{
    Command, LITERALTABLE, LabeledParsedLines, LiteralTable, SYMBOLTABLE, SymbolTable,
};

// Helper function to parse literal and extract value
fn parse_literal(literal: &str) -> Option<(String, u32)> {
    // Literals format: =C'...' for character, =X'...' for hex
    if let Some(stripped) = literal.strip_prefix('=') {
        if let Some(char_lit) = stripped.strip_prefix("C'") {
            if let Some(content) = char_lit.strip_suffix('\'') {
                // Convert characters to hex
                let hex_value: String = content.bytes().map(|b| format!("{:02X}", b)).collect();
                let length = content.len() as u32;
                return Some((hex_value, length));
            }
        } else if let Some(hex_lit) = stripped.strip_prefix("X'") {
            if let Some(content) = hex_lit.strip_suffix('\'') {
                let length = (content.len() / 2) as u32;
                return Some((content.to_uppercase(), length));
            }
        }
    }
    None
}

// Helper to check if operand is a literal
fn is_literal(operand: &str) -> bool {
    operand.starts_with('=')
}

pub fn pass1asm(buffer: &str) -> (Vec<LabeledParsedLines>, u32, u32, Vec<SymbolTable>) {
    let parsed_lines = parser(buffer);
    let mut symbol_table = SYMBOLTABLE.lock().unwrap();
    let mut literal_table = LITERALTABLE.lock().unwrap();
    let mut labeledparsedline: Vec<LabeledParsedLines> = Vec::new();
    let mut locctr: u32 = 0x9999999;
    let mut length = 0;
    let mut startaddr = 0x00;
    let mut pending_literals: Vec<String> = Vec::new();

    for lines in parsed_lines.iter() {
        labeledparsedline.push(LabeledParsedLines {
            parsedtoken: lines.clone(),
            locctr,
        });

        match &lines.command {
            Command::Instruction(instr) => {
                if locctr != 0x9999999 {
                    // Check if operand is a literal
                    if let Some(operand) = &lines.operand1 {
                        if is_literal(operand) {
                            // Add to literal table if not already present
                            if !literal_table.iter().any(|lit| lit.literal == *operand)
                                && !pending_literals.contains(operand)
                            {
                                pending_literals.push(operand.clone());

                                if let Some((value, lit_length)) = parse_literal(operand) {
                                    literal_table.push(LiteralTable {
                                        literal: operand.clone(),
                                        value,
                                        length: lit_length,
                                        address: None,
                                    });
                                    log_info(&format!(
                                        "Found literal: {} (length: {} bytes)",
                                        operand, lit_length
                                    ));
                                }
                            }
                        }
                    }

                    let format = instr.opcode.format;
                    if let Some(label) = lines.label.clone() {
                        symbol_table.push(SymbolTable {
                            label,
                            address: locctr,
                        });
                    }

                    locctr += format as u32;
                }
            }
            Command::Directive(directive) => {
                if locctr == 0x9999999 {
                    match directive.to_uppercase().as_str() {
                        "START" => {
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<u32> =
                                operand.as_ref().and_then(|s| s.parse::<u32>().ok());
                            if let Some(value) = num {
                                startaddr = value;
                                locctr = value;
                                if let Some(label) = lines.label.clone() {
                                    symbol_table.push(SymbolTable {
                                        label,
                                        address: locctr,
                                    });
                                }
                            } else {
                                locctr = 0x00;
                            }
                        }
                        _ => {
                            println!("Did not get START");
                        }
                    }
                } else {
                    match directive.to_uppercase().as_str() {
                        "LTORG" | "END" => {
                            if !pending_literals.is_empty() {
                                log_info(&format!(
                                    "Allocating {} literals at {:06X}",
                                    pending_literals.len(),
                                    locctr
                                ));

                                for literal in &pending_literals {
                                    if let Some(lit_entry) = literal_table.iter_mut().find(|lit| {
                                        lit.literal == *literal && lit.address.is_none()
                                    }) {
                                        lit_entry.address = Some(locctr);
                                        log_info(&format!(
                                            "  Literal {} assigned address {:06X}",
                                            literal, locctr
                                        ));
                                        locctr += lit_entry.length;
                                    }
                                }
                                pending_literals.clear();
                            }

                            if directive.to_uppercase() == "END" {
                                length = locctr - startaddr;
                                break;
                            }
                        }
                        "WORD" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            locctr += 3;
                        }
                        "RESW" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<u32> =
                                operand.as_ref().and_then(|s| s.parse::<u32>().ok());
                            if let Some(value) = num {
                                locctr += 3 * value;
                            }
                        }
                        "RESB" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<u32> =
                                operand.as_ref().and_then(|s| s.parse::<u32>().ok());
                            if let Some(value) = num {
                                locctr += value;
                            }
                        }
                        "BYTE" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let len: Option<u32> = operand.as_ref().map(|s| s.len() as u32);
                            if let Some(value) = len {
                                locctr += value - 3;
                            }
                        }
                        _ => {
                            log_info(&format!("Unknown directive: {}", directive));
                        }
                    }
                }
            }
        }
    }

    // Log literal table summary
    log_info(&format!(
        "=== LITERAL TABLE ({} entries) ===",
        literal_table.len()
    ));
    for lit in literal_table.iter() {
        log_info(&format!(
            "  {} = {} (length: {}, address: {:?})",
            lit.literal, lit.value, lit.length, lit.address
        ));
    }

    (labeledparsedline, length, startaddr, symbol_table.to_vec())
}
