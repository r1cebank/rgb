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
    pub fn tick(&mut self, cycles: u32) {}

    /// When no boot rom is supplied, we set the following states in memory just like the boot rom
    pub fn simulate_boot_rom(&mut self) {
        self.set(0xff05, 0x00);
        self.set(0xff06, 0x00);
        self.set(0xff07, 0x00);
        self.set(0xff10, 0x80);
        self.set(0xff11, 0xbf);
        self.set(0xff12, 0xf3);
        self.set(0xff14, 0xbf);
        self.set(0xff16, 0x3f);
        self.set(0xff17, 0x00);
        self.set(0xff19, 0xbf);
        self.set(0xff1a, 0x7f);
        self.set(0xff1b, 0xff);
        self.set(0xff1c, 0x9f);
        self.set(0xff1e, 0xbf);
        self.set(0xff20, 0xff);
        self.set(0xff21, 0x00);
        self.set(0xff22, 0x00);
        self.set(0xff23, 0xbf);
        self.set(0xff24, 0x77);
        self.set(0xff25, 0xf3);
        self.set(0xff26, 0xf1);
        self.set(0xff40, 0x91);
        self.set(0xff42, 0x00);
        self.set(0xff43, 0x00);
        self.set(0xff45, 0x00);
        self.set(0xff47, 0xfc);
        self.set(0xff48, 0xff);
        self.set(0xff49, 0xff);
        self.set(0xff4a, 0x00);
        self.set(0xff4b, 0x00);
        self.set(0xffff, 0x00);
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
