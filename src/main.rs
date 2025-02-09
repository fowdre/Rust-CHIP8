// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
// TODO: documentation
#![warn(missing_debug_implementations, rust_2018_idioms)]

use raylib::prelude::*;
mod chip8;
mod constants;
mod draw;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("[Usage] <ROM file>");
        std::process::exit(1);
    }

    // Init
    let (mut rl_handle, rl_thread) = raylib::init()
        .size(640 * 6, 320 * 6)
        .title("CHIP8 Emulator")
        .build();
    rl_handle.set_target_fps(constants::SCREEN_FPS);

    // UI elements
    let mut display = draw::MonoChromeDisplay::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(
            constants::SCREEN_WIDTH as f32,
            constants::SCREEN_HEIGHT as f32,
        ),
        Color::new(255, 200, 45, 255),
        Color::new(45, 10, 45, 255),
        48.0,
    );
    display.center(&rl_handle);

    // Emulator
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&args[1])?;
    chip8.load_fontset();

    let mut timer = 0.0;
    while !rl_handle.window_should_close() {
        let frame_time = rl_handle.get_frame_time();

        if timer > 0.0 {
            timer -= frame_time;
        } else {
            // The inside of this block will run at `constants::FPS` frames per second
            timer += constants::FRAME_TIME - frame_time;

            chip8.step();
        }

        // Non frame-rate dependent logic here
        display.update(&mut rl_handle, &rl_thread, chip8.get_display());

        // Drawing
        let mut d = rl_handle.begin_drawing(&rl_thread);
        d.clear_background(Color::new(50, 50, 50, 255));
        display.draw(&mut d);
    }

    Ok(())
}
