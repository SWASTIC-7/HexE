use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct OpCode {
    code: u8,   // opcode value
    format: u8, // instruction format (1, 2, 3/4)
}

pub fn build_optab() -> HashMap<&'static str, OpCode> {
    let mut table = HashMap::new();

    table.insert(
        "ADD",
        OpCode {
            code: 0x18,
            format: 3,
        },
    );
    table.insert(
        "ADDF",
        OpCode {
            code: 0x58,
            format: 3,
        },
    );
    table.insert(
        "+ADD",
        OpCode {
            code: 0x18,
            format: 4,
        },
    );
    table.insert(
        "+ADDF",
        OpCode {
            code: 0x58,
            format: 4,
        },
    );
    table.insert(
        "ADDR",
        OpCode {
            code: 0x90,
            format: 2,
        },
    );
    table.insert(
        "AND",
        OpCode {
            code: 0x40,
            format: 3,
        },
    );
    table.insert(
        "+AND",
        OpCode {
            code: 0x40,
            format: 4,
        },
    );
    table.insert(
        "CLEAR",
        OpCode {
            code: 0xB4,
            format: 2,
        },
    );
    table.insert(
        "AND",
        OpCode {
            code: 0x40,
            format: 3,
        },
    );
    table.insert(
        "COMPR",
        OpCode {
            code: 0xA0,
            format: 2,
        },
    );
    table.insert(
        "COMP",
        OpCode {
            code: 0x28,
            format: 3,
        },
    );
    table.insert(
        "COMPF",
        OpCode {
            code: 0x88,
            format: 3,
        },
    );
    table.insert(
        "+COMP",
        OpCode {
            code: 0x28,
            format: 4,
        },
    );
    table.insert(
        "+COMPF",
        OpCode {
            code: 0x88,
            format: 4,
        },
    );
    table.insert(
        "DIV",
        OpCode {
            code: 0x24,
            format: 3,
        },
    );
    table.insert(
        "DIVF",
        OpCode {
            code: 0x64,
            format: 3,
        },
    );
    table.insert(
        "+DIV",
        OpCode {
            code: 0x24,
            format: 4,
        },
    );
    table.insert(
        "+DIVF",
        OpCode {
            code: 0x64,
            format: 4,
        },
    );
    table.insert(
        "DIVR",
        OpCode {
            code: 0x9C,
            format: 2,
        },
    );
    table.insert(
        "FIX",
        OpCode {
            code: 0xC4,
            format: 1,
        },
    );
    table.insert(
        "FLOAT",
        OpCode {
            code: 0xC0,
            format: 1,
        },
    );
    table.insert(
        "HIO",
        OpCode {
            code: 0xF4,
            format: 1,
        },
    );
    table.insert(
        "J",
        OpCode {
            code: 0x3C,
            format: 3,
        },
    );
    table.insert(
        "JEQ",
        OpCode {
            code: 0x30,
            format: 3,
        },
    );
    table.insert(
        "JGT",
        OpCode {
            code: 0x34,
            format: 3,
        },
    );
    table.insert(
        "JLT",
        OpCode {
            code: 0x38,
            format: 3,
        },
    );
    table.insert(
        "JSUB",
        OpCode {
            code: 0x48,
            format: 3,
        },
    );
    table.insert(
        "LDA",
        OpCode {
            code: 0x00,
            format: 3,
        },
    );
    table.insert(
        "LDB",
        OpCode {
            code: 0x68,
            format: 3,
        },
    );
    table.insert(
        "LDCH",
        OpCode {
            code: 0x50,
            format: 3,
        },
    );
    table.insert(
        "LDF",
        OpCode {
            code: 0x70,
            format: 3,
        },
    );
    table.insert(
        "LDL",
        OpCode {
            code: 0x08,
            format: 3,
        },
    );
    table.insert(
        "LDS",
        OpCode {
            code: 0x6C,
            format: 3,
        },
    );
    table.insert(
        "LDT",
        OpCode {
            code: 0x74,
            format: 3,
        },
    );
    table.insert(
        "LDX",
        OpCode {
            code: 0x04,
            format: 3,
        },
    );
    table.insert(
        "LPS",
        OpCode {
            code: 0xD0,
            format: 3,
        },
    );
    table.insert(
        "MUL",
        OpCode {
            code: 0x20,
            format: 3,
        },
    );
    table.insert(
        "MULF",
        OpCode {
            code: 0x60,
            format: 3,
        },
    );
    table.insert(
        "+J",
        OpCode {
            code: 0x3C,
            format: 4,
        },
    );
    table.insert(
        "+JEQ",
        OpCode {
            code: 0x30,
            format: 4,
        },
    );
    table.insert(
        "+JGT",
        OpCode {
            code: 0x34,
            format: 4,
        },
    );
    table.insert(
        "+JLT",
        OpCode {
            code: 0x38,
            format: 4,
        },
    );
    table.insert(
        "+JSUB",
        OpCode {
            code: 0x48,
            format: 4,
        },
    );
    table.insert(
        "+LDA",
        OpCode {
            code: 0x00,
            format: 4,
        },
    );
    table.insert(
        "+LDB",
        OpCode {
            code: 0x68,
            format: 4,
        },
    );
    table.insert(
        "+LDCH",
        OpCode {
            code: 0x50,
            format: 4,
        },
    );
    table.insert(
        "+LDF",
        OpCode {
            code: 0x70,
            format: 4,
        },
    );
    table.insert(
        "+LDL",
        OpCode {
            code: 0x08,
            format: 4,
        },
    );
    table.insert(
        "+LDS",
        OpCode {
            code: 0x6C,
            format: 4,
        },
    );
    table.insert(
        "+LDT",
        OpCode {
            code: 0x74,
            format: 4,
        },
    );
    table.insert(
        "+LDX",
        OpCode {
            code: 0x04,
            format: 4,
        },
    );
    table.insert(
        "+LPS",
        OpCode {
            code: 0xD0,
            format: 4,
        },
    );
    table.insert(
        "+MUL",
        OpCode {
            code: 0x20,
            format: 4,
        },
    );
    table.insert(
        "+MULF",
        OpCode {
            code: 0x60,
            format: 4,
        },
    );
    table.insert(
        "MULR",
        OpCode {
            code: 0x98,
            format: 2,
        },
    );
    table.insert(
        "NORM",
        OpCode {
            code: 0xC8,
            format: 1,
        },
    );
    table.insert(
        "OR",
        OpCode {
            code: 0x44,
            format: 3,
        },
    );
    table.insert(
        "RD",
        OpCode {
            code: 0xD8,
            format: 3,
        },
    );
    table.insert(
        "+OR",
        OpCode {
            code: 0x44,
            format: 4,
        },
    );
    table.insert(
        "+RD",
        OpCode {
            code: 0xD8,
            format: 4,
        },
    );
    table.insert(
        "RMO",
        OpCode {
            code: 0xAC,
            format: 2,
        },
    );
    table.insert(
        "RSUB",
        OpCode {
            code: 0x4C,
            format: 3,
        },
    );
    table.insert(
        "+RSUB",
        OpCode {
            code: 0x4C,
            format: 4,
        },
    );
    table.insert(
        "SHIFTL",
        OpCode {
            code: 0xA4,
            format: 2,
        },
    );
    table.insert(
        "SHIFTR",
        OpCode {
            code: 0xA8,
            format: 2,
        },
    );
    table.insert(
        "SIO",
        OpCode {
            code: 0xF0,
            format: 1,
        },
    );
    table.insert(
        "SSK",
        OpCode {
            code: 0xEC,
            format: 3,
        },
    );
    table.insert(
        "STA",
        OpCode {
            code: 0x0C,
            format: 3,
        },
    );
    table.insert(
        "STB",
        OpCode {
            code: 0x78,
            format: 3,
        },
    );
    table.insert(
        "STCH",
        OpCode {
            code: 0x54,
            format: 3,
        },
    );
    table.insert(
        "STF",
        OpCode {
            code: 0x80,
            format: 3,
        },
    );
    table.insert(
        "STI",
        OpCode {
            code: 0xD4,
            format: 3,
        },
    );
    table.insert(
        "STL",
        OpCode {
            code: 0x14,
            format: 3,
        },
    );
    table.insert(
        "STS",
        OpCode {
            code: 0x7C,
            format: 3,
        },
    );
    table.insert(
        "STSW",
        OpCode {
            code: 0xE8,
            format: 3,
        },
    );
    table.insert(
        "STT",
        OpCode {
            code: 0x84,
            format: 3,
        },
    );
    table.insert(
        "STX",
        OpCode {
            code: 0x10,
            format: 3,
        },
    );
    table.insert(
        "SUB",
        OpCode {
            code: 0x1C,
            format: 3,
        },
    );
    table.insert(
        "SUBF",
        OpCode {
            code: 0x5C,
            format: 3,
        },
    );
    table.insert(
        "+SSK",
        OpCode {
            code: 0xEC,
            format: 4,
        },
    );
    table.insert(
        "+STA",
        OpCode {
            code: 0x0C,
            format: 4,
        },
    );
    table.insert(
        "+STB",
        OpCode {
            code: 0x78,
            format: 4,
        },
    );
    table.insert(
        "+STCH",
        OpCode {
            code: 0x54,
            format: 4,
        },
    );
    table.insert(
        "+STF",
        OpCode {
            code: 0x80,
            format: 4,
        },
    );
    table.insert(
        "+STI",
        OpCode {
            code: 0xD4,
            format: 4,
        },
    );
    table.insert(
        "+STL",
        OpCode {
            code: 0x14,
            format: 4,
        },
    );
    table.insert(
        "+STS",
        OpCode {
            code: 0x7C,
            format: 4,
        },
    );
    table.insert(
        "+STSW",
        OpCode {
            code: 0xE8,
            format: 4,
        },
    );
    table.insert(
        "+STT",
        OpCode {
            code: 0x84,
            format: 4,
        },
    );
    table.insert(
        "+STX",
        OpCode {
            code: 0x10,
            format: 4,
        },
    );
    table.insert(
        "+SUB",
        OpCode {
            code: 0x1C,
            format: 4,
        },
    );
    table.insert(
        "+SUBF",
        OpCode {
            code: 0x5C,
            format: 4,
        },
    );
    table.insert(
        "SUBR",
        OpCode {
            code: 0x94,
            format: 2,
        },
    );
    table.insert(
        "SVC",
        OpCode {
            code: 0xB0,
            format: 2,
        },
    );
    table.insert(
        "TD",
        OpCode {
            code: 0xE0,
            format: 3,
        },
    );
    table.insert(
        "+TD",
        OpCode {
            code: 0xE0,
            format: 4,
        },
    );
    table.insert(
        "TIO",
        OpCode {
            code: 0xF8,
            format: 1,
        },
    );
    table.insert(
        "TIX",
        OpCode {
            code: 0x2C,
            format: 3,
        },
    );
    table.insert(
        "+TIX",
        OpCode {
            code: 0x2C,
            format: 4,
        },
    );
    table.insert(
        "TIXR",
        OpCode {
            code: 0xB8,
            format: 2,
        },
    );
    table.insert(
        "WD",
        OpCode {
            code: 0xDC,
            format: 3,
        },
    );
    table.insert(
        "+WD",
        OpCode {
            code: 0xDC,
            format: 4,
        },
    );
    table
}
