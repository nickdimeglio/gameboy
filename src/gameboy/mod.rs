mod display;
mod screen;
mod memory;
mod registers;

use crate::gameboy::display::Display;
use crate::gameboy::screen::Screen;
use crate::gameboy::memory::Memory;
use crate::gameboy::registers::Registers;

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
            registers: Registers::new()
        }
    }

    pub fn execute(&mut self) -> Result<u8, &str> {
        match self.rom.get(self.registers.get_pc()) {
            None => Err("Program counter out of bounds"),
            Some(instruction) => {
                self.instruction = *instruction;
                self.registers.inc_pc();
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
        // NOP
        Ok(0x01)
    }

    fn op0x02(&mut self) -> Result<u8, &str> {
        // LD BC, d16
        match self.memory.read_16(self.registers.get_pc()) {
            None => Err("ROM read out of bounds"),
            Some(nn) => {
                self.registers.inc_pc();
                self.registers.inc_pc();
                self.registers.set_bc(nn);
                Ok(0x02)
            }
        }
    }
}