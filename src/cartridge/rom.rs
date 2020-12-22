use super::Cartridge;
use crate::memory::Memory;
use crate::save::Savable;
use std::path::PathBuf;

pub struct Rom {
    rom: Vec<u8>,
}

impl Rom {
    pub fn new(rom: Vec<u8>) -> Rom {
        Self { rom }
    }
}

impl Memory for Rom {
    fn get(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        // Rom will not allow set actions
    }
}

impl Savable for Rom {
    fn save(&self, _: PathBuf) {
        // Will not save anything for the Rom, no state to persist
    }

    fn load(&mut self, _: PathBuf) {
        // Will not load anything for the Rom, no state to load from
    }
}

impl Cartridge for Rom {}
