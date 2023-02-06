use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 640; 
const HEIGHT: usize = 360;

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
    .expect("Unable to create window...")

    let mut buffer: Vec<u32> = Vec::with_capacity(WIDTH * HEIGHT);
    let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != size {syn match    cCustomParen    "(" contains=cParen,cCppParen
syn match    cCustomFunc     "\w\+\s*(" contains=cCustomParen
syn match    cCustomScope    "::"
syn match    cCustomClass    "\w\+\s*::" contains=cCustomScope
syn match    cCustomProp     "\.\w\+\s*."
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

        // Update buffer
        for pixel in buffer.iter_mut() {
            *pixel = color;
        }

        // Update window
        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
