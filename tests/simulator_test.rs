use hexe::predefined::common::{AddressFlags, Command, DisAssembledToken, Instruction, OpCode};
use hexe::simulator::sim::Simulator;

#[cfg(test)]
mod simulator_tests {
    use super::*;

    #[test]
    fn test_register_operations() {
        let mut sim = Simulator::new();
        sim.machine.reg_a = 0x5;
        sim.machine.reg_x = 0x3;

        let token = DisAssembledToken {
            locctr: 0x1000,
            command: Command::Instruction(Instruction {
                instr: "ADD".to_string(),
                opcode: OpCode {
                    code: 0x18,
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
        assert_eq!(sim.machine.reg_a, 0x8);
    }

    #[test]
    fn test_immediate_addressing() {
        let mut sim = Simulator::new();
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
        assert_eq!(sim.machine.reg_a, 0x42);
    }

    #[test]
    fn test_memory_operations() {
        let mut sim = Simulator::new();
        sim.machine.memory[0x1000] = 0xFF;
        assert_eq!(sim.machine.memory[0x1000], 0xFF);
    }
}
