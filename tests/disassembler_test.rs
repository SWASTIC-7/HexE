use hexe::disassembler::disassembler::disassemble;
use hexe::predefined::common::{Command, OBJECTPROGRAM, ObjectRecord};

#[cfg(test)]
mod disassembler_tests {
    use super::*;

    #[test]
    fn test_format3_instruction() {
        // Clear and set up
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

        let result = disassemble();

        // Clean up
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(!result.is_empty(), "Disassembled code should not be empty");
        assert_eq!(
            result[0].locctr, 0x1000,
            "Location counter should be 0x1000"
        );

        match &result[0].command {
            Command::Instruction(instr) => {
                assert_eq!(instr.instr, "JSUB", "Instruction should be JSUB");
                assert_eq!(instr.opcode.format, 3, "Format should be 3");
                assert_eq!(instr.opcode.code, 0x48, "Opcode should be 0x48 (masked)");
            }
            _ => panic!("Expected instruction, got directive"),
        }

        assert!(
            result[0].flags.is_some(),
            "Flags should be present for format 3"
        );
        if let Some(flags) = &result[0].flags {
            assert!(flags.n, "n flag should be set");
            assert!(flags.i, "i flag should be set");
        }
    }

    #[test]
    fn test_format4_instruction() {
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
                    length: 4,
                    objcodes: vec!["03100000".to_string()],
                },
                ObjectRecord::End { start: 0x1000 },
            ];
        }

        let result = disassemble();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(!result.is_empty(), "Disassembled code should not be empty");
        assert_eq!(
            result[0].locctr, 0x1000,
            "Location counter should be 0x1000"
        );

        match &result[0].command {
            Command::Instruction(instr) => {
                assert_eq!(instr.opcode.format, 4, "Format should be 4");
                assert_eq!(instr.instr, "LDA", "Instruction should be LDA");
            }
            _ => panic!("Expected format 4 instruction"),
        }

        assert!(result[0].flags.is_some(), "Flags should be present");
        if let Some(flags) = &result[0].flags {
            assert!(flags.e, "Extended flag should be set for format 4");
        }
    }

    #[test]
    fn test_format2_instruction() {
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
                    length: 2,
                    objcodes: vec!["B410".to_string()],
                },
                ObjectRecord::End { start: 0x1000 },
            ];
        }

        let result = disassemble();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(!result.is_empty(), "Disassembled code should not be empty");
        assert_eq!(
            result[0].locctr, 0x1000,
            "Location counter should be 0x1000"
        );

        match &result[0].command {
            Command::Instruction(instr) => {
                assert_eq!(instr.opcode.format, 2, "Format should be 2");
                assert_eq!(instr.instr, "CLEAR", "Instruction should be CLEAR");
            }
            _ => panic!("Expected format 2 instruction"),
        }

        assert!(
            result[0].reg.is_some(),
            "Register field should be present for format 2"
        );
    }

    #[test]
    fn test_multiple_instructions() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
            *program = vec![
                ObjectRecord::Header {
                    name: "MULTI".to_string(),
                    start: 0x1000,
                    length: 0x100,
                },
                ObjectRecord::Text {
                    start: 0x1000,
                    length: 6,
                    objcodes: vec!["4B1000".to_string(), "4B1003".to_string()],
                },
                ObjectRecord::End { start: 0x1000 },
            ];
        }

        let result = disassemble();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert_eq!(result.len(), 2, "Should have 2 instructions");
        assert_eq!(result[0].locctr, 0x1000, "First instruction at 0x1000");
        assert_eq!(result[1].locctr, 0x1003, "Second instruction at 0x1003");
    }
}
