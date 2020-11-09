use super::Memory;
use crate::cartridge::{load_cartridge, Cartridge};
use crate::util::BOOT_ROM_SIZE;

pub struct MMU {
    pub boot_rom: Option<[u8; 256]>,
    pub cartridge: Box<dyn Cartridge>,
}

impl MMU {
    pub fn new(boot_rom: Option<Vec<u8>>, rom: Vec<u8>) -> MMU {
        let boot_rom = boot_rom.map(|boot_rom_buffer| {
            if boot_rom_buffer.len() != BOOT_ROM_SIZE {
                panic!(
                    "Bootroom size mismatch, expected {}, got {}",
                    BOOT_ROM_SIZE,
                    boot_rom_buffer.len()
                );
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);
            boot_rom
        });
        Self {
            boot_rom,
            cartridge: load_cartridge(rom),
        }
    }
}

impl Memory for MMU {
    fn get(&self, address: u16) -> u8 {
        unimplemented!()
    }

    fn set(&mut self, address: u16, value: u8) {
        unimplemented!()
    }
}
