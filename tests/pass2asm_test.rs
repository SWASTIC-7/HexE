use hexe::assembler::pass2asm::pass2asm;
use hexe::predefined::common::ObjectRecord;

#[cfg(test)]
mod pass2asm_tests {
    use super::*;

    #[test]
    fn test_basic_assembly() {
        let input = "COPY    START   1000\n\
                    FIRST   LDA     #5\n\
                            STA     ALPHA\n\
                    ALPHA   RESW    1\n\
                            END     FIRST";

        let result = pass2asm(input);

        // Check header record
        match &result[0] {
            ObjectRecord::Header {
                name,
                start,
                length,
            } => {
                assert_eq!(name, "COPY");
                assert_eq!(*start, 0x1000);
            }
            _ => panic!("Expected header record"),
        }
    }

    #[test]
    fn test_format4_instruction() {
        let input = "PROG    START   1000\n\
                            +LDA    #1234\n\
                            END     PROG";

        let result = pass2asm(input);

        // Check for format 4 instruction in text record
        if let ObjectRecord::Text { objcodes, .. } = &result[1] {
            assert!(objcodes.iter().any(|code| code.len() == 8)); // Format 4 instructions are 8 hex digits
        }
    }
}
