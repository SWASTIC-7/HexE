#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // ---- Data Movement ----
    LDA,
    LDX,
    LDL,
    LDB,
    LDS,
    LDT,
    LDF,
    LDCH,
    STA,
    STX,
    STL,
    STB,
    STS,
    STT,
    STF,
    STI,
    STCH,
    STSW,

    // ---- Arithmetic ----
    ADD,
    ADDF,
    SUB,
    SUBF,
    MUL,
    MULF,
    DIV,
    DIVF,

    // ---- Comparison ----
    COMP,
    COMPF,
    COMPR,

    // ---- Register Ops (Format 2) ----
    ADDR,
    SUBR,
    MULR,
    DIVR,
    RMO,
    CLEAR,
    TIXR,
    SHIFTL,
    SHIFTR,

    // ---- Control Transfer ----
    J,
    JEQ,
    JGT,
    JLT,
    JSUB,
    RSUB,
    TIX,

    // ---- Input/Output ----
    RD,
    WD,
    TD,
    SIO,
    TIO,
    HIO,

    // ---- Misc ----
    FIX,
    FLOAT,
    NORM,
    SSK,
    LPS,
    SVC,
}

impl Opcode {
    pub fn execute(&self) {
        match self {
            Opcode::LDA => self.load_accumulator(),
            Opcode::LDX => self.load_index(),
            Opcode::LDL => self.load_linkage(),
            Opcode::LDB => self.load_base(),
            Opcode::LDS => self.load_s_register(),
            Opcode::LDT => self.load_t_register(),
            Opcode::LDF => self.load_float(),
            Opcode::LDCH => self.load_character(),
            Opcode::STA => self.store_accumulator(),
            Opcode::STX => self.store_index(),
            Opcode::STL => self.store_linkage(),
            Opcode::STB => self.store_base(),
            Opcode::STS => self.store_s_register(),
            Opcode::STT => self.store_t_register(),
            Opcode::STF => self.store_float(),
            Opcode::STI => self.store_interval_timer(),
            Opcode::STCH => self.store_character(),
            Opcode::STSW => self.store_status_word(),
            Opcode::ADD => self.add(),
            Opcode::ADDF => self.add_float(),
            Opcode::SUB => self.subtract(),
            Opcode::SUBF => self.subtract_float(),
            Opcode::MUL => self.multiply(),
            Opcode::MULF => self.multiply_float(),
            Opcode::DIV => self.divide(),
            Opcode::DIVF => self.divide_float(),
            Opcode::COMP => self.compare(),
            Opcode::COMPF => self.compare_float(),
            Opcode::COMPR => self.compare_register(),
            Opcode::ADDR => self.add_register(),
            Opcode::SUBR => self.subtract_register(),
            Opcode::MULR => self.multiply_register(),
            Opcode::DIVR => self.divide_register(),
            Opcode::RMO => self.register_move(),
            Opcode::CLEAR => self.clear_register(),
            Opcode::TIXR => self.test_index_register(),
            Opcode::SHIFTL => self.shift_left(),
            Opcode::SHIFTR => self.shift_right(),
            Opcode::J => self.jump(),
            Opcode::JEQ => self.jump_equal(),
            Opcode::JGT => self.jump_greater(),
            Opcode::JLT => self.jump_less(),
            Opcode::JSUB => self.jump_subroutine(),
            Opcode::RSUB => self.return_subroutine(),
            Opcode::TIX => self.test_index(),
            Opcode::RD => self.read_device(),
            Opcode::WD => self.write_device(),
            Opcode::TD => self.test_device(),
            Opcode::SIO => self.start_io(),
            Opcode::TIO => self.test_io(),
            Opcode::HIO => self.halt_io(),
            Opcode::FIX => self.fix_float(),
            Opcode::FLOAT => self.float_convert(),
            Opcode::NORM => self.normalize_float(),
            Opcode::SSK => self.set_system_key(),
            Opcode::LPS => self.load_processor_status(),
            Opcode::SVC => self.supervisor_call(),
        }
    }

    fn load_accumulator(&self) {}
    fn load_index(&self) {}
    fn load_linkage(&self) {}
    fn load_base(&self) {}
    fn load_s_register(&self) {}
    fn load_t_register(&self) {}
    fn load_float(&self) {}
    fn load_character(&self) {}
    fn store_accumulator(&self) {}
    fn store_index(&self) {}
    fn store_linkage(&self) {}
    fn store_base(&self) {}
    fn store_s_register(&self) {}
    fn store_t_register(&self) {}
    fn store_float(&self) {}
    fn store_interval_timer(&self) {}
    fn store_character(&self) {}
    fn store_status_word(&self) {}

    fn add(&self) {}
    fn add_float(&self) {}
    fn subtract(&self) {}
    fn subtract_float(&self) {}
    fn multiply(&self) {}
    fn multiply_float(&self) {}
    fn divide(&self) {}
    fn divide_float(&self) {}

    fn compare(&self) {}
    fn compare_float(&self) {}
    fn compare_register(&self) {}

    fn add_register(&self) {}
    fn subtract_register(&self) {}
    fn multiply_register(&self) {}
    fn divide_register(&self) {}
    fn register_move(&self) {}
    fn clear_register(&self) {}
    fn test_index_register(&self) {}
    fn shift_left(&self) {}
    fn shift_right(&self) {}

    fn jump(&self) {}
    fn jump_equal(&self) {}
    fn jump_greater(&self) {}
    fn jump_less(&self) {}
    fn jump_subroutine(&self) {}
    fn return_subroutine(&self) {}
    fn test_index(&self) {}

    fn read_device(&self) {}
    fn write_device(&self) {}
    fn test_device(&self) {}
    fn start_io(&self) {}
    fn test_io(&self) {}
    fn halt_io(&self) {}

    fn fix_float(&self) {}
    fn float_convert(&self) {}
    fn normalize_float(&self) {}
    fn set_system_key(&self) {}
    fn load_processor_status(&self) {}
    fn supervisor_call(&self) {}
}
