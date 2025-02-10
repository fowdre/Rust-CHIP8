use super::super::{Chip8, Instruction};

/// 0xAnnn - LD I, addr
///
/// I = nnn.
const fn LD_I_addr(chip8: &mut Chip8, instruction: Instruction) {
    chip8.index = instruction.nnn;
}

/// 0xAnnn - LD I, addr
pub fn _Axxx(chip8: &mut Chip8, instruction: Instruction) {
    LD_I_addr(chip8, instruction);
}
