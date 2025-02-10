/// The target **emulator IPS** (instructions per second).
///
/// How many cycles per second the emulator should execute.
pub const EMULATOR_IPS: f32 = 1000.0;

/// The **screen refresh rate** for rendering frames.
///
/// This value is set to **double the emulator FPS** (`EMULATOR_FPS * 2.0`).
pub const SCREEN_FPS: u32 = (EMULATOR_IPS * 2.0) as u32;

/// The **target frame time** for the emulator in seconds.
///
/// This is the value that the emulator will try to match for each frame.
pub const FRAME_TIME: f32 = 1.0 / EMULATOR_IPS;

/// The **width** of the screen in pixels.
pub const SCREEN_WIDTH: usize = chip8_constants::WIDTH;

/// The **height** of the screen in pixels.
pub const SCREEN_HEIGHT: usize = chip8_constants::HEIGHT;

/// Constants related to system memory and display.
pub mod chip8_constants {
    /// The **width** of the display (64 pixels).
    pub const WIDTH: usize = 64;

    /// The **height** of the display (32 pixels).
    pub const HEIGHT: usize = 32;

    /// The **total size of RAM** in the system (4 KB or 4096 bytes).
    pub const RAM_SIZE: usize = 4096;

    /// The **starting memory address** for programs (0x200).
    ///
    /// The first 512 bytes (0x000–0x1FF) are reserved and read/write.
    pub const START_ADDRESS: usize = 0x200;

    /// The **starting address of the built-in fontset** in memory (0x50).
    ///
    /// uses a built-in hexadecimal fontset for drawing characters 0-F.
    pub const FONTSET_START_ADDRESS: usize = 0x50;

    /// The **size of the fontset** (80 bytes).
    ///
    /// Each character in the fontset is **5 bytes** (5x4 pixels).
    pub const FONTSET_SIZE: usize = 80;

    /// The **default fontset** (16 characters, each 5 bytes).
    ///
    /// This fontset consists of **hexadecimal digits (0-9, A-F)**,
    /// where each character is stored as a 5-byte value.
    ///
    /// Example for `F`:
    /// ```
    /// 0xF0 = 1111 0000
    /// 0x80 = 1000 0000
    /// 0xF0 = 1111 0000
    /// 0x80 = 1000 0000
    /// 0x80 = 1000 0000
    /// ```
    ///
    /// The full fontset is as follows:
    pub const FONTSET: [u8; FONTSET_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
}
