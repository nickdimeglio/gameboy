use crate::CGB_WIDTH;
use crate::CGB_HEIGHT;
use crate::gameboy::Screen;

pub struct Display {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}
    
impl Display {
    pub fn new() -> Display {
        Display {
            width: CGB_WIDTH,
            height: CGB_HEIGHT,
            pixels: Vec::from([0; CGB_WIDTH * CGB_HEIGHT]),
        }
    }
    
    pub fn resize(&mut self) {
        self.pixels.resize(self.width * self.height, 0);
    }
    
    pub fn update(&mut self, cgb_screen: &Screen) {
        // Update buffer
        for i in 0..(self.width * self.height) {
    
            // Get relative x and y for this buffer pixel (0 to 1)
            let x = (i % self.width) as f64 / self.width as f64;
            let y = (i as f64 / self.width as f64) / self.height as f64;
    
            // Calculate absolute position for gameboy pixel
            let row = (y * cgb_screen.height as f64).floor() as usize;     // round down
            let col = (x * cgb_screen.width as f64).floor() as usize;      // round down
            let pos = row * cgb_screen.width + col;
    
            // Update buffer pixel with color from gameboy pixel
            self.pixels[i] = cgb_screen.pixels[pos];
        }
    }
}
