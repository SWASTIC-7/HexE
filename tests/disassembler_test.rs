use hexe::disassembler::disassembler::disassemble;
use hexe::predefined::common::{Command, Instruction, OBJECTPROGRAM, ObjectRecord};

#[cfg(test)]
mod disassembler_tests {
    use super::*;

    #[test]
    fn test_format3_instruction() {
        let obj_program = vec![
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

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            *program = obj_program;
        }

        let result = disassemble();
        assert_eq!(result[0].locctr, 0x1000);
        match &result[0].command {
            Command::Instruction(instr) => {
                assert_eq!(instr.instr, "JSUB");
                assert_eq!(instr.opcode.format, 3);
            }
            _ => panic!("Expected instruction"),
        }
    }

    // #[test]
    // fn test_format4_instruction() {
    //     let obj_program = vec![
    //         ObjectRecord::Header {
    //             name: "TEST".to_string(),
    //             start: 0x1000,
    //             length: 0x100
    //         },
    //         ObjectRecord::Text {
    //             start: 0x1000,
    //             length: 4,
    //             objcodes: vec!["050010000".to_string()]
    //         },
    //         ObjectRecord::End { start: 0x1000 }
    //     ];

    //     {
    //         let mut program = OBJECTPROGRAM.lock().unwrap();
    //         *program = obj_program;
    //     }

    //     let result = disassemble();
    //     assert!(result[0].command.is_instruction());
    //     match &result[0].command {
    //         Command::Instruction(instr) => {
    //             assert_eq!(instr.opcode.format, 4);
    //             assert_eq!(instr.instr, "LDA");
    //         },
    //         _ => panic!("Expected format 4 instruction"),
    //     }
    // }
}
