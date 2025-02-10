use super::super::{Chip8, Instruction};

/// 0x3xnn - SE Vx, byte
///
/// Skip next instruction if Vx = nn.
fn SE_Vx_byte(chip8: &mut Chip8, instruction: Instruction) {
    if chip8.registers[instruction.x as usize] == instruction.nn {
        chip8.pc = chip8.pc.wrapping_add(2);
    }
}

/// 0x3xnn - SE Vx, byte
pub fn _3xxx(chip8: &mut Chip8, instruction: Instruction) {
    SE_Vx_byte(chip8, instruction)
}
