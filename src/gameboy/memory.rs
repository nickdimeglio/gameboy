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

    pub fn read_8(&self, address: usize) -> Option<u8> {
        match self.memory.get(address) {
            None => None,
            Some(n) => Some(*n),
        }
    }

    pub fn write_8(&mut self, address: usize, val: u8) -> Option<u8> {
        match self.memory.get_mut(address) {
            None => None,
            Some(byte) => {
                *byte = val;
                Some(val)
            }
        }
    }

    pub fn read_16(&mut self, address: usize) -> Option<u16> {
        match self.read_8(address) {
            None => None,
            Some(lo) => match self.read_8(address + 1) {
                None => None,
                Some(hi) => {
                    println!("hi: {}, lo: {}", hi, lo);
                    Some((hi as u16) << 8 + lo as u16)
                }
            },
        }
    }
}
