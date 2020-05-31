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
}

pub mod instruction {
    use std::fmt;

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

    #[derive(Debug, Copy, Clone)]
    pub struct OpCode(u16);

    impl OpCode {
        fn to_match_tuple(&self) -> (u8, u8, u8, u8) {
            self.into()
        }
    }

    #[derive(Debug)]
    pub enum Operands {
        Empty,
        Address(u16),
        Reg(u8),
        Regs(u8, u8),
        RegAndConst(u8, u8),
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
    impl From<&OpCode> for (u8, u8, u8, u8) {
        fn from(opcode: &OpCode) -> Self {
            (
                ((opcode.0 & 0xF000) >> 12) as u8,
                ((opcode.0 & 0x0F00) >> 8) as u8,
                ((opcode.0 & 0x00F0) >> 4) as u8,
                ((opcode.0 & 0x000F) >> 0) as u8,
            )
        }
    }

    impl fmt::UpperHex for OpCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let val = self.0;

            fmt::UpperHex::fmt(&val, f)
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
