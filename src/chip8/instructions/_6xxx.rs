use super::super::{Chip8, Instruction};

/// 0x6xnn - LD Vx, byte
///
/// Load byte nn into register Vx.
fn LD_Vx_byte(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] = instruction.nn;
}

/// 0x6xnn - LD Vx, byte
pub fn _6xxx(chip8: &mut Chip8, instruction: Instruction) {
    LD_Vx_byte(chip8, instruction)
}
