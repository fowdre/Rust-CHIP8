use super::super::{chip8_constants, Chip8, Instruction};

/// 0xDxyn - DRW Vx, Vy, n
///
/// Display a sprite at coordinate (Vx, Vy) that has a width of 8 pixels and a height of n pixels.
/// VF is set if there is a collision.
fn DRW_Vx_Vy_n(chip8: &mut Chip8, instruction: Instruction) {
    let x = chip8.registers[instruction.x as usize] as usize % chip8_constants::WIDTH;
    let y = chip8.registers[instruction.y as usize] as usize % chip8_constants::HEIGHT;
    chip8.registers[0xF] = 0;

    for row in 0..instruction.n as usize {
        let pixel = chip8.memory[(chip8.index + row as u16) as usize];
        for bit in 0..8 {
            if (pixel & ((1 << 7) >> bit)) != 0 {
                let x = (x + bit) % chip8_constants::WIDTH;
                let y = (y + row) % chip8_constants::HEIGHT;
                let index = (y * chip8_constants::WIDTH).wrapping_add(x);

                if chip8.display[index] {
                    chip8.registers[0xF] = 1;
                }
                chip8.display[index] ^= true;
            }
        }
    }
}

pub fn _Dxxx(chip8: &mut Chip8, instruction: Instruction) {
    DRW_Vx_Vy_n(chip8, instruction);
}
