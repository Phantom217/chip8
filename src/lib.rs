pub mod error;
pub mod instruction;
pub mod memory;
pub mod opcode;
pub mod register;
pub mod types;

use std::io;
use std::path::Path;

use error::{Chip8Error, Result};
use opcode::OpCode;

/// CPU clock speed.
const CLOCK_HZ: f32 = 600.0;
/// Size of the stack.
const STACK_SIZE: usize = 16;

pub trait Emulator: std::fmt::Debug {
    /// Load a ROM into memory of the emulator.
    fn load_rom(&mut self, reader: &dyn AsRef<Path>) -> Result<()>;
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
    /// Initialize `Chip8` to default state and load in system fonts.
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

    /// Get [`OpCode`] from `idx`
    ///
    /// [`OpCode`]: opcode/struct.OpCode.html
    pub fn get_opcode(&self, idx: u16) -> OpCode {
        let idx = idx as usize;
        OpCode::from((self.ram[idx], self.ram[idx + 1]))
    }
}

impl Emulator for Chip8 {
    fn load_rom(&mut self, reader: &dyn AsRef<Path>) -> Result<()> {
        use memory::Ram;
        use std::fs;
        use std::io::Write;

        let rom = fs::read(reader)?;
        let rom_len = rom.len();

        // ensure the ROM is smaller than available RAM
        if rom_len > Ram::RAM_SIZE {
            log::error!(
                "Rom size ({}) is greater than available RAM ({})",
                rom_len,
                Ram::RAM_SIZE
            );
            return Err(Chip8Error::Io(io::Error::new(
                io::ErrorKind::WriteZero,
                "ROM was larger than available RAM",
            )));
        }

        // TODO: check if ROM is valid before loading it into memory
        //       (needs to contain at least 1 instruction)
        // TODO: Get range indexing to work without interacting with the underlying field
        let mut ram =
            io::BufWriter::new(&mut self.ram.0[register::PROGRAM_START as usize..Ram::RAM_SIZE]);
        ram.write_all(rom.as_ref())?;

        log::debug!("Loaded ROM of size {}", rom_len);
        Ok(())
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            ram: memory::Ram::default(),

            regs: register::Regs::default(),
            i: 0x000,
            pc: register::PROGRAM_START,
            dt: 0x0,
            st: 0x0,
        }
    }
}
