use std::fmt;
use std::ops::{Index, IndexMut};

const MEM_SIZE: usize = 4 * 1024;
const NUM_GP_REGS: usize = 16;

#[derive(Debug)]
pub struct Chip8 {
    mem: Mem,

    regs: Regs,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Regs([u8; NUM_GP_REGS]);

impl Default for Regs {
    fn default() -> Self {
        Self([0x0; NUM_GP_REGS])
    }
}

impl Index<u8> for Regs {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u8> for Regs {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[repr(transparent)]
pub struct Mem([u8; MEM_SIZE]);

impl Default for Mem {
    fn default() -> Self {
        Self([0x00; MEM_SIZE])
    }
}

impl Index<u16> for Mem {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Mem {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // this function doesn't seem efficient
        let mut vec: Vec<String> = Vec::with_capacity(17);
        vec.push(String::from("Address"));
        for digit in 0x0..0xF + 1 {
            vec.push(format!("{:X} ", digit));
        }

        // write out the headers
        for header in &vec {
            write!(f, "{} ", header)?;
        }
        write!(f, "\n")?;
        // we're finished printing the headers now
        // clear the vec and get it ready for a line of data
        vec.clear();

        let mut pc = 0x00;
        for chunk in self.0.chunks(16) {
            vec.push(format!("${:#06X}", pc));
            for addr in chunk {
                vec.push(format!("{:02X}", addr));
            }
            for i in &vec {
                write!(f, "{} ", i)?;
            }
            write!(f, "\n")?;
            vec.clear();
            pc += 0x10;
        }

        Ok(())
    }
}

pub mod types {
    use std::fmt;

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
}

pub mod instruction {
    use std::fmt;

    use log::trace;

    use super::{
        opcode::{OpCode, Operands},
        Chip8,
    };

    pub type InstrFn = fn(&mut Chip8, Operands) -> bool;
    pub type InstrName = &'static str;

    pub struct Instruction {
        opcode: OpCode,
        name: InstrName,
        operands: Operands,
        instruction: InstrFn,
    }

    impl Instruction {
        /// Create a new `Instruction`
        fn new(opcode: OpCode, name: InstrName, ops: Operands, inst: InstrFn) -> Self {
            Self {
                opcode,
                name,
                operands: ops,
                instruction: inst,
            }
        }

        /// Execute an `Instruction`
        fn exec(self, chip8: &mut Chip8) {
            trace!("Execute `{}`", self);
            let inst = self.instruction;
            inst(chip8, self.operands);
        }
    }

    impl fmt::Display for Instruction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "({:04X}) {:<4}\t{:}",
                self.opcode, self.name, self.operands
            )
        }
    }
}

pub mod opcode {
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
}
