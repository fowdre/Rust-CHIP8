// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
// TODO: documentation
#![warn(missing_debug_implementations, rust_2018_idioms)]

use raylib::prelude::*;
mod constants;
mod draw;

fn main() {
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
        Color::new(45, 10, 45, 255),
        Color::new(100, 20, 70, 255),
        48.0,
    );
    display.center(&rl_handle);

    let mut screen = [false; constants::SCREEN_WIDTH * constants::SCREEN_HEIGHT];

    let mut timer = 0.0;
    while !rl_handle.window_should_close() {
        let frame_time = rl_handle.get_frame_time();

        if timer > 0.0 {
            timer -= frame_time;
        } else {
            // The inside of this block will run at `constants::FPS` frames per second
            timer += constants::FRAME_TIME - frame_time;
        }

        // Non frame-rate dependent logic here
        display.update(&mut rl_handle, &rl_thread, &screen);

        // Drawing
        let mut d = rl_handle.begin_drawing(&rl_thread);
        d.clear_background(Color::new(50, 50, 50, 255));
        display.draw(&mut d);
    }
}
