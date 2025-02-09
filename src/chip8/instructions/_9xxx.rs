use super::super::{Chip8, Instruction};

/// 0x9xy0 - SNE Vx, Vy
///
/// Skip next instruction if Vx != Vy.
pub fn SNE_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    if chip8.registers[instruction.x as usize] != chip8.registers[instruction.y as usize] {
        chip8.pc = chip8.pc.wrapping_add(2);
    }
}

pub fn _9xxx(chip8: &mut Chip8, instruction: Instruction) {
    SNE_Vx_Vy(chip8, instruction)
}
