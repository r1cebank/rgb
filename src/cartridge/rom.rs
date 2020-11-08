use crate::memory::Memory;

pub struct Rom {
    rom: Vec<u8>,
}

impl Rom {
    pub fn new(rom: Vec<u8>) -> Rom {
        Self { rom }
    }
}

impl Memory for Rom {
    fn get(&self, a: u16) -> u8 {
        self.rom[a as usize]
    }

    fn set(&mut self, a: u16, v: u8) {
        // Rom will not allow set actions
    }
}
