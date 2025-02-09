use super::super::{Chip8, Instruction};

/// 0xEx9E - SKP Vx
///
/// Skip next instruction if key with the value of Vx is pressed.
fn SKP_Vx(chip8: &mut Chip8, instruction: Instruction) {
    if chip8.keypad[chip8.registers[instruction.x as usize] as usize] {
        chip8.pc = chip8.pc.wrapping_add(2);
    }
}

/// 0xExA1 - SKNP Vx
///
/// Skip next instruction if key with the value of Vx is not pressed.
fn SKNP_Vx(chip8: &mut Chip8, instruction: Instruction) {
    if !chip8.keypad[chip8.registers[instruction.x as usize] as usize] {
        chip8.pc = chip8.pc.wrapping_add(2);
    }
}

pub fn _Exxx(chip8: &mut Chip8, instruction: Instruction) {
    match instruction.nn {
        0x9E => SKP_Vx(chip8, instruction),
        0xA1 => SKNP_Vx(chip8, instruction),
        _ => panic!("Unknown opcode [0xE???]: {:X}", instruction.opcode),
    }
}
