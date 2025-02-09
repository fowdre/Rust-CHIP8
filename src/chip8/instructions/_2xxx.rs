use super::super::{Chip8, Instruction};

/// 0x2nnn - CALL addr
///
/// Call subroutine at address nnn.
fn CALL_addr(chip8: &mut Chip8, instruction: Instruction) {
    chip8.stack[chip8.sp as usize] = chip8.pc;
    chip8.sp = chip8.sp.wrapping_add(1);
    chip8.pc = instruction.nnn;
}

pub fn _2xxx(chip8: &mut Chip8, instruction: Instruction) {
    CALL_addr(chip8, instruction)
}
