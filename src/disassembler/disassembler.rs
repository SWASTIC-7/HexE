use crate::error::{log_info, log_warning};
use crate::loader::linker::ControlSection;
use crate::predefined::common::{
    AddressFlags, Command, DisAssembledToken, EXTERNALDEFS, EXTERNALREFS, ExternalDefinition,
    ExternalReference, Instruction, ModificationInfo, OBJECTPROGRAM, ObjectRecord, OpCode,
    PROGRAMBLOCK, Reg,
};
use crate::predefined::opcode::reverse_optab;
use crate::predefined::registers::reverse_register_map;
use hex;

// Notes point to remember:::--->

// if n=0 and i=1 or n=1 and i=0, then x = 0 always
// if n=0 and i=0 or n=1 and i=1 simple addressing
// if n=0 and i=0 then bpe is use in address feild

pub fn disassemble() -> Vec<DisAssembledToken> {
    log_info("Starting disassembly process");

    let mut starting_addr = 0u32;
    let mut locctr: u32;
    let mut parsed_dissassembled_code: Vec<DisAssembledToken> = Vec::new();
    let mut modification_records: Vec<(u32, u8, bool, String)> = Vec::new();
    let mut program_name = String::from("UNKNOWN");
    let mut current_control_section: Option<ControlSection> = None;

    EXTERNALDEFS.lock().unwrap().clear();
    EXTERNALREFS.lock().unwrap().clear();

        let program_blocks = PROGRAMBLOCK.lock().unwrap().clone();
        let object_records = OBJECTPROGRAM.lock().unwrap().clone();
        let control_sections: Vec<ControlSection> = {
            log_warning("Linker unavailable; proceeding without relocation");
            Vec::new()
        };

    for lines in object_records.iter() {
        match lines {
            ObjectRecord::Header {
                name,
                start,
                length,
            } => {
                starting_addr = *start;
                program_name = name.clone();

                current_control_section = control_sections
                    .iter()
                    .find(|cs| cs.name == *name)
                    .cloned();

                if let Some(ref cs) = current_control_section {
                    log_info(&format!(
                        "Program: {}, Original Start: {:06X}, Relocated to: {:06X}, Length: {:06X}",
                        name, cs.original_start, cs.load_address, length
                    ));
                } else {
                    log_info(&format!(
                        "Program: {}, Start: {:06X}, Length: {:06X}",
                        name, start, length
                    ));
                }
            }
            ObjectRecord::Text {
                start,
                length,
                objcodes,
            } => {
                // Calculate relocated address
                let relocation_factor = current_control_section
                    .as_ref()
                    .map(|cs| cs.load_address as i32 - cs.original_start as i32)
                    .unwrap_or(0);

                locctr = (*start as i32 + relocation_factor) as u32;

                log_info(&format!(
                    "Text section at {:06X} (original {:06X}), length: {:02X}",
                    locctr, start, length
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
                                modification: None,
                            };
                            parsed_dissassembled_code.push(code_line);
                            1 // Return instruction size
                        }
                        2 => {
                            //format 2
                            let bytes = hex::decode(item).expect("Invalid hex string");
                            let reverse_table = reverse_optab();
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
                                .unwrap_or_else(|| format!("{}", r1));
                            let r2_name = register_map
                                .get(&r2)
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| format!("{}", r2));

                            let code_line = DisAssembledToken {
                                locctr,
                                command: Command::Instruction(instr),
                                flags: None,
                                address: None,
                                reg: Some(Reg {
                                    r1: r1_name,
                                    r2: r2_name,
                                }),
                                modification: None,
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

                            // Calculate actual target address for PC-relative or base-relative
                            let target_address = if flags.p {
                                // PC-relative: TA = (PC) + disp
                                let pc = locctr + 3;
                                let signed_disp = if displacement & 0x800 != 0 {
                                    // Negative displacement
                                    (displacement as i16 | 0xF000_u16 as i16) as i32
                                } else {
                                    displacement as i32
                                };
                                (pc as i32 + signed_disp) as u32
                            } else if flags.b {
                                // Base-relative: TA = (B) + disp
                                // We don't know the base register value here, so use displacement
                                displacement as u32
                            } else {
                                displacement as u32
                            };

                            let code_line = DisAssembledToken {
                                locctr,
                                command: Command::Instruction(instr),
                                flags: Some(flags),
                                address: Some(target_address),
                                reg: None,
                                modification: None,
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

                            let address = ((bytes[1] & 0x0F) as u32) << 16
                                | (bytes[2] as u32) << 8
                                | (bytes[3] as u32);

                            let code_line = DisAssembledToken {
                                locctr,
                                command: Command::Instruction(instr),
                                flags: Some(flags),
                                address: Some(address),
                                reg: None,
                                modification: None,
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
            ObjectRecord::Define { name, address } => {
                // Calculate relocated address
                let relocated_address = if let Some(ref cs) = current_control_section {
                    let relocation_factor = cs.load_address as i32 - cs.original_start as i32;
                    (*address as i32 + relocation_factor) as u32
                } else {
                    *address
                };

                log_info(&format!(
                    "Define record: {} at original {:06X}, relocated to {:06X}",
                    name, address, relocated_address
                ));

                // Store external definition
                let mut extdefs = EXTERNALDEFS.lock().unwrap();
                extdefs.push(ExternalDefinition {
                    name: name.clone(),
                    address: relocated_address,
                    control_section: program_name.clone(),
                });

                log_info(&format!(
                    "  Registered external definition: {} = {:06X} in control section '{}'",
                    name, address, program_name
                ));
            }
            ObjectRecord::Refer { name } => {
                log_info(&format!("External reference: {}", name));

                // Store external reference
                let mut extrefs = EXTERNALREFS.lock().unwrap();
                extrefs.push(ExternalReference {
                    name: name.clone(),
                    control_section: program_name.clone(),
                    resolved: false,
                    resolved_address: None,
                });

                log_info(&format!(
                    "  Registered external reference: {} in control section '{}'",
                    name, program_name
                ));
            }
            ObjectRecord::End { start } => {
                let end_start_addr = *start;
                
                // Check if it matches either original or relocated start
                let matches = if let Some(ref cs) = current_control_section {
                    end_start_addr == cs.original_start || end_start_addr == cs.load_address
                } else {
                    end_start_addr == starting_addr
                };

                if matches {
                    log_info(&format!(
                        "File disassembled successfully (end address: {:06X})",
                        end_start_addr
                    ));

                    if !modification_records.is_empty() {
                        log_info(&format!(
                            "Found {} modification records",
                            modification_records.len()
                        ));
                    }

                    break;
                } else {
                    log_warning(&format!(
                        "End address {:06X} doesn't match start address {:06X}",
                        end_start_addr, starting_addr
                    ));
                }
            }
            ObjectRecord::Modification {
                address,
                length,
                sign,
                variable,
            } => {
                // Store with original address - will be relocated when applied
                modification_records.push((*address, *length, *sign, variable.clone()));
                log_info(&format!(
                    "Modification record at address {:06X}, length: {} half-bytes (modifying {} bits), {} {}",
                    address,
                    length,
                    length * 4,
                    if *sign { "+" } else { "-" },
                    variable
                ));
            }
        }
    }

    // After processing all records, attempt to resolve external references
    resolve_external_references();

    // Apply modifications with proper relocation
    if !modification_records.is_empty() {
        log_info(&format!(
            "Applying {} modification records to disassembled code",
            modification_records.len()
        ));

        for (mod_addr, mod_length, sign, variable) in &modification_records {
            // Find which control section this modification belongs to
            let relocated_mod_addr = if let Some(cs) = control_sections
                .iter()
                .find(|cs| *mod_addr >= cs.original_start && *mod_addr < cs.original_start + cs.length)
            {
                let relocation_factor = cs.load_address as i32 - cs.original_start as i32;
                (*mod_addr as i32 + relocation_factor) as u32
            } else {
                *mod_addr
            };

            if let Some(instruction) = parsed_dissassembled_code.iter_mut().find(|token| {
                let instr_addr = token.locctr;
                let instr_size = match &token.command {
                    Command::Instruction(instr) => instr.opcode.format as u32,
                    _ => 0,
                };
                relocated_mod_addr >= instr_addr && relocated_mod_addr < instr_addr + instr_size
            }) {
                log_info(&format!(
                    "  Applying modification to instruction at {:06X}: {} {} (length: {} half-bytes)",
                    instruction.locctr,
                    if *sign { "+" } else { "-" },
                    variable,
                    mod_length
                ));

                instruction.modification = Some(ModificationInfo {
                    symbol: variable.clone(),
                    sign: *sign,
                    length: *mod_length,
                });
            } else {
                log_warning(&format!(
                    "  Modification record at {:06X} (relocated: {:06X}) doesn't match any instruction",
                    mod_addr, relocated_mod_addr
                ));
            }
        }
    }

    // Log program blocks if any
    if !program_blocks.is_empty() {
        log_info(&format!(
            "=== PROGRAM BLOCKS ({} entries) ===",
            program_blocks.len()
        ));
        for block in program_blocks.iter() {
            log_info(&format!(
                "  Block '{}' #{}: {:04X} - {:04X} (length: {:04X})",
                block.name,
                block.number,
                block.start_address,
                block.start_address + block.length,
                block.length
            ));
        }
    }

    log_info(&format!(
        "Disassembly completed: {} instructions",
        parsed_dissassembled_code.len()
    ));

    log_info("=== DISASSEMBLED PROGRAM ===");
    for item in &parsed_dissassembled_code {
        log_info(&format_disassembled_instruction(item));
    }

    parsed_dissassembled_code
}

fn resolve_external_references() {
    let extdefs = EXTERNALDEFS.lock().unwrap();
    let mut extrefs = EXTERNALREFS.lock().unwrap();

    if !extrefs.is_empty() {
        log_info(&format!(
            "=== RESOLVING {} EXTERNAL REFERENCES ===",
            extrefs.len()
        ));
    }

    for extref in extrefs.iter_mut() {
        if let Some(def) = extdefs.iter().find(|d| d.name == extref.name) {
            extref.resolved = true;
            extref.resolved_address = Some(def.address);
            log_info(&format!(
                "  Resolved '{}' from '{}' -> {:06X} (defined in '{}')",
                extref.name, extref.control_section, def.address, def.control_section
            ));
        } else {
            log_warning(&format!(
                "  Unresolved external reference: '{}' in control section '{}'",
                extref.name, extref.control_section
            ));
        }
    }

    // Log summary
    if !extdefs.is_empty() {
        log_info(&format!(
            "=== EXTERNAL DEFINITIONS ({} entries) ===",
            extdefs.len()
        ));
        for def in extdefs.iter() {
            log_info(&format!(
                "  {} @ {:06X} ({})",
                def.name, def.address, def.control_section
            ));
        }
    }

    if !extrefs.is_empty() {
        let resolved_count = extrefs.iter().filter(|r| r.resolved).count();
        let unresolved_count = extrefs.len() - resolved_count;

        log_info(&format!(
            "=== EXTERNAL REFERENCES ({} total: {} resolved, {} unresolved) ===",
            extrefs.len(),
            resolved_count,
            unresolved_count
        ));

        for extref in extrefs.iter() {
            if extref.resolved {
                log_info(&format!(
                    "  {} -> {:06X} ({})",
                    extref.name,
                    extref.resolved_address.unwrap(),
                    extref.control_section
                ));
            } else {
                log_info(&format!(
                    "  {} -> UNRESOLVED ({})",
                    extref.name, extref.control_section
                ));
            }
        }
    }
}

fn format_disassembled_instruction(token: &DisAssembledToken) -> String {
    match &token.command {
        Command::Instruction(instr) => {
            let mut result = format!("{:06X}  {:<8}", token.locctr, instr.instr);

            match instr.opcode.format {
                1 => result,
                2 => {
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

                        if let Some(mod_info) = &token.modification {
                            result.push_str(&format!(
                                " [EXT: {}{}]",
                                if mod_info.sign { "+" } else { "-" },
                                mod_info.symbol
                            ));
                        }

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
