use super::constants::chip8_constants;

mod instructions;

// TODO: remove this
pub fn test(chip8: &mut Chip8, instruction: Instruction) {
    todo!("opcode: {:04X?}", instruction.opcode);
}

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
    lookup: [fn(&mut Chip8, Instruction); 16],
}

pub struct Instruction {
    opcode: u16,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
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
            lookup: [
                instructions::_0xxx::_0xxx,
                instructions::_1xxx::_1xxx,
                test, // 2
                test, // 3
                test, // 4
                test, // 5
                instructions::_6xxx::_6xxx,
                instructions::_7xxx::_7xxx,
                test, // 8
                test, // 9
                instructions::_Axxx::_Axxx,
                test, // B
                test, // C
                instructions::_Dxxx::_Dxxx,
                test, // E
                test, // F
            ],
        }
    }

    pub fn get_display(&self) -> &[bool; 64 * 32] {
        &self.display
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

    pub fn step(&mut self) {
        self.pc %= chip8_constants::RAM_SIZE as u16;
        self.opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;

        let instruction = Instruction {
            opcode: self.opcode,
            x: ((self.opcode & 0x0F00) >> 8) as u8,
            y: ((self.opcode & 0x00F0) >> 4) as u8,
            n: (self.opcode & 0x000F) as u8,
            nn: (self.opcode & 0x00FF) as u8,
            nnn: self.opcode & 0x0FFF,
        };

        (self.lookup[((self.opcode & 0xF000) >> 12) as usize])(self, instruction);
    }
}
