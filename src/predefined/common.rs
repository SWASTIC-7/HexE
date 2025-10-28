// use super::opcode::OpCode;
use once_cell::sync::Lazy;
use std::sync::Mutex;
pub static OBJECTPROGRAM: Lazy<Mutex<Vec<ObjectRecord>>> = Lazy::new(|| Mutex::new(vec![]));
pub static SYMBOLTABLE: Lazy<Mutex<Vec<SymbolTable>>> = Lazy::new(|| Mutex::new(vec![]));
pub static LITERALTABLE: Lazy<Mutex<Vec<LiteralTable>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ObjectRecord {
    Header {
        name: String,
        start: u32,
        length: u32,
    },
    Text {
        start: u32,
        length: u8,
        objcodes: Vec<String>,
    },
    Modification {
        address: u32,
        length: u8,
        sign: bool, // true == + && false == -
        variable: String,
    },
    End {
        start: u32,
    },
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct Instruction {
    pub instr: String,
    pub opcode: OpCode,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Command {
    Directive(String),
    Instruction(Instruction),
}
#[allow(dead_code)]
impl Command {
    pub fn is_instruction(&self) -> bool {
        matches!(self, Command::Instruction(_))
    }
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct OpCode {
    pub code: u8,   // opcode value
    pub format: u8, // instruction format (1, 2, 3/4)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParsedToken {
    pub label: Option<String>,
    pub command: Command,
    pub operand1: Option<String>,
    pub operand2: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SymbolTable {
    pub label: String,
    pub address: u32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LabeledParsedLines {
    pub parsedtoken: ParsedToken,
    pub locctr: u32,
}

// flags =
// n i x b p e  -- addressing modes to check
// indirect , immediate, indexed, base relative, pc relative, format
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AddressFlags {
    pub n: bool,
    pub i: bool,
    pub x: bool,
    pub b: bool,
    pub p: bool,
    pub e: bool,
}

// registers for format 2
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Reg {
    pub r1: String,
    pub r2: String,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DisAssembledToken {
    pub locctr: u32,
    pub command: Command,
    pub flags: Option<AddressFlags>,
    pub address: Option<u32>,
    pub reg: Option<Reg>,
}

#[derive(Debug, Clone)]
pub struct LiteralTable {
    pub literal: String,      // e.g., "=C'EOF'"
    pub value: String,        // e.g., "454F46" (hex)
    pub length: u32,          // Length in bytes
    pub address: Option<u32>, // Address assigned in Pass 2
}
