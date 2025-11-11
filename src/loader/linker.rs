use crate::error::{log_info, log_warning};
use crate::predefined::common::{
    EXTERNALDEFS, EXTERNALREFS, ExternalDefinition, ExternalReference, ObjectRecord,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ControlSection {
    pub name: String,
    pub load_address: u32,
    pub length: u32,
    pub original_start: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LinkedProgram {
    pub memory: Vec<u8>,
    pub start_address: u32,
    pub program_length: u32,
    pub control_sections: Vec<ControlSection>,
}

#[allow(dead_code)]
pub struct Linker {
    control_sections: Vec<ControlSection>,
    external_symbols: HashMap<String, u32>, // symbol_name -> absolute_address
    current_load_address: u32,
}

#[allow(dead_code)]
impl Linker {
    pub fn new(start_address: u32) -> Self {
        Self {
            control_sections: Vec::new(),
            external_symbols: HashMap::new(),
            current_load_address: start_address,
        }
    }

    pub fn link_object_programs(
        &mut self,
        object_records: &[ObjectRecord],
    ) -> Result<LinkedProgram, String> {
        log_info("=== STARTING LINKING PROCESS ===");

        // First pass: collect all control sections and external definitions
        self.collect_control_sections(object_records)?;
        self.collect_external_definitions(object_records)?;

        // Second pass: apply relocations and resolve external references
        let linked_memory = self.apply_relocations(object_records)?;

        let total_length = self.control_sections.iter().map(|cs| cs.length).sum();

        log_info(&format!(
            "=== LINKING COMPLETE: {} control sections, {} bytes ===",
            self.control_sections.len(),
            total_length
        ));

        Ok(LinkedProgram {
            memory: linked_memory,
            start_address: self
                .control_sections
                .first()
                .map(|cs| cs.load_address)
                .unwrap_or(0),
            program_length: total_length,
            control_sections: self.control_sections.clone(),
        })
    }

    fn collect_control_sections(&mut self, records: &[ObjectRecord]) -> Result<(), String> {
        for record in records {
            if let ObjectRecord::Header {
                name,
                start,
                length,
            } = record
            {
                let cs = ControlSection {
                    name: name.clone(),
                    load_address: self.current_load_address,
                    length: *length,
                    original_start: *start,
                };

                log_info(&format!(
                    "Control section '{}': original {:06X}, relocated to {:06X}, length {:06X}",
                    cs.name, cs.original_start, cs.load_address, cs.length
                ));

                self.control_sections.push(cs);
                self.current_load_address += length;
            }
        }

        if self.control_sections.is_empty() {
            return Err("No control sections found".to_string());
        }

        Ok(())
    }

    fn collect_external_definitions(&mut self, records: &[ObjectRecord]) -> Result<(), String> {
        let mut current_cs: Option<&ControlSection> = None;

        for record in records {
            match record {
                ObjectRecord::Header { name, .. } => {
                    current_cs = self.control_sections.iter().find(|cs| cs.name == *name);
                }
                ObjectRecord::Define { name, address } => {
                    if let Some(cs) = current_cs {
                        // Calculate absolute address
                        let relocation_factor = cs.load_address.wrapping_sub(cs.original_start);
                        let absolute_addr = address.wrapping_add(relocation_factor);

                        self.external_symbols.insert(name.clone(), absolute_addr);

                        log_info(&format!(
                            "External symbol '{}': original {:06X} -> absolute {:06X}",
                            name, address, absolute_addr
                        ));

                        // Update EXTERNALDEFS
                        let mut extdefs = EXTERNALDEFS.lock().unwrap();
                        extdefs.push(ExternalDefinition {
                            name: name.clone(),
                            address: absolute_addr,
                            control_section: cs.name.clone(),
                        });
                    }
                }
                _ => {}
            }
        }

        log_info(&format!(
            "Collected {} external symbols",
            self.external_symbols.len()
        ));

        Ok(())
    }

    fn apply_relocations(&mut self, records: &[ObjectRecord]) -> Result<Vec<u8>, String> {
        let total_size: usize = self
            .control_sections
            .iter()
            .map(|cs| cs.length as usize)
            .sum();
        let mut memory = vec![0u8; total_size];

        let mut current_cs: Option<&ControlSection> = None;
        let mut modifications: Vec<(u32, u8, bool, String)> = Vec::new();

        // Load text records and collect modifications
        for record in records {
            match record {
                ObjectRecord::Header { name, .. } => {
                    current_cs = self.control_sections.iter().find(|cs| cs.name == *name);
                }
                ObjectRecord::Text {
                    start, objcodes, ..
                } => {
                    if let Some(cs) = current_cs {
                        let relocation_factor = cs.load_address.wrapping_sub(cs.original_start);
                        let relocated_start = start.wrapping_add(relocation_factor);

                        log_info(&format!(
                            "Loading text at {:06X} (original {:06X})",
                            relocated_start, start
                        ));

                        let mut current_addr = relocated_start;
                        for objcode in objcodes {
                            let bytes = hex::decode(objcode)
                                .map_err(|e| format!("Hex decode error: {}", e))?;

                            for byte in bytes {
                                let offset = current_addr
                                    .wrapping_sub(self.control_sections[0].load_address)
                                    as usize;
                                if offset < memory.len() {
                                    memory[offset] = byte;
                                }
                                current_addr = current_addr.wrapping_add(1);
                            }
                        }
                    }
                }
                ObjectRecord::Modification {
                    address,
                    length,
                    sign,
                    variable,
                } => {
                    modifications.push((*address, *length, *sign, variable.clone()));
                }
                ObjectRecord::Refer { name } => {
                    // Register external reference
                    if let Some(cs) = current_cs {
                        let mut extrefs = EXTERNALREFS.lock().unwrap();
                        extrefs.push(ExternalReference {
                            name: name.clone(),
                            control_section: cs.name.clone(),
                            resolved: false,
                            resolved_address: None,
                        });
                    }
                }
                _ => {}
            }
        }

        // Apply all modifications
        self.apply_modifications(&mut memory, &modifications)?;

        Ok(memory)
    }

    fn apply_modifications(
        &self,
        memory: &mut [u8],
        modifications: &[(u32, u8, bool, String)],
    ) -> Result<(), String> {
        log_info(&format!(
            "=== APPLYING {} MODIFICATIONS ===",
            modifications.len()
        ));

        for (address, length, sign, variable) in modifications {
            let mut current_cs: Option<&ControlSection> = None;
            for cs in &self.control_sections {
                if *address >= cs.original_start && *address < cs.original_start + cs.length {
                    current_cs = Some(cs);
                    break;
                }
            }

            if let Some(cs) = current_cs {
                let relocation_factor = cs.load_address.wrapping_sub(cs.original_start);
                let absolute_addr = address.wrapping_add(relocation_factor);

                let offset =
                    absolute_addr.wrapping_sub(self.control_sections[0].load_address) as usize;

                let modification_value = if variable.is_empty() {
                    relocation_factor
                } else {
                    self.external_symbols
                        .get(variable)
                        .copied()
                        .ok_or_else(|| format!("Unresolved external symbol: {}", variable))?
                };

                let half_bytes = *length as usize;
                let bits_to_modify = half_bytes * 4;

                if offset < memory.len() {
                    let mut current_value = 0u32;
                    let bytes_to_read = bits_to_modify.div_ceil(8);

                    for i in 0..bytes_to_read.min(4) {
                        if offset + i < memory.len() {
                            current_value = (current_value << 8) | (memory[offset + i] as u32);
                        }
                    }

                    let mask = if bits_to_modify >= 32 {
                        0xFFFFFFFF
                    } else {
                        (1u32 << bits_to_modify) - 1
                    };

                    let new_value = if *sign {
                        (current_value & !mask) | ((current_value + modification_value) & mask)
                    } else {
                        (current_value & !mask)
                            | ((current_value.wrapping_sub(modification_value)) & mask)
                    };

                    for i in 0..bytes_to_read.min(4) {
                        if offset + i < memory.len() {
                            let shift = (bytes_to_read - 1 - i) * 8;
                            memory[offset + i] = ((new_value >> shift) & 0xFF) as u8;
                        }
                    }

                    log_info(&format!(
                        "  Modified {:06X}: {:08X} {} {:08X} = {:08X} (symbol: {})",
                        absolute_addr,
                        current_value,
                        if *sign { "+" } else { "-" },
                        modification_value,
                        new_value,
                        if variable.is_empty() {
                            "<relocation>"
                        } else {
                            variable
                        }
                    ));

                    if !variable.is_empty() {
                        let mut extrefs = EXTERNALREFS.lock().unwrap();
                        if let Some(extref) = extrefs.iter_mut().find(|r| r.name == *variable) {
                            extref.resolved = true;
                            extref.resolved_address = Some(modification_value);
                        }
                    }
                }
            } else {
                log_warning(&format!(
                    "Modification address {:06X} doesn't belong to any control section",
                    address
                ));
            }
        }

        Ok(())
    }

    pub fn get_relocation_factor(&self, control_section_name: &str) -> Option<i32> {
        self.control_sections
            .iter()
            .find(|cs| cs.name == control_section_name)
            .map(|cs| cs.load_address as i32 - cs.original_start as i32)
    }

    pub fn get_control_sections(&self) -> Vec<ControlSection> {
        self.control_sections.clone()
    }
}

pub fn link_and_load(
    object_programs: Vec<Vec<ObjectRecord>>,
    start_address: u32,
) -> Result<LinkedProgram, String> {
    let mut linker = Linker::new(start_address);

    let all_records: Vec<ObjectRecord> = object_programs.into_iter().flatten().collect();

    linker.link_object_programs(&all_records)
}
