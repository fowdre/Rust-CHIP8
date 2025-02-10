//! # CHIP-8 Emulator Core
//!
//! This module implements the CHIP-8 emulator, including memory, registers, display, input handling,
//! and instruction execution. It also handles sound using the `rodio` crate for audio playback.
//!
//! ## Features
//! - CHIP-8 CPU emulation
//! - 4 KB of RAM with program memory starting at `0x200`
//! - 16 registers (`V0`-`VF`)
//! - Stack with 16 levels for subroutine calls
//! - Timers (delay and sound timers)
//! - Keypad input handling
//! - Display rendering (64x32 monochrome)
//! - Sound playback using `rodio`
//!
//! ## Used modules
//! - `instructions`: Handles CHIP-8 opcode execution.
//! - `constants`: Stores emulator constants (e.g., memory size, fontset, screen dimensions).
//!
//! ## Usage
//! The `Chip8` struct provides methods to initialize, load a ROM, update input, and execute instructions step by step.

use rodio::{Decoder, OutputStream, Sink};
use std::sync::{Arc, Mutex};

use super::constants::chip8_constants;

pub mod instructions;

impl std::fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chip8")
            .field("registers", &self.registers)
            .field("memory", &"[...]")
            .field("index", &self.index)
            .field("pc", &self.pc)
            .field("stack", &self.stack)
            .field("sp", &self.sp)
            .field("delay_timer", &self.delay_timer)
            .field("sound_timer", &self.sound_timer)
            .field("keypad", &self.keypad)
            .field("key_pressed", &self.key_pressed)
            .field("display", &"[...]")
            .field("opcode", &self.opcode)
            .finish()
    }
}

/// Represents the CHIP-8 system, including memory, registers, display, and execution state.
pub struct Chip8 {
    /// 16 general-purpose registers (`V0`-`VF`).
    registers: [u8; 16],

    /// 4 KB (4096 bytes) of memory.
    memory: [u8; 4096],

    /// 16-bit index register, used for memory operations.
    index: u16,

    /// 16-bit program counter, points to the next instruction.
    pc: u16,

    /// Stack for storing return addresses during subroutine calls.
    stack: [u16; 16],

    /// Stack pointer, keeps track of the stack level.
    sp: u8,

    /// Delay timer (decrements at `EMULATOR_IPS` Hz).
    delay_timer: u8,

    /// Sound timer (decrements at `EMULATOR_IPS` Hz, plays sound when zero).
    sound_timer: u8,

    /// CHIP-8 16-key keypad state (true = pressed, false = not pressed).
    keypad: [bool; 16],

    /// Tracks if a key was pressed (for input-related opcodes).
    key_pressed: bool,

    /// Display buffer (64x32 pixels), `true` represents a `on` pixel.
    display: [bool; 64 * 32],

    /// The current opcode being executed.
    opcode: u16,

    /// Lookup table for opcode handlers.
    lookup: [fn(&mut Chip8, Instruction); 16],

    /// Audio sink for sound playback.
    sound_sink: Arc<Mutex<Sink>>,

    /// Audio output stream (kept to prevent garbage collection).
    _audio_stream: OutputStream,
}

/// Represents a decoded CHIP-8 instruction.
#[derive(Debug)]
pub struct Instruction {
    /// The raw 16-bit opcode.
    opcode: u16,

    /// The `X` register field (`0x0F00` bitmask).
    x: u8,

    /// The `Y` register field (`0x00F0` bitmask).
    y: u8,

    /// The last 4 bits (`0x000F`), typically used as a nibble.
    n: u8,

    /// The last 8 bits (`0x00FF`), used for immediate values.
    nn: u8,

    /// The last 12 bits (`0x0FFF`), typically used as an address.
    nnn: u16,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    /// Creates a new `Chip8` instance with initialized memory, registers, and sound.
    ///
    /// # Returns
    ///
    /// A new instance of `Chip8` ready for execution.
    pub fn new() -> Self {
        let (stream, sound_sink) =
            rodio::OutputStream::try_default().expect("Failed to create output stream");
        let sound_sink = Sink::try_new(&sound_sink).expect("Failed to create sound sink");

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
            key_pressed: false,
            display: [false; 64 * 32],
            opcode: 0,
            lookup: [
                instructions::_0xxx::_0xxx,
                instructions::_1xxx::_1xxx,
                instructions::_2xxx::_2xxx,
                instructions::_3xxx::_3xxx,
                instructions::_4xxx::_4xxx,
                instructions::_5xxx::_5xxx,
                instructions::_6xxx::_6xxx,
                instructions::_7xxx::_7xxx,
                instructions::_8xxx::_8xxx,
                instructions::_9xxx::_9xxx,
                instructions::_Axxx::_Axxx,
                instructions::_Bxxx::_Bxxx,
                instructions::_Cxxx::_Cxxx,
                instructions::_Dxxx::_Dxxx,
                instructions::_Exxx::_Exxx,
                instructions::_Fxxx::_Fxxx,
            ],

            sound_sink: Arc::new(Mutex::new(sound_sink)),
            _audio_stream: stream,
        }
    }

    /// Returns a reference to the display buffer (64x32 pixels).
    pub fn get_display(&self) -> &[bool; 64 * 32] {
        &self.display
    }

    /// Loads a CHIP-8 ROM into memory.
    ///
    /// # Arguments
    /// * `path` - The file path of the ROM to load.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if the file cannot be read.
    pub fn load_rom(&mut self, path: &str) -> Result<(), std::io::Error> {
        let rom = std::fs::read(path)?;

        for (i, &byte) in rom.iter().enumerate() {
            self.memory[chip8_constants::START_ADDRESS + i] = byte;
        }

        Ok(())
    }

    /// Loads the built-in CHIP-8 fontset into memory.
    pub fn load_fontset(&mut self) {
        for (i, &byte) in chip8_constants::FONTSET.iter().enumerate() {
            self.memory[chip8_constants::FONTSET_START_ADDRESS + i] = byte;
        }
    }

    /// Updates the state of a specific key.
    ///
    /// # Arguments
    ///
    /// * `key` - The CHIP-8 key index (0x0 to 0xF).
    /// * `pressed` - `true` if the key is pressed, `false` otherwise.
    pub fn update_key(&mut self, key: u8, pressed: bool) {
        self.keypad[key as usize] = pressed;
    }

    /// Executes a single CHIP-8 CPU cycle.
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

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            let sound_sink = self.sound_sink.lock().unwrap();
            if sound_sink.empty() {
                let file =
                    std::fs::File::open("assets/audio/orb.mp3").expect("Failed to open audio file");
                let source = Decoder::new(std::io::BufReader::new(file))
                    .expect("Failed to create audio decoder");

                sound_sink.append(source);
                sound_sink.play();
            }
            self.sound_timer -= 1;
        }
    }
}
