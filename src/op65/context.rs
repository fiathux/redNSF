use std::result::Result;

/// AddressError represents an error that can occur when accessing memory
pub enum AddressError {
    /// indecated address is not available for access.
    Unavailable(u16),
    /// indecated address is read-only.
    ReadOnly(u16),
    /// indecated address is write-only.
    WriteOnly(u16),
    /// indecated device requested a halt.
    Halt(u16),
    /// indecated address is out of bounds.
    OutOfBounds,
}

/// Flags represents the 6502 CPU flags
pub struct Flags {
    pub flags: u8,
}

/// Flag Carry
pub const C: Flags = Flags { flags: 0b0000_0001 };
/// Flag Zero
pub const Z: Flags = Flags { flags: 0b0000_0010 };
/// Flag Interrupt
pub const I: Flags = Flags { flags: 0b0000_0100 };
/// Flag Decimal
pub const D: Flags = Flags { flags: 0b0000_1000 };
/// Flag Break
pub const B: Flags = Flags { flags: 0b0001_0000 };
/// Flag Unused
pub const U: Flags = Flags { flags: 0b0010_0000 };
/// Flag Overflow
pub const O: Flags = Flags { flags: 0b0100_0000 };
/// Flag Negative
pub const N: Flags = Flags { flags: 0b1000_0000 };

/// Registers represents the 6502 CPU registers
pub struct Registers {
    pub a: u8,        // accumulator
    pub x: u8,        // x index
    pub y: u8,        // y index
    pub sp: u8,       // stack pointer
    pub pc: u16,      // program counter
    pub flags: Flags, // processor status
}

/// Bus provides a way to access 6502 address space
pub trait Bus {
    /// read a byte from the address space
    fn get(&self, addr: u16) -> Result<u8, AddressError>;

    /// write a byte to the address space
    fn set(&self, addr: u16, val: u8) -> Result<(), AddressError>;

    /// read a little-endian word from the address space, which be used for
    /// reading 16-bit addresses
    fn get_pointer(&self, addr: u16) -> Result<u16, AddressError>;

    /// read a data-window of memory from the address space
    fn get_window(&self, addr: u16, size: u16) -> Result<&[u8], AddressError>;

    /// write a data-window of memory to the address space
    fn set_window(&self, addr: u16, val: &[u8]) -> Result<(), AddressError>;
}

// Stringfy the AddressError
impl std::fmt::Display for AddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AddressError::Unavailable(addr) => {
                write!(f, "<AddressError> Address unavailable: ${:04X}", addr)
            }
            AddressError::ReadOnly(addr) => {
                write!(f, "<AddressError> Address read-only: ${:04X}", addr)
            }
            AddressError::WriteOnly(addr) => {
                write!(f, "<AddressError> Address write-only: ${:04X}", addr)
            }
            AddressError::Halt(addr) => {
                write!(f, "<AddressError> Address halt: ${:04X}", addr)
            }
            AddressError::OutOfBounds => {
                write!(f, "<AddressError> Address out of bounds")
            }
        }
    }
}

// Common methods for Flags
impl Flags {
    /// check the carry flag
    pub fn c(&self) -> bool {
        self.flags & 0b0000_0001 != 0
    }

    /// check the zero flag
    pub fn z(&self) -> bool {
        self.flags & 0b0000_0010 != 0
    }

    /// check the interrupt flag
    pub fn i(&self) -> bool {
        self.flags & 0b0000_0100 != 0
    }

    /// check the decimal flag
    pub fn d(&self) -> bool {
        self.flags & 0b0000_1000 != 0
    }

    /// check the break flag
    pub fn b(&self) -> bool {
        self.flags & 0b0001_0000 != 0
    }

    /// check the unused flag
    pub fn u(&self) -> bool {
        self.flags & 0b0010_0000 != 0
    }

    /// check the overflow flag
    pub fn v(&self) -> bool {
        self.flags & 0b0100_0000 != 0
    }

    /// check the negative flag
    pub fn n(&self) -> bool {
        self.flags & 0b1000_0000 != 0
    }
}

// Bitwise AND for Flags
impl std::ops::BitAnd<Flags> for Flags {
    type Output = Flags;

    fn bitand(self, rhs: Flags) -> Flags {
        Flags {
            flags: self.flags & rhs.flags,
        }
    }
}

// Bitwise OR for Flags
impl std::ops::BitOr<Flags> for Flags {
    type Output = Flags;

    fn bitor(self, rhs: Flags) -> Flags {
        Flags {
            flags: self.flags | rhs.flags,
        }
    }
}

// Stringfy the Flags
impl std::fmt::Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "C:{}, Z:{}, I:{}, D:{}, B:{}, U:{}, V:{}, N:{}",
            if self.c() { 1 } else { 0 },
            if self.z() { 1 } else { 0 },
            if self.i() { 1 } else { 0 },
            if self.d() { 1 } else { 0 },
            if self.b() { 1 } else { 0 },
            if self.u() { 1 } else { 0 },
            if self.v() { 1 } else { 0 },
            if self.n() { 1 } else { 0 },
        )
    }
}

// Convert Flags to u8
impl std::convert::From<u8> for Flags {
    fn from(val: u8) -> Flags {
        Flags { flags: val }
    }
}

// Convert u8 to Flags
impl std::convert::From<Flags> for u8 {
    fn from(val: Flags) -> u8 {
        val.flags
    }
}
