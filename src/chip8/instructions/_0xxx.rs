use super::super::{Chip8, Instruction};

/// 0x00E0 - CLS
///
/// Clear the display
const fn CLS(chip8: &mut Chip8) {
    chip8.display = [false; 64 * 32];
}

pub fn _0xxx(chip8: &mut Chip8, instruction: Instruction) {
    match instruction.opcode {
        0x00E0 => {
            CLS(chip8);
        }
        0x00EE => {
            unimplemented!();
        }
        _ => panic!("Unknown opcode [0x0???]: {:X}", instruction.opcode),
    }
}
