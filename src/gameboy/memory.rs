#[derive(Clone, Debug, PartialEq)]
pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF],
        }
    }

    pub fn read(&self, address: usize) -> Option<u8> {
        if address < self.memory.len() {
            Some(self.memory[address])
        } else {
            None
        } 
    }

    pub fn write(&mut self, address: usize, val: u8) {
        if address < self.memory.len() {
            self.memory[address] = val;
        }
    }

    pub fn read_16(&mut self, address: usize) -> Option<u16> {
        match self.read(address) {
            None => None,
            Some(lo) => {
                match self.read(address + 1) {
                    None => None,
                    Some(hi) => Some(&((hi as u16) << 8) + lo as u16)
                }
            }
        }
    }
}
