use super::pass1asm::pass1asm;
use crate::error::{log_error, log_info, log_warning};
use crate::predefined::common::{
    Command, LITERALTABLE, LabeledParsedLines, OBJECTPROGRAM, ObjectRecord, SymbolTable,
};
use crate::predefined::registers;

// pass 2 creates the object program

pub fn pass2asm(buffer: &str) -> Vec<ObjectRecord> {
    let (labeled_parsed_lines, len, start_addr, symbol_table): (
        Vec<LabeledParsedLines>,
        u32,
        u32,
        Vec<SymbolTable>,
    ) = pass1asm(buffer);
    let mut object_program = OBJECTPROGRAM.lock().unwrap();
    let literal_table = LITERALTABLE.lock().unwrap();
    let mut base_address = start_addr;
    let mut text_length = 0;
    let mut text = ObjectRecord::Text {
        start: start_addr,
        length: 0,
        objcodes: Vec::new(),
    };
    let mut modification_records: Vec<ObjectRecord> = Vec::new();

    for lines in labeled_parsed_lines.iter() {
        if text_length == 0 {
            text = ObjectRecord::Text {
                start: lines.locctr,
                length: 0,
                objcodes: Vec::new(),
            };
        }
        match &lines.parsedtoken.command {
            Command::Directive(directive) => match directive.to_uppercase().as_str() {
                "START" => {
                    let prog_name = lines.parsedtoken.label.clone();
                    object_program.push(header_record(prog_name, len, start_addr))
                }
                "BASE" => {
                    let base_operand = lines.parsedtoken.operand1.clone().unwrap();
                    if let Some(sym) = symbol_table.iter().find(|sym| sym.label == base_operand) {
                        base_address = sym.address;
                    }
                }
                "LTORG" | "END" => {
                    // Generate object code for literals at this location
                    let literals_at_this_location: Vec<_> = literal_table
                        .iter()
                        .filter(|lit| lit.address == Some(lines.locctr))
                        .collect();

                    if !literals_at_this_location.is_empty() {
                        log_info(&format!(
                            "Generating object code for {} literals at {:06X}",
                            literals_at_this_location.len(),
                            lines.locctr
                        ));

                        for lit in literals_at_this_location {
                            // Add literal data to text record
                            text_length += lit.length as u8;
                            if let ObjectRecord::Text {
                                length, objcodes, ..
                            } = &mut text
                            {
                                *length = text_length;
                                objcodes.push(lit.value.clone());
                                log_info(&format!(
                                    "  Added literal {} = {} (length: {})",
                                    lit.literal, lit.value, lit.length
                                ));
                            }

                            if text_length >= 55 {
                                text_length = 0;
                                object_program.push(text.clone());
                                text = ObjectRecord::Text {
                                    start: lines.locctr + lit.length,
                                    length: 0,
                                    objcodes: Vec::new(),
                                };
                            }
                        }
                    }

                    if directive.to_uppercase() == "END" {
                        // Add final text record if it has content
                        if text_length > 0 {
                            if let ObjectRecord::Text { length, .. } = &mut text {
                                *length = text_length;
                            }
                            object_program.push(text.clone());
                        }

                        // Add all modification records before END record
                        for mod_record in modification_records.iter() {
                            object_program.push(mod_record.clone());
                        }

                        // Add END record
                        object_program.push(ObjectRecord::End { start: start_addr });
                    }
                }
                _ => {
                    log_warning(&format!("Unknown directive: {}", directive));
                }
            },
            Command::Instruction(instr) => {
                let format = instr.opcode.format;
                let opcode = instr.opcode.code;
                let locctr = lines.locctr;
                let obj_code;
                match &format {
                    1 => {
                        obj_code = object_code1(opcode);
                        text_length += 1;
                        if let ObjectRecord::Text {
                            length, objcodes, ..
                        } = &mut text
                        {
                            *length = text_length;
                            objcodes.push(obj_code);
                        }
                        if text_length >= 55 {
                            text_length = 0;
                            object_program.push(text.clone());
                        }
                    }
                    2 => {
                        obj_code = object_code2(
                            opcode,
                            &lines.parsedtoken.operand1.clone(),
                            &lines.parsedtoken.operand2.clone(),
                        );
                        text_length += 2;
                        if let ObjectRecord::Text {
                            length, objcodes, ..
                        } = &mut text
                        {
                            *length = text_length;
                            objcodes.push(obj_code);
                        }
                        if text_length >= 55 {
                            text_length = 0;
                            object_program.push(text.clone());
                        }
                    }
                    3 => {
                        obj_code = object_code3(
                            opcode,
                            &lines.parsedtoken.operand1.clone(),
                            &lines.parsedtoken.operand2.clone(),
                            &symbol_table,
                            &literal_table,
                            locctr,
                            base_address,
                        );

                        // Check if format 3 was extended to format 4
                        let actual_length = if obj_code.len() == 8 {
                            // Extended to format 4 - add modification record
                            log_info(&format!(
                                "Format 3 at {:06X} extended to format 4, adding modification record",
                                locctr
                            ));

                            // Modification record: address + 1 (skip opcode byte), modify 5 half-bytes (20 bits)
                            modification_records.push(make_modification_record(
                                locctr,
                                &lines.parsedtoken.operand1,
                            ));
                            4
                        } else {
                            3
                        };

                        text_length += actual_length;
                        if let ObjectRecord::Text {
                            length, objcodes, ..
                        } = &mut text
                        {
                            *length = text_length;
                            objcodes.push(obj_code);
                        }
                        if text_length >= 55 {
                            text_length = 0;
                            object_program.push(text.clone());
                        }
                    }
                    4 => {
                        obj_code = object_code4(
                            opcode,
                            &lines.parsedtoken.operand1.clone(),
                            &lines.parsedtoken.operand2.clone(),
                            &symbol_table,
                            &literal_table,
                            locctr,
                        );

                        // Format 4 instruction - add modification record
                        log_info(&format!(
                            "Format 4 instruction at {:06X}, adding modification record",
                            locctr
                        ));

                        // Modification record: address + 1 (skip opcode byte), modify 5 half-bytes (20 bits)
                        modification_records.push(make_modification_record(
                            locctr,
                            &lines.parsedtoken.operand1,
                        ));

                        text_length += 4;
                        if let ObjectRecord::Text {
                            length, objcodes, ..
                        } = &mut text
                        {
                            *length = text_length;
                            objcodes.push(obj_code);
                        }
                        if text_length >= 55 {
                            text_length = 0;
                            object_program.push(text.clone());
                        }
                    }
                    _ => {
                        log_error(&format!(
                            "Invalid format {} found to make object code",
                            format
                        ));
                    }
                }
            }
        }
    }

    object_program.to_vec()
}

fn header_record(prog_name: Option<String>, len: u32, starting_addr: u32) -> ObjectRecord {
    ObjectRecord::Header {
        name: prog_name.unwrap_or_else(|| String::from("DEFAULT")),
        start: starting_addr,
        length: len,
    }
}

// fomat 1
//  1 byte == 8bit opcode

// format 2
// 2 byte == 8bit opcode + 4bit R1 + 4bit R2

//format 3
// 3 byte == 6bit opcode + 6bit flag + 12bit disp

//format 4
// 4 byte == 6bit opcode + 6bit flag + 20bit disp

// flag =
// n i x b p e  -- addressing modes to check
// indirect , immediate, indexed, base relative, pc relative, format

//object code for format 1
pub fn object_code1(opcode: u8) -> String {
    opcode.to_string()
}

//object code for fromat 3
pub fn object_code2(opcode: u8, operand1: &Option<String>, operand2: &Option<String>) -> String {
    let reg = registers::register_map();
    let r1 = operand1.as_ref().unwrap().to_uppercase();
    let r2 = operand2.as_ref().unwrap().to_uppercase();
    let r1_code = reg.get(r1.as_str()).expect("error getting r1 hexcode");
    let r2_code = reg.get(r2.as_str()).expect("error getting r2 hexcode");

    let combined_reg = (r1_code << 4) | r2_code;

    format!("{:02X}{:02X}", opcode, combined_reg) // 2digit of opcode + 2digit of (r1_code+r2_code)
}

//object code for format 3
#[warn(unused_mut)]
pub fn object_code3(
    //TODO: Add feature of literal
    opcode: u8,
    operand1: &Option<String>,
    operand2: &Option<String>,
    symbol_table: &[SymbolTable],
    literal_table: &[crate::predefined::common::LiteralTable],
    current_locctr: u32,
    base_address: u32,
) -> String {
    let mut flag_n: u8 = 0;
    let mut flag_i: u8 = 0;
    let mut flag_x: u8 = 0;
    let flag_e: u8 = 0;
    let mut flag_b: u8 = 0;
    let mut flag_p: u8 = 0;

    let _reg = registers::register_map();
    if let Some(v) = operand2 {
        if v.to_uppercase() == "X" {
            flag_x = 1;
        } else {
            log_warning(&format!("Incorrect register '{}' in indexed mode", v));
        }
    }
    if let Some(opr) = operand1 {
        let mut operand = opr.clone();

        // Check if operand is a literal
        if let Some(stripped) = opr.strip_prefix('=') {
            // It's a literal - look it up in literal table
            if let Some(lit) = literal_table.iter().find(|l| l.literal == *opr) {
                if let Some(lit_addr) = lit.address {
                    log_info(&format!(
                        "Using literal {} at address {:06X}",
                        opr, lit_addr
                    ));

                    // Use literal address for displacement calculation
                    let target_addr = lit_addr;
                    let program_counter = current_locctr + 3;
                    let mut displacement = target_addr as i32 - program_counter as i32;

                    // Simple addressing (n=1, i=1)
                    flag_n = 1;
                    flag_i = 1;

                    if (-2048..=2047).contains(&displacement) {
                        flag_p = 1;
                        let disp_12bit = (displacement & 0xFFF) as u16;
                        let first_byte: u8 = opcode | flag_n << 1 | flag_i;
                        let second_byte = (flag_x << 7)
                            | (flag_b << 6)
                            | (flag_p << 5)
                            | (flag_e << 4)
                            | ((disp_12bit >> 8) & 0x0F) as u8;
                        let third_byte = (disp_12bit & 0xFF) as u8;

                        return format!("{:02X}{:02X}{:02X}", first_byte, second_byte, third_byte);
                    } else {
                        // Try base-relative
                        displacement =
                            target_addr as i32 - base_address as i32 + program_counter as i32;
                        if (0..=4095).contains(&displacement) {
                            flag_b = 1;
                            let disp_12bit = (displacement & 0xFFF) as u16;
                            let first_byte: u8 = opcode | flag_n << 1 | flag_i;
                            let second_byte = (flag_x << 7)
                                | (flag_b << 6)
                                | (flag_p << 5)
                                | (flag_e << 4)
                                | ((disp_12bit >> 8) & 0x0F) as u8;
                            let third_byte = (disp_12bit & 0xFF) as u8;

                            return format!(
                                "{:02X}{:02X}{:02X}",
                                first_byte, second_byte, third_byte
                            );
                        } else {
                            // Extend to format 4
                            log_warning(&format!(
                                "Literal {} address {:06X} out of range for format 3, extending to format 4",
                                opr, target_addr
                            ));
                            return object_code4(
                                opcode,
                                operand1,
                                operand2,
                                symbol_table,
                                literal_table,
                                current_locctr,
                            );
                        }
                    }
                } else {
                    log_warning(&format!("Literal {} has no assigned address", opr));
                    return String::new();
                }
            } else {
                log_warning(&format!("Literal {} not found in literal table", opr));
                return String::new();
            }
        }

        // Not a literal - handle as regular operand
        if let Some(stripped) = opr.strip_prefix('#') {
            flag_i = 1;
            operand = stripped.to_string();
        } else if let Some(stripped) = opr.strip_prefix('@') {
            flag_n = 1;
            operand = stripped.to_string();
        }

        if let Some(sym) = symbol_table.iter().find(|sym| sym.label == operand) {
            let target_addr = sym.address;
            let program_counter = current_locctr + 3;
            let mut displacement = target_addr as i32 - program_counter as i32;

            if (-2048..=2047).contains(&displacement) {
                flag_p = 1;
                if flag_n == 0 && flag_i == 0 {
                    flag_i = 1;
                    flag_n = 1;
                }
                let disp_12bit = (displacement & 0xFFF) as u16;
                let first_byte: u8 = opcode | flag_n << 1 | flag_i;
                let second_byte = (flag_x << 7)
                    | (flag_b << 6)
                    | (flag_p << 5)
                    | (flag_e << 4)
                    | ((disp_12bit >> 8) & 0x0F) as u8;
                let third_byte = (disp_12bit & 0xFF) as u8;

                return format!("{:02X}{:02X}{:02X}", first_byte, second_byte, third_byte);
            } else {
                displacement = target_addr as i32 - base_address as i32 + program_counter as i32;
                if (0..=4095).contains(&displacement) {
                    flag_b = 1;
                    if flag_n == 0 && flag_i == 0 {
                        flag_i = 1;
                        flag_n = 1;
                    }
                    let disp_12bit = (displacement & 0xFFF) as u16;
                    let first_byte: u8 = opcode | flag_n << 1 | flag_i;
                    let second_byte = (flag_x << 7)
                        | (flag_b << 6)
                        | (flag_p << 5)
                        | (flag_e << 4)
                        | ((disp_12bit >> 8) & 0x0F) as u8;
                    let third_byte = (disp_12bit & 0xFF) as u8;

                    return format!("{:02X}{:02X}{:02X}", first_byte, second_byte, third_byte);
                } else {
                    // Neither PC-relative nor base-relative works - extend to format 4
                    log_warning(&format!(
                        "Address {:06X} out of range for format 3, extending to format 4",
                        target_addr
                    ));
                    //TODO: update locctr  --- chicken egg problem
                    return object_code4(
                        opcode,
                        operand1,
                        operand2,
                        symbol_table,
                        literal_table,
                        current_locctr,
                    );
                }
            }
        } else {
            log_warning(&format!(
                "Symbol '{}' not found in symbol table, treating as immediate value",
                operand
            ));
            let displacement: i32 = operand.parse().expect("not a vaid string");
            let disp_12bit = (displacement & 0xFFF) as u16;
            let first_byte: u8 = opcode | flag_n << 1 | flag_i;
            let second_byte = (flag_x << 7)
                | (flag_b << 6)
                | (flag_p << 5)
                | (flag_e << 4)
                | ((disp_12bit >> 8) & 0x0F) as u8;
            let third_byte = (disp_12bit & 0xFF) as u8;

            return format!("{:02X}{:02X}{:02X}", first_byte, second_byte, third_byte);
        }
    }
    String::new()
}

// objct code for format 4
#[warn(unused_mut)]
pub fn object_code4(
    opcode: u8,
    operand1: &Option<String>,
    operand2: &Option<String>,
    symbol_table: &[SymbolTable],
    literal_table: &[crate::predefined::common::LiteralTable],
    _current_locctr: u32,
) -> String {
    let mut flag_n: u8 = 0;
    let mut flag_i: u8 = 0;
    let mut flag_x: u8 = 0;
    let flag_e: u8 = 1;
    let flag_b: u8 = 0;
    let flag_p: u8 = 0;

    let _reg = registers::register_map();
    if let Some(v) = operand2 {
        if v.to_uppercase() == "X" {
            flag_x = 1;
        } else {
            log_warning(&format!("Incorrect register '{}' in indexed mode", v));
        }
    }
    if let Some(opr) = operand1 {
        let mut operand = opr.clone();

        // Check if operand is a literal
        if let Some(stripped) = opr.strip_prefix('=') {
            if let Some(lit) = literal_table.iter().find(|l| l.literal == *opr) {
                if let Some(lit_addr) = lit.address {
                    log_info(&format!(
                        "Using literal {} at address {:06X} in format 4",
                        opr, lit_addr
                    ));

                    let addr_20bit = lit_addr & 0xFFFFF;

                    // Simple addressing (n=1, i=1)
                    flag_n = 1;
                    flag_i = 1;

                    let first_byte = opcode | (flag_n << 1) | flag_i;
                    let second_byte = (flag_x << 7)
                        | (flag_b << 6)
                        | (flag_p << 5)
                        | (flag_e << 4)
                        | ((addr_20bit >> 16) & 0x0F) as u8;
                    let third_byte = ((addr_20bit >> 8) & 0xFF) as u8;
                    let fourth_byte = (addr_20bit & 0xFF) as u8;

                    return format!(
                        "{:02X}{:02X}{:02X}{:02X}",
                        first_byte, second_byte, third_byte, fourth_byte
                    );
                } else {
                    log_warning(&format!("Literal {} has no assigned address", opr));
                    return String::new();
                }
            } else {
                log_warning(&format!("Literal {} not found in literal table", opr));
                return String::new();
            }
        }

        // Handle regular operands
        if let Some(stripped) = opr.strip_prefix('#') {
            flag_i = 1;
            operand = stripped.to_string();
        } else if let Some(stripped) = opr.strip_prefix('@') {
            flag_n = 1;
            operand = stripped.to_string();
        }

        if let Some(sym) = symbol_table.iter().find(|sym| sym.label == operand) {
            let target_addr = sym.address;

            let addr_20bit = target_addr & 0xFFFFF; // 20-bit address

            if flag_n == 0 && flag_i == 0 {
                flag_i = 1;
                flag_n = 1;
            }

            let first_byte = opcode | (flag_n << 1) | flag_i;

            let second_byte = (flag_x << 7)
                | (flag_b << 6)
                | (flag_p << 5)
                | (flag_e << 4)
                | ((addr_20bit >> 16) & 0x0F) as u8;

            let third_byte = ((addr_20bit >> 8) & 0xFF) as u8;

            let fourth_byte = (addr_20bit & 0xFF) as u8;

            return format!(
                "{:02X}{:02X}{:02X}{:02X}",
                first_byte, second_byte, third_byte, fourth_byte
            );
        }
    }
    String::new()
}

pub fn make_modification_record(current_locctr: u32, operand1: &Option<String>) -> ObjectRecord {
    return ObjectRecord::Modification {
        address: current_locctr + 1,
        length: 5,
        sign: true,
        variable: operand1.clone().unwrap_or_default(),
    };
}
//TODO : add the feature of Literals support
