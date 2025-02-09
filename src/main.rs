// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
// TODO: documentation
#![warn(missing_debug_implementations, rust_2018_idioms)]

mod constants;
use raylib::prelude::*;

fn main() {
    // Init
    let (mut rl_handle, rl_thread) = raylib::init()
        .size(800, 600)
        .title("CHIP8 Emulator")
        .build();
    rl_handle.set_target_fps(constants::SCREEN_FPS);
    rl_handle.toggle_fullscreen();

    let mut timer = 0.0;
    while !rl_handle.window_should_close() {
        let frame_time = rl_handle.get_frame_time();

        if timer > 0.0 {
            timer -= frame_time;
        } else {
            // The inside of this block will run at `constants::FPS` frames per second
            timer += constants::FRAME_TIME - frame_time;
        }

        // Non frrame-rate dependent logic here

        // Drawing
        let mut d = rl_handle.begin_drawing(&rl_thread);
        d.clear_background(Color::WHITE);
    }
}
