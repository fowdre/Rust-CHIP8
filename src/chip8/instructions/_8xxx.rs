use super::super::{Chip8, Instruction};

/// 0x8xy0 - LD Vx, Vy
///
/// Vx = Vy.
pub fn LD_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] = chip8.registers[instruction.y as usize];
}

/// 0x8xy1 - OR Vx, Vy
///
/// Vx = Vx | Vy.
pub fn OR_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] |= chip8.registers[instruction.y as usize];
}

/// 0x8xy2 - AND Vx, Vy
///
/// Vx = Vx & Vy.
pub fn AND_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] &= chip8.registers[instruction.y as usize];
}

/// 0x8xy3 - XOR Vx, Vy
///
/// Vx = Vx ^ Vy.
pub fn XOR_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] ^= chip8.registers[instruction.y as usize];
}

/// 0x8xy4 - ADD Vx, Vy
///
/// Vx = Vx + Vy.
/// Set VF to 1 if there is a carry, 0 otherwise.
pub fn ADD_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    let (result, overflow) = chip8.registers[instruction.x as usize]
        .overflowing_add(chip8.registers[instruction.y as usize]);

    chip8.registers[instruction.x as usize] = result;
    chip8.registers[0xF] = overflow as u8;
}

/// 0x8xy5 - SUB Vx, Vy
///
/// Vx = Vx - Vy.
/// Set VF to 0 if there is a borrow, 1 otherwise.
pub fn SUB_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    let (result, overflow) = chip8.registers[instruction.x as usize]
        .overflowing_sub(chip8.registers[instruction.y as usize]);

    chip8.registers[instruction.x as usize] = result;
    chip8.registers[0xF] = !overflow as u8;
}

/// 0x8xy6 - SHR Vx
///
/// Vx = Vx >> 1.
/// Set VF to the least significant bit of Vx before the shift.
pub fn SHR_Vx(chip8: &mut Chip8, instruction: Instruction) {
    let temp = chip8.registers[instruction.x as usize];

    chip8.registers[instruction.x as usize] >>= 1;
    chip8.registers[0xF] = temp & 0x1;
}

/// 0x8xy7 - SUBN Vx, Vy
///
/// Set VF to 0 if Vy > Vx, 1 otherwise. Then Vx = Vy - Vx.
pub fn SUBN_Vx_Vy(chip8: &mut Chip8, instruction: Instruction) {
    let temp = chip8.registers[instruction.y as usize];
    let (result, overflow) = chip8.registers[instruction.y as usize]
        .overflowing_sub(chip8.registers[instruction.x as usize]);

    chip8.registers[instruction.x as usize] = result;
    chip8.registers[0xF] = !overflow as u8;
}

/// 0x8xyE - SHL Vx
///
/// Vx = Vx << 1.
/// Set VF to the most significant bit of Vx before the shift.
pub fn SHL_Vx(chip8: &mut Chip8, instruction: Instruction) {
    let temp = chip8.registers[instruction.x as usize];

    chip8.registers[instruction.x as usize] <<= 1;
    chip8.registers[0xF] = (temp & (1 << 7)) >> 7;
}

pub fn _8xxx(chip8: &mut Chip8, instruction: Instruction) {
    match instruction.n {
        0x0 => LD_Vx_Vy(chip8, instruction),
        0x1 => OR_Vx_Vy(chip8, instruction),
        0x2 => AND_Vx_Vy(chip8, instruction),
        0x3 => XOR_Vx_Vy(chip8, instruction),
        0x4 => ADD_Vx_Vy(chip8, instruction),
        0x5 => SUB_Vx_Vy(chip8, instruction),
        0x6 => SHR_Vx(chip8, instruction),
        0x7 => SUBN_Vx_Vy(chip8, instruction),
        0xE => SHL_Vx(chip8, instruction),

        _ => panic!("Unknown opcode [0x8???]: {:X}", instruction.opcode),
    }
}
