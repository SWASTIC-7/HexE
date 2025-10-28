use crate::error::{log_info, log_warning};
use crate::predefined::common::{OBJECTPROGRAM, ObjectRecord};
use crate::predefined::opcode::get_instruction_format;

//Object program structure
//H == 3byte name ,3 byte starting addr of program,  3byte length
//T == 3byte starting add, 1 byte length, 30byte obj_program
//E == 3byte starting address of executable instructions

pub fn loader(buffer: String) -> Vec<ObjectRecord> {
    let mut parsed_obj_prog = OBJECTPROGRAM.lock().unwrap();

    for line in buffer.lines() {
        let mut trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        // Remove spaces and ^ characters from the line
        let filtered_line = trimmed_line
            .chars()
            .filter(|&c| c != ' ' && c != '^')
            .collect::<String>();
        trimmed_line = &filtered_line;

        let record_header: char = match trimmed_line.chars().next() {
            Some(c) => c,
            None => continue,
        };
        let record: &str = &trimmed_line[1..];

        match record_header {
            'H' => {
                // Header must be at least 18 chars (6 name + 6 start + 6 length)
                if record.len() < 18 {
                    log_warning(&format!(
                        "Invalid header record: too short (expected 18 chars, got {})",
                        record.len()
                    ));
                    continue;
                }

                let program_name = &record[0..6];
                let start_addr_hex = &record[6..12];
                let length_hex = &record[12..18];

                let start_addr = u32::from_str_radix(start_addr_hex, 16).unwrap_or(0);
                let length = u32::from_str_radix(length_hex, 16).unwrap_or(0);

                let parsed_obj = ObjectRecord::Header {
                    name: program_name.trim().to_string(),
                    start: start_addr,
                    length,
                };
                parsed_obj_prog.push(parsed_obj);
            }
            'T' => {
                // Text record must be at least 8 chars (6 start + 2 length)
                if record.len() < 8 {
                    log_warning("Invalid text record: too short");
                    continue;
                }

                let start_addr_hex = &record[0..6];
                let length_hex = &record[6..8];
                let obj_code = &record[8..];

                let start_addr = u32::from_str_radix(start_addr_hex, 16).unwrap_or(0);
                let length = u8::from_str_radix(length_hex, 16).unwrap_or(0);

                let mut objcodes: Vec<String> = Vec::new();
                let mut i = 0;

                while i < obj_code.len() {
                    if i + 1 >= obj_code.len() {
                        break;
                    }

                    let s = &obj_code[i..i + 2];

                    if let Ok(byte_val) = u8::from_str_radix(s, 16) {
                        let mut processed = false;

                        // First check for format 1 and 2 (exact opcode match)
                        if let Some(format) = get_instruction_format(byte_val) {
                            match format {
                                1 => {
                                    objcodes.push(obj_code[i..i + 2].to_string());
                                    i += 2;
                                    processed = true;
                                }
                                2 => {
                                    if i + 4 <= obj_code.len() {
                                        objcodes.push(obj_code[i..i + 4].to_string());
                                        i += 4;
                                        processed = true;
                                    }
                                }
                                _ => {}
                            }
                        }

                        // If not processed, check for format 3/4 (masked opcode)
                        if !processed {
                            let opcode = byte_val & 0xFC;
                            if let Some(format) = get_instruction_format(opcode) {
                                match format {
                                    3 => {
                                        if i + 6 <= obj_code.len() {
                                            objcodes.push(obj_code[i..i + 6].to_string());
                                            i += 6;
                                            processed = true;
                                        }
                                    }
                                    4 => {
                                        if i + 8 <= obj_code.len() {
                                            objcodes.push(obj_code[i..i + 8].to_string());
                                            i += 8;
                                            processed = true;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        //  If nothing was processed, increment by 2 to avoid infinite loop
                        if !processed {
                            log_warning(&format!(
                                "Unknown opcode: 0x{:02X}, skipping 2 bytes",
                                byte_val
                            ));
                            i += 2;
                        }
                    } else {
                        log_warning(&format!("Invalid hex string: {}, skipping 2 bytes", s));
                        i += 2;
                    }
                }

                let parsed_obj = ObjectRecord::Text {
                    start: start_addr,
                    length,
                    objcodes,
                };
                parsed_obj_prog.push(parsed_obj);
            }
            'E' => {
                let start_addr_hex = record;
                let start_addr = u32::from_str_radix(start_addr_hex, 16).unwrap_or(0);

                let parsed_obj = ObjectRecord::End { start: start_addr };
                parsed_obj_prog.push(parsed_obj);
            }
            'M' => {
                //TODO: implement properly
                // Modification record: M + 6char address + 2char length (half-bytes) + sign(1bit) + variable
                if record.len() < 8 {
                    log_warning("Invalid modification record: too short");
                    continue;
                }

                let addr_hex = &record[0..6];
                let length_hex = &record[6..8];

                let address = u32::from_str_radix(addr_hex, 16).unwrap_or(0);
                let length = u8::from_str_radix(length_hex, 16).unwrap_or(0);

                log_info(&format!(
                    "Loaded modification record: address {:06X}, length {} half-bytes",
                    address, length
                ));

                // let parsed_obj = ObjectRecord::Modification { address, length};
                // parsed_obj_prog.push(parsed_obj);
            }
            _ => {
                log_warning(&format!("Unknown record type: {}", record_header));
            }
        }
    }
    // for items in parsed_obj_prog.iter() {
    //     println!("{:?}", items);
    // }
    parsed_obj_prog.clone()
}
