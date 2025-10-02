use super::inistialize_machine::Machine;

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
#[derive(PartialEq, Debug, Clone)]
pub enum AddressingMode {
    Direct,
    Indirect,
    Immediate,
    Indexed,
}

impl Opcode {
    pub fn execute(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        match self {
            Opcode::LDA => self.load_accumulator(machine, operand, mode),
            Opcode::LDX => self.load_index(machine, operand, mode),
            Opcode::LDL => self.load_linkage(machine, operand, mode),
            Opcode::LDB => self.load_base(machine, operand, mode),
            Opcode::LDS => self.load_s_register(machine, operand, mode),
            Opcode::LDT => self.load_t_register(machine, operand, mode),
            Opcode::LDF => self.load_float(machine, operand, mode),
            Opcode::LDCH => self.load_character(machine, operand, mode),
            Opcode::STA => self.store_accumulator(machine, operand, mode),
            Opcode::STX => self.store_index(machine, operand, mode),
            Opcode::STL => self.store_linkage(machine, operand, mode),
            Opcode::STB => self.store_base(machine, operand, mode),
            Opcode::STS => self.store_s_register(machine, operand, mode),
            Opcode::STT => self.store_t_register(machine, operand, mode),
            Opcode::STF => self.store_float(machine, operand, mode),
            Opcode::STI => self.store_interval_timer(machine, operand, mode),
            Opcode::STCH => self.store_character(machine, operand, mode),
            Opcode::STSW => self.store_status_word(machine, operand, mode),
            Opcode::ADD => self.add(machine, operand, mode),
            Opcode::ADDF => self.add_float(machine, operand, mode),
            Opcode::SUB => self.subtract(machine, operand, mode),
            Opcode::SUBF => self.subtract_float(machine, operand, mode),
            Opcode::MUL => self.multiply(machine, operand, mode),
            Opcode::MULF => self.multiply_float(machine, operand, mode),
            Opcode::DIV => self.divide(machine, operand, mode),
            Opcode::DIVF => self.divide_float(machine, operand, mode),
            Opcode::COMP => self.compare(machine, operand, mode),
            Opcode::COMPF => self.compare_float(machine, operand, mode),
            Opcode::COMPR => self.compare_register(machine, operand),
            Opcode::ADDR => self.add_register(machine, operand),
            Opcode::SUBR => self.subtract_register(machine, operand),
            Opcode::MULR => self.multiply_register(machine, operand),
            Opcode::DIVR => self.divide_register(machine, operand),
            Opcode::RMO => self.register_move(machine, operand),
            Opcode::CLEAR => self.clear_register(machine, operand),
            Opcode::TIXR => self.test_index_register(machine, operand),
            Opcode::SHIFTL => self.shift_left(machine, operand),
            Opcode::SHIFTR => self.shift_right(machine, operand),
            Opcode::J => self.jump(machine, operand),
            Opcode::JEQ => self.jump_equal(machine, operand),
            Opcode::JGT => self.jump_greater(machine, operand),
            Opcode::JLT => self.jump_less(machine, operand),
            Opcode::JSUB => self.jump_subroutine(machine, operand),
            Opcode::RSUB => self.return_subroutine(machine),
            Opcode::TIX => self.test_index(machine, operand, mode),
            Opcode::RD => self.read_device(machine, operand, mode),
            Opcode::WD => self.write_device(machine, operand, mode),
            Opcode::TD => self.test_device(machine, operand, mode),
            Opcode::SIO => self.start_io(machine),
            Opcode::TIO => self.test_io(machine),
            Opcode::HIO => self.halt_io(machine),
            Opcode::FIX => self.fix_float(machine),
            Opcode::FLOAT => self.float_convert(machine),
            Opcode::NORM => self.normalize_float(machine),
            Opcode::SSK => self.set_system_key(machine, operand, mode),
            Opcode::LPS => self.load_processor_status(machine, operand, mode),
            Opcode::SVC => self.supervisor_call(machine, operand),
        }
    }

    fn get_effective_address(&self, machine: &Machine, operand: u32, mode: &AddressingMode) -> u32 {
        match mode {
            AddressingMode::Direct => operand,
            AddressingMode::Indirect => {
                let addr = operand as usize;
                self.load_word(machine, addr)
            }
            AddressingMode::Immediate => operand,
            AddressingMode::Indexed => operand.wrapping_add(machine.reg_x),
        }
    }

    fn get_operand_value(&self, machine: &Machine, operand: u32, mode: &AddressingMode) -> u32 {
        match mode {
            AddressingMode::Immediate => operand,
            _ => {
                let addr = self.get_effective_address(machine, operand, mode);
                self.load_word(machine, addr as usize)
            }
        }
    }

    fn load_word(&self, machine: &Machine, address: usize) -> u32 {
        if address + 2 < machine.memory.len() {
            ((machine.memory[address] as u32) << 16)
                | ((machine.memory[address + 1] as u32) << 8)
                | (machine.memory[address + 2] as u32)
        } else {
            0
        }
    }

    fn store_word(&self, machine: &mut Machine, address: u32, value: u32) {
        let addr = address as usize;
        if addr + 2 < machine.memory.len() {
            machine.memory[addr] = ((value >> 16) & 0xFF) as u8;
            machine.memory[addr + 1] = ((value >> 8) & 0xFF) as u8;
            machine.memory[addr + 2] = (value & 0xFF) as u8;
        }
    }

    fn load_byte(&self, machine: &Machine, address: usize) -> u8 {
        if address < machine.memory.len() {
            machine.memory[address]
        } else {
            0
        }
    }

    fn store_byte(&self, machine: &mut Machine, address: u32, value: u8) {
        let addr = address as usize;
        if addr < machine.memory.len() {
            machine.memory[addr] = value;
        }
    }

    fn get_register_value(&self, machine: &Machine, reg_num: u8) -> u32 {
        match reg_num {
            0 => machine.reg_a,
            1 => machine.reg_x,
            2 => machine.reg_l,
            3 => machine.reg_b,
            4 => machine.reg_s,
            5 => machine.reg_t,
            6 => machine.reg_f as u32,
            8 => machine.reg_pc,
            9 => machine.reg_sw,
            _ => 0,
        }
    }

    fn set_register_value(&self, machine: &mut Machine, reg_num: u8, value: u32) {
        match reg_num {
            0 => machine.reg_a = value,
            1 => machine.reg_x = value,
            2 => machine.reg_l = value,
            3 => machine.reg_b = value,
            4 => machine.reg_s = value,
            5 => machine.reg_t = value,
            6 => machine.reg_f = value as f64,
            8 => machine.reg_pc = value,
            9 => machine.reg_sw = value,
            _ => {}
        }
    }

    fn load_accumulator(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_a = value;
    }

    fn load_index(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_x = value;
    }

    fn load_linkage(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_l = value;
    }

    fn load_base(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_b = value;
    }

    fn load_s_register(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_s = value;
    }

    fn load_t_register(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_t = value;
    }

    fn load_float(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_f = f64::from_bits(value as u64);
    }

    fn load_character(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        let byte_val = self.load_byte(machine, addr as usize);
        // Load rightmost byte of accumulator, clear other bytes
        machine.reg_a = (machine.reg_a & 0xFFFF00) | (byte_val as u32);
    }

    fn store_accumulator(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_a);
    }

    fn store_index(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_x);
    }

    fn store_linkage(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_l);
    }

    fn store_base(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_b);
    }

    fn store_s_register(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_s);
    }

    fn store_t_register(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_t);
    }

    fn store_float(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_f.to_bits() as u32);
    }

    fn store_interval_timer(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        // Store interval timer value (implementation specific)
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, 0); // Placeholder
    }

    fn store_character(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        let byte_val = (machine.reg_a & 0xFF) as u8;
        self.store_byte(machine, addr, byte_val);
    }

    fn store_status_word(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let addr = self.get_effective_address(machine, operand, &mode);
        self.store_word(machine, addr, machine.reg_sw);
    }

    // ---- Arithmetic Instructions ----
    fn add(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_a = machine.reg_a.wrapping_add(value);
    }

    fn add_float(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        let float_val = f64::from_bits(value as u64);
        machine.reg_f += float_val;
    }

    fn subtract(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_a = machine.reg_a.wrapping_sub(value);
    }

    fn subtract_float(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        let float_val = f64::from_bits(value as u64);
        machine.reg_f -= float_val;
    }

    fn multiply(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_a = machine.reg_a.wrapping_mul(value);
    }

    fn multiply_float(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        let float_val = f64::from_bits(value as u64);
        machine.reg_f *= float_val;
    }

    fn divide(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        if value != 0 {
            machine.reg_a = machine.reg_a / value;
        }
    }

    fn divide_float(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        let float_val = f64::from_bits(value as u64);
        if float_val != 0.0 {
            machine.reg_f /= float_val;
        }
    }

    fn compare(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.cc = if machine.reg_a < value {
            -1
        } else if machine.reg_a > value {
            1
        } else {
            0
        };
    }

    fn compare_float(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        let float_val = f64::from_bits(value as u64);
        machine.cc = if machine.reg_f < float_val {
            -1
        } else if machine.reg_f > float_val {
            1
        } else {
            0
        };
    }

    fn compare_register(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let r2 = operand & 0xF;
        let val1 = self.get_register_value(machine, r1 as u8);
        let val2 = self.get_register_value(machine, r2 as u8);
        machine.cc = if val1 < val2 {
            -1
        } else if val1 > val2 {
            1
        } else {
            0
        };
    }

    fn add_register(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let r2 = operand & 0xF;
        let val1 = self.get_register_value(machine, r1 as u8);
        let val2 = self.get_register_value(machine, r2 as u8);
        self.set_register_value(machine, r2 as u8, val2.wrapping_add(val1));
    }

    fn subtract_register(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let r2 = operand & 0xF;
        let val1 = self.get_register_value(machine, r1 as u8);
        let val2 = self.get_register_value(machine, r2 as u8);
        self.set_register_value(machine, r2 as u8, val2.wrapping_sub(val1));
    }

    fn multiply_register(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let r2 = operand & 0xF;
        let val1 = self.get_register_value(machine, r1 as u8);
        let val2 = self.get_register_value(machine, r2 as u8);
        self.set_register_value(machine, r2 as u8, val2.wrapping_mul(val1));
    }

    fn divide_register(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let r2 = operand & 0xF;
        let val1 = self.get_register_value(machine, r1 as u8);
        let val2 = self.get_register_value(machine, r2 as u8);
        if val1 != 0 {
            self.set_register_value(machine, r2 as u8, val2 / val1);
        }
    }

    fn register_move(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let r2 = operand & 0xF;
        let val1 = self.get_register_value(machine, r1 as u8);
        self.set_register_value(machine, r2 as u8, val1);
    }

    fn clear_register(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        self.set_register_value(machine, r1 as u8, 0);
    }

    fn test_index_register(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        machine.reg_x = machine.reg_x.wrapping_add(1);
        let reg_val = self.get_register_value(machine, r1 as u8);
        machine.cc = if machine.reg_x < reg_val {
            -1
        } else if machine.reg_x > reg_val {
            1
        } else {
            0
        };
    }

    fn shift_left(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let n = operand & 0xF;
        let val = self.get_register_value(machine, r1 as u8);
        self.set_register_value(machine, r1 as u8, val << n);
    }

    fn shift_right(&self, machine: &mut Machine, operand: u32) {
        let r1 = (operand >> 4) & 0xF;
        let n = operand & 0xF;
        let val = self.get_register_value(machine, r1 as u8);
        self.set_register_value(machine, r1 as u8, val >> n);
    }

    fn jump(&self, machine: &mut Machine, operand: u32) {
        machine.reg_pc = operand;
    }

    fn jump_equal(&self, machine: &mut Machine, operand: u32) {
        if machine.cc == 0 {
            machine.reg_pc = operand;
        }
    }

    fn jump_greater(&self, machine: &mut Machine, operand: u32) {
        if machine.cc > 0 {
            machine.reg_pc = operand;
        }
    }

    fn jump_less(&self, machine: &mut Machine, operand: u32) {
        if machine.cc < 0 {
            machine.reg_pc = operand;
        }
    }

    fn jump_subroutine(&self, machine: &mut Machine, operand: u32) {
        machine.reg_l = machine.reg_pc;
        machine.reg_pc = operand;
    }

    fn return_subroutine(&self, machine: &mut Machine) {
        machine.reg_pc = machine.reg_l;
    }

    fn test_index(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        machine.reg_x = machine.reg_x.wrapping_add(1);
        let value = self.get_operand_value(machine, operand, &mode);
        machine.cc = if machine.reg_x < value {
            -1
        } else if machine.reg_x > value {
            1
        } else {
            0
        };
    }

    fn read_device(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        // Read from device - implementation specific
        let _device = self.get_operand_value(machine, operand, &mode);
        // Placeholder: read character into rightmost byte of A
        machine.reg_a = (machine.reg_a & 0xFFFF00) | 0x41; // 'A'
    }

    fn write_device(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        // Write to device - implementation specific
        let _device = self.get_operand_value(machine, operand, &mode);
        let output_char = (machine.reg_a & 0xFF) as u8;
        print!("{}", output_char as char);
    }

    fn test_device(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        // Test device status - implementation specific
        let _device = self.get_operand_value(machine, operand, &mode);
        machine.cc = 0; // Device ready
    }

    fn start_io(&self, _machine: &mut Machine) {
        // Start I/O operation - implementation specific
        println!("Starting I/O operation");
    }

    fn test_io(&self, machine: &mut Machine) {
        // Test I/O status - implementation specific
        machine.cc = 0; // I/O complete
    }

    fn halt_io(&self, _machine: &mut Machine) {
        // Halt I/O operation - implementation specific
        println!("Halting I/O operation");
    }

    fn fix_float(&self, machine: &mut Machine) {
        machine.reg_a = machine.reg_f as u32;
    }

    fn float_convert(&self, machine: &mut Machine) {
        machine.reg_f = machine.reg_a as f64;
    }

    fn normalize_float(&self, machine: &mut Machine) {
        // Normalize floating point number - implementation specific
        if machine.reg_f != 0.0 {
            machine.reg_f = machine.reg_f.abs();
        }
    }

    fn set_system_key(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        // Set protection key - implementation specific
        let _key = self.get_operand_value(machine, operand, &mode);
        // Update status word with protection key
    }

    fn load_processor_status(&self, machine: &mut Machine, operand: u32, mode: AddressingMode) {
        let value = self.get_operand_value(machine, operand, &mode);
        machine.reg_sw = value;
    }

    fn supervisor_call(&self, machine: &mut Machine, operand: u32) {
        // Supervisor call - implementation specific
        let _call_code = operand;
        println!("Supervisor call: {}", operand);
        // Could trigger system call handling
    }
}
