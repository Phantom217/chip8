pub mod instruction;
pub mod memory;
pub mod opcode;
pub mod register;
pub mod types;

#[derive(Debug)]
pub struct Chip8 {
    mem: memory::Mem,

    regs: register::Regs,
}
