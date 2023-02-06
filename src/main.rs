use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 240;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Game Boy Color")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(
                window_size.width,
                window_size.height,
                &window
            );
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };


    let mut rgba = [0, 0, 0, 0];

    event_loop.run(move |event, _, control_flow| {
        
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {

            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Keyboard input
            let mut draw = true;
            if input.key_pressed(VirtualKeyCode::W) {
                rgba = [0xFF, 0x00, 0x00, 0xFF];
            }
            else if input.key_pressed(VirtualKeyCode::A) {
                rgba = [0x00, 0xFF, 0xFF, 0xFF];
            }
            else if input.key_pressed(VirtualKeyCode::S) {
                rgba = [0x00, 0x00, 0xFF, 0xFF];
            }
            else if input.key_pressed(VirtualKeyCode::D) {
                rgba = [0xF0, 0x50, 0x10, 0xFF];
            }
            else if input.key_pressed(VirtualKeyCode::N) {
                rgba = [0x0A, 0xAA, 0x00, 0xFF];
            }
            else if input.key_pressed(VirtualKeyCode::J) {
                rgba = [0x10, 0x10, 0x10, 0xFF];
            }
            else if input.key_pressed(VirtualKeyCode::M) {
                rgba = [0xFF, 0xFF, 0xFF, 0xFF];
            }
            else if input.key_pressed(VirtualKeyCode::K) {
                rgba = [0x0B, 0xCD, 0x02, 0xFF];
            }
            else {
                draw = false;
            }

            // Render pixels
            for pixel in pixels.get_frame_mut().chunks_exact_mut(4) {
                pixel[0] = rgba[0];
                pixel[1] = rgba[1];
                pixel[2] = rgba[2];
                pixel[3] = rgba[3];
            }

            pixels.render();
        
        }

    });
}
