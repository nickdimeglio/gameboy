use crate::CGB_WIDTH;
use crate::CGB_HEIGHT;

#[derive(Clone, Debug, PartialEq)]
pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}
    
impl Screen {
    pub fn new() -> Screen {
        let mut pixels: Vec<u32> = Vec::from([0; CGB_WIDTH * CGB_HEIGHT]);
    
        // Initialize game boy screen with checkered pattern
        for i in 0..(CGB_WIDTH * CGB_HEIGHT) {
            let row = i / CGB_WIDTH;
            let col = i % CGB_HEIGHT;
            if ((row % 32 < 16) & (col % 32 < 16)) | ((row % 32 > 15) & (col % 32 > 15)) {
                pixels[i] = 0x1FF;
            }
        }
        
        Screen { 
            width: CGB_WIDTH,
            height: CGB_HEIGHT,
            pixels: pixels,
        }
    }

    pub fn change_color(&mut self, color: u32) {
        for i in 0..(self.width * self.height) {
            let row = i / self.width;
            let col = i % self.height;
            if ((row % 32 < 16) & (col % 32 < 16)) | ((row % 32 > 15) & (col % 32 > 15)) {
                self.pixels[i] = color;
            }
        }
    }
}
