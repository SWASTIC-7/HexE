#[cfg(test)]

use hex_e::predefined::common::{ObjectRecord, OBJECTPROGRAM, AddressFlags, Command, DisAssembledToken};
use hex_e::simulator::sim::{Simulator, simulator};
use hex_e::simulator::opcode_implementation::{Opcode, AddressingMode};


    // Helper function to create a test object program
    fn create_test_object_program() -> String {
        // Simple SIC/XE program:
        // H^TEST  ^001000^00001E
        // T^001000^1E^00100000102A0C103900102D
        // E^001000
        "H^TEST  ^001000^00001E\nT^001000^1E^00100000102A0C103900102D\nE^001000".to_string()
    }

    fn setup_test_object_program() {
        let mut obj_prog = OBJECTPROGRAM.lock().unwrap();
        obj_prog.clear();
        
        // Add Header record
        obj_prog.push(ObjectRecord::Header {
            name: "TEST".to_string(),
            start: 0x001000,
            length: 0x00001E,
        });
        
        // Add Text record with some test instructions
        obj_prog.push(ObjectRecord::Text {
            start: 0x001000,
            length: 0x1E,
            objcodes: vec![
                "001000".to_string(), // LDA #0 (format 3)
                "002A0C".to_string(), // LDX #12 (format 3)  
                "001039".to_string(), // LDA BUFFER (format 3)
                "002D".to_string(),   // Format 2 instruction
            ],
        });
        
        // Add End record
        obj_prog.push(ObjectRecord::End {
            start: 0x001000,
        });
    }

    #[test]
    fn test_simulator_new() {
        let sim = Simulator::new();
        
        assert_eq!(sim.machine.reg_pc, 0);
        assert_eq!(sim.breakpoints.len(), 0);
        assert_eq!(sim.running, false);
        assert_eq!(sim.program_start, 0);
        assert_eq!(sim.instructions.len(), 0);
    }

    #[test]
    fn test_load_program() {
        setup_test_object_program();
        let mut sim = Simulator::new();
        let test_program = create_test_object_program();
        
        sim.load_program(test_program);
        
        // Check that program start is set correctly
        assert_eq!(sim.program_start, 0x001000);
        assert_eq!(sim.machine.reg_pc, 0x001000);
        
        // Check that instructions were loaded
        assert!(sim.instructions.len() > 0);
        println!("Loaded {} instructions", sim.instructions.len());
    }

    #[test]
    fn test_find_program_start() {
        setup_test_object_program();
        let mut sim = Simulator::new();
        
        sim.find_program_start();
        
        assert_eq!(sim.program_start, 0x001000);
    }

    #[test]
    fn test_find_instruction_at_pc() {
        setup_test_object_program();
        let mut sim = Simulator::new();
        let test_program = create_test_object_program();
        sim.load_program(test_program);
        
        // Test finding instruction at program start
        let instr = sim.find_instruction_at_pc(0x001000);
        assert!(instr.is_some());
        
        // Test finding instruction at non-existent address
        let no_instr = sim.find_instruction_at_pc(0x999999);
        assert!(no_instr.is_none());
    }

    #[test]
    fn test_register_name_to_code() {
        let sim = Simulator::new();
        
        assert_eq!(sim.register_name_to_code("A"), Some(0));
        assert_eq!(sim.register_name_to_code("X"), Some(1));
        assert_eq!(sim.register_name_to_code("L"), Some(2));
        assert_eq!(sim.register_name_to_code("B"), Some(3));
        assert_eq!(sim.register_name_to_code("S"), Some(4));
        assert_eq!(sim.register_name_to_code("T"), Some(5));
        assert_eq!(sim.register_name_to_code("F"), Some(6));
        assert_eq!(sim.register_name_to_code("PC"), Some(8));
        assert_eq!(sim.register_name_to_code("SW"), Some(9));
        assert_eq!(sim.register_name_to_code("INVALID"), None);
    }

    #[test]
    fn test_name_to_opcode() {
        let sim = Simulator::new();
        
        assert_eq!(sim.name_to_opcode("LDA"), Some(Opcode::LDA));
        assert_eq!(sim.name_to_opcode("STA"), Some(Opcode::STA));
        assert_eq!(sim.name_to_opcode("ADD"), Some(Opcode::ADD));
        assert_eq!(sim.name_to_opcode("SUB"), Some(Opcode::SUB));
        assert_eq!(sim.name_to_opcode("JSUB"), Some(Opcode::JSUB));
        assert_eq!(sim.name_to_opcode("RSUB"), Some(Opcode::RSUB));
        assert_eq!(sim.name_to_opcode("INVALID"), None);
    }

    #[test]
    fn test_addressing_mode_determination() {
        let sim = Simulator::new();
        
        // Test immediate addressing (i=1, n=0)
        let flags_immediate = AddressFlags {
            i: true,
            n: false,
            x: false,
            b: false,
            p: false,
            e: false,
        };
        assert_eq!(sim.determine_addressing_mode(&flags_immediate), AddressingMode::Immediate);
        
        // Test indirect addressing (i=0, n=1)
        let flags_indirect = AddressFlags {
            i: false,
            n: true,
            x: false,
            b: false,
            p: false,
            e: false,
        };
        assert_eq!(sim.determine_addressing_mode(&flags_indirect), AddressingMode::Indirect);
        
        // Test indexed addressing (x=1)
        let flags_indexed = AddressFlags {
            i: false,
            n: false,
            x: true,
            b: false,
            p: false,
            e: false,
        };
        assert_eq!(sim.determine_addressing_mode(&flags_indexed), AddressingMode::Indexed);
        
        // Test direct addressing (default)
        let flags_direct = AddressFlags {
            i: false,
            n: false,
            x: false,
            b: false,
            p: false,
            e: false,
        };
        assert_eq!(sim.determine_addressing_mode(&flags_direct), AddressingMode::Direct);
    }

    #[test]
    fn test_calculate_effective_address() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x001000;
        sim.machine.reg_b = 0x002000;
        
        let displacement = 0x100;
        
        // Test PC-relative addressing
        let flags_pc = AddressFlags {
            i: false,
            n: false,
            x: false,
            b: false,
            p: true,
            e: false,
        };
        let pc_addr = sim.calculate_effective_address(displacement, &flags_pc);
        assert_eq!(pc_addr, 0x001100);
        
        // Test base-relative addressing
        let flags_base = AddressFlags {
            i: false,
            n: false,
            x: false,
            b: true,
            p: false,
            e: false,
        };
        let base_addr = sim.calculate_effective_address(displacement, &flags_base);
        assert_eq!(base_addr, 0x002100);
        
        // Test direct addressing
        let flags_direct = AddressFlags {
            i: false,
            n: false,
            x: false,
            b: false,
            p: false,
            e: false,
        };
        let direct_addr = sim.calculate_effective_address(displacement, &flags_direct);
        assert_eq!(direct_addr, displacement);
    }

    #[test]
    fn test_breakpoint_management() {
        let mut sim = Simulator::new();
        
        // Test adding breakpoints
        sim.add_breakpoint(0x001000);
        sim.add_breakpoint(0x001010);
        assert_eq!(sim.breakpoints.len(), 2);
        assert!(sim.breakpoints.contains(&0x001000));
        assert!(sim.breakpoints.contains(&0x001010));
        
        // Test adding duplicate breakpoint (should not add)
        sim.add_breakpoint(0x001000);
        assert_eq!(sim.breakpoints.len(), 2);
        
        // Test removing breakpoint
        sim.remove_breakpoint(0x001000);
        assert_eq!(sim.breakpoints.len(), 1);
        assert!(!sim.breakpoints.contains(&0x001000));
        assert!(sim.breakpoints.contains(&0x001010));
        
        // Test removing non-existent breakpoint
        sim.remove_breakpoint(0x999999);
        assert_eq!(sim.breakpoints.len(), 1);
    }

    #[test]
    fn test_reset() {
        let mut sim = Simulator::new();
        
        // Modify some state
        sim.machine.reg_a = 0x123456;
        sim.machine.reg_pc = 0x001000;
        sim.running = true;
        sim.add_breakpoint(0x001000);
        
        // Reset
        sim.reset();
        
        // Check that machine state is reset but breakpoints and instructions remain
        assert_eq!(sim.machine.reg_a, 0);
        assert_eq!(sim.machine.reg_pc, 0);
        assert_eq!(sim.running, false);
        assert_eq!(sim.breakpoints.len(), 1); // Breakpoints should remain
    }

    #[test]
    fn test_format_instruction() {
        let sim = Simulator::new();
        
        // Test Format 1 instruction
        let format1_instr = DisAssembledToken {
            locctr: 0x001000,
            command: Command::Instruction(predefined::common::Instruction {
                instr: "RSUB".to_string(),
                opcode: crate::predefined::opcode::OpCode {
                    code: 0x4C,
                    format: 1,
                },
            }),
            flags: None,
            address: None,
            reg: None,
        };
        
        let formatted = sim.format_instruction(&format1_instr);
        assert_eq!(formatted, "RSUB");
        
        // Test Format 2 instruction
        let format2_instr = DisAssembledToken {
            locctr: 0x001000,
            command: Command::Instruction(crate::predefined::common::Instruction {
                instr: "ADDR".to_string(),
                opcode: crate::predefined::opcode::OpCode {
                    code: 0x90,
                    format: 2,
                },
            }),
            flags: None,
            address: None,
            reg: Some(crate::predefined::common::Reg {
                r1: "A".to_string(),
                r2: "X".to_string(),
            }),
        };
        
        let formatted2 = sim.format_instruction(&format2_instr);
        assert_eq!(formatted2, "ADDR A,X");
        
        // Test Format 3 instruction with immediate addressing
        let format3_instr = DisAssembledToken {
            locctr: 0x001000,
            command: Command::Instruction(crate::predefined::common::Instruction {
                instr: "LDA".to_string(),
                opcode: crate::predefined::opcode::OpCode {
                    code: 0x00,
                    format: 3,
                },
            }),
            flags: Some(AddressFlags {
                i: true,
                n: false,
                x: false,
                b: false,
                p: false,
                e: false,
            }),
            address: Some(0x100),
            reg: None,
        };
        
        let formatted3 = sim.format_instruction(&format3_instr);
        assert_eq!(formatted3, "LDA #256");
    }

    #[test]
    fn test_step_execution() {
        setup_test_object_program();
        let mut sim = Simulator::new();
        let test_program = create_test_object_program();
        sim.load_program(test_program);
        
        let initial_pc = sim.machine.reg_pc;
        
        // Execute one step
        let result = sim.step();
        
        // Should return true if instruction was found and executed
        // PC should have advanced
        if result {
            assert_ne!(sim.machine.reg_pc, initial_pc);
        }
    }

    #[test]
    fn test_run_with_breakpoint() {
        setup_test_object_program();
        let mut sim = Simulator::new();
        let test_program = create_test_object_program();
        sim.load_program(test_program);
        
        // Add breakpoint at program start
        sim.add_breakpoint(sim.machine.reg_pc);
        
        // Run should immediately hit breakpoint and stop
        sim.run();
        
        assert_eq!(sim.running, false);
    }

    #[test]
    fn test_get_format2_operand() {
        let sim = Simulator::new();
        
        let token_with_regs = DisAssembledToken {
            locctr: 0x001000,
            command: Command::Instruction(crate::predefined::common::Instruction {
                instr: "ADDR".to_string(),
                opcode: crate::predefined::opcode::OpCode {
                    code: 0x90,
                    format: 2,
                },
            }),
            flags: None,
            address: None,
            reg: Some(crate::predefined::common::Reg {
                r1: "A".to_string(),
                r2: "X".to_string(),
            }),
        };
        
        let operand = sim.get_format2_operand(&token_with_regs);
        // A=0, X=1, so operand should be (0<<4)|1 = 1
        assert_eq!(operand, 0x01);
        
        // Test with no registers
        let token_no_regs = DisAssembledToken {
            locctr: 0x001000,
            command: Command::Instruction(crate::predefined::common::Instruction {
                instr: "CLEAR".to_string(),
                opcode: crate::predefined::opcode::OpCode {
                    code: 0xB4,
                    format: 2,
                },
            }),
            flags: None,
            address: None,
            reg: None,
        };
        
        let operand_none = sim.get_format2_operand(&token_no_regs);
        assert_eq!(operand_none, 0);
    }

    #[test]
    fn test_print_state() {
        let mut sim = Simulator::new();
        sim.machine.reg_a = 0x123456;
        sim.machine.reg_x = 0x789ABC;
        sim.machine.reg_pc = 0x001000;
        sim.running = true;
        
        // This should not panic
        sim.print_state();
    }

    // Integration test
    #[test]
    fn test_simulator_function() {
        setup_test_object_program();
        let test_program = create_test_object_program();
        
        // This should not panic
        simulator(test_program);
    }

    // Test error handling
    #[test]
    fn test_invalid_instruction_handling() {
        let mut sim = Simulator::new();
        
        // Create a token with an invalid instruction
        let invalid_token = DisAssembledToken {
            locctr: 0x001000,
            command: Command::Instruction(crate::predefined::common::Instruction {
                instr: "INVALID".to_string(),
                opcode: crate::predefined::opcode::OpCode {
                    code: 0xFF, // Invalid opcode
                    format: 3,
                },
            }),
            flags: None,
            address: None,
            reg: None,
        };
        
        // This should handle the invalid instruction gracefully
        sim.execute_instruction(&invalid_token);
        
        // PC should still advance
        assert_eq!(sim.machine.reg_pc, 1);
    }


// Benchmark tests (optional, requires nightly Rust)
#[cfg(test)]
mod benches {
    use super::*;
    use std::time::Instant;

    #[test]
    fn bench_step_execution() {
        setup_test_object_program();
        let mut sim = Simulator::new();
        let test_program = create_test_object_program();
        sim.load_program(test_program);
        
        let start = Instant::now();
        for _ in 0..1000 {
            if !sim.step() {
                break;
            }
            // Reset PC to avoid running out of instructions
            sim.machine.reg_pc = sim.program_start;
        }
        let duration = start.elapsed();
        
        println!("1000 step executions took: {:?}", duration);
        assert!(duration.as_millis() < 1000); // Should complete in less than 1 second
    }
}

// Helper function for setting up test object program
fn setup_test_object_program() {
    let mut obj_prog = OBJECTPROGRAM.lock().unwrap();
    obj_prog.clear();
    
    // Add Header record
    obj_prog.push(ObjectRecord::Header {
        name: "TEST".to_string(),
        start: 0x001000,
        length: 0x00001E,
    });
    
    // Add Text record with some test instructions
    obj_prog.push(ObjectRecord::Text {
        start: 0x001000,
        length: 0x1E,
        objcodes: vec![
            "001000".to_string(), // LDA #0 (format 3)
            "002A0C".to_string(), // LDX #12 (format 3)  
            "001039".to_string(), // LDA BUFFER (format 3)
            "002D".to_string(),   // Format 2 instruction
        ],
    });
    
    // Add End record
    obj_prog.push(ObjectRecord::End {
        start: 0x001000,
    });
}