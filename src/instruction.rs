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
#![allow(unused_variables)]
use std::fmt;

use super::{
    opcode::{OpCode, Operands},
    Chip8,
};

/// The function an [`OpCode`] executes
///
/// [`OpCode`]: ../opcode/struct.OpCode.html
pub type InstrFn = fn(&mut Chip8, Operands);
pub type InstrName = &'static str;

pub struct Instruction {
    opcode: OpCode,
    name: InstrName,
    operands: Operands,
    #[allow(dead_code)]
    instruction: InstrFn,
}

impl Instruction {
    /// Create a new `Instruction`
    pub fn new(opcode: OpCode, name: InstrName, ops: Operands, inst: InstrFn) -> Self {
        Self {
            opcode,
            name,
            operands: ops,
            instruction: inst,
        }
    }

    /// Execute an `Instruction`
    pub fn exec(self, chip8: &mut Chip8) {
        log::trace!("Execute `{}`", self);
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

pub fn not_implemented(chip8: &mut Chip8, operands: Operands) {
    let instruction = chip8.get_opcode(chip8.pc - 2).decode();
    log::warn!("Ignoring unimplemented instruction: {}", instruction);
}

/// `0nnn - SYS addr`
///
/// Jump to a machine code routine at `nnn`.
///
/// This instruction is only used on the old computers on which Chip-8 was originally implemented.
///
/// **NOTE** It is ignored by modern interpreters.
pub fn sys(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `00E0 - CLS`
///
/// Clear the display.
pub fn clear(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `00EE - RET`
///
/// Return from a subroutine.
///
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts
/// 1 from the stack pointer.
pub fn r#return(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `1nnn - JP addr`
///
/// Jump to location `nnn`.
///
/// The interpreter sets the program counter to `nnn`.
pub fn jump(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `2nnn - CALL addr`
///
/// Call subroutine at `nnn`.
///
/// The interpreter increments the stack pointer, then puts the current `PC` on the top of the
/// stack. The `PC` is then set to `nnn`.
pub fn call(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `3xkk - SE Vx, byte`
///
/// Skip next instruction if `Vx = kk`.
///
/// The interpreter compares register `Vx` to `kk`, and if they are equal, increments the program
/// counter by 2.
pub fn skip_eq_byte(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `4xkk - SNE Vx, byte`
///
/// Skip next instruction if `Vx != kk`.
///
/// The interpreter compares register `Vx` to `kk`, and if they are not equal, increments the
/// program counter by 2.
pub fn skip_ne_byte(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `5xy0 - SE Vx, Vy`
///
/// Skip next instruction if `Vx = Vy`.
///
/// The interpreter compares register `Vx` to register `Vy`, and if they are equal, increments the
/// program counter by 2.
pub fn skip_eq(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `6xkk - LD Vx, byte`
///
/// Set `Vx = kk`.
///
/// The interpreter puts the value `kk` into register `Vx`.
pub fn load_byte(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `7xkk - ADD Vx, byte`
///
/// Set `Vx = Vx + kk`.
///
/// Adds the value kk to the value of register `Vx`, then stores the result in `Vx`.
pub fn add_byte(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy0 - LD Vx, Vy`
///
/// Set `Vx = Vy`.
///
/// Stores the value of register `Vy` in register `Vx`.
pub fn load(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy1 - OR Vx, Vy`
///
/// Set `Vx = Vx OR Vy`.
///
/// Performs a bitwise OR on the values of `Vx` and `Vy`, then stores the result in `Vx`.
///
/// **NOTE** A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
/// then the same bit in the result is also 1. Otherwise, it is 0.
pub fn or(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy2 - AND Vx, Vy`
///
/// Set `Vx = Vx AND Vy`.
///
/// Performs a bitwise AND on the values of `Vx` and `Vy`, then stores the result in `Vx`.
///
/// **NOTE** A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
/// then the same bit in the result is also 1. Otherwise, it is 0.
pub fn and(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy3 - XOR Vx, Vy`
///
/// Set `Vx = Vx XOR Vy`.
///
/// Performs a bitwise exclusive OR on the values of `Vx` and `Vy`, then stores the result in `Vx`.
///
/// **NOTE** An exclusive OR compares the corrseponding bits from two values, and if the bits are
/// not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
pub fn xor(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy4 - ADD Vx, Vy`
///
/// Set `Vx = Vx + Vy`, set `VF = carry`.
///
/// The values of `Vx` and `Vy` are added together. If the result is greater than 8 bits (i.e.,
/// `> 255`,) `VF` is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and
/// stored in `Vx`.
pub fn add(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy5 - SUB Vx, Vy`
///
/// Set `Vx = Vx - Vy`, set `VF = NOT borrow`.
///
/// If `Vx > Vy`, then `VF` is set to 1, otherwise 0. Then `Vy` is subtracted from `Vx`, and the
/// results stored in `Vx`.
pub fn sub(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy6 - SHR Vx {, Vy}`
///
/// Set `Vx = Vx SHR 1`.
///
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided
/// by 2.
pub fn shift_right(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xy7 - SUBN Vx, Vy`
///
/// Set `Vx = Vy - Vx`, set `VF = NOT borrow`.
///
/// If `Vy > Vx`, then `VF` is set to 1, otherwise 0. Then `Vx` is subtracted from `Vy`, and the
/// results stored in `Vx`.
pub fn sub_inv(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `8xyE - SHL Vx {, Vy}`
///
/// Set `Vx = Vx SHL 1`.
///
/// If the most-significant bit of `Vx` is 1, then `VF` is set to 1, otherwise to 0. Then `Vx` is
/// multiplied by 2.
pub fn shift_left(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `9xy0 - SNE Vx, Vy`
///
/// Skip next instruction if `Vx != Vy`.
///
/// The values of `Vx` and `Vy` are compared, and if they are not equal, the program counter is
/// increased by 2.
pub fn skip_ne(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Annn - LD I, addr`
///
/// Set `I = nnn`.
///
/// The value of register `I` is set to `nnn`.
pub fn load_i(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Bnnn - JP V0, addr`
///
/// Jump to location `nnn + V0`.
///
/// The program counter is set to `nnn` plus the value of `V0`.
pub fn jump0(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Cxkk - RND Vx, byte`
///
/// Set `Vx = random byte AND kk`.
///
/// The interpreter generates a random number from `0..255`, inclusive, which is then ANDed with
/// the value `kk`. The results are stored in `Vx`. See instruction [`8xy2`] for more information on
/// AND.
///
/// [`8xy2`]: TODO
pub fn rand_byte(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Dxyn - DRW Vx, Vy, nibble`
///
/// Display n-byte sprite starting at memory location I at (Vx, Vy), set `VF = collision`.
///
/// The interpreter reads n bytes from memory, starting at the address stored in `I`. These bytes
/// are then displayed as sprites on screen at coordinates `(Vx, Vy)`. Sprites are XORed onto the
/// existing screen. If this causes any pixels to be erased, `VF` is set to 1, otherwise it is set
/// to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it
/// wraps around to the opposite side of the screen. See instruction [`8xy3`] for more information
/// on XOR, and [`Display`], for more information on the Chip-8 screen and sprites.
///
/// [`8xy3`]: TODO
/// [`Display`]: TODO
pub fn draw_sprite(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Ex9E - SKP Vx`
///
/// Skip next instruction if key with the value of `Vx` is pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of `Vx` is currently in the down
/// position, `PC` is increased by 2.
pub fn skip_pressed(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `ExA1 - SKNP Vx`
///
/// Skip next instruction if key with the value of `Vx` is not pressed.
///
/// Checks the keyboard, and if the key corresponding to the value of `Vx` is currently in the up
/// position, `PC` is increased by 2.
pub fn skip_not_pressed(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx07 - LD Vx, DT`
///
/// Set `Vx = delay timer value`.
///
/// The value of `DT` is placed into `Vx`.
pub fn load_dt(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx0A - LD Vx, K`
///
/// Wait for a key press, store the value of the key in `Vx`.
///
/// All execution stops until a key is pressed, then the value of that key is stored in `Vx`.
pub fn wait_for_key(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx15 - LD DT, Vx`
///
/// Set `delay timer = Vx`.
///
/// `DT` is set equal to the value of `Vx`.
pub fn set_delay_timer(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx18 - LD ST, Vx`
///
/// Set `sound timer = Vx`.
///
/// `ST` is set equal to the value of `Vx`.
pub fn set_sound_timer(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx1E - ADD I, Vx`
///
/// Set `I = I + Vx`.
///
/// The values of `I` and `Vx` are added, and the results are stored in `I`.
pub fn add_i(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx29 - LD F, Vx`
///
/// Set `I = location of sprite for digit Vx`.
///
/// The value of `I` is set to the location for the hexadecimal sprite corresponding to the value
/// of `Vx`. See [`Display`], for more information on the Chip-8 hexadecimal font.
///
/// [`Display`]: TODO
pub fn load_sprite(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx33 - LD B, Vx`
///
/// Store BCD representation of `Vx` in memory locations `I`, `I+1`, and `I+2`.
///
/// The interpreter takes the decimal value of `Vx`, and places the hundreds digit in memory at
/// location in `I`, the tens digit at location `I+1`, and the ones digit at location `I+2`.
pub fn store_bcd(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx55 - LD [I], Vx`
///
/// Store registers `V0` through `Vx` in memory starting at location `I`.
///
/// The interpreter copies the values of registers `V0` through `Vx` into memory, starting at the
/// address in `I`.
pub fn store_regs(chip8: &mut Chip8, operands: Operands) {
    todo!()
}

/// `Fx65 - LD Vx, [I]`
///
/// Read registers `V0` through `Vx` from memory starting at location `I`.
///
/// The interpreter reads values from memory starting at location `I` into registers `V0` through
/// `Vx`.
pub fn load_regs(chip8: &mut Chip8, operands: Operands) {
    todo!()
}
