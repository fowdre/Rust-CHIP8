use super::super::{chip8_constants, Chip8, Instruction};

/// 0xFx07 - LD Vx, DT
///
/// Set Vx = delay timer value.
pub fn LD_Vx_DT(chip8: &mut Chip8, instruction: Instruction) {
    chip8.registers[instruction.x as usize] = chip8.delay_timer;
}

/// 0xFx0A - LD Vx, K
///
/// Wait for a key press, store the value of the key in Vx.
pub fn LD_Vx_K(chip8: &mut Chip8, instruction: Instruction) {
    chip8.pause_flag = true;

    if chip8.key_pressed {
        for i in 0..16 {
            if chip8.keypad[i] {
                chip8.registers[instruction.x as usize] = i as u8;
                chip8.key_pressed = false;
                break;
            }
        }
    } else {
        chip8.pc = chip8.pc.wrapping_sub(2);
    }
}

/// 0xFx15 - LD DT, Vx
///
/// Set delay timer to Vx.
pub fn LD_DT_Vx(chip8: &mut Chip8, instruction: Instruction) {
    chip8.delay_timer = chip8.registers[instruction.x as usize];
}

/// 0xFx18 - LD ST, Vx
///
/// Set sound timer to Vx.
pub fn LD_ST_Vx(chip8: &mut Chip8, instruction: Instruction) {
    chip8.sound_timer = chip8.registers[instruction.x as usize];
}

/// 0xFx1E - ADD I, Vx
///
/// I = I + Vx
pub fn ADD_I_Vx(chip8: &mut Chip8, instruction: Instruction) {
    chip8.index = chip8
        .index
        .wrapping_add(chip8.registers[instruction.x as usize] as u16);
}

/// 0xFx29 - LD F, Vx
///
/// I = digit Vx sprite address.
pub fn LD_F_Vx(chip8: &mut Chip8, instruction: Instruction) {
    chip8.index = (chip8_constants::FONTSET_START_ADDRESS as u16)
        .wrapping_add((chip8.registers[instruction.x as usize] as u16).wrapping_mul(5));
}

/// 0xFx33 - LD B, Vx
///
/// Store BCD representation of Vx in memory locations I, I+1, and I+2.
pub fn LD_B_Vx(chip8: &mut Chip8, instruction: Instruction) {
    let mut x = chip8.registers[instruction.x as usize];
    let index = chip8.index as usize;

    chip8.memory[index.wrapping_add(2)] = x % 10;
    x /= 10;
    chip8.memory[index.wrapping_add(1)] = x % 10;
    x /= 10;
    chip8.memory[index] = x % 10;
}

/// 0xFx55 - LD [I], Vx
///
/// Store V0 to Vx in memory starting at address I.
pub fn LD_I_Vx(chip8: &mut Chip8, instruction: Instruction) {
    for i in 0..=instruction.x {
        chip8.memory[(chip8.index + i as u16) as usize] = chip8.registers[i as usize];
    }
}

/// 0xFx65 - LD Vx, [I]
///
/// Read V0 to Vx from memory starting at address I.
pub fn LD_Vx_I(chip8: &mut Chip8, instruction: Instruction) {
    for i in 0..=instruction.x {
        chip8.registers[i as usize] = chip8.memory[(chip8.index + i as u16) as usize];
    }
}

pub fn _Fxxx(chip8: &mut Chip8, instruction: Instruction) {
    match instruction.nn {
        0x07 => LD_Vx_DT(chip8, instruction),
        0x0A => LD_Vx_K(chip8, instruction),
        0x15 => LD_DT_Vx(chip8, instruction),
        0x18 => LD_ST_Vx(chip8, instruction),
        0x1E => ADD_I_Vx(chip8, instruction),
        0x29 => LD_F_Vx(chip8, instruction),
        0x33 => LD_B_Vx(chip8, instruction),
        0x55 => LD_I_Vx(chip8, instruction),
        0x65 => LD_Vx_I(chip8, instruction),
        _ => panic!("Unknown opcode [0xF???]: {:X}", instruction.opcode),
    }
}
