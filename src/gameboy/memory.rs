pub struct GameBoyMemory {
    memory: [u8; 0xFFFF],
}

impl GameBoyMemory {
    pub fn new() -> GameBoyMemory {
        GameBoyMemory { memory: [0; 0xFFFF] }
    }

    pub fn read(&self, address: usize) -> u8 {
        if address < self.memory.len() {
            self.memory[address]
        }
        else {
            0 as u8
        }   // TODO: Invalid access handling
    }

    pub fn write(&mut self, address: usize, val: u8) {
        if address < self.memory.len() {
            self.memory[address] = val;
        }
    }
}
