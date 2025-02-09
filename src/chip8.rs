use super::constants::chip8_constants;

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    display: [bool; 64 * 32],
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: chip8_constants::START_ADDRESS as u16,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            display: [false; 64 * 32],
            opcode: 0,
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), std::io::Error> {
        let rom = std::fs::read(path)?;

        for (i, &byte) in rom.iter().enumerate() {
            self.memory[chip8_constants::START_ADDRESS + i] = byte;
        }

        Ok(())
    }

    pub fn load_fontset(&mut self) {
        for (i, &byte) in chip8_constants::FONTSET.iter().enumerate() {
            self.memory[chip8_constants::FONTSET_START_ADDRESS + i] = byte;
        }
    }
}
