#![allow(unused_mut)]
mod gameboy;

use std::fs::File;
use std::fs::read;
use std::io;
use std::io::Read;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use gameboy::{GameBoy};

const CGB_WIDTH: usize = 320;
const CGB_HEIGHT: usize = 288;

fn main() -> io::Result<()> {
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
  

    // Read in the ROM
    let path = "./roms/pokemon-yellow.gbc";
    let rom = read(path)?;
    for i in 0..rom.len() {
        if i % 10 == 0 {
            println!("{:x}:  {:x}", i, rom[i]);
        } else {
            println!("        {:x}", rom[i]);
        }
    }

    // Begin Fetch-Decode-Execute loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Fetch next instruction

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

        // Update gameboy pixels
        gameboy.screen.change_color(color);

        // Update display pixels
        gameboy.display.update(&gameboy.screen);

        // Update window
        window
            .update_with_buffer(&gameboy.display.pixels, gameboy.display.width, gameboy.display.height)
            .unwrap();
    }

    Ok(())

}
