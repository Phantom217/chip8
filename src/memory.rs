//! Chip-8 memory.
//!
//! The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM, from location
//! `0x000..0xFFF`, inclusive. The first 512 bytes, from `0x000..0x1FF`, inclusive, are where the
//! original interpreter was located, and should not be used by programs.
//!
//! Most Chip-8 programs start at location `0x200`, but some begin at `0x600`. Programs beginning
//! at `0x600` are intended for the ETI 660 computer.
//!
//! Memory Map:
//! ```text
//! +---------------+= 0xFFF End of Chip-8 RAM
//! |               |
//! |               |
//! |               |
//! |               |
//! |               |
//! | 0x200 to 0xFFF|
//! |     Chip-8    |
//! | Program / Data|
//! |     Space     |
//! |               |
//! |               |
//! |               |
//! +- - - - - - - -+= 0x600 Start of ETI 660 Chip-8 programs
//! |               |
//! |               |
//! |               |
//! +---------------+= 0x200 Start of most Chip-8 programs
//! | 0x000 to 0x1FF|
//! | Reserved for  |
//! |  interpreter  |
//! +---------------+= 0x000 Start of Chip-8 RAM
//! ```

use std::fmt;
use std::ops::{Index, IndexMut};

use crate::types::Addr;

/// Struct representing the CHIP-8 system RAM
#[repr(transparent)]
pub struct Mem([u8; Self::MEM_SIZE]);

impl Mem {
    /// Size of memory, 4096 bytes
    const MEM_SIZE: usize = 0x1000;

    // XXX: this method is not required since we impl Default, and that's all we call in new()
    /// Create new `Mem`
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Mem {
    fn default() -> Self {
        Self([0x00; Self::MEM_SIZE])
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
