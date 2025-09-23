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

impl Opcode {}
