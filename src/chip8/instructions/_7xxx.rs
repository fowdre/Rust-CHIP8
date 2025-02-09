use super::super::{Chip8, Instruction};

/// 0x7xnn - ADD Vx, byte
///
/// Vx = Vx + nn.
fn ADD_Vx_byte(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] += instruction.nn;
}

pub fn _7xxx(chip8: &mut Chip8, instruction: Instruction) {
    ADD_Vx_byte(chip8, instruction)
}
