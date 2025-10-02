use crate::disassembler::disassembler;
use crate::loader::loader;

pub fn simulator(buffer: String) {
    loader::loader(buffer);
    disassembler::disassemble();
}
