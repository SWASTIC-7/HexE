use hexe::loader::loader::loader;
use hexe::predefined::common::ObjectRecord;

#[cfg(test)]
mod loader_tests {
    use super::*;

    #[test]
    fn test_header_parsing() {
        let input = "H^COPY  ^001000^000100\n\
                    E^001000";

        let result = loader(input.to_string());

        match &result[0] {
            ObjectRecord::Header {
                name,
                start,
                length,
            } => {
                assert_eq!(name.trim(), "COPY");
                assert_eq!(*start, 0x1000);
                assert_eq!(*length, 0x100);
            }
            _ => panic!("Expected header record"),
        }
    }

    #[test]
    fn test_text_record_parsing() {
        let input = "H^PROG  ^001000^000020\n\
                    T^001000^0C^1B1000^050000\n\
                    E^001000";

        let result = loader(input.to_string());

        if let ObjectRecord::Text {
            start,
            length,
            objcodes,
        } = &result[1]
        {
            assert_eq!(*start, 0x1000);
            assert_eq!(*length, 0x0C);
            assert!(!objcodes.is_empty());
        } else {
            panic!("Expected text record");
        }
    }
}
