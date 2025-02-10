use super::super::{Chip8, Instruction};

/// 0xBnnn - JP V0, addr
///
/// Jump to location nnn + V0.
pub fn JP_V0_addr(chip8: &mut Chip8, instruction: Instruction) {
    chip8.pc = instruction.nnn + chip8.registers[0] as u16;
}

/// 0xBnnn - JP V0, addr
pub fn _Bxxx(chip8: &mut Chip8, instruction: Instruction) {
    JP_V0_addr(chip8, instruction);
}
