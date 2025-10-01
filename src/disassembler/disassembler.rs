use crate::predefined::opcode::{reverse_optab};
use crate::assembler::LabeledParsedLines;
use craate::predefined::common::{OBJECTPROGRAM, ObjectRecord, DisAssembledToken,Command, Instruction, Reg, AddressFlags};
pub fn disassemble() {
    let mut starting_addr = 0u32; 
    let locctr: u32 = 0;
    for lines in OBJECTPROGRAM.iter() {
        match lines {
            ObjectRecord::Header { name, start, length } => { 
                let file_name = name;
                starting_addr = *start;
                locctr = starting_addr;
                let prog_length = *length;
            }
            ObjectRecord::Text { start, length, objcodes } => { 
                let start_text_addr = *start;
                let text_section_length = *length;
                for item in objcodes.iter() { 
                    match item.length()/2 {
                        1 => {
                            //format 1
                            let mut instr_name = reverse_optab.get(item);
                            let instr = Instruction{
                                instr: instr_name,
                                opcode: Opcode{
                                    item,
                                    1
                                }
                            };
                            let code_line = DisAssembledToken {
                                locctr: locctr,
                                command: Command::Instruction(instr),
                                flags: None,
                                address: None,
                                reg: None
                            };
                            
                        }
                        2 =>{
                            //format 2
                            let bytes = hex::decode(item).expect("Invalid hex string");
                              let mut instr_name = reverse_optab.get(bytes[0]);
                            let instr = Instruction{
                                instr: instr_name,
                                opcode: Opcode{
                                    bytes[0],
                                    2
                                }
                            };
                            let b = bytes[1];
                            let r1 = (b & 0xF0) >> 4; 
                            let r2  = b & 0x0F; 
                            let code_line = DisAssembledToken {
                                locctr: locctr,
                                command: Command::Instruction(instr),
                                flags: None,
                                address: None,
                                reg: Reg {
                                    r1,
                                    r2
                                }
                            };

                        }
                        3 => {
                            //format 3
                            let bytes = hex::decode(item).expect("invalid hex string");
                            let mut instr_name = reverse_optab.get(bytes[0]&& 0xFC);
                            let instr = Instruction{
                                instr: instr_name,
                                opcode: Opcode{
                                    bytes[0]&&0xFC,
                                    3
                                }
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
                                    reg: None
                                };
                        }
                        4 => {
                            //format 4

                            let bytes = hex::decode(item).expect("invalid hex string");
                            let mut instr_name = reverse_optab.get(bytes[0]&& 0xFC);
                            let instr = Instruction{
                                instr: instr_name,
                                opcode: Opcode{
                                    bytes[0]&&0xFC,
                                    4
                                }
                            };
                            let flags = AddressFlags {
                                    i: (bytes[0] & 0b00000001) != 0, 
                                    n: (bytes[0] & 0b00000010) != 0,  
                                    x: (bytes[1] & 0b10000000) != 0,  
                                    b: (bytes[1] & 0b01000000) != 0,  
                                    p: (bytes[1] & 0b00100000) != 0,  
                                    e: (bytes[1] & 0b00010000) != 0,  
                                };

                            let displacement = ((bytes[1] & 0x0F) as u32) << 16 | 
                                    (bytes[2] as u32) << 8 | 
                                    (bytes[3] as u32);
                            let code_line = DisAssembledToken {
                                    locctr: locctr,
                                    command: Command::Instruction(instr),
                                    flags: Some(flags),
                                    address: Some(displacement as u32),
                                    reg: None
                                };
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
}