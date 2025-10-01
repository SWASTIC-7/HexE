use crate::predefined::common::{ObjectRecord, OBJECTPROGRAM};
use crate::predefined::opcode::{reverse_optab};

//Object program structure
//H == 3byte name ,3 byte starting addr of program,  3byte length
//T == 3byte starting add, 1 byte length, 30byte obj_program
//E == 3byte starting address of executable instructions

pub fn loader(buffer: String) -> Vec<ObjectRecord> {
    let mut parsed_obj_prog = OBJECTPROGRAM.lock().unwrap();

    for line in buffer.lines() { 
        let trimmed_line = line.trim(); 
        if trimmed_line.is_empty() {
            continue;
        }
        // Remove spaces and ^ characters from the line
        trimmed_line = trimmed_line.chars()
            .filter(|&c| c != ' ' && c != '^')
            .collect();
        let record_header: char = trimmed_line.chars().next().unwrap();
        let record: &str = &trimmed_line[1..]; 
        match record_header {
            'H' => {
                let program_name = &record[0..6];
                let start_addr_hex = &record[6..12];
                let length_hex = &record[12..18];
                
                let start_addr = u32::from_str_radix(start_addr_hex, 16).unwrap_or(0);
                let length = u32::from_str_radix(length_hex, 16).unwrap_or(0);
                
                let parsed_obj = ObjectRecord::Header {
                    name: program_name.trim().to_string(),
                    start: start_addr,
                    length: length,
                };
                parsed_obj_prog.push(parsed_obj);
            }
            'T' => {             //TODO: complete the implementation
               
                let start_addr_hex = &record[0..6];
                let length_hex = &record[6..8];
                let obj_code = &record[8..];
                
                let start_addr = u32::from_str_radix(start_addr_hex, 16).unwrap_or(0);
                let length = u8::from_str_radix(length_hex, 16).unwrap_or(0);
                
               
                let mut objcodes = Vec::new();
                let mut i=0;
                while i < obj_code.len()-1 {

                    let s = &obj_code[i..i+2];
                    
                    if let Some (format) = get_instruction_format(s){
                        if format == 1 {
                            objcodes.push(&obj_code[i..i+2]);
                            i=i+2;
                        }
                        if format == 2 {
                            objcodes.push(&obj_code[i..i+4]);
                            i=i+4;
                        }
                    }else {
                        if let Ok(byte_val) = u8::from_str_radix(s, 16) {
                        let opcode = byte_val & 0xFC; 
                        if let Some (format) = get_instruction_format(opcode){
                        if format == 3 {
                            objcodes.push(&obj_code[i..i+6]);
                            i=i+6;
                        }
                        if format == 4 {
                            objcodes.push(&obj_code[i..i+8]);
                            i=i+8;
                        }
                    }
                }
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
                
                let parsed_obj = ObjectRecord::End {
                    start: start_addr,
                };
                parsed_obj_prog.push(parsed_obj);
            }
            _ => {
                println!("Unknown record type: {}", record_header);
            }
        }
    }
    parsed_obj_prog
}