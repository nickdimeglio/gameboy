pub struct GameBoyMemory {
    pub memory: [u8; 0xFFFF],
}

impl GameBoyMemory {
    pub fn new() -> GameBoyMemory {
        GameBoyMemory { memory: [0; 0xFFFF] }
    }
}
