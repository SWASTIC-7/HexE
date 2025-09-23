use crate::predefined::opcode::{reverse_optab};
use crate::assembler::LabeledParsedLines;
use craate::predefined::common::{OBJECTPROGRAM, ObjectRecord, DisAssembledToken,Command, Instruction};
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
                                address: None,
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
                                address: None,
                            };

                        }
                        3 => {
                            //format 3

                        }
                        4 => {
                            //format 4

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