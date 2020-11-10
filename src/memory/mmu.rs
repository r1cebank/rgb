use super::timer::Timer;
use super::Memory;
use crate::cartridge::{load_cartridge, Cartridge};
use crate::util::BOOT_ROM_SIZE;

pub struct MMU {
    pub boot_rom: Option<[u8; 256]>,
    pub cartridge: Box<dyn Cartridge>,
    pub boot_rom_enabled: bool,
    timer: Timer,
    work_ram: [u8; 0x8000],
    high_ram: [u8; 0x7f],
    work_ram_bank: usize,
    interrupt_flags: u8,
    interrupt_enabled: u8,
}

impl MMU {
    pub fn new(boot_rom: Option<Vec<u8>>, rom: Vec<u8>) -> MMU {
        let boot_rom = boot_rom.map(|boot_rom_buffer| {
            if boot_rom_buffer.len() != BOOT_ROM_SIZE {
                panic!(
                    "Bootrom size mismatch, expected {}, got {}",
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
            timer: Timer::new(),
            boot_rom_enabled: true,
            cartridge: load_cartridge(rom),
            high_ram: [0x00; 0x7f],
            work_ram: [0x00; 0x8000],
            work_ram_bank: 0x01,
            interrupt_flags: 0x00,
            interrupt_enabled: 0x00,
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
        match address {
            // Last instruction is at 0xfe and its two bytes, therefore excluding 0xff from rom addressing
            0x0000...0x00ff => {
                if self.boot_rom_enabled {
                    if let Some(boot_rom) = self.boot_rom {
                        return boot_rom[address as usize];
                    }
                    return self.cartridge.get(address);
                }
                self.cartridge.get(address)
            }
            0x0000..=0x7fff => self.cartridge.get(address),
            0x8000..=0x9fff => {
                // Get GPU
                0
            }
            0xa000..=0xbfff => self.cartridge.get(address),
            0xc000..=0xcfff => self.work_ram[address as usize - 0xc000],
            0xd000..=0xdfff => {
                self.work_ram[address as usize - 0xd000 + 0x1000 * self.work_ram_bank]
            }
            0xe000..=0xefff => self.work_ram[address as usize - 0xe000],
            0xf000..=0xfdff => {
                self.work_ram[address as usize - 0xf000 + 0x1000 * self.work_ram_bank]
            }
            0xfe00..=0xfe9f => {
                // Get GPU
                0
            }
            0xfea0..=0xfeff => 0x00, // Invalid address
            0xff00 => {
                // IO
                0
            }
            0xff01..=0xff02 => {
                // Serial
                0
            }
            0xff04..=0xff07 => {
                // Clock
                0
            }
            0xff0f => self.interrupt_flags,
            0xff10..=0xff3f => {
                // APU
                0
            }
            0xf40...0xff4b => {
                // GPU
                0
            }
            0xff80..=0xfffe => self.high_ram[address as usize - 0xff80],
            0xffff => self.interrupt_enabled,
            _ => 0x0000,
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        unimplemented!()
    }
}
