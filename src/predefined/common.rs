use super::opcode::OpCode;
use once_cell::sync::Lazy;
use std::sync::Mutex;
pub static OBJECTPROGRAM: Lazy<Mutex<Vec<ObjectRecord>>> = Lazy::new(|| Mutex::new(vec![]));

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
    n: bool,
    i: bool,
    x: bool,
    b: bool,
    p: bool,
    e: bool,
}

// registers for format 2
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Reg {
    r1: String,
    r2: String,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DisAssembledToken {
    locctr: u32,
    command: Command,
    flags: Option<AddressFlags>,
    address: Option<u32>,
    reg: Option<Reg>,
}
