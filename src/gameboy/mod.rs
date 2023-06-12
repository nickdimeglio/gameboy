mod display;
mod screen;
mod memory;
mod registers;

use crate::gameboy::display::Display;
use crate::gameboy::screen::Screen;
use crate::gameboy::memory::Memory;
use crate::gameboy::registers::Registers;

pub struct GameBoy {
    pub display: Display,
    pub screen: Screen,
    pub registers: Registers,
    memory: Memory,
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
            registers: Registers::new()
        }
    }

    pub fn execute(&mut self) -> Result<u8, &str> {
        match self.rom.get(self.registers.get_pc()) {
            None => Err("Program counter out of bounds"),
            Some(instruction) => {
                self.instruction = *instruction;
                self.registers.set_pc(self.registers.get_pc() + 1);
                match self.instruction {
                    0x00 => self.op0x01(),
                    0x01 => self.op0x02(),
                    _ => Err("Invalid instruction"),
                }
            }
        }
    }

    /*
     *  OPCODES
     *
     */
    fn op0x01(&mut self) -> Result<u8, &str> {
        Ok(0x01)
    }

    fn op0x02(&mut self) -> Result<u8, &str> {
        match self.get_16() {
            None => Err("ROM read out of bounds"),
            Some(nn) => {
                self.registers.set_bc(nn);
                Ok(0x02)
            }
        }
    }

    /*
     * HELPER FUNCTIONS
     * 
     */

    fn get_8(&mut self) -> Option<u8> {
        self.registers.inc_pc();
        match self.rom.get(self.registers.get_pc() as usize) {
            None => None,
            Some(n) => Some(*n)
        }
    }

    fn get_16(&mut self) -> Option<u16> {
        match self.get_8() {
            None => None,
            Some(lo) => {
                match self.get_8() {
                    None => None,
                    Some(hi) => Some(&((hi as u16) << 8) + lo as u16)
                }
            }
        }
    }

}