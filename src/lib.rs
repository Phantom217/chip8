pub mod instruction;
pub mod memory;
pub mod opcode;
pub mod register;
pub mod types;

use opcode::OpCode;

#[derive(Debug)]
pub struct Chip8 {
    /// System RAM
    pub ram: memory::Mem,

    /// General purpose registers, `V0..VF`
    regs: register::Regs,
    /// Address register
    i: u16,
    /// Program counter
    pc: u16,
    /// Delay timer
    dt: u8,
    /// Sound timer
    st: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            ram: memory::Mem::default(),

            regs: register::Regs::default(),
            i: 0x000,
            pc: 0x200,
            dt: 0x0,
            st: 0x0,
        }
    }

    pub fn get_opcode(&self, idx: u16) -> OpCode {
        let idx = idx as usize;
        OpCode::from((self.ram[idx], self.ram[idx + 1]))
    }

    pub fn load_rom(&mut self, content: Vec<u8>) {
        for (idx, byte) in content.into_iter().enumerate() {
            self.ram[self.pc as usize + idx] = byte;
        }
    }
}
