use super::rtc::RealTimeClock;
use super::Cartridge;
use crate::memory::Memory;
use crate::save::Savable;
use std::path::PathBuf;

/// MBC3 - Memory Bank Controller 3
/// Can include additional RAM, battery, timer
/// Rom bank will be selected to bank 1
pub struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: usize,
    ram_bank: usize,
    rtc: RealTimeClock,
    ram_enabled: bool,
}

impl Mbc3 {
    pub fn new(rom: Vec<u8>, ram_size: usize) -> Mbc3 {
        Self {
            rom,
            ram: vec![0; ram_size],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            rtc: RealTimeClock::new(),
        }
    }
}

impl Memory for Mbc3 {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => self.rom[address as usize],
            0x4000..=0x7fff => {
                // Rom banks 01-7F (Read only)
                let offset = self.rom_bank * 0x4000;
                self.rom[address as usize - 0x4000 + offset]
            }
            0xa000..=0xbfff => {
                if self.ram_enabled {
                    if self.ram_bank <= 0x03 {
                        // Ram bank 00-03 is actual ram banks
                        self.ram[self.ram_bank * 0x2000 + address as usize - 0xa000]
                    } else {
                        // Ram bank 08-0C means we are reading from RTC
                        self.rtc.get(self.ram_bank as u16)
                    }
                } else {
                    0
                }
            }
            _ => 0x00,
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            // Mostly the same as for MBC1, a value of 0Ah will enable reading and writing to external RAM
            // and to the RTC Registers! A value of 00h will disable either.
            0x0000..=0x1fff => {
                self.ram_enabled = value & 0x0f == 0x0a;
            }
            // Same as for MBC1, except that the whole 7 bits of the RAM Bank Number are written
            // directly to this address. As for the MBC1, writing a value of 00h, will select
            // Bank 01h instead. All other values 01-7Fh select the corresponding ROM Banks.
            0x2000..=0x3fff => {
                let bank = (value & 0x7f) as usize;
                // Both 0 and 1 are pointed to the bank 1.
                let bank = match bank {
                    0x00 => 0x01,
                    _ => bank,
                };
                self.rom_bank = bank;
            }
            // Selecting the ram banks
            0x4000..=0x5fff => {
                let bank = (value & 0x0f) as usize;
                self.ram_bank = bank;
            }
            0x6000..=0x7fff => {
                // if writing 01h to the address, we latch the time to the register. We can achieve
                // this by calling tick on t he rtc
                if value & 0x01 != 0 {
                    self.rtc.tick();
                }
            }
            0xa000..=0xbfff => {
                if self.ram_enabled {
                    if self.ram_bank <= 0x03 {
                        // Ram bank 00-03 is actual ram banks
                        self.ram[self.ram_bank * 0x2000 + address as usize - 0xa000] = value;
                    } else {
                        // Ram bank 08-0C means we are setting from RTC
                        self.rtc.set(self.ram_bank as u16, value);
                    }
                }
            }
            _ => {}
        }
    }
}

impl Savable for Mbc3 {
    fn save(&self, _: PathBuf) {
        unimplemented!()
    }

    fn load(&self, _: PathBuf) {
        unimplemented!()
    }
}

impl Cartridge for Mbc3 {}
