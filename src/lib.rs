pub mod instruction;
pub mod opcode;
pub mod types;

use std::fmt;
use std::ops::{Index, IndexMut};

use crate::types::{Addr, Nibble};

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

// XXX: this impl is not required since we impl Default, and that's all we call in new()
impl Regs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Regs {
    fn default() -> Self {
        Self([0x00; NUM_GP_REGS])
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

impl Index<Nibble> for Regs {
    type Output = u8;

    fn index(&self, index: Nibble) -> &Self::Output {
        &self.0[usize::from(index)]
    }
}

impl IndexMut<Nibble> for Regs {
    fn index_mut(&mut self, index: Nibble) -> &mut Self::Output {
        &mut self.0[usize::from(index)]
    }
}

#[repr(transparent)]
pub struct Mem([u8; MEM_SIZE]);

// XXX: this impl is not required since we impl Default, and that's all we call in new()
impl Mem {
    /// Create new `Mem`
    pub fn new() -> Self {
        Self::default()
    }
}

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

impl Index<Addr> for Mem {
    type Output = u8;

    fn index(&self, index: Addr) -> &Self::Output {
        &self.0[usize::from(index)]
    }
}

impl IndexMut<Addr> for Mem {
    fn index_mut(&mut self, index: Addr) -> &mut Self::Output {
        &mut self.0[usize::from(index)]
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
