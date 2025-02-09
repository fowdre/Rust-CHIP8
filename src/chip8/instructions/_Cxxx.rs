use super::super::{Chip8, Instruction};

/// 0xCxnn - RND Vx, byte
///
/// Set Vx = random byte AND nn.
pub fn RND_Vx_byte(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] = rand::random::<u8>() & instruction.nn;
}

pub fn _Cxxx(chip8: &mut Chip8, instruction: Instruction) {
    RND_Vx_byte(chip8, instruction)
}
