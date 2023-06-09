#![allow(non_snake_case)]
use crate::gameboy::GameBoyMemory;

pub struct Registers {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: usize,
    F: u8,
}
pub struct GameBoyCPU {
    pub registers: Registers,
    pub debug_mode: bool,
}

type GameBoyOpcode = fn(
    &mut GameBoyCPU,
    instruction: u8,
    rom: &Vec<u8>,
    mem: &mut GameBoyMemory,
) -> Result<(), String>;

impl GameBoyCPU {
    pub fn new() -> GameBoyCPU {
        //use crate::Registers;
        GameBoyCPU {
            registers: Registers {
                AF: 0,
                BC: 0,
                DE: 0,
                HL: 0,
                SP: 0,
                PC: 0x0100,
                F: 0,
            },
            debug_mode: true,
        }
    }

    fn op_0x01(
        &mut self,
        instruction: u8,
        rom: &Vec<u8>,
        mem: &mut GameBoyMemory,
    ) -> Result<(), String> {
        Ok(())
    }

    pub fn execute(
        &mut self,
        instruction: u8,
        rom: &Vec<u8>,
        memory: &mut GameBoyMemory,
    ) -> Result<(), String> {
        let opcode: &GameBoyOpcode = match instruction {
            0x01 => self.op_0x01,
            _ => (),
        };

        opcode(&mut self, instruction, &rom, &mut memory)
    }

    /*
    pub fn execute(
        &mut self,
        instruction: u8,
        rom: &Vec<u8>,
        memory: &mut GameBoyMemory,
    ) -> String {
        // Decode

        let opcode = opcodes[instruction];
        opcode(rom, memory);

        let mut operation = match instruction {
            0x01 => self.op_0x01(),
            0x02 => self.op_0x02(),
            0x03 => self.op_0x03(),

            0x00 => self.no_op(),

            0xCB => self.cb_prefix(),

            // 8-bit load/store/move
            i if (0x40 <= i && i < 0x80 && i != 0x76) => self.load_8(instruction, rom, memory),

            0x02 | 0x12 | 0x22 | 0x32 | 0x06 | 0x16 | 0x26 | 0x36 | 0x0A | 0x1A | 0x2A | 0x3A
            | 0x0E | 0x1E | 0x2E | 0x3E | 0xE0 | 0xF0 | 0xE2 | 0xF2 | 0xEA | 0xFA => {
                self.load_8(instruction, rom, memory)
            }

            // 16-bit load/store/move
            0x01 | 0x11 | 0x21 | 0x31 | 0xC1 | 0xD1 | 0xE1 | 0xF1 | 0xC5 | 0xD5 | 0xE5 | 0xF5
            | 0x08 | 0xF8 | 0xF9 => self.load_16(instruction, rom, memory),

            // Unknown opcode
            _ => self.unknown(),
        };

        if self.debug_mode {
            println!(
                "0x{:0>4x} --- {:0>2x} {:}",
                self.get_PC(),
                instruction,
                operation
            );
        };

        operation
    }
    */

    fn load_8(&mut self, instruction: u8, rom: &Vec<u8>, memory: &mut GameBoyMemory) -> String {
        match instruction {
            // Direct 8-bit loads
            i if 0x40 <= i && i <= 0x7F => {
                // Loading from which address?
                let Y = match (instruction & 0xF) % 8 {
                    0x0 => self.get_B(), // LD X, B
                    0x1 => self.get_C(), // LD X, C
                    0x2 => self.get_D(), // LD X, D
                    0x3 => self.get_E(), // LD X, E
                    0x4 => self.get_H(), // LD X, H
                    0x5 => self.get_L(), // LD X, L
                    0x6 => {
                        // LD X, mem[HL]
                        memory.read(self.get_HL() as usize)
                    }
                    0x7 => self.get_A(), // LD X, A
                    _ => self.get_B(),   // UKNOWN OPCODE
                };

                // Which address to load into?
                match (instruction & 0xF0) + (instruction & 0xF) / 8 {
                    0x40 => self.set_B(Y), // LD B, Y
                    0x41 => self.set_C(Y), // LD C, Y
                    0x50 => self.set_D(Y), // LD D, Y
                    0x51 => self.set_E(Y), // LD E, Y
                    0x60 => self.set_H(Y), // LD H, Y
                    0x61 => self.set_L(Y), // LD L, Y
                    0x70 => {
                        // LD mem[HL], Y
                        memory.write(self.get_HL() as usize, Y);
                    }
                    0x71 => self.set_A(Y), // LD A, Y
                    _ => (),
                }
            }

            // Loads from A (indirect)
            //
            0x02 | 0x12 | 0x22 | 0x32 | 0xE0 | 0xE2 | 0xEA => {
                let X = match instruction {
                    0x02 => self.get_BC(),                   // X = (BC)
                    0x12 => self.get_DE(),                   // X = (DE)
                    0x22 => self.get_inc_HL(),               // X = (HL+)
                    0x32 => self.get_dec_HL(),               // X = (HL-)
                    0xE0 => 0xFF00 + self.get_8(rom) as u16, // X = (a8)
                    0xE2 => 0xFF00 + self.get_C() as u16,    // X = (C)
                    0xEA => self.get_16(rom),                // X = (a16)
                    _ => 0,                                  // unknown
                };
                memory.write(X as usize, self.get_A()); // LD X, A
            }

            // Indirect loads
            _ => {
                let Y = match instruction {
                    0x06 | 0x16 | 0x26 | 0x36 | 0x0E | 0x1E | 0x2E | 0x3E => self.get_8(rom),
                    0x0A => memory.read(self.get_BC() as usize),
                    0x1A => memory.read(self.get_DE() as usize),
                    0x2A => memory.read(self.get_inc_HL() as usize),
                    0x3A => memory.read(self.get_dec_HL() as usize),
                    0xF0 => memory.read(0xFF00 + (self.get_8(rom) as u16) as usize),
                    0xF2 => memory.read(0xFF00 + (self.get_C() as u16) as usize),
                    0xFA => memory.read(self.get_16(rom) as usize),
                    _ => 0, // unknown
                };

                // Which address to load into?
                match instruction {
                    0x06 => self.set_B(Y), // LD B, Y
                    0x0E => self.set_C(Y), // LD C, Y
                    0x16 => self.set_D(Y), // LD D, Y
                    0x1E => self.set_E(Y), // LD E, Y
                    0x26 => self.set_H(Y), // LD H, Y
                    0x2E => self.set_L(Y), // LD L, Y
                    0x36 => memory.write(
                        self.get_HL() as usize, // LD (HL), Y
                        Y,
                    ),
                    _ => self.set_A(Y), // LD A, Y
                };
            }
        };
        String::from("LD X, Y")
    }

    fn load_16(&mut self, instruction: u8, rom: &Vec<u8>, memory: &mut GameBoyMemory) -> String {
        match instruction {
            0x01 => {
                let nn = self.get_16(&rom);
                self.set_BC(nn);
            }
            0x08 => {
                let nn = self.get_16(&rom);
                memory.write(nn as usize, (self.get_SP() & 0xFF) as u8);
                memory.write((nn + 1) as usize, (self.get_SP() >> 8) as u8);
            }
            0x11 => {
                let nn = self.get_16(&rom);
                self.set_DE(nn);
            }
            0x21 => {
                let nn = self.get_16(&rom);
                self.set_HL(nn);
            }
            0x31 => {
                let nn = self.get_16(&rom);
                self.set_SP(nn);
            }
            0xC1 => {
                let nn = self.pop_stack(memory);
                self.set_BC(nn);
            }
            _ => (), // Unknown
        };
        /*
        0x01 | 0x08 | 0x11 | 0x21 | 0x31 |
        0xC1 | 0xD1 | 0xE1 | 0xF1 | 0xC5 |
        0xD5 | 0xE5 | 0xF5 | 0xF8 | 0xF9
        */
        String::from("LD X, Y (16)")
    }

    /*
     *  REGISTER ACCESS
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

    pub fn get_inc_HL(&mut self) -> u16 {
        self.set_HL(self.get_HL() + 1);
        self.get_HL() - 1
    }

    pub fn get_dec_HL(&mut self) -> u16 {
        self.set_HL(self.get_HL() - 1);
        self.get_HL() + 1
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

    pub fn get_8(&mut self, rom: &Vec<u8>) -> u8 {
        self.set_PC(self.get_PC() + 1);
        rom[self.get_PC() as usize]
    }

    pub fn get_16(&mut self, rom: &Vec<u8>) -> u16 {
        self.set_PC(self.get_PC() + 2);
        ((rom[self.get_PC() as usize] as u16) << 8) + rom[self.get_PC() - 1 as usize] as u16
    }

    pub fn pop_stack(&mut self, memory: &mut GameBoyMemory) -> u16 {
        self.set_SP(self.get_SP() + 2);
        ((memory.read((self.get_SP() - 1) as usize) as u16) << 8)
            + (memory.read((self.get_SP() - 2) as usize) as u16)
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
