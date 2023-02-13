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
        
        let mut cgb_screen = GameBoyScreen { 
            width: CGB_WIDTH,
            height: CGB_HEIGHT,
            pixels: pixels,
        };
    
        cgb_screen
    }
}
    
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
    
    pub fn update(&mut self, cgb_screen: &GameBoyScreen) {
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