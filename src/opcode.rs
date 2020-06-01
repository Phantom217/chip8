use std::fmt;

use super::types::Nibble;

#[derive(Debug, Copy, Clone)]
pub struct OpCode(u16);

impl OpCode {
    pub fn to_match_tuple(&self) -> (Nibble, Nibble, Nibble, Nibble) {
        self.into()
    }
}

/// Operands variants for an opcode
#[derive(Debug)]
pub enum Operands {
    /// No operands
    Empty,
    /// 12 bit address (`nnn`)
    Address(u16),
    /// Register name (`x`)
    Reg(u8),
    /// Register names (`xy`)
    Regs(u8, u8),
    /// Register name and 8 bit constant (`xkk`)
    RegAndConst(u8, u8),
    /// Register names, and 4 bit constant (`xyn`)
    RegsAndConst(u8, u8, u8),
}

// Only need this is we can't get chunks() to work for [u8;2]
use std::convert::TryFrom;
impl TryFrom<&[u8]> for OpCode {
    type Error = String; // TODO: use proper error type

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(format!(
                "invalid slice length for opcode conversion: expected {}, got {}",
                2,
                value.len(),
            ));
        }

        let b1 = value[0] as u16;
        let b2 = value[1] as u16;

        Ok(Self((b1 << 8) | b2))
    }
}

impl From<&[u8; 2]> for OpCode {
    fn from(value: &[u8; 2]) -> Self {
        let b1 = value[0] as u16;
        let b2 = value[1] as u16;

        Self((b1 << 8) | b2)
    }
}
impl From<(u8, u8)> for OpCode {
    fn from(value: (u8, u8)) -> Self {
        let b1 = value.0 as u16;
        let b2 = value.1 as u16;

        Self((b1 << 8) | b2)
    }
}
impl From<&OpCode> for (Nibble, Nibble, Nibble, Nibble) {
    fn from(opcode: &OpCode) -> Self {
        (
            Nibble::from(((opcode.0 & 0xF000) >> 12) as u8),
            Nibble::from(((opcode.0 & 0x0F00) >> 8) as u8),
            Nibble::from(((opcode.0 & 0x00F0) >> 4) as u8),
            Nibble::from(((opcode.0 & 0x000F) >> 0) as u8),
        )
    }
}

impl fmt::UpperHex for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::UpperHex::fmt(&val, f)
    }
}
impl fmt::LowerHex for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::LowerHex::fmt(&val, f)
    }
}
impl fmt::Display for Operands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Empty => write!(f, ""),
            Self::Address(addr) => write!(f, "{:#03X}", addr),
            Self::Reg(vx) => write!(f, "V{:X}", vx),
            Self::Regs(vx, vy) => write!(f, "V{:X} V{:X}", vx, vy),
            Self::RegAndConst(vx, kk) => write!(f, "V{:X} {:#04X}", vx, kk),
            Self::RegsAndConst(vx, vy, n) => write!(f, "V{:X} V{:X} {:#03X}", vx, vy, n),
        }
    }
}
