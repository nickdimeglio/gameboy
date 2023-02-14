use crate::CGB_WIDTH;
use crate::CGB_HEIGHT;

pub struct GameBoyScreen {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}
    
impl GameBoyScreen {
    pub fn new() -> GameBoyScreen {
        let mut pixels: Vec<u32> = Vec::from([0; CGB_WIDTH * CGB_HEIGHT]);
    
        // Initialize game boy screen with checkered pattern
        for i in 0..(CGB_WIDTH * CGB_HEIGHT) {
            let row = i / CGB_WIDTH;
            let col = i % CGB_HEIGHT;
            if ((row % 32 < 16) & (col % 32 < 16)) | ((row % 32 > 15) & (col % 32 > 15)) {
                pixels[i] = 0x1FF;
            }
        }
        
        GameBoyScreen { 
            width: CGB_WIDTH,
            height: CGB_HEIGHT,
            pixels: pixels,
        }
    }
}
