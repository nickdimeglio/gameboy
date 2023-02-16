struct Registers {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: u16,
}

pub struct GameBoyCPU {
    registers: Registers,
}

impl GameBoyCPU {
    pub fn new() -> GameBoyCPU {
        GameBoyCPU {
            registers: Registers { 
                            AF: 0, BC: 0, 
                            DE: 0, HL: 0, 
                            SP: 0, PC: 0x0100 },
        }
    }
}
