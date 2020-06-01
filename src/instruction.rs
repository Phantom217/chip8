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
