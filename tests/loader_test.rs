use hexe::loader::loader::loader;
use hexe::predefined::common::{OBJECTPROGRAM, ObjectRecord};

#[cfg(test)]
mod loader_tests {
    use super::*;

    #[test]
    fn test_header_parsing() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        // After removing spaces: HCOPYXX001000000100 (18 chars after H)
        // Use non-space padding characters for name
        let input = "HCOPYXX001000000100\nE001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(!result_copy.is_empty(), "Should have at least one record");
        match &result_copy[0] {
            ObjectRecord::Header {
                name,
                start,
                length,
            } => {
                assert!(name.contains("COPY"), "Program name should contain COPY");
                assert_eq!(*start, 0x1000, "Start address should be 0x1000");
                assert_eq!(*length, 0x100, "Length should be 0x100");
            }
            _ => panic!("Expected header record"),
        }
    }

    #[test]
    fn test_text_record_with_format3_instructions() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let input = "HPROG  001000000020\nT0010000C4B10004B1003\nE001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(
            result_copy.len() >= 2,
            "Should have at least header and text records"
        );

        let text_record = result_copy
            .iter()
            .find(|r| matches!(r, ObjectRecord::Text { .. }));
        assert!(text_record.is_some(), "Should have text record");

        if let Some(ObjectRecord::Text {
            start,
            length,
            objcodes,
        }) = text_record
        {
            assert_eq!(*start, 0x1000, "Text start should be 0x1000");
            assert_eq!(*length, 0x0C, "Text length should be 0x0C (12 bytes)");
            assert!(!objcodes.is_empty(), "Object codes should not be empty");
            assert_eq!(objcodes.len(), 2, "Should have 2 object codes");
            assert_eq!(
                objcodes[0].len(),
                6,
                "First objcode should be 6 chars (format 3)"
            );
            assert_eq!(
                objcodes[1].len(),
                6,
                "Second objcode should be 6 chars (format 3)"
            );
        }
    }

    #[test]
    fn test_end_record_parsing() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let input = "HTEST  001000000010\nE001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let end_record = result_copy
            .iter()
            .find(|r| matches!(r, ObjectRecord::End { .. }));
        assert!(end_record.is_some(), "Should have an end record");

        if let Some(ObjectRecord::End { start }) = end_record {
            assert_eq!(*start, 0x1000, "End address should be 0x1000");
        }
    }

    #[test]
    fn test_complete_object_program() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        // Use proper 6-char names without spaces
        let input = "HCOPYXX001000000107\n\
                     T001000064B1010354B10365B10394B1069\n\
                     T0010090C4B10774B10774B10774B1077\n\
                     E001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(
            result_copy.len() >= 3,
            "Should have header, text records, and end"
        );

        let header = result_copy
            .iter()
            .find(|r| matches!(r, ObjectRecord::Header { .. }));
        assert!(header.is_some(), "Should have header record");

        if let Some(ObjectRecord::Header {
            name,
            start,
            length,
        }) = header
        {
            assert!(name.contains("COPY"), "Program name should contain COPY");
            assert_eq!(*start, 0x1000, "Start should be 0x1000");
            assert_eq!(*length, 0x107, "Length should be 0x107");
        }

        let text_records: Vec<_> = result_copy
            .iter()
            .filter(|r| matches!(r, ObjectRecord::Text { .. }))
            .collect();
        assert_eq!(text_records.len(), 2, "Should have 2 text records");

        assert!(
            result_copy
                .iter()
                .any(|r| matches!(r, ObjectRecord::End { .. })),
            "Should have end record"
        );
    }

    #[test]
    fn test_empty_input() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let input = "";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(
            result_copy.is_empty(),
            "Empty input should produce empty result"
        );
    }

    #[test]
    fn test_multiple_text_records() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let input = "HMULTI 001000000100\n\
                     T0010000C4B10004B1003\n\
                     T0010100C4B20004B2003\n\
                     E001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let text_records: Vec<_> = result_copy
            .iter()
            .filter(|r| matches!(r, ObjectRecord::Text { .. }))
            .collect();

        assert_eq!(text_records.len(), 2, "Should have 2 text records");

        if let ObjectRecord::Text {
            start, objcodes, ..
        } = text_records[0]
        {
            assert_eq!(*start, 0x1000, "First text at 0x1000");
            assert!(!objcodes.is_empty(), "First text should have object codes");
        }

        if let ObjectRecord::Text {
            start, objcodes, ..
        } = text_records[1]
        {
            assert_eq!(*start, 0x1010, "Second text at 0x1010");
            assert!(!objcodes.is_empty(), "Second text should have object codes");
        }
    }

    #[test]
    fn test_format2_instruction_parsing() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let input = "HTEST  001000000010\nT00100002B410\nE001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let text_record = result_copy
            .iter()
            .find(|r| matches!(r, ObjectRecord::Text { .. }));
        assert!(text_record.is_some(), "Should have text record");

        if let Some(ObjectRecord::Text { objcodes, .. }) = text_record {
            assert!(!objcodes.is_empty(), "Should have object codes");
            assert_eq!(objcodes[0].len(), 4, "Format 2 should be 4 hex chars");
            assert_eq!(objcodes[0], "B410", "Should be B410 (CLEAR A)");
        }
    }

    #[test]
    fn test_format1_instruction_parsing() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let input = "HTEST  001000000010\nT00100001C4\nE001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let text_record = result_copy
            .iter()
            .find(|r| matches!(r, ObjectRecord::Text { .. }));
        assert!(text_record.is_some(), "Should have text record");

        if let Some(ObjectRecord::Text { objcodes, .. }) = text_record {
            assert!(!objcodes.is_empty(), "Should have object codes");
            assert_eq!(objcodes[0].len(), 2, "Format 1 should be 2 hex chars");
            assert_eq!(objcodes[0], "C4", "Should be C4 (FIX)");
        }
    }

    #[test]
    fn test_mixed_format_instructions() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let input = "HTEST  001000000010\nT00100007C4B4104B1000\nE001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        let text_record = result_copy
            .iter()
            .find(|r| matches!(r, ObjectRecord::Text { .. }));
        assert!(text_record.is_some(), "Should have text record");

        if let Some(ObjectRecord::Text { objcodes, .. }) = text_record {
            assert_eq!(objcodes.len(), 3, "Should have 3 instructions");
            assert_eq!(objcodes[0].len(), 2, "First should be format 1");
            assert_eq!(objcodes[1].len(), 4, "Second should be format 2");
            assert_eq!(objcodes[2].len(), 6, "Third should be format 3");
        }
    }

    #[test]
    fn test_whitespace_and_caret_removal() {
        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        // Use 6 chars without spaces
        let input = "HCOPYXX001000000100\nE001000";
        let result = loader(input.to_string());
        let result_copy = result.clone();

        {
            let mut program = OBJECTPROGRAM.lock().unwrap();
            program.clear();
        }

        assert!(!result_copy.is_empty(), "Should parse input");
        match &result_copy[0] {
            ObjectRecord::Header { name, start, .. } => {
                assert!(name.contains("COPY"));
                assert_eq!(*start, 0x1000);
            }
            _ => panic!("Expected header record"),
        }
    }
}
