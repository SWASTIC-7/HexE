use hexe::predefined::common::{AddressFlags, Command, DisAssembledToken, Instruction, OpCode};
use hexe::simulator::sim::Simulator;

#[cfg(test)]
mod jump_instruction_tests {
    use super::*;

    fn create_jump_token(
        locctr: u32,
        instr_name: &str,
        opcode: u8,
        target_address: u32,
    ) -> DisAssembledToken {
        DisAssembledToken {
            locctr,
            command: Command::Instruction(Instruction {
                instr: instr_name.to_string(),
                opcode: OpCode {
                    code: opcode,
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
            address: Some(target_address),
            reg: None,
            modification: None,
        }
    }

    fn create_comp_token(locctr: u32, value: u32) -> DisAssembledToken {
        DisAssembledToken {
            locctr,
            command: Command::Instruction(Instruction {
                instr: "COMP".to_string(),
                opcode: OpCode {
                    code: 0x28, // COMP opcode
                    format: 3,
                },
            }),
            flags: Some(AddressFlags {
                n: false,
                i: true, // Immediate addressing
                x: false,
                b: false,
                p: false,
                e: false,
            }),
            address: Some(value),
            reg: None,
            modification: None,
        }
    }

    #[test]
    fn test_unconditional_jump() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        // J instruction - unconditional jump to 0x2000
        let token = create_jump_token(0x1000, "J", 0x3C, 0x2000);

        sim.execute_instruction(&token);

        assert_eq!(
            sim.machine.reg_pc, 0x2000,
            "PC should jump to 0x2000 unconditionally"
        );
    }

    #[test]
    fn test_jeq_when_equal() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x42;

        // COMP with immediate value 0x42 (should set CC to 0 - equal)
        let comp_token = create_comp_token(0x1000, 0x42);
        sim.execute_instruction(&comp_token);

        assert_eq!(sim.machine.cc, 0, "CC should be 0 (equal)");
        assert_eq!(sim.machine.reg_pc, 0x1003, "PC should advance after COMP");

        // JEQ instruction - should jump since CC == 0
        let jeq_token = create_jump_token(0x1003, "JEQ", 0x30, 0x2000);
        sim.execute_instruction(&jeq_token);

        assert_eq!(
            sim.machine.reg_pc, 0x2000,
            "PC should jump to 0x2000 when CC == 0"
        );
    }

    #[test]
    fn test_jeq_when_not_equal() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x42;

        // COMP with immediate value 0x50 (should set CC to -1 - less than)
        let comp_token = create_comp_token(0x1000, 0x50);
        sim.execute_instruction(&comp_token);

        assert_eq!(sim.machine.cc, -1, "CC should be -1 (less than)");
        assert_eq!(sim.machine.reg_pc, 0x1003, "PC should advance after COMP");

        // JEQ instruction - should NOT jump since CC != 0
        let jeq_token = create_jump_token(0x1003, "JEQ", 0x30, 0x2000);
        sim.execute_instruction(&jeq_token);

        assert_eq!(
            sim.machine.reg_pc, 0x1006,
            "PC should continue to next instruction when CC != 0"
        );
    }

    #[test]
    fn test_jgt_when_greater() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x50;

        // COMP with immediate value 0x42 (should set CC to 1 - greater than)
        let comp_token = create_comp_token(0x1000, 0x42);
        sim.execute_instruction(&comp_token);

        assert_eq!(sim.machine.cc, 1, "CC should be 1 (greater than)");
        assert_eq!(sim.machine.reg_pc, 0x1003, "PC should advance after COMP");

        // JGT instruction - should jump since CC > 0
        let jgt_token = create_jump_token(0x1003, "JGT", 0x34, 0x2000);
        sim.execute_instruction(&jgt_token);

        assert_eq!(
            sim.machine.reg_pc, 0x2000,
            "PC should jump to 0x2000 when CC > 0"
        );
    }

    #[test]
    fn test_jgt_when_not_greater() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x42;

        // COMP with immediate value 0x42 (should set CC to 0 - equal)
        let comp_token = create_comp_token(0x1000, 0x42);
        sim.execute_instruction(&comp_token);

        assert_eq!(sim.machine.cc, 0, "CC should be 0 (equal)");
        assert_eq!(sim.machine.reg_pc, 0x1003, "PC should advance after COMP");

        // JGT instruction - should NOT jump since CC == 0
        let jgt_token = create_jump_token(0x1003, "JGT", 0x34, 0x2000);
        sim.execute_instruction(&jgt_token);

        assert_eq!(
            sim.machine.reg_pc, 0x1006,
            "PC should continue to next instruction when CC <= 0"
        );
    }

    #[test]
    fn test_jlt_when_less() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x30;

        // COMP with immediate value 0x42 (should set CC to -1 - less than)
        let comp_token = create_comp_token(0x1000, 0x42);
        sim.execute_instruction(&comp_token);

        assert_eq!(sim.machine.cc, -1, "CC should be -1 (less than)");
        assert_eq!(sim.machine.reg_pc, 0x1003, "PC should advance after COMP");

        // JLT instruction - should jump since CC < 0
        let jlt_token = create_jump_token(0x1003, "JLT", 0x38, 0x2000);
        sim.execute_instruction(&jlt_token);

        assert_eq!(
            sim.machine.reg_pc, 0x2000,
            "PC should jump to 0x2000 when CC < 0"
        );
    }

    #[test]
    fn test_jlt_when_not_less() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x50;

        // COMP with immediate value 0x42 (should set CC to 1 - greater than)
        let comp_token = create_comp_token(0x1000, 0x42);
        sim.execute_instruction(&comp_token);

        assert_eq!(sim.machine.cc, 1, "CC should be 1 (greater than)");
        assert_eq!(sim.machine.reg_pc, 0x1003, "PC should advance after COMP");

        // JLT instruction - should NOT jump since CC >= 0
        let jlt_token = create_jump_token(0x1003, "JLT", 0x38, 0x2000);
        sim.execute_instruction(&jlt_token);

        assert_eq!(
            sim.machine.reg_pc, 0x1006,
            "PC should continue to next instruction when CC >= 0"
        );
    }

    #[test]
    fn test_jsub_and_rsub() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        // JSUB instruction - jump to subroutine at 0x3000
        let jsub_token = create_jump_token(0x1000, "JSUB", 0x48, 0x3000);
        sim.execute_instruction(&jsub_token);

        assert_eq!(
            sim.machine.reg_pc, 0x3000,
            "PC should jump to subroutine at 0x3000"
        );
        assert_eq!(
            sim.machine.reg_l, 0x1000,
            "L register should store return address (original PC)"
        );

        // RSUB instruction - return from subroutine
        let rsub_token = DisAssembledToken {
            locctr: 0x3000,
            command: Command::Instruction(Instruction {
                instr: "RSUB".to_string(),
                opcode: OpCode {
                    code: 0x4C,
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
            address: None,
            reg: None,
            modification: None,
        };
        sim.execute_instruction(&rsub_token);

        assert_eq!(
            sim.machine.reg_pc, 0x1000,
            "PC should return to original address"
        );
    }

    #[test]
    fn test_jump_sequence() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x10;

        // Step 1: COMP A with 0x20 (A < 0x20, so CC = -1)
        let comp_token = create_comp_token(0x1000, 0x20);
        sim.execute_instruction(&comp_token);
        assert_eq!(sim.machine.cc, -1);
        assert_eq!(sim.machine.reg_pc, 0x1003);

        // Step 2: JLT should jump to 0x2000
        let jlt_token = create_jump_token(0x1003, "JLT", 0x38, 0x2000);
        sim.execute_instruction(&jlt_token);
        assert_eq!(sim.machine.reg_pc, 0x2000);

        // Step 3: Load A with 0x20
        sim.machine.reg_a = 0x20;

        // Step 4: COMP A with 0x20 (A == 0x20, so CC = 0)
        let comp_token2 = create_comp_token(0x2000, 0x20);
        sim.execute_instruction(&comp_token2);
        assert_eq!(sim.machine.cc, 0);
        assert_eq!(sim.machine.reg_pc, 0x2003);

        // Step 5: JEQ should jump to 0x3000
        let jeq_token = create_jump_token(0x2003, "JEQ", 0x30, 0x3000);
        sim.execute_instruction(&jeq_token);
        assert_eq!(sim.machine.reg_pc, 0x3000);

        // Step 6: Load A with 0x30
        sim.machine.reg_a = 0x30;

        // Step 7: COMP A with 0x20 (A > 0x20, so CC = 1)
        let comp_token3 = create_comp_token(0x3000, 0x20);
        sim.execute_instruction(&comp_token3);
        assert_eq!(sim.machine.cc, 1);
        assert_eq!(sim.machine.reg_pc, 0x3003);

        // Step 8: JGT should jump to 0x4000
        let jgt_token = create_jump_token(0x3003, "JGT", 0x34, 0x4000);
        sim.execute_instruction(&jgt_token);
        assert_eq!(sim.machine.reg_pc, 0x4000);
    }

    #[test]
    fn test_format4_jump() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        // Format 4 J instruction - unconditional jump with extended addressing
        let token = DisAssembledToken {
            locctr: 0x1000,
            command: Command::Instruction(Instruction {
                instr: "J".to_string(),
                opcode: OpCode {
                    code: 0x3C,
                    format: 4,
                },
            }),
            flags: Some(AddressFlags {
                n: true,
                i: true,
                x: false,
                b: false,
                p: false,
                e: true, // Extended format
            }),
            address: Some(0x10000), // Address beyond 16-bit range
            reg: None,
            modification: None,
        };

        sim.execute_instruction(&token);

        assert_eq!(
            sim.machine.reg_pc, 0x10000,
            "PC should jump to 0x10000 with format 4"
        );
    }

    #[test]
    fn test_comp_sets_correct_condition_codes() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        // Test CC = -1 (less than)
        sim.machine.reg_a = 0x10;
        let comp_lt = create_comp_token(0x1000, 0x20);
        sim.execute_instruction(&comp_lt);
        assert_eq!(sim.machine.cc, -1, "CC should be -1 when A < value");

        // Test CC = 0 (equal)
        sim.machine.reg_pc = 0x1003;
        sim.machine.reg_a = 0x42;
        let comp_eq = create_comp_token(0x1003, 0x42);
        sim.execute_instruction(&comp_eq);
        assert_eq!(sim.machine.cc, 0, "CC should be 0 when A == value");

        // Test CC = 1 (greater than)
        sim.machine.reg_pc = 0x1006;
        sim.machine.reg_a = 0x50;
        let comp_gt = create_comp_token(0x1006, 0x20);
        sim.execute_instruction(&comp_gt);
        assert_eq!(sim.machine.cc, 1, "CC should be 1 when A > value");
    }

    #[test]
    fn test_multiple_conditional_jumps_in_sequence() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;
        sim.machine.reg_a = 0x42;

        // COMP A with 0x42 -> CC = 0
        let comp = create_comp_token(0x1000, 0x42);
        sim.execute_instruction(&comp);
        assert_eq!(sim.machine.cc, 0);

        // JEQ should take branch
        let jeq = create_jump_token(0x1003, "JEQ", 0x30, 0x2000);
        sim.execute_instruction(&jeq);
        assert_eq!(sim.machine.reg_pc, 0x2000, "JEQ should jump");

        // JGT should NOT take branch (CC is still 0)
        let jgt = create_jump_token(0x2000, "JGT", 0x34, 0x3000);
        sim.execute_instruction(&jgt);
        assert_eq!(sim.machine.reg_pc, 0x2003, "JGT should not jump");

        // JLT should NOT take branch (CC is still 0)
        let jlt = create_jump_token(0x2003, "JLT", 0x38, 0x3000);
        sim.execute_instruction(&jlt);
        assert_eq!(sim.machine.reg_pc, 0x2006, "JLT should not jump");
    }

    #[test]
    fn test_nested_subroutine_calls() {
        let mut sim = Simulator::new();
        sim.machine.reg_pc = 0x1000;

        // First JSUB - call subroutine at 0x2000
        let jsub1 = create_jump_token(0x1000, "JSUB", 0x48, 0x2000);
        sim.execute_instruction(&jsub1);
        assert_eq!(sim.machine.reg_pc, 0x2000);
        assert_eq!(sim.machine.reg_l, 0x1000);

        let saved_return = sim.machine.reg_l;

        // Second JSUB - call nested subroutine at 0x3000
        let jsub2 = create_jump_token(0x2000, "JSUB", 0x48, 0x3000);
        sim.execute_instruction(&jsub2);
        assert_eq!(sim.machine.reg_pc, 0x3000);
        assert_eq!(sim.machine.reg_l, 0x2000);

        // RSUB from nested subroutine
        let rsub1 = DisAssembledToken {
            locctr: 0x3000,
            command: Command::Instruction(Instruction {
                instr: "RSUB".to_string(),
                opcode: OpCode {
                    code: 0x4C,
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
            address: None,
            reg: None,
            modification: None,
        };
        sim.execute_instruction(&rsub1);
        assert_eq!(sim.machine.reg_pc, 0x2000);

        // Restore L register for first return
        sim.machine.reg_l = saved_return;

        // RSUB from first subroutine
        let rsub2 = DisAssembledToken {
            locctr: 0x2000,
            command: Command::Instruction(Instruction {
                instr: "RSUB".to_string(),
                opcode: OpCode {
                    code: 0x4C,
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
            address: None,
            reg: None,
            modification: None,
        };
        sim.execute_instruction(&rsub2);
        assert_eq!(sim.machine.reg_pc, 0x1000);
    }
}
