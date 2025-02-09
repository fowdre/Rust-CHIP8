use super::super::{Chip8, Instruction};

/// 0x4xnn - SNE Vx, byte
///
/// Skip next instruction if Vx != nn.
fn SNE_Vx_byte(chip8: &mut Chip8, instruction: Instruction) {
    if chip8.registers[instruction.x as usize] != instruction.nn {
        chip8.pc = chip8.pc.wrapping_add(2);
    }
}

pub fn _4xxx(chip8: &mut Chip8, instruction: Instruction) {
    SNE_Vx_byte(chip8, instruction)
}
