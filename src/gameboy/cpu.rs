#![allow(non_snake_case)]
use crate::gameboy::GameBoyMemory;

pub struct Registers {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: usize,
}

pub struct GameBoyCPU {
    pub registers: Registers,
    pub debug_mode: bool,
}

impl GameBoyCPU {
    pub fn new() -> GameBoyCPU {
        //use crate::Registers;
        GameBoyCPU {
            registers: Registers { 
                            AF: 0, BC: 0, 
                            DE: 0, HL: 0, 
                            SP: 0, PC: 0x0100 },
            debug_mode: true,
        }
    }

    pub fn execute(&mut self, instruction: u8, memory: &mut GameBoyMemory) -> String {

        // Decode
        let mut operation = match instruction {
            0x00 => self.no_op(),
            0xCB => self.cb_prefix(),

            // 8-bit load/store/move
            i if 0x40 <= i && i < 0x80
                 ||
                 (i & 0xF == 0x2 || i & 0xF0 == 0xE0) && i != 0xF2
                => self.load_X_Y(instruction, memory),

            // Unknown opcode
            _    => self.unknown(),
        };

        if self.debug_mode {
            println!("0x{:0>4x} --- {:0>2x} {:}", self.get_PC(), instruction, operation);
        };

        self.set_PC(self.get_PC() + 1);

        operation
    }

    fn load_X_Y(&mut self, instruction: u8, memory: &mut GameBoyMemory) -> String {
        match instruction {
            // Typical 8-bit loads
            i if 0x40 <= i && i <= 0x7F => { 
                // Loading from which address?
                let Y = match (instruction & 0xF) % 8 {
                    0x0 => self.get_B(),    // LD X, B
                    0x1 => self.get_C(),    // LD X, C
                    0x2 => self.get_D(),    // LD X, D
                    0x3 => self.get_E(),    // LD X, E
                    0x4 => self.get_H(),    // LD X, H
                    0x5 => self.get_L(),    // LD X, L
                    0x6 => {                // LD X, mem[HL]
                        memory.read(self.get_HL() as usize)
                    },
                    0x7 => self.get_A(),    // LD X, A
                    _ => self.get_B()       // UKNOWN OPCODE 
                };

                // Which address to load into?
                match (instruction & 0xF0) + (instruction & 0xF) / 8 {
                    0x40 => self.set_B(Y),  // LD B, Y
                    0x41 => self.set_C(Y),  // LD C, Y
                    0x50 => self.set_D(Y),  // LD D, Y
                    0x51 => self.set_E(Y),  // LD E, Y
                    0x60 => self.set_H(Y),  // LD H, Y
                    0x61 => self.set_L(Y),  // LD L, Y
                    0x70 => {               // LD mem[HL], Y
                        memory.write(self.get_HL() as usize, Y);
                    }
                    0x71 => self.set_A(Y),  // LD A, Y
                    _    => (),
                }
            }
            
            // Loads from A (indirect HL)
            i if (i & 0xF == 2 || i & 0xF0 == 0xE0) && i != 0xF2 => {
                let X = match (instruction & 0xF0) >> 4 {
                    0x0 => self.get_BC(),    // LD (BC), A
                    0x1 => self.get_DE(),    // LD (DE), A
                    0x2 => {                // LD (HL+), A
                        self.set_HL(self.get_HL() + 1);
                        self.get_HL() - 1
                    },
                    0x3 => {                // LD (HL-), A
                        self.set_HL(self.get_HL() - 1);
                        self.get_HL() + 1
                    },
                    0xE => match instruction & 0xF {
                        0x0 => {            // LD (a8), A
                            self.set_PC(self.get_PC() + 1);
                            0xFF00 + memory.read(self.get_PC() as usize) as u16
                        },
                        0x2 => self.get_C() as u16,    // LD (C), A
                        0xA => {            // LD (a16), A
                            self.set_PC(self.get_PC() + 2);
                            ((memory.read(self.get_PC() as usize) as u16) << 8)
                            + 
                            memory.read((self.get_PC() - 1) as usize) as u16
                        }.into(),
                        _ => 0,  // Unknown
                    }.into(),
                    _ => 0, // Unknown
                };
                memory.write(X as usize, self.get_A());
            }
            _ => (),    // Unknown LD X, Y
        }

        String::from("LD X, Y")
    }

    /* 
     *  REGISTER GETTERS AND SETTERS 
     *
    */

    pub fn get_AF(&self) -> u16 {
        self.registers.AF
    }

    pub fn get_BC(&self) -> u16 {
        self.registers.BC
    }

    pub fn get_DE(&self) -> u16 {
        self.registers.DE
    }

    pub fn get_HL(&self) -> u16 {
        self.registers.HL
    }

    pub fn get_SP(&self) -> u16 {
        self.registers.SP
    }

    pub fn get_PC(&self) -> usize {
        self.registers.PC
    }
   
    pub fn set_AF(&mut self, val: u16) {
        self.registers.AF = val;
    }

    pub fn set_BC(&mut self, val: u16) {
        self.registers.BC = val;
    }

    pub fn set_DE(&mut self, val: u16) {
        self.registers.DE = val;
    }

    pub fn set_HL(&mut self, val: u16) {
        self.registers.HL = val;
    }

    pub fn set_SP(&mut self, val: u16) {
        self.registers.SP = val;
    }

    pub fn set_PC(&mut self, address: usize) {
        self.registers.PC = address;
    }
   
    pub fn get_A(&self) -> u8 {
        (self.registers.AF >> 8) as u8
    }
    pub fn set_A(&mut self, val: u8) {
        self.registers.AF = ((val as u16) << 8) | (self.registers.AF & 0x00FF);
    }

    pub fn get_F(&self) -> u8 {
        (self.registers.AF & 0x00FF) as u8
    }

    pub fn set_F(&mut self, val: u8) {
        self.registers.AF = (self.registers.AF & 0xFF00) | val as u16;
    }

    pub fn get_B(&self) -> u8 {
        (self.registers.BC >> 8) as u8
    }

    pub fn set_B(&mut self, val: u8) {
        self.registers.BC = ((val as u16) << 8) | (self.registers.BC & 0x00FF);
    }

    pub fn get_C(&self) -> u8 {
        (self.registers.BC & 0x00FF) as u8
    }

    pub fn set_C(&mut self, val: u8) {
        self.registers.BC = (self.registers.BC & 0xFF00) | val as u16;
    }

    pub fn get_D(&self) -> u8 {
        (self.registers.DE >> 8) as u8
    }

    pub fn set_D(&mut self, val: u8) {
        self.registers.DE = ((val as u16) << 8) | (self.registers.DE & 0x00FF);
    }

    pub fn get_E(&self) -> u8 {
        (self.registers.DE & 0x00FF) as u8
    }

    pub fn set_E(&mut self, val: u8) {
        self.registers.DE = (self.registers.DE & 0xFF00) | val as u16;
    }

    pub fn get_H(&self) -> u8 {
        (self.registers.HL >> 8) as u8
    }

    pub fn set_H(&mut self, val: u8) {
        self.registers.HL = ((val as u16) << 8) | (self.registers.HL & 0x00FF);
    }

    pub fn get_L(&self) -> u8 {
        (self.registers.HL & 0x00FF) as u8
    }

    pub fn set_L(&mut self, val: u8) {
        self.registers.HL = (self.registers.HL & 0xFF00) | val as u16;
    }

    fn no_op(&mut self) -> String {
        String::from("NOP")
    }

    fn cb_prefix(&mut self) -> String {
        String::from("PREFIX CB")
    }

    fn unknown(&mut self) -> String {
        String::from("UNKNOWN")
    }
}
