//! Chip-8 registers.
//!
//! Chip-8 has 16 general purpose 8-bit registers, usually referred to as `Vx`, where `x` is a
//! hexadecimal digit (`0..F`). There is also a 16-bit register called `I`. This register is
//! generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually
//! used.
//!
//! The `VF` register should not be used by any program, as it is used as a flag by some
//! instructions. See [`instruction`] for details.
//!
//! Chip-8 also has two special purpose 8-bit registers, for the delay and sound timers. When
//! these registers are non-zero, they are automatically decremented at a rate of 60Hz. See the
//! section 2.5, Timers & Sound, for more information on these.
//!
//! There are also some "pseudo-registers" which are not accessable from Chip-8 programs. The
//! program counter (`PC`) should be 16-bit, and is used to store the currently executing address.
//! The stack pointer (`SP`) can be 8-bit, it is used to point to the topmost level of the stack.
//!
//! The stack is an array of 16 16-bit values, used to store the address that the interpreter
//! shoud return to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested
//! subroutines.
//!
//! [`instruction`]: super::instruction

use std::ops::{Index, IndexMut};

use crate::types::Nibble;

/// Memory address for program (ROM) start.
pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
#[repr(transparent)]
pub struct Regs([u8; Self::NUM_GP_REGS]);

impl Regs {
    /// Number of general purpose registers, `Vx`
    const NUM_GP_REGS: usize = 16;

    // XXX: this method is not required since we impl Default, and that's all we call in new()
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Regs {
    fn default() -> Self {
        Self([0x00; Self::NUM_GP_REGS])
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
