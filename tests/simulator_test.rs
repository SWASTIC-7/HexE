use hexe::predefined::common::{
    AddressFlags, Command, DisAssembledToken, Instruction, OBJECTPROGRAM, ObjectRecord, OpCode,
};
use hexe::simulator::sim::Simulator;

#[cfg(test)]
mod simulator_tests {
    use super::*;

    fn setup_simulator_with_memory() -> Simulator {
        let mut sim = Simulator::new();
        // Initialize memory with some test data
        sim.machine.memory[0x1000] = 0x05;
        sim.machine.memory[0x1001] = 0x00;
        sim.machine.memory[0x1002] = 0x00;
        sim
    }

    #[test]
    fn test_immediate_addressing() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        let token = DisAssembledToken {
            locctr: 0x1000,
            command: Command::Instruction(Instruction {
                instr: "LDA".to_string(),
                opcode: OpCode {
                    code: 0x00,
                    format: 3,
                },
            }),
            flags: Some(AddressFlags {
                n: false,
                i: true,
                x: false,
                b: false,
                p: false,
                e: false,
            }),
            address: Some(0x42),
            reg: None,
        };

        sim.execute_instruction(&token);
        assert_eq!(
            sim.machine.reg_a, 0x42,
            "Register A should contain immediate value 0x42"
        );
        assert_eq!(
            sim.machine.reg_pc, 0x1003,
            "PC should advance by 3 for format 3"
        );
    }

    #[test]
    fn test_memory_operations() {
        let mut sim = Simulator::new();
        sim.machine.memory[0x1000] = 0xFF;
        assert_eq!(sim.machine.memory[0x1000], 0xFF, "Memory should store 0xFF");

        sim.machine.memory[0x1001] = 0xAB;
        assert_eq!(sim.machine.memory[0x1001], 0xAB, "Memory should store 0xAB");
    }

    #[test]
    fn test_pc_increment() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        // Format 1 instruction
        let token = DisAssembledToken {
            locctr: 0x1000,
            command: Command::Instruction(Instruction {
                instr: "FIX".to_string(),
                opcode: OpCode {
                    code: 0xC4,
                    format: 1,
                },
            }),
            flags: None,
            address: None,
            reg: None,
        };

        sim.execute_instruction(&token);
        assert_eq!(
            sim.machine.reg_pc, 0x1001,
            "PC should increment by 1 for format 1"
        );
    }

    #[test]
    fn test_direct_addressing() {
        let mut sim = setup_simulator_with_memory();
        sim.machine.reg_pc = 0x1000;

        let token = DisAssembledToken {
            locctr: 0x1000,
            command: Command::Instruction(Instruction {
                instr: "LDA".to_string(),
                opcode: OpCode {
                    code: 0x00,
                    format: 3,
                },
            }),
            flags: Some(AddressFlags {
                n: true,
                i: true,
                x: false,
                b: false,
                p: false,
                e: false,
            }),
            address: Some(0x1000),
            reg: None,
        };

        sim.execute_instruction(&token);
        assert_eq!(sim.machine.reg_pc, 0x1003, "PC should advance by 3");
    }

    #[test]
    fn test_program_loading() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
            *program = vec![
                ObjectRecord::Header {
                    name: "TEST".to_string(),
                    start: 0x1000,
                    length: 0x100,
                },
                ObjectRecord::Text {
                    start: 0x1000,
                    length: 3,
                    objcodes: vec!["4B1000".to_string()],
                },
                ObjectRecord::End { start: 0x1000 },
            ];
        }

        let mut sim = Simulator::new();
        sim.load_program();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert_eq!(sim.program_start, 0x1000, "Program should start at 0x1000");
        assert_eq!(
            sim.machine.reg_pc, 0x1000,
            "PC should be set to program start"
        );
        assert!(
            !sim.instructions.is_empty(),
            "Instructions should be loaded"
        );
    }

    #[test]
    fn test_format4_addressing() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        let token = DisAssembledToken {
            locctr: 0x1000,
            command: Command::Instruction(Instruction {
                instr: "LDA".to_string(),
                opcode: OpCode {
                    code: 0x00,
                    format: 4,
                },
            }),
            flags: Some(AddressFlags {
                n: true,
                i: true,
                x: false,
                b: false,
                p: false,
                e: true,
            }),
            address: Some(0x10000),
            reg: None,
        };

        sim.execute_instruction(&token);
        assert_eq!(
            sim.machine.reg_pc, 0x1004,
            "PC should advance by 4 for format 4"
        );
    }

    #[test]
    fn test_breakpoint_management() {
        let mut sim = Simulator::new();

        sim.add_breakpoint(0x1000);
        assert!(
            sim.breakpoints.contains(&0x1000),
            "Breakpoint should be added"
        );

        sim.add_breakpoint(0x2000);
        assert_eq!(sim.breakpoints.len(), 2, "Should have 2 breakpoints");

        // Adding same breakpoint shouldn't duplicate
        sim.add_breakpoint(0x1000);
        assert_eq!(
            sim.breakpoints.len(),
            2,
            "Duplicate breakpoint shouldn't be added"
        );
    }

    #[test]
    fn test_simulator_reset() {
        let mut sim = Simulator::new();
        sim.machine.reg_a = 0x1234;
        sim.machine.reg_pc = 0x5678;
        sim.running = true;

        sim.reset();

        assert_eq!(sim.machine.reg_a, 0, "Register A should be reset to 0");
        assert_eq!(sim.machine.reg_pc, 0, "PC should be reset to 0");
        assert!(!sim.running, "Running flag should be false");
    }
}
