use super::super::{Chip8, Instruction};

/// 0x5xy0 - SE Vx, Vy
///
/// Skip next instruction if Vx = Vy.
fn SE_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    if chip8.registers[instruction.x as usize] == chip8.registers[instruction.y as usize] {
        chip8.pc = chip8.pc.wrapping_add(2);
    }
}

/// 0x5xy0 - SE Vx, Vy
pub fn _5xxx(chip8: &mut Chip8, instruction: Instruction) {
    SE_Vx_Vy(chip8, instruction)
}
