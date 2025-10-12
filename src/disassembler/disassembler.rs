use crate::error::{log_error, log_info, log_warning};
use crate::predefined::common::{
    AddressFlags, Command, DisAssembledToken, Instruction, OBJECTPROGRAM, ObjectRecord, OpCode, Reg,
};
use crate::predefined::opcode::reverse_optab;
use crate::predefined::registers::reverse_register_map;
use hex;

pub fn disassemble() -> Vec<DisAssembledToken> {
    log_info("Starting disassembly process");

    let mut starting_addr = 0u32;
    let mut locctr: u32;
    let mut parsed_dissassembled_code: Vec<DisAssembledToken> = Vec::new();

    for lines in OBJECTPROGRAM.lock().unwrap().iter() {
        match lines {
            ObjectRecord::Header {
                name,
                start,
                length,
            } => {
                starting_addr = *start;
                log_info(&format!(
                    "Program: {}, Start: {:06X}, Length: {:06X}",
                    name, start, length
                ));
            }
            ObjectRecord::Text {
                start,
                length,
                objcodes,
            } => {
                locctr = *start;
                log_info(&format!(
                    "Text section at {:06X}, length: {:02X}",
                    start, length
                ));

                for item in objcodes.iter() {
                    let instruction_size = match item.len() / 2 {
                        1 => {
                            //format 1
                            let reverse_table = reverse_optab();
                            let opcode = u8::from_str_radix(item, 16).expect("Invalid hex string");
                            let instr_name = reverse_table.get(&opcode);

                            let instr = Instruction {
                                instr: instr_name
                                    .map(|(name, _)| name.to_string())
                                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                                opcode: OpCode {
                                    code: opcode,
                                    format: 1,
                                },
                            };

                            let code_line = DisAssembledToken {
                                locctr,
                                command: Command::Instruction(instr),
                                flags: None,
                                address: None,
                                reg: None,
                            };
                            parsed_dissassembled_code.push(code_line);
                            1 // Return instruction size
                        }
                        2 => {
                            //format 2
                            let bytes = hex::decode(item).expect("Invalid hex string");
                            let reverse_table: std::collections::HashMap<u8, (&'static str, u8)> =
                                reverse_optab();
                            let instr_name = reverse_table.get(&bytes[0]);
                            let register_map = reverse_register_map();

                            let instr = Instruction {
                                instr: instr_name
                                    .map(|(name, _)| name.to_string())
                                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                                opcode: OpCode {
                                    code: bytes[0],
                                    format: 2,
                                },
                            };

                            let b = bytes[1];
                            let r1 = (b & 0xF0) >> 4;
                            let r2 = b & 0x0F;

                            // Convert register numbers to names
                            let r1_name = register_map
                                .get(&r1)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| format!("R{}", r1));
                            let r2_name = register_map
                                .get(&r2)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| format!("R{}", r2));

                            let code_line = DisAssembledToken {
                                locctr,
                                command: Command::Instruction(instr),
                                flags: None,
                                address: None,
                                reg: Some(Reg {
                                    r1: r1_name,
                                    r2: r2_name,
                                }),
                            };
                            parsed_dissassembled_code.push(code_line);
                            2 // Return instruction size
                        }
                        3 => {
                            //format 3
                            let bytes = hex::decode(item).expect("invalid hex string");
                            let reverse_table = reverse_optab();
                            let opcode = bytes[0] & 0xFC;
                            let instr_name = reverse_table.get(&opcode);

                            let instr = Instruction {
                                instr: instr_name
                                    .map(|(name, _)| name.to_string())
                                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                                opcode: OpCode {
                                    code: opcode,
                                    format: 3,
                                },
                            };

                            let flags = AddressFlags {
                                i: (bytes[0] & 0b00000001) != 0,
                                n: (bytes[0] & 0b00000010) != 0,
                                x: (bytes[1] & 0b10000000) != 0,
                                b: (bytes[1] & 0b01000000) != 0,
                                p: (bytes[1] & 0b00100000) != 0,
                                e: (bytes[1] & 0b00010000) != 0,
                            };

                            let displacement = ((bytes[1] & 0x0F) as u16) << 8 | bytes[2] as u16;

                            let code_line = DisAssembledToken {
                                locctr,
                                command: Command::Instruction(instr),
                                flags: Some(flags),
                                address: Some(displacement as u32),
                                reg: None,
                            };
                            parsed_dissassembled_code.push(code_line);
                            3 // Return instruction size
                        }
                        4 => {
                            //format 4
                            let bytes = hex::decode(item).expect("invalid hex string");
                            let reverse_table = reverse_optab();
                            let opcode = bytes[0] & 0xFC;
                            let instr_name = reverse_table.get(&opcode);

                            let instr = Instruction {
                                instr: instr_name
                                    .map(|(name, _)| name.to_string())
                                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                                opcode: OpCode {
                                    code: opcode,
                                    format: 4,
                                },
                            };

                            let flags = AddressFlags {
                                i: (bytes[0] & 0b00000001) != 0,
                                n: (bytes[0] & 0b00000010) != 0,
                                x: (bytes[1] & 0b10000000) != 0,
                                b: (bytes[1] & 0b01000000) != 0,
                                p: (bytes[1] & 0b00100000) != 0,
                                e: (bytes[1] & 0b00010000) != 0,
                            };

                            let displacement = ((bytes[1] & 0x0F) as u32) << 16
                                | (bytes[2] as u32) << 8
                                | (bytes[3] as u32);

                            let code_line = DisAssembledToken {
                                locctr,
                                command: Command::Instruction(instr),
                                flags: Some(flags),
                                address: Some(displacement),
                                reg: None,
                            };
                            parsed_dissassembled_code.push(code_line);
                            4 // Return instruction size
                        }
                        _ => {
                            log_warning(&format!(
                                "Unexpected instruction length: {} bytes for instruction: {}",
                                item.len() / 2,
                                item
                            ));
                            0 // No size increment for unknown instructions
                        }
                    };

                    // Increment location counter by instruction size
                    locctr += instruction_size;
                }
            }
            ObjectRecord::End { start } => {
                let end_start_addr = *start;
                if end_start_addr == starting_addr {
                    log_info("File disassembled successfully");
                    break;
                } else {
                    log_error(&format!(
                        "Corrupt object program: start address {:06X} doesn't match end address {:06X}",
                        starting_addr, end_start_addr
                    ));
                }
            }
            ObjectRecord::Modification { address, length } => {
                log_info(&format!(
                    "Modification record found at address {:06X}, length: {}",
                    address, length
                ));
            }
        }
    }

    log_info(&format!(
        "Disassembly completed: {} instructions",
        parsed_dissassembled_code.len()
    ));

    // Print disassembled instructions
    log_info("=== DISASSEMBLED PROGRAM ===");
    for item in &parsed_dissassembled_code {
        log_info(&format_disassembled_instruction(item));
    }

    parsed_dissassembled_code
}

// Helper function to format disassembled instructions for display
fn format_disassembled_instruction(token: &DisAssembledToken) -> String {
    match &token.command {
        Command::Instruction(instr) => {
            let mut result = format!("{:06X}  {:<8}", token.locctr, instr.instr);

            match instr.opcode.format {
                1 => {
                    // Format 1: Just opcode
                    result
                }
                2 => {
                    // Format 2: Register operations
                    if let Some(reg) = &token.reg {
                        result.push_str(&format!(" {},{}", reg.r1, reg.r2));
                    }
                    result
                }
                3 | 4 => {
                    // Format 3/4: Address operations
                    if let (Some(flags), Some(address)) = (&token.flags, &token.address) {
                        let mut operand = String::new();

                        // Determine addressing mode based on flags
                        if flags.i && !flags.n {
                            // Immediate addressing
                            operand.push('#');
                            operand.push_str(&format!("{}", address));
                        } else if !flags.i && flags.n {
                            // Indirect addressing
                            operand.push('@');
                            operand.push_str(&format!("{:06X}", address));
                        } else {
                            // Direct addressing
                            operand.push_str(&format!("{:06X}", address));
                        }

                        // Add indexed flag
                        if flags.x {
                            operand.push_str(",X");
                        }

                        result.push_str(&format!(" {}", operand));

                        // Add addressing mode indicators
                        let mut mode_info = String::new();
                        if flags.p {
                            mode_info.push_str(" [PC-rel]");
                        }
                        if flags.b {
                            mode_info.push_str(" [Base-rel]");
                        }
                        if flags.e && instr.opcode.format == 4 {
                            mode_info.push_str(" [Extended]");
                        }

                        if !mode_info.is_empty() {
                            result.push_str(&mode_info);
                        }
                    }
                    result
                }
                _ => result,
            }
        }
        Command::Directive(dir) => {
            format!("{:06X}  {}", token.locctr, dir)
        }
    }
}
