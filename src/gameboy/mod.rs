mod cpu;
mod display;
mod memory;
mod screen;

use crate::gameboy::cpu::{GameBoyCPU};
use crate::gameboy::display::{Display};
use crate::gameboy::memory::{GameBoyMemory};
use crate::gameboy::screen::{GameBoyScreen};

pub struct GameBoy {
    pub cpu: GameBoyCPU,
    pub display: Display,
    pub memory: GameBoyMemory,
    pub screen: GameBoyScreen,
}

impl GameBoy {
    pub fn new() -> GameBoy {
        GameBoy {
            cpu: GameBoyCPU::new(),
            display: Display::new(),
            memory: GameBoyMemory::new(), 
            screen: GameBoyScreen::new(),
        }
    }
}
