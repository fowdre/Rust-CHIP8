use super::super::{Chip8, Instruction};

/// 0x7xnn - ADD Vx, byte
///
/// Adds the value nn to the value of register Vx, then stores the result in Vx.
fn ADD_Vx_byte(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] =
        chip8.registers[instruction.x as usize].wrapping_add(instruction.nn);
}

/// 0x6xnn - LD Vx, byte
pub fn _7xxx(chip8: &mut Chip8, instruction: Instruction) {
    ADD_Vx_byte(chip8, instruction)
}
