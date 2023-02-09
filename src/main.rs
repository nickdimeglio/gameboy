use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 320; 
const HEIGHT: usize = 288;

fn main() {
    let mut window = Window::new(
        "Welcome to the Game Boy!",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window...");

    let mut pixels: Vec<u32> = Vec::from([0; WIDTH * HEIGHT]);
    let mut buffer: Vec<u32> = Vec::from([0; WIDTH * HEIGHT]);
    let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != size {
            size = new_size;
            buffer.resize(size.0 * size.1, 0);
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

        // Update pixels
        for i in 0..(WIDTH * HEIGHT) {
            let row = i / WIDTH;
            let col = i % HEIGHT;
            if ((row % 32 < 16) & (col % 32 < 16)) | ((row % 32 > 15) & (col % 32 > 15)) {
                pixels[i] = color;
            }
        }

        // Update buffer
        let buffer_width = size.0;
        let buffer_height = size.1;
        for i in 0..(buffer_width * buffer_height) {

            // Get relative x and y for this buffer pixel (0 to 1)
            let x = (i % buffer_width) as f64 / buffer_width as f64;
            let y = (i as f64 / buffer_width as f64) / buffer_height as f64;

            // Calculate absolute position for gameboy pixel
            let row = (y * HEIGHT as f64).floor() as usize;     // round down
            let col = (x * WIDTH as f64).floor() as usize;      // round down
            let pos = row * WIDTH + col;
            buffer[i] = pixels[pos];
        }

        // Update window
        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
