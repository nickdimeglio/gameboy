pub struct GameBoyCPU {
    pub ip: u16,
}

impl GameBoyCPU {
    pub fn new() -> GameBoyCPU {
        GameBoyCPU {
            ip: 0x0100,
        }
    }
}
