use super::Cartridge;
use crate::memory::Memory;
use crate::save::Savable;
use std::path::PathBuf;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    bank: usize,
    bank_mode: BankMode,
    ram_enabled: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum BankMode {
    Rom,
    Ram,
}

/// MBC1 - Memory Bank Controller 1
/// Will include ram or battery or both
/// Bank will be selected to bank 1
impl Mbc1 {
    pub fn new(rom: Vec<u8>, ram_size: usize) -> Mbc1 {
        Self {
            rom,
            ram: vec![0; ram_size],
            bank: 0x01,
            bank_mode: BankMode::Rom,
            ram_enabled: false,
        }
    }
}

impl Memory for Mbc1 {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => self.rom[address as usize],
            0x4000..=0x7fff => {
                let selected_bank = if self.bank_mode == BankMode::Ram {
                    self.bank & 0x1f
                } else {
                    self.bank & 0x7f
                } as usize;
                let offset = selected_bank * 0x4000;
                self.rom[address as usize - 0x4000 + offset]
            }
            0xa000..=0xbfff => {
                if self.ram_enabled {
                    let selected_bank = if self.bank_mode == BankMode::Ram {
                        (self.bank & 0x60) >> 5
                    } else {
                        0x00
                    } as usize;
                    let offset = selected_bank * 0x2000;
                    self.ram[address as usize - 0xa000 + offset]
                } else {
                    0x00
                }
            }
            _ => 0x00,
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1fff => {
                if self.ram.len() > 0 {
                    self.ram_enabled = (value & 0x0f) == 0x0a
                }
            }
            0x2000..=0x3fff => {
                let bank = value & 0x1f;
                // Setting this value here will select the ROM bank at 0x4000 - 0x7fff
                // The zero rom bank can not be accessed from the address range 0x4000 - 0x7fff
                // Using a match to the value 0 and redirect it to the bank 1 instead of 0
                // Note: both 0x00 and 0x01 point to ROM bank 1, it is not a bug
                let bank = match bank {
                    0x00 => 0x01,
                    _ => bank,
                } as usize;
                self.bank = (self.bank & 0x60) | bank;
            }
            0x4000..=0x5fff => {
                let bank = (value & 0x03) as usize; // Select the upper two bits as bank number
                self.bank = self.bank & 0x9f | (bank << 5)
            }
            // The program may freely switch between both modes, the only limitation is that
            // only RAM Bank 00h can be used during Mode 0, and only ROM Banks 00-1Fh can be used during Mode 1.
            0x6000..=0x7fff => match value {
                0x00 => self.bank_mode = BankMode::Rom,
                0x01 => self.bank_mode = BankMode::Ram,
                n => panic!("Invalid bank mode selector type {:04x}", n),
            },
            _ => {}
        }
    }
}

impl Savable for Mbc1 {
    fn save(&self, _: PathBuf) {
        unimplemented!()
    }

    fn load(&self, _: PathBuf) {
        unimplemented!()
    }
}

impl Cartridge for Mbc1 {}
