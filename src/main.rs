#![allow(unused_mut)]
mod gameboy;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use gameboy::{GameBoy};

const CGB_WIDTH: usize = 320;
const CGB_HEIGHT: usize = 288;

fn main() {
    let mut gameboy = GameBoy::new();

    // Initialize emulator window
    let mut window = Window::new(
        "Welcome to the Game Boy!",
        CGB_WIDTH,
        CGB_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window...");

    // Begin Fetch-Decode-Execute loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Resize screen if needed
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != (gameboy.display.width, gameboy.display.height) {
            (gameboy.display.width, gameboy.display.height) = new_size;
            gameboy.display.resize();
        }

        let mut color: u32 = 0x0FFF;

        // Get user input
        window.get_keys().iter().for_each(|key| match key {
            Key::W => color = 0x0F00,
            Key::A => color = 0x00F0,
            Key::S => color = 0x000F,
            Key::D => color = 0x0ABC,
            _ => (),
        });

        // Update display pixels
        gameboy.display.update(&gameboy.screen);

        // Update window
        window
            .update_with_buffer(&gameboy.display.pixels, gameboy.display.width, gameboy.display.height)
            .unwrap();
    }
}
