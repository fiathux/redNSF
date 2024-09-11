use crate::op65::context::{AddressError, Bus, Registers};

pub trait Addressing {
    fn addr(&self, bus: &dyn Bus, reg: &Registers)
        -> Result<u16, AddressError>;
}

/// Absolute Addressing - $HHLL
pub struct OpAbs {
    pub addr: u16,
}

/// Absolute Addressing with X Offset - $HHLL,X
pub struct OpAbsX {
    pub addr: u16,
}

/// Absolute Addressing with Y Offset - $HHLL,Y
pub struct OpAbsY {
    pub addr: u16,
}

/// Immediate Number - #$NN
pub struct OpImmd {
    pub val: u8,
}

/// Implied
pub struct OpImpl;

/// Indirect Addressing - ($HHLL)
pub struct OpInd {
    pub ind: u16,
}

/// Indexed Indirect Addressing - ($LL,X)
pub struct OpIndX {
    pub ind: u8,
}

/// Indirect Indexed Addressing - ($LL),Y
pub struct OpIndY {
    pub ind: u8,
}

/// Relative Addressing - $NN
pub struct OpRel {
    pub rel: u8,
}

/// Zero Page Addressing - $LL
pub struct OpZp {
    pub addr: u8,
}

/// Zero Page Addressing with X Offset - $LL,X
pub struct OpZpX {
    pub addr: u8,
}

/// Zero Page Addressing with Y Offset - $LL,Y
pub struct OpZpY {
    pub addr: u8,
}
