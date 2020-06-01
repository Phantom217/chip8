use std::fmt;

/// A byte (8 bits)
pub type Byte = u8;

/// Absolute memory address
///
/// Valid values are within `0x000..0xFFF`, inclusive
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Addr(u16);

impl From<u16> for Addr {
    /// Convert from a `u16` to an `Addr`, ignoring any high bits
    fn from(bits: u16) -> Self {
        Self(bits & 0x0FFF)
    }
}

impl From<Addr> for usize {
    fn from(addr: Addr) -> Self {
        addr.0 as usize
    }
}

impl fmt::UpperHex for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::UpperHex::fmt(&val, f)
    }
}

impl fmt::LowerHex for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::LowerHex::fmt(&val, f)
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#05X}", self.0)
    }
}

/// A nibble (4 bits)
///
/// Valid values are within `0x0..0xF`, inclusive
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Nibble(u8);

impl From<u8> for Nibble {
    /// Convert from a `u8` to a `Nibble`, ignoring any high bits
    fn from(bits: u8) -> Self {
        Self(bits & 0x0F)
    }
}

impl From<Nibble> for usize {
    fn from(nibble: Nibble) -> Self {
        nibble.0 as usize
    }
}

impl fmt::UpperHex for Nibble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::UpperHex::fmt(&val, f)
    }
}

impl fmt::LowerHex for Nibble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::LowerHex::fmt(&val, f)
    }
}

impl fmt::Display for Nibble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::Display::fmt(&val, f)
    }
}