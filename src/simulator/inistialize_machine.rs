#[derive(Debug, Clone)]
pub struct Machine {
    // Registers
    pub reg_a: u32,  // Accumulator
    pub reg_x: u32,  // Index register
    pub reg_l: u32,  // Linkage register
    pub reg_b: u32,  // Base register
    pub reg_s: u32,  // General purpose
    pub reg_t: u32,  // General purpose
    pub reg_f: f64,  // Floating point
    pub reg_pc: u32, // Program counter
    pub reg_sw: u32, // Status word

    // Memory
    pub memory: Vec<u8>,

    // Control
    // pub running: bool,
    pub cc: i8, // Condition code (-1, 0, 1)
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            reg_a: 0,
            reg_x: 0,
            reg_l: 0,
            reg_b: 0,
            reg_s: 0,
            reg_t: 0,
            reg_f: 0.0,
            reg_pc: 0,
            reg_sw: 0,
            memory: vec![0; 1048576], // 1MB
            // running: false,
            cc: 0,
        }
    }
}
