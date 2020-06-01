//! Chip-8 instructions.
//!
//! The original implementation of the Chip-8 language includes 36 different instructions,
//! including math, graphics, and flow control functions.
//!
//! Super Chip-48 added an additional 10 instructions, for a total of 46.
//!
//! All instructions are 2 bytes long and are stored most-significant-byte first. In memory, the
//! first byte of each instruction should be located at an even addresses. If a program includes
//! sprite data, it should be padded so any instructions following it will be properly situated in
//! RAM.
//!
//! In the descriptions of this module, the following variables are used:
//!
//! * `nnn` or `addr` - A 12-bit value, the lowest 12 bits of the instruction
//! * `n` or `nibble` - A 4-bit value, the lowest 4 bits of the instruction
//! * `x` - A 4-bit value, the lower 4 bits of the high byte of the instruction
//! * `y` - A 4-bit value, the upper 4 bits of the low byte of the instruction
//! * `kk` or `byte` - An 8-bit value, the lowest 8 bits of the instruction

use std::fmt;

use log::trace;

use super::{
    opcode::{OpCode, Operands},
    Chip8,
};

/// The function an [`OpCode`] executes
///
/// [`OpCode`]: ../opcode/struct.OpCode.html
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
