pub mod error;
pub mod instruction;
pub mod memory;
pub mod opcode;
pub mod register;
pub mod types;

use error::{Chip8Error, Result};
use opcode::OpCode;

/// CPU clock speed.
const CLOCK_HZ: f32 = 600.0;
/// Size of the stack.
const STACK_SIZE: usize = 16;

pub trait Emulator: std::fmt::Debug {
    /// Load a rom into memory of the emulator.
    fn load_rom(&mut self, rom: Vec<u8>) -> Result<()>;
}

#[derive(Debug)]
pub struct Chip8 {
    /// System RAM
    pub ram: memory::Ram, // TODO: remove `pub`

    /// General purpose registers, `V0..VF`
    regs: register::Regs,
    /// Address register
    i: u16,
    /// Program counter
    pc: u16,
    /// Delay timer
    dt: u8,
    /// Sound timer.
    st: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            ram: memory::Ram::default(),

            regs: register::Regs::default(),
            i: 0x000,
            pc: register::PROGRAM_START,
            dt: 0x0,
            st: 0x0,
        }

        // TODO: load builtin font
    }

    pub fn get_opcode(&self, idx: u16) -> OpCode {
        let idx = idx as usize;
        OpCode::from((self.ram[idx], self.ram[idx + 1]))
    }
}

impl Emulator for Chip8 {
    fn load_rom(&mut self, rom: Vec<u8>) -> Result<()> {
        use std::io;
        if rom.len() > memory::Ram::RAM_SIZE {
            return Err(Chip8Error::Io(io::Error::new(
                io::ErrorKind::WriteZero,
                "ROM was larger than available RAM",
            )));
        }
        for (idx, byte) in rom.into_iter().enumerate() {
            self.ram[self.pc as usize + idx] = byte;
        }

        Ok(())
    }
}
