use super::inistialize_machine;
use crate::disassembler::disassembler::disassemble;
use crate::loader::loader;
use crate::predefined::common::OBJECTPROGRAM;

pub fn simulator(buffer: String) {
    loader::loader(buffer);
    disassemble();
}
