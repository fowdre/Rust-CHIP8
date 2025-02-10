//! # CHIP-8 Instruction Handlers
//!
//! This module contains the **opcode implementations** for the CHIP-8 emulator.
//! Each module corresponds to a specific **opcode category**, following the CHIP-8 instruction set.
//!
//! ## **Opcode Categorization**
//!
//! CHIP-8 instructions are **2 bytes long** and can be grouped based on their **first hexadecimal digit**:
//!
//! | Opcode Range | Functionality                                            |
//! |--------------|--------------                                            |
//! | `0x0XXX`     | System instructions (clear screen, return, etc.)         |
//! | `0x1XXX`     | Jump instructions (`JP addr`)                            |
//! | `0x2XXX`     | Subroutine calls (`CALL addr`)                           |
//! | `0x3XXX`     | Skip instructions (`SE Vx, byte`)                        |
//! | `0x4XXX`     | Skip if not equal (`SNE Vx, byte`)                       |
//! | `0x5XXX`     | Skip if register values match (`SE Vx, Vy`)              |
//! | `0x6XXX`     | Load immediate values into registers (`LD Vx, byte`)     |
//! | `0x7XXX`     | Add immediate values to registers (`ADD Vx, byte`)       |
//! | `0x8XXX`     | Register operations (`AND`, `OR`, `XOR`, etc.)           |
//! | `0x9XXX`     | Skip instructions for register comparison (`SNE Vx, Vy`) |
//! | `0xAXXX`     | Load address into `I` (`LD I, addr`)                     |
//! | `0xBXXX`     | Jump to `addr + V0` (`JP V0, addr`)                      |
//! | `0xCXXX`     | Random number generation (`RND Vx, byte`)                |
//! | `0xDXXX`     | Display/draw instructions (`DRW Vx, Vy, nibble`)         |
//! | `0xEXXX`     | Key input handling (`SKP Vx`, `SKNP Vx`)                 |
//! | `0xFXXX`     | Miscellaneous operations (timers, memory, input/output)  |
//!
//! ## **Usage**
//!
//! Each submodule implements **a set of CHIP-8 instructions** corresponding to its opcode range.
//!
//! ## **Module Breakdown**
//!
//! Each module implements the opcodes for a specific range.

#![allow(non_snake_case)]

/// System instructions (`0x0XXX`)
/// - `00E0` - Clear screen (`CLS`)
/// - `00EE` - Return from subroutine (`RET`)
pub mod _0xxx;

/// Jump instructions (`0x1XXX`)
/// - `1nnn` - Jump to address nnn (`JP addr`)
pub mod _1xxx;

/// Subroutine calls (`0x2XXX`)
/// - `2nnn` - Call subroutine at address nnn (`CALL addr`)
pub mod _2xxx;

/// Skip instructions (`0x3XXX`)
/// - `3xnn` - Skip next instruction if Vx == nn (`SE Vx, byte`)
pub mod _3xxx;

/// Skip if not equal (`0x4XXX`)
/// - `4xnn` - Skip if Vx != nn (`SNE Vx, byte`)
pub mod _4xxx;

/// Skip if registers match (`0x5XXX`)
/// - `5xy0` - Skip next instruction if Vx == Vy (`SE Vx, Vy`)
pub mod _5xxx;

/// Load immediate values (`0x6XXX`)
/// - `6Xnn` - Load nn into Vx (`LD Vx, byte`)
pub mod _6xxx;

/// Add immediate values (`0x7XXX`)
/// - `7xnn` - Add nn to Vx (`ADD Vx, byte`)
pub mod _7xxx;

/// Register operations (`0x8XXX`)
/// - `8xy0` - Load (`LD Vx, Vy`)
/// - `8xy1` - Bitwise OR (`OR Vx, Vy`)
/// - `8xy2` - Bitwise AND (`AND Vx, Vy`)
/// - `8xy3` - Bitwise XOR (`XOR Vx, Vy`)
/// - `8xy4` - Add (`ADD Vx, Vy`)
/// - `8xy5` - Subtract (`SUB Vx, Vy`)
/// - `8xy6` - Shift right (`SHR Vx`)
/// - `8xy7` - Reverse subtract (`SUBN Vx, Vy`)
/// - `8xyE` - Shift left (`SHL Vx`)
pub mod _8xxx;

/// Skip if registers don’t match (`0x9XXX`)
/// - `9xy0` - Skip if Vx != Vy (`SNE Vx, Vy`)
pub mod _9xxx;

/// Set index register (`0xAXXX`)
/// - `Annn` - Set I to address (`LD I, addr`)
pub mod _Axxx;

/// Jump with offset (`0xBXXX`)
/// - `Bnnn` - Jump to `V0 + addr` (`JP V0, addr`)
pub mod _Bxxx;

/// Random number generation (`0xCXXX`)
/// - `Cxnn` - Generate random number (`RND Vx, byte`)
pub mod _Cxxx;

/// Display/draw instructions (`0xDXXX`)
/// - `Dxyn` - Draw sprite at `(Vx, Vy)` with `n` height (`DRW Vx, Vy, nibble`)
pub mod _Dxxx;

/// Key input instructions (`0xEXXX`)
/// - `Ex9E` - Skip if key pressed (`SKP Vx`)
/// - `ExA1` - Skip if key not pressed (`SKNP Vx`)
pub mod _Exxx;

/// Miscellaneous operations (`0xFXXX`)
/// - `Fx07` - Get delay timer value (`LD Vx, DT`)
/// - `Fx0A` - Wait for key press (`LD Vx, K`)
/// - `Fx15` - Set delay timer (`LD DT, Vx`)
/// - `Fx18` - Set sound timer (`LD ST, Vx`)
/// - `Fx1E` - Add to index register (`ADD I, Vx`)
/// - `Fx29` - Font character (`LD F, Vx`)
/// - `Fx33` - Binary-coded decimal conversion (`LD B, Vx`)
/// - `Fx55` - Store registers in memory (`LD [I], Vx`)
/// - `Fx65` - Load registers from memory (`LD Vx, [I]`)
pub mod _Fxxx;
