// 6502 Opcodes
// Fiathux Su
// 2024-09

//! 6502 OpCode and Parameters definition
//!
//! Reference:
//! - <https://www.masswerk.at/6502/6502_instruction_set.html>
//! - <http://www.6502.org/tutorials/6502opcodes.html>
//! - <https://www.nesdev.org/wiki/CPU_unofficial_opcodes>
//! - <https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes>
//! - <https://www.oxyron.de/html/opcodes02.html>

/// Absolute Addressing
pub struct OpAbs {
    pub addr: u16,
}

/// Absolute Addressing with X Offset
pub struct OpAbsX {
    pub addr: u16,
}

/// Absolute Addressing with Y Offset
pub struct OpAbsY {
    pub addr: u16,
}

/// Immediate Number
pub struct OpImmd {
    pub val: u8,
}

/// Implied
pub struct OpImpl;

/// Indirect Addressing
pub struct OpInd {
    pub ind: u16,
}

/// Indirect Addressing with X Offset
pub struct OpIndX {
    pub ind: u8,
}

/// Indirect Addressing with Y Offset
pub struct OpIndY {
    pub ind: u8,
}

/// Relative Addressing
pub struct OpRel {
    pub rel: u8,
}

/// Zero Page Addressing
pub struct OpZp {
    pub addr: u8,
}

/// Zero Page Addressing with X Offset
pub struct OpZpX {
    pub addr: u8,
}

/// Zero Page Addressing with Y Offset
pub struct OpZpY {
    pub addr: u8,
}

/// 6502 OpCode Definitions
pub struct OpCode {
    /// OpCode
    pub op: u8,
    /// Assembly Name
    pub name: &'static str,
    /// Number of Bytes
    pub n: u8,
    /// Cycles
    pub cyc: u8,
}

// 6502 OP Codes
// ref:
//  - https://www.masswerk.at/6502/6502_instruction_set.html
//  - http://www.6502.org/tutorials/6502opcodes.html
pub const ADC_ABS: OpCode = OpCode {
    op: 0x6D,
    name: "ADC",
    n: 3,
    cyc: 4,
};
pub const ADC_ABS_X: OpCode = OpCode {
    op: 0x7D,
    name: "ADC",
    n: 3,
    cyc: 4,
};
pub const ADC_ABS_Y: OpCode = OpCode {
    op: 0x79,
    name: "ADC",
    n: 3,
    cyc: 4,
};
pub const ADC_IMM: OpCode = OpCode {
    op: 0x69,
    name: "ADC",
    n: 2,
    cyc: 2,
};
pub const ADC_IND_Y: OpCode = OpCode {
    op: 0x71,
    name: "ADC",
    n: 2,
    cyc: 5,
};
pub const ADC_X_IND: OpCode = OpCode {
    op: 0x61,
    name: "ADC",
    n: 2,
    cyc: 6,
};
pub const ADC_ZP: OpCode = OpCode {
    op: 0x65,
    name: "ADC",
    n: 2,
    cyc: 3,
};
pub const ADC_ZP_X: OpCode = OpCode {
    op: 0x75,
    name: "ADC",
    n: 2,
    cyc: 4,
};
pub const AND_ABS: OpCode = OpCode {
    op: 0x2D,
    name: "AND",
    n: 3,
    cyc: 4,
};
pub const AND_ABS_X: OpCode = OpCode {
    op: 0x3D,
    name: "AND",
    n: 3,
    cyc: 4,
};
pub const AND_ABS_Y: OpCode = OpCode {
    op: 0x39,
    name: "AND",
    n: 3,
    cyc: 4,
};
pub const AND_IMM: OpCode = OpCode {
    op: 0x29,
    name: "AND",
    n: 2,
    cyc: 2,
};
pub const AND_IND_Y: OpCode = OpCode {
    op: 0x31,
    name: "AND",
    n: 2,
    cyc: 5,
};
pub const AND_X_IND: OpCode = OpCode {
    op: 0x21,
    name: "AND",
    n: 2,
    cyc: 6,
};
pub const AND_ZP: OpCode = OpCode {
    op: 0x25,
    name: "AND",
    n: 2,
    cyc: 3,
};
pub const AND_ZP_X: OpCode = OpCode {
    op: 0x35,
    name: "AND",
    n: 2,
    cyc: 4,
};
pub const ASL_A: OpCode = OpCode {
    op: 0x0A,
    name: "ASL",
    n: 1,
    cyc: 2,
};
pub const ASL_ABS: OpCode = OpCode {
    op: 0x0E,
    name: "ASL",
    n: 3,
    cyc: 6,
};
pub const ASL_ABS_X: OpCode = OpCode {
    op: 0x1E,
    name: "ASL",
    n: 3,
    cyc: 7,
};
pub const ASL_ZP: OpCode = OpCode {
    op: 0x06,
    name: "ASL",
    n: 2,
    cyc: 5,
};
pub const ASL_ZP_X: OpCode = OpCode {
    op: 0x16,
    name: "ASL",
    n: 2,
    cyc: 6,
};
pub const BCC_REL: OpCode = OpCode {
    op: 0x90,
    name: "BCC",
    n: 2,
    cyc: 2,
};
pub const BCS_REL: OpCode = OpCode {
    op: 0xB0,
    name: "BCS",
    n: 2,
    cyc: 2,
};
pub const BEQ_REL: OpCode = OpCode {
    op: 0xF0,
    name: "BEQ",
    n: 2,
    cyc: 2,
};
pub const BIT_ABS: OpCode = OpCode {
    op: 0x2C,
    name: "BIT",
    n: 3,
    cyc: 4,
};
pub const BIT_ZP: OpCode = OpCode {
    op: 0x24,
    name: "BIT",
    n: 2,
    cyc: 3,
};
pub const BMI_REL: OpCode = OpCode {
    op: 0x30,
    name: "BMI",
    n: 2,
    cyc: 2,
};
pub const BNE_REL: OpCode = OpCode {
    op: 0xD0,
    name: "BNE",
    n: 2,
    cyc: 2,
};
pub const BPL_REL: OpCode = OpCode {
    op: 0x10,
    name: "BPL",
    n: 2,
    cyc: 2,
};
pub const BRK: OpCode = OpCode {
    op: 0x00,
    name: "BRK",
    n: 1,
    cyc: 7,
};
pub const BVC_REL: OpCode = OpCode {
    op: 0x50,
    name: "BVC",
    n: 2,
    cyc: 2,
};
pub const BVS_REL: OpCode = OpCode {
    op: 0x70,
    name: "BVS",
    n: 2,
    cyc: 2,
};
pub const CLC: OpCode = OpCode {
    op: 0x18,
    name: "CLC",
    n: 1,
    cyc: 2,
};
pub const CLD: OpCode = OpCode {
    op: 0xD8,
    name: "CLD",
    n: 1,
    cyc: 2,
};
pub const CLI: OpCode = OpCode {
    op: 0x58,
    name: "CLI",
    n: 1,
    cyc: 2,
};
pub const CLV: OpCode = OpCode {
    op: 0xB8,
    name: "CLV",
    n: 1,
    cyc: 2,
};
pub const CMP_ABS: OpCode = OpCode {
    op: 0xCD,
    name: "CMP",
    n: 3,
    cyc: 4,
};
pub const CMP_ABS_X: OpCode = OpCode {
    op: 0xDD,
    name: "CMP",
    n: 3,
    cyc: 4,
};
pub const CMP_ABS_Y: OpCode = OpCode {
    op: 0xD9,
    name: "CMP",
    n: 3,
    cyc: 4,
};
pub const CMP_IMM: OpCode = OpCode {
    op: 0xC9,
    name: "CMP",
    n: 2,
    cyc: 2,
};
pub const CMP_IND_Y: OpCode = OpCode {
    op: 0xD1,
    name: "CMP",
    n: 2,
    cyc: 5,
};
pub const CMP_X_IND: OpCode = OpCode {
    op: 0xC1,
    name: "CMP",
    n: 2,
    cyc: 6,
};
pub const CMP_ZP: OpCode = OpCode {
    op: 0xC5,
    name: "CMP",
    n: 2,
    cyc: 3,
};
pub const CMP_ZP_X: OpCode = OpCode {
    op: 0xD5,
    name: "CMP",
    n: 2,
    cyc: 4,
};
pub const CPX_ABS: OpCode = OpCode {
    op: 0xEC,
    name: "CPX",
    n: 3,
    cyc: 4,
};
pub const CPX_IMM: OpCode = OpCode {
    op: 0xE0,
    name: "CPX",
    n: 2,
    cyc: 2,
};
pub const CPX_ZP: OpCode = OpCode {
    op: 0xE4,
    name: "CPX",
    n: 2,
    cyc: 3,
};
pub const CPY_ABS: OpCode = OpCode {
    op: 0xCC,
    name: "CPY",
    n: 3,
    cyc: 4,
};
pub const CPY_IMM: OpCode = OpCode {
    op: 0xC0,
    name: "CPY",
    n: 2,
    cyc: 2,
};
pub const CPY_ZP: OpCode = OpCode {
    op: 0xC4,
    name: "CPY",
    n: 2,
    cyc: 3,
};
pub const DEC_ABS: OpCode = OpCode {
    op: 0xCE,
    name: "DEC",
    n: 3,
    cyc: 6,
};
pub const DEC_ABS_X: OpCode = OpCode {
    op: 0xDE,
    name: "DEC",
    n: 3,
    cyc: 7,
};
pub const DEC_ZP: OpCode = OpCode {
    op: 0xC6,
    name: "DEC",
    n: 2,
    cyc: 5,
};
pub const DEC_ZP_X: OpCode = OpCode {
    op: 0xD6,
    name: "DEC",
    n: 2,
    cyc: 6,
};
pub const DEX: OpCode = OpCode {
    op: 0xCA,
    name: "DEX",
    n: 1,
    cyc: 2,
};
pub const DEY: OpCode = OpCode {
    op: 0x88,
    name: "DEY",
    n: 1,
    cyc: 2,
};
pub const EOR_ABS: OpCode = OpCode {
    op: 0x4D,
    name: "EOR",
    n: 3,
    cyc: 4,
};
pub const EOR_ABS_X: OpCode = OpCode {
    op: 0x5D,
    name: "EOR",
    n: 3,
    cyc: 4,
};
pub const EOR_ABS_Y: OpCode = OpCode {
    op: 0x59,
    name: "EOR",
    n: 3,
    cyc: 4,
};
pub const EOR_IMM: OpCode = OpCode {
    op: 0x49,
    name: "EOR",
    n: 2,
    cyc: 2,
};
pub const EOR_IND_Y: OpCode = OpCode {
    op: 0x51,
    name: "EOR",
    n: 2,
    cyc: 5,
};
pub const EOR_X_IND: OpCode = OpCode {
    op: 0x41,
    name: "EOR",
    n: 2,
    cyc: 6,
};
pub const EOR_ZP: OpCode = OpCode {
    op: 0x45,
    name: "EOR",
    n: 2,
    cyc: 3,
};
pub const EOR_ZP_X: OpCode = OpCode {
    op: 0x55,
    name: "EOR",
    n: 2,
    cyc: 4,
};
pub const INC_ABS: OpCode = OpCode {
    op: 0xEE,
    name: "INC",
    n: 3,
    cyc: 6,
};
pub const INC_ABS_X: OpCode = OpCode {
    op: 0xFE,
    name: "INC",
    n: 3,
    cyc: 7,
};
pub const INC_ZP: OpCode = OpCode {
    op: 0xE6,
    name: "INC",
    n: 2,
    cyc: 5,
};
pub const INC_ZP_X: OpCode = OpCode {
    op: 0xF6,
    name: "INC",
    n: 2,
    cyc: 6,
};
pub const INX: OpCode = OpCode {
    op: 0xE8,
    name: "INX",
    n: 1,
    cyc: 2,
};
pub const INY: OpCode = OpCode {
    op: 0xC8,
    name: "INY",
    n: 1,
    cyc: 2,
};
pub const JMP_ABS: OpCode = OpCode {
    op: 0x4C,
    name: "JMP",
    n: 3,
    cyc: 3,
};
pub const JMP_IND: OpCode = OpCode {
    op: 0x6C,
    name: "JMP",
    n: 3,
    cyc: 5,
};
pub const JSR_ABS: OpCode = OpCode {
    op: 0x20,
    name: "JSR",
    n: 3,
    cyc: 6,
};
pub const LDA_ABS: OpCode = OpCode {
    op: 0xAD,
    name: "LDA",
    n: 3,
    cyc: 4,
};
pub const LDA_ABS_X: OpCode = OpCode {
    op: 0xBD,
    name: "LDA",
    n: 3,
    cyc: 4,
};
pub const LDA_ABS_Y: OpCode = OpCode {
    op: 0xB9,
    name: "LDA",
    n: 3,
    cyc: 4,
};
pub const LDA_IMM: OpCode = OpCode {
    op: 0xA9,
    name: "LDA",
    n: 2,
    cyc: 2,
};
pub const LDA_IND_Y: OpCode = OpCode {
    op: 0xB1,
    name: "LDA",
    n: 2,
    cyc: 5,
};
pub const LDA_X_IND: OpCode = OpCode {
    op: 0xA1,
    name: "LDA",
    n: 2,
    cyc: 6,
};
pub const LDA_ZP: OpCode = OpCode {
    op: 0xA5,
    name: "LDA",
    n: 2,
    cyc: 3,
};
pub const LDA_ZP_X: OpCode = OpCode {
    op: 0xB5,
    name: "LDA",
    n: 2,
    cyc: 4,
};
pub const LDX_ABS: OpCode = OpCode {
    op: 0xAE,
    name: "LDX",
    n: 3,
    cyc: 4,
};
pub const LDX_ABS_Y: OpCode = OpCode {
    op: 0xBE,
    name: "LDX",
    n: 3,
    cyc: 4,
};
pub const LDX_IMM: OpCode = OpCode {
    op: 0xA2,
    name: "LDX",
    n: 2,
    cyc: 2,
};
pub const LDX_ZP: OpCode = OpCode {
    op: 0xA6,
    name: "LDX",
    n: 2,
    cyc: 3,
};
pub const LDX_ZP_Y: OpCode = OpCode {
    op: 0xB6,
    name: "LDX",
    n: 2,
    cyc: 4,
};
pub const LDY_ABS: OpCode = OpCode {
    op: 0xAC,
    name: "LDY",
    n: 3,
    cyc: 4,
};
pub const LDY_ABS_X: OpCode = OpCode {
    op: 0xBC,
    name: "LDY",
    n: 3,
    cyc: 4,
};
pub const LDY_IMM: OpCode = OpCode {
    op: 0xA0,
    name: "LDY",
    n: 2,
    cyc: 2,
};
pub const LDY_ZP: OpCode = OpCode {
    op: 0xA4,
    name: "LDY",
    n: 2,
    cyc: 3,
};
pub const LDY_ZP_X: OpCode = OpCode {
    op: 0xB4,
    name: "LDY",
    n: 2,
    cyc: 4,
};
pub const LSR_A: OpCode = OpCode {
    op: 0x4A,
    name: "LSR",
    n: 1,
    cyc: 2,
};
pub const LSR_ABS: OpCode = OpCode {
    op: 0x4E,
    name: "LSR",
    n: 3,
    cyc: 6,
};
pub const LSR_ABS_X: OpCode = OpCode {
    op: 0x5E,
    name: "LSR",
    n: 3,
    cyc: 7,
};
pub const LSR_ZP: OpCode = OpCode {
    op: 0x46,
    name: "LSR",
    n: 2,
    cyc: 5,
};
pub const LSR_ZP_X: OpCode = OpCode {
    op: 0x56,
    name: "LSR",
    n: 2,
    cyc: 6,
};
pub const NOP: OpCode = OpCode {
    op: 0xEA,
    name: "NOP",
    n: 1,
    cyc: 2,
};
pub const ORA_ABS: OpCode = OpCode {
    op: 0x0D,
    name: "ORA",
    n: 3,
    cyc: 4,
};
pub const ORA_ABS_X: OpCode = OpCode {
    op: 0x1D,
    name: "ORA",
    n: 3,
    cyc: 4,
};
pub const ORA_ABS_Y: OpCode = OpCode {
    op: 0x19,
    name: "ORA",
    n: 3,
    cyc: 4,
};
pub const ORA_IMM: OpCode = OpCode {
    op: 0x09,
    name: "ORA",
    n: 2,
    cyc: 2,
};
pub const ORA_IND_Y: OpCode = OpCode {
    op: 0x11,
    name: "ORA",
    n: 2,
    cyc: 5,
};
pub const ORA_X_IND: OpCode = OpCode {
    op: 0x01,
    name: "ORA",
    n: 2,
    cyc: 6,
};
pub const ORA_ZP: OpCode = OpCode {
    op: 0x05,
    name: "ORA",
    n: 2,
    cyc: 3,
};
pub const ORA_ZP_X: OpCode = OpCode {
    op: 0x15,
    name: "ORA",
    n: 2,
    cyc: 4,
};
pub const PHA: OpCode = OpCode {
    op: 0x48,
    name: "PHA",
    n: 1,
    cyc: 3,
};
pub const PHP: OpCode = OpCode {
    op: 0x08,
    name: "PHP",
    n: 1,
    cyc: 3,
};
pub const PLA: OpCode = OpCode {
    op: 0x68,
    name: "PLA",
    n: 1,
    cyc: 4,
};
pub const PLP: OpCode = OpCode {
    op: 0x28,
    name: "PLP",
    n: 1,
    cyc: 4,
};
pub const ROL_A: OpCode = OpCode {
    op: 0x2A,
    name: "ROL",
    n: 1,
    cyc: 2,
};
pub const ROL_ABS: OpCode = OpCode {
    op: 0x2E,
    name: "ROL",
    n: 3,
    cyc: 6,
};
pub const ROL_ABS_X: OpCode = OpCode {
    op: 0x3E,
    name: "ROL",
    n: 3,
    cyc: 7,
};
pub const ROL_ZP: OpCode = OpCode {
    op: 0x26,
    name: "ROL",
    n: 2,
    cyc: 5,
};
pub const ROL_ZP_X: OpCode = OpCode {
    op: 0x36,
    name: "ROL",
    n: 2,
    cyc: 6,
};
pub const ROR_A: OpCode = OpCode {
    op: 0x6A,
    name: "ROR",
    n: 1,
    cyc: 2,
};
pub const ROR_ABS: OpCode = OpCode {
    op: 0x6E,
    name: "ROR",
    n: 3,
    cyc: 6,
};
pub const ROR_ABS_X: OpCode = OpCode {
    op: 0x7E,
    name: "ROR",
    n: 3,
    cyc: 7,
};
pub const ROR_ZP: OpCode = OpCode {
    op: 0x66,
    name: "ROR",
    n: 2,
    cyc: 5,
};
pub const ROR_ZP_X: OpCode = OpCode {
    op: 0x76,
    name: "ROR",
    n: 2,
    cyc: 6,
};
pub const RTI: OpCode = OpCode {
    op: 0x40,
    name: "RTI",
    n: 1,
    cyc: 6,
};
pub const RTS: OpCode = OpCode {
    op: 0x60,
    name: "RTS",
    n: 1,
    cyc: 6,
};
pub const SBC_ABS: OpCode = OpCode {
    op: 0xED,
    name: "SBC",
    n: 3,
    cyc: 4,
};
pub const SBC_ABS_X: OpCode = OpCode {
    op: 0xFD,
    name: "SBC",
    n: 3,
    cyc: 4,
};
pub const SBC_ABS_Y: OpCode = OpCode {
    op: 0xF9,
    name: "SBC",
    n: 3,
    cyc: 4,
};
pub const SBC_IMM: OpCode = OpCode {
    op: 0xE9,
    name: "SBC",
    n: 2,
    cyc: 2,
};
pub const SBC_IND_Y: OpCode = OpCode {
    op: 0xF1,
    name: "SBC",
    n: 2,
    cyc: 5,
};
pub const SBC_X_IND: OpCode = OpCode {
    op: 0xE1,
    name: "SBC",
    n: 2,
    cyc: 6,
};
pub const SBC_ZP: OpCode = OpCode {
    op: 0xE5,
    name: "SBC",
    n: 2,
    cyc: 3,
};
pub const SBC_ZP_X: OpCode = OpCode {
    op: 0xF5,
    name: "SBC",
    n: 2,
    cyc: 4,
};
pub const SEC: OpCode = OpCode {
    op: 0x38,
    name: "SEC",
    n: 1,
    cyc: 2,
};
pub const SED: OpCode = OpCode {
    op: 0xF8,
    name: "SED",
    n: 1,
    cyc: 2,
};
pub const SEI: OpCode = OpCode {
    op: 0x78,
    name: "SEI",
    n: 1,
    cyc: 2,
};
pub const STA_ABS: OpCode = OpCode {
    op: 0x8D,
    name: "STA",
    n: 3,
    cyc: 4,
};
pub const STA_ABS_X: OpCode = OpCode {
    op: 0x9D,
    name: "STA",
    n: 3,
    cyc: 5,
};
pub const STA_ABS_Y: OpCode = OpCode {
    op: 0x99,
    name: "STA",
    n: 3,
    cyc: 5,
};
pub const STA_IND_Y: OpCode = OpCode {
    op: 0x91,
    name: "STA",
    n: 2,
    cyc: 6,
};
pub const STA_X_IND: OpCode = OpCode {
    op: 0x81,
    name: "STA",
    n: 2,
    cyc: 6,
};
pub const STA_ZP: OpCode = OpCode {
    op: 0x85,
    name: "STA",
    n: 2,
    cyc: 3,
};
pub const STA_ZP_X: OpCode = OpCode {
    op: 0x95,
    name: "STA",
    n: 2,
    cyc: 4,
};
pub const STX_ABS: OpCode = OpCode {
    op: 0x8E,
    name: "STX",
    n: 3,
    cyc: 4,
};
pub const STX_ZP: OpCode = OpCode {
    op: 0x86,
    name: "STX",
    n: 2,
    cyc: 3,
};
pub const STX_ZP_Y: OpCode = OpCode {
    op: 0x96,
    name: "STX",
    n: 2,
    cyc: 4,
};
pub const STY_ABS: OpCode = OpCode {
    op: 0x8C,
    name: "STY",
    n: 3,
    cyc: 4,
};
pub const STY_ZP: OpCode = OpCode {
    op: 0x84,
    name: "STY",
    n: 2,
    cyc: 3,
};
pub const STY_ZP_X: OpCode = OpCode {
    op: 0x94,
    name: "STY",
    n: 2,
    cyc: 4,
};
pub const TAX: OpCode = OpCode {
    op: 0xAA,
    name: "TAX",
    n: 1,
    cyc: 2,
};
pub const TAY: OpCode = OpCode {
    op: 0xA8,
    name: "TAY",
    n: 1,
    cyc: 2,
};
pub const TSX: OpCode = OpCode {
    op: 0xBA,
    name: "TSX",
    n: 1,
    cyc: 2,
};
pub const TXA: OpCode = OpCode {
    op: 0x8A,
    name: "TXA",
    n: 1,
    cyc: 2,
};
pub const TXS: OpCode = OpCode {
    op: 0x9A,
    name: "TXS",
    n: 1,
    cyc: 2,
};
pub const TYA: OpCode = OpCode {
    op: 0x98,
    name: "TYA",
    n: 1,
    cyc: 2,
};

// NES Unofficial Opcodes
// ref:
//  - https://www.nesdev.org/wiki/CPU_unofficial_opcodes
//  - https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes
//  - https://www.oxyron.de/html/opcodes02.html
pub const ALR_IMM: OpCode = OpCode {
    op: 0x4B,
    name: "ALR",
    n: 2,
    cyc: 2,
};
pub const ANC_2B_IMM: OpCode = OpCode {
    op: 0x2B,
    name: "ANC",
    n: 2,
    cyc: 2,
};
pub const ANC_IMM: OpCode = OpCode {
    op: 0x0B,
    name: "ANC",
    n: 2,
    cyc: 2,
};
pub const ARR_IMM: OpCode = OpCode {
    op: 0x6B,
    name: "ARR",
    n: 2,
    cyc: 2,
};
pub const AXS_IMM: OpCode = OpCode {
    op: 0xCB,
    name: "AXS",
    n: 2,
    cyc: 2,
};
pub const DCP_ABS: OpCode = OpCode {
    op: 0xCF,
    name: "DCP",
    n: 3,
    cyc: 6,
};
pub const DCP_ABS_X: OpCode = OpCode {
    op: 0xDF,
    name: "DCP",
    n: 3,
    cyc: 7,
};
pub const DCP_ABS_Y: OpCode = OpCode {
    op: 0xDB,
    name: "DCP",
    n: 3,
    cyc: 7,
};
pub const DCP_IND_Y: OpCode = OpCode {
    op: 0xD3,
    name: "DCP",
    n: 2,
    cyc: 8,
};
pub const DCP_X_IND: OpCode = OpCode {
    op: 0xC3,
    name: "DCP",
    n: 2,
    cyc: 8,
};
pub const DCP_ZP: OpCode = OpCode {
    op: 0xC7,
    name: "DCP",
    n: 2,
    cyc: 5,
};
pub const DCP_ZP_X: OpCode = OpCode {
    op: 0xD7,
    name: "DCP",
    n: 2,
    cyc: 6,
};
pub const IGN_04_ZP: OpCode = OpCode {
    op: 0x04,
    name: "IGN",
    n: 2,
    cyc: 3,
};
pub const IGN_14_ZP_X: OpCode = OpCode {
    op: 0x14,
    name: "IGN",
    n: 2,
    cyc: 4,
};
pub const IGN_1C_ABS_X: OpCode = OpCode {
    op: 0x1C,
    name: "IGN",
    n: 3,
    cyc: 4,
};
pub const IGN_34_ZP_X: OpCode = OpCode {
    op: 0x34,
    name: "IGN",
    n: 2,
    cyc: 4,
};
pub const IGN_3C_ABS_X: OpCode = OpCode {
    op: 0x3C,
    name: "IGN",
    n: 3,
    cyc: 4,
};
pub const IGN_44_ZP: OpCode = OpCode {
    op: 0x44,
    name: "IGN",
    n: 2,
    cyc: 3,
};
pub const IGN_54_ZP_X: OpCode = OpCode {
    op: 0x54,
    name: "IGN",
    n: 2,
    cyc: 4,
};
pub const IGN_5C_ABS_X: OpCode = OpCode {
    op: 0x5C,
    name: "IGN",
    n: 3,
    cyc: 4,
};
pub const IGN_64_ZP: OpCode = OpCode {
    op: 0x64,
    name: "IGN",
    n: 2,
    cyc: 3,
};
pub const IGN_74_ZP_X: OpCode = OpCode {
    op: 0x74,
    name: "IGN",
    n: 2,
    cyc: 4,
};
pub const IGN_7C_ABS_X: OpCode = OpCode {
    op: 0x7C,
    name: "IGN",
    n: 3,
    cyc: 4,
};
pub const IGN_ABS: OpCode = OpCode {
    op: 0x0C,
    name: "IGN",
    n: 3,
    cyc: 4,
};
pub const IGN_D4_ZP_X: OpCode = OpCode {
    op: 0xD4,
    name: "IGN",
    n: 2,
    cyc: 4,
};
pub const IGN_DC_ABS_X: OpCode = OpCode {
    op: 0xDC,
    name: "IGN",
    n: 3,
    cyc: 4,
};
pub const IGN_F4_ZP_X: OpCode = OpCode {
    op: 0xF4,
    name: "IGN",
    n: 2,
    cyc: 4,
};
pub const IGN_FC_ABS_X: OpCode = OpCode {
    op: 0xFC,
    name: "IGN",
    n: 3,
    cyc: 4,
};
pub const ISC_ABS: OpCode = OpCode {
    op: 0xEF,
    name: "ISC",
    n: 3,
    cyc: 6,
};
pub const ISC_ABS_X: OpCode = OpCode {
    op: 0xFF,
    name: "ISC",
    n: 3,
    cyc: 7,
};
pub const ISC_ABS_Y: OpCode = OpCode {
    op: 0xFB,
    name: "ISC",
    n: 3,
    cyc: 7,
};
pub const ISC_IND_Y: OpCode = OpCode {
    op: 0xF3,
    name: "ISC",
    n: 2,
    cyc: 8,
};
pub const ISC_X_IND: OpCode = OpCode {
    op: 0xE3,
    name: "ISC",
    n: 2,
    cyc: 8,
};
pub const ISC_ZP: OpCode = OpCode {
    op: 0xE7,
    name: "ISC",
    n: 2,
    cyc: 5,
};
pub const ISC_ZP_X: OpCode = OpCode {
    op: 0xF7,
    name: "ISC",
    n: 2,
    cyc: 6,
};
pub const LAX_ABS: OpCode = OpCode {
    op: 0xAF,
    name: "LAX",
    n: 3,
    cyc: 4,
};
pub const LAX_ABS_Y: OpCode = OpCode {
    op: 0xBF,
    name: "LAX",
    n: 3,
    cyc: 4,
};
pub const LAX_IND_Y: OpCode = OpCode {
    op: 0xB3,
    name: "LAX",
    n: 2,
    cyc: 5,
};
pub const LAX_X_IND: OpCode = OpCode {
    op: 0xA3,
    name: "LAX",
    n: 2,
    cyc: 6,
};
pub const LAX_ZP: OpCode = OpCode {
    op: 0xA7,
    name: "LAX",
    n: 2,
    cyc: 3,
};
pub const LAX_ZP_Y: OpCode = OpCode {
    op: 0xB7,
    name: "LAX",
    n: 2,
    cyc: 4,
};
pub const NOP_1A: OpCode = OpCode {
    op: 0x1A,
    name: "NOP",
    n: 1,
    cyc: 2,
};
pub const NOP_3A: OpCode = OpCode {
    op: 0x3A,
    name: "NOP",
    n: 1,
    cyc: 2,
};
pub const NOP_5A: OpCode = OpCode {
    op: 0x5A,
    name: "NOP",
    n: 1,
    cyc: 2,
};
pub const NOP_7A: OpCode = OpCode {
    op: 0x7A,
    name: "NOP",
    n: 1,
    cyc: 2,
};
pub const NOP_DA: OpCode = OpCode {
    op: 0xDA,
    name: "NOP",
    n: 1,
    cyc: 2,
};
pub const NOP_FA: OpCode = OpCode {
    op: 0xFA,
    name: "NOP",
    n: 1,
    cyc: 2,
};
pub const RLA_ABS: OpCode = OpCode {
    op: 0x2F,
    name: "RLA",
    n: 3,
    cyc: 6,
};
pub const RLA_ABS_X: OpCode = OpCode {
    op: 0x3F,
    name: "RLA",
    n: 3,
    cyc: 7,
};
pub const RLA_ABS_Y: OpCode = OpCode {
    op: 0x3B,
    name: "RLA",
    n: 3,
    cyc: 7,
};
pub const RLA_IND_Y: OpCode = OpCode {
    op: 0x33,
    name: "RLA",
    n: 2,
    cyc: 8,
};
pub const RLA_X_IND: OpCode = OpCode {
    op: 0x23,
    name: "RLA",
    n: 2,
    cyc: 8,
};
pub const RLA_ZP: OpCode = OpCode {
    op: 0x27,
    name: "RLA",
    n: 2,
    cyc: 5,
};
pub const RLA_ZP_X: OpCode = OpCode {
    op: 0x37,
    name: "RLA",
    n: 2,
    cyc: 6,
};
pub const RRA_ABS: OpCode = OpCode {
    op: 0x6F,
    name: "RRA",
    n: 3,
    cyc: 6,
};
pub const RRA_ABS_X: OpCode = OpCode {
    op: 0x7F,
    name: "RRA",
    n: 3,
    cyc: 7,
};
pub const RRA_ABS_Y: OpCode = OpCode {
    op: 0x7B,
    name: "RRA",
    n: 3,
    cyc: 7,
};
pub const RRA_IND_Y: OpCode = OpCode {
    op: 0x73,
    name: "RRA",
    n: 2,
    cyc: 8,
};
pub const RRA_X_IND: OpCode = OpCode {
    op: 0x63,
    name: "RRA",
    n: 2,
    cyc: 8,
};
pub const RRA_ZP: OpCode = OpCode {
    op: 0x67,
    name: "RRA",
    n: 2,
    cyc: 5,
};
pub const RRA_ZP_X: OpCode = OpCode {
    op: 0x77,
    name: "RRA",
    n: 2,
    cyc: 6,
};
pub const SAX_ABS: OpCode = OpCode {
    op: 0x8F,
    name: "SAX",
    n: 3,
    cyc: 4,
};
pub const SAX_X_IND: OpCode = OpCode {
    op: 0x83,
    name: "SAX",
    n: 2,
    cyc: 6,
};
pub const SAX_ZP: OpCode = OpCode {
    op: 0x87,
    name: "SAX",
    n: 2,
    cyc: 3,
};
pub const SAX_ZP_Y: OpCode = OpCode {
    op: 0x97,
    name: "SAX",
    n: 2,
    cyc: 4,
};
pub const SBC_EB_IMM: OpCode = OpCode {
    op: 0xEB,
    name: "SBC",
    n: 2,
    cyc: 2,
};
pub const SKB_80_IMM: OpCode = OpCode {
    op: 0x80,
    name: "SKB",
    n: 2,
    cyc: 2,
};
pub const SKB_82_IMM: OpCode = OpCode {
    op: 0x82,
    name: "SKB",
    n: 2,
    cyc: 2,
};
pub const SKB_89_IMM: OpCode = OpCode {
    op: 0x89,
    name: "SKB",
    n: 2,
    cyc: 2,
};
pub const SKB_C2_IMM: OpCode = OpCode {
    op: 0xC2,
    name: "SKB",
    n: 2,
    cyc: 2,
};
pub const SKB_E2_IMM: OpCode = OpCode {
    op: 0xE2,
    name: "SKB",
    n: 2,
    cyc: 2,
};
pub const SLO_ABS: OpCode = OpCode {
    op: 0x0F,
    name: "SLO",
    n: 3,
    cyc: 6,
};
pub const SLO_ABS_X: OpCode = OpCode {
    op: 0x1F,
    name: "SLO",
    n: 3,
    cyc: 7,
};
pub const SLO_ABS_Y: OpCode = OpCode {
    op: 0x1B,
    name: "SLO",
    n: 3,
    cyc: 7,
};
pub const SLO_IND_Y: OpCode = OpCode {
    op: 0x13,
    name: "SLO",
    n: 2,
    cyc: 8,
};
pub const SLO_X_IND: OpCode = OpCode {
    op: 0x03,
    name: "SLO",
    n: 2,
    cyc: 8,
};
pub const SLO_ZP: OpCode = OpCode {
    op: 0x07,
    name: "SLO",
    n: 2,
    cyc: 5,
};
pub const SLO_ZP_X: OpCode = OpCode {
    op: 0x17,
    name: "SLO",
    n: 2,
    cyc: 6,
};
pub const SRE_ABS: OpCode = OpCode {
    op: 0x4F,
    name: "SRE",
    n: 3,
    cyc: 6,
};
pub const SRE_ABS_X: OpCode = OpCode {
    op: 0x5F,
    name: "SRE",
    n: 3,
    cyc: 7,
};
pub const SRE_ABS_Y: OpCode = OpCode {
    op: 0x5B,
    name: "SRE",
    n: 3,
    cyc: 7,
};
pub const SRE_IND_Y: OpCode = OpCode {
    op: 0x53,
    name: "SRE",
    n: 2,
    cyc: 8,
};
pub const SRE_X_IND: OpCode = OpCode {
    op: 0x43,
    name: "SRE",
    n: 2,
    cyc: 8,
};
pub const SRE_ZP: OpCode = OpCode {
    op: 0x47,
    name: "SRE",
    n: 2,
    cyc: 5,
};
pub const SRE_ZP_X: OpCode = OpCode {
    op: 0x57,
    name: "SRE",
    n: 2,
    cyc: 6,
};
pub const XAA_IMM: OpCode = OpCode {
    op: 0x8B,
    name: "XAA",
    n: 2,
    cyc: 2,
};

// Other illegal opcodes
pub const LAS_ABS_Y: OpCode = OpCode {
    op: 0xBB,
    name: "LAS",
    n: 3,
    cyc: 4,
};
pub const LXA_IMM: OpCode = OpCode {
    op: 0xAB,
    name: "LXA",
    n: 2,
    cyc: 2,
};
pub const SHA_ABS_Y: OpCode = OpCode {
    op: 0x9F,
    name: "SHA",
    n: 3,
    cyc: 5,
};
pub const SHA_IND_Y: OpCode = OpCode {
    op: 0x93,
    name: "SHA",
    n: 2,
    cyc: 6,
};
pub const SHX_ABS_Y: OpCode = OpCode {
    op: 0x9E,
    name: "SHX",
    n: 3,
    cyc: 5,
};
pub const SHY_ABS_X: OpCode = OpCode {
    op: 0x9C,
    name: "SHY",
    n: 3,
    cyc: 5,
};
pub const TAS_ABS_Y: OpCode = OpCode {
    op: 0x9B,
    name: "TAS",
    n: 3,
    cyc: 5,
};

// KIL
pub const KIL: OpCode = OpCode {
    op: 0x02,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_12: OpCode = OpCode {
    op: 0x12,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_22: OpCode = OpCode {
    op: 0x22,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_32: OpCode = OpCode {
    op: 0x32,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_42: OpCode = OpCode {
    op: 0x42,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_52: OpCode = OpCode {
    op: 0x52,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_62: OpCode = OpCode {
    op: 0x62,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_72: OpCode = OpCode {
    op: 0x72,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_92: OpCode = OpCode {
    op: 0x92,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_B2: OpCode = OpCode {
    op: 0xB2,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_D2: OpCode = OpCode {
    op: 0xD2,
    name: "KIL",
    n: 1,
    cyc: 2,
};
pub const KIL_F2: OpCode = OpCode {
    op: 0xF2,
    name: "KIL",
    n: 1,
    cyc: 2,
};
