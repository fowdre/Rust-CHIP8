use super::super::{Chip8, Instruction};

/// 0x00E0 - CLS
///
/// Clear the display.
const fn CLS(chip8: &mut Chip8) {
    chip8.display = [false; 64 * 32];
}

/// 0x00EE - RET
///
/// Return from a subroutine.
fn RET(chip8: &mut Chip8) {
    chip8.sp = chip8.sp.wrapping_sub(1);
    chip8.pc = chip8.stack[chip8.sp as usize];
}

/// Handle `0x0???` opcodes.
pub fn _0xxx(chip8: &mut Chip8, instruction: Instruction) {
    match instruction.opcode {
        0x00E0 => CLS(chip8),
        0x00EE => RET(chip8),
        _ => panic!("Unknown opcode [0x0???]: {:X}", instruction.opcode),
    }
}
