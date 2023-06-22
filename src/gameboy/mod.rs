mod display;
mod memory;
mod registers;
mod screen;

use crate::gameboy::display::Display;
use crate::gameboy::memory::Memory;
use crate::gameboy::registers::Registers;
use crate::gameboy::screen::Screen;

#[derive(Clone, Debug, PartialEq)]
pub struct GameBoy {
    pub display: Display,
    pub screen: Screen,
    pub registers: Registers,
    pub memory: Memory,
    rom: Vec<u8>,
    instruction: u8,
}

impl GameBoy {
    pub fn new(rom: Vec<u8>) -> GameBoy {
        GameBoy {
            display: Display::new(),
            memory: Memory::new(),
            instruction: 0x01,
            rom,
            screen: Screen::new(),
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self) -> Result<u8, &str> {
        match self.rom.get(self.registers.get_pc()) {
            None => Err("Program counter out of bounds"),
            Some(instruction) => {
                self.instruction = *instruction;
                self.registers.inc_pc();
                match self.instruction {
                    0x00 => self.op0x00(),
                    0x01 => self.op0x01(),
                    0x02 => self.op0x02(),
                    0x03 => self.op0x03(),
                    0x11 => self.op0x11(),
                    _ => Err("Invalid instruction"),
                }
            }
        }
    }

    /*
     *  OPCODES
     *
     */
    fn op0x00(&mut self) -> Result<u8, &str> {
        // NOP
        Ok(0x00)
    }

    fn op0x01(&mut self) -> Result<u8, &str> {
        // LD BC, d16
        // 1-byte, 12-cycle
        match self.rom_read_16(self.registers.get_pc()) {
            None => Err("ROM read out of bounds"),
            Some(nn) => {
                self.registers.inc_pc();
                self.registers.inc_pc();
                self.registers.set_bc(nn);
                Ok(0x01)
            }
        }
    }

    fn op0x02(&mut self) -> Result<u8, &str> {
        // LD (BC), A
        // 1-byte, 8-cycle
        let a = self.registers.get_a();
        let bc = self.registers.get_bc();
        match self.memory.write_8(bc as usize, a) {
            None => Err("Memory write out of bounds"),
            Some(_) => Ok(0x02)
        }
    }

    fn op0x03(&mut self) -> Result<u8, &str> {
        // INC BC
        // 1-byte, 8-cycle
        self.registers.set_bc(self.registers.get_bc() + 1);
        Ok(0x03)
    }

    fn op0x11(&mut self) -> Result<u8, &str> {
        // LD DE, d16
        // 3-byte, 12-cycle
        match self.rom_read_16(self.registers.get_pc()) {
            None => Err("ROM read out of bounds"),
            Some(nn) => {
                self.registers.inc_pc();
                self.registers.inc_pc();
                self.registers.set_de(nn);
                Ok(0x11)
            }
        }
    }

    fn rom_read_8(&mut self, address: usize) -> Option<u8> {
        // Read 8-bit data from ROM
        match self.rom.get(address) {
            None => None,
            Some(n) => Some(*n),
        }
    }

    fn rom_read_16(&mut self, address: usize) -> Option<u16> {
        // Read 16-bit data from ROM
        match self.rom_read_8(address) {
            None => None,
            Some(lo) => match self.rom_read_8(address + 1) {
                None => None,
                Some(hi) => Some(((hi as u16) << 8) + lo as u16),
            },
        }
    }
}
