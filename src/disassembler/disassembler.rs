use crate::predefined::common::{
    AddressFlags, Command, DisAssembledToken, Instruction, OBJECTPROGRAM, ObjectRecord, Reg,
};
use crate::predefined::opcode::{OpCode, reverse_optab};
use hex;
pub fn disassemble() -> Vec<DisAssembledToken> {
    let mut starting_addr = 0u32;
    let mut locctr: u32 = 0;
    let mut parsed_dissassembled_code: Vec<DisAssembledToken> = Vec::new();
    for lines in OBJECTPROGRAM.lock().unwrap().iter() {
        match lines {
            ObjectRecord::Header {
                name,
                start,
                length,
            } => {
                let file_name = name;
                starting_addr = *start;
                locctr = starting_addr;
                let prog_length = *length;
            }
            ObjectRecord::Text {
                start,
                length,
                objcodes,
            } => {
                let start_text_addr = *start;
                let text_section_length = *length;
                for item in objcodes.iter() {
                    match item.len() / 2 {
                        1 => {
                            //format 1
                            let reverse_table = reverse_optab();
                            let mut instr_name = reverse_table
                                .get(&u8::from_str_radix(item, 16).expect("Invalid hex string"));
                            let instr = Instruction {
                                instr: instr_name
                                    .map(|(name, _)| name.to_string())
                                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                                opcode: OpCode {
                                    code: u8::from_str_radix(item, 16).expect("Invalid hex string"),
                                    format: 1,
                                },
                            };
                            let code_line = DisAssembledToken {
                                locctr: locctr,
                                command: Command::Instruction(instr),
                                flags: None,
                                address: None,
                                reg: None,
                            };
                            parsed_dissassembled_code.push(code_line);
                        }
                        2 => {
                            //format 2
                            let bytes = hex::decode(item).expect("Invalid hex string");
                            let reverse_table = reverse_optab();
                            let mut instr_name = reverse_table.get(&bytes[0]);
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
                            let code_line = DisAssembledToken {
                                locctr: locctr,
                                command: Command::Instruction(instr),
                                flags: None,
                                address: None,
                                reg: Some(Reg {
                                    r1: r1.to_string(),
                                    r2: r2.to_string(),
                                }),
                            };
                            parsed_dissassembled_code.push(code_line);
                        }
                        3 => {
                            //format 3
                            let bytes = hex::decode(item).expect("invalid hex string");
                            let reverse_table = reverse_optab();
                            let mut instr_name = reverse_table.get(&(bytes[0] & 0xFC));
                            let instr = Instruction {
                                instr: instr_name
                                    .map(|(name, _)| name.to_string())
                                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                                opcode: OpCode {
                                    code: bytes[0] & 0xFC,
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
                                locctr: locctr,
                                command: Command::Instruction(instr),
                                flags: Some(flags),
                                address: Some(displacement as u32),
                                reg: None,
                            };
                            parsed_dissassembled_code.push(code_line);
                        }
                        4 => {
                            //format 4

                            let bytes = hex::decode(item).expect("invalid hex string");
                            let reverse_table = reverse_optab();
                            let mut instr_name = reverse_table.get(&(bytes[0] & 0xFC));
                            let instr = Instruction {
                                instr: instr_name
                                    .map(|(name, _)| name.to_string())
                                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                                opcode: OpCode {
                                    code: bytes[0] & 0xFC,
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
                                locctr: locctr,
                                command: Command::Instruction(instr),
                                flags: Some(flags),
                                address: Some(displacement as u32),
                                reg: None,
                            };
                            parsed_dissassembled_code.push(code_line);
                        }
                        _ => {
                            println!("unexpected pattern found");
                        }
                    }
                }
            }
            ObjectRecord::End { start } => {
                let end_start_addr = *start;
                if end_start_addr == starting_addr {
                    println!("File disassembled successfully");
                    break;
                } else {
                    println!("Corrupt object program: start addresses don't match");
                }
            }
            ObjectRecord::Modification { address, length } => {
                // TODO: add modification in objectprogram feature
            }
        }
    }
    for items in parsed_dissassembled_code.iter() {
        println!("{:?}", items);
    }
    parsed_dissassembled_code
}
