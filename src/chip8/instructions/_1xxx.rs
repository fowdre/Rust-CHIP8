use super::super::{Chip8, Instruction};

/// 0x1nnn - JP addr
///
/// Jump to address nnn.
fn JP_addr(chip8: &mut Chip8, instruction: Instruction) {
    chip8.pc = instruction.nnn;
}

pub fn _1xxx(chip8: &mut Chip8, instruction: Instruction) {
    JP_addr(chip8, instruction);
}
