use crate::memory::Memory;

use std::str;

#[derive(Debug)]
pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub cartridge_rom_size: CartridgeRomSize,
    pub cartridge_ram_size: CartridgeRamSize,
}

#[derive(Debug)]
pub enum Region {
    JP,
    NONJP,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CartridgeType {
    ROM {
        ram: bool,
        battery: bool,
    },
    MBC1 {
        ram: bool,
        battery: bool,
    },
    MBC2 {
        battery: bool,
    },
    MBC3 {
        ram: bool,
        battery: bool,
        rtc: bool,
    },
    MBC5 {
        ram: bool,
        battery: bool,
        rumble: bool,
    },
    HUC1,
    HUC3,
}

impl CartridgeType {
    fn from_u8(value: u8) -> Option<CartridgeType> {
        match value {
            0x0 => Some(CartridgeType::ROM {
                ram: false,
                battery: false,
            }),
            0x1 => Some(CartridgeType::MBC1 {
                ram: false,
                battery: false,
            }),
            0x2 => Some(CartridgeType::MBC1 {
                ram: true,
                battery: false,
            }),
            0x3 => Some(CartridgeType::MBC1 {
                ram: true,
                battery: true,
            }),
            0x5 => Some(CartridgeType::MBC2 { battery: false }),
            0x6 => Some(CartridgeType::MBC2 { battery: true }),
            0x8 => Some(CartridgeType::ROM {
                ram: true,
                battery: false,
            }),
            0x9 => Some(CartridgeType::ROM {
                ram: true,
                battery: true,
            }),
            0x10 => Some(CartridgeType::MBC3 {
                rtc: true,
                ram: true,
                battery: true,
            }),
            0x11 => Some(CartridgeType::MBC3 {
                rtc: false,
                ram: false,
                battery: false,
            }),
            0x12 => Some(CartridgeType::MBC3 {
                ram: true,
                rtc: false,
                battery: false,
            }),
            0x13 => Some(CartridgeType::MBC3 {
                ram: true,
                rtc: false,
                battery: true,
            }),
            0x19 => Some(CartridgeType::MBC5 {
                ram: false,
                rumble: false,
                battery: false,
            }),
            0x1a => Some(CartridgeType::MBC5 {
                ram: true,
                rumble: false,
                battery: false,
            }),
            0x1b => Some(CartridgeType::MBC5 {
                ram: true,
                rumble: false,
                battery: true,
            }),
            0x1c => Some(CartridgeType::MBC5 {
                ram: false,
                rumble: true,
                battery: false,
            }),
            0x1d => Some(CartridgeType::MBC5 {
                ram: true,
                rumble: true,
                battery: false,
            }),
            0x1e => Some(CartridgeType::MBC5 {
                ram: true,
                rumble: true,
                battery: true,
            }),
            0xfe => Some(CartridgeType::HUC3),
            0xff => Some(CartridgeType::HUC1),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CartridgeRomSize {
    RomBanks2 = 0x0,
    RomBanks4 = 0x1,
    RomBanks8 = 0x2,
    RomBanks16 = 0x3,
    RomBanks32 = 0x4,
    RomBanks64 = 0x5,
    RomBanks128 = 0x6,
}

impl CartridgeRomSize {
    fn from_u8(value: u8) -> Option<CartridgeRomSize> {
        match value {
            0x0 => Some(CartridgeRomSize::RomBanks2),
            0x1 => Some(CartridgeRomSize::RomBanks4),
            0x2 => Some(CartridgeRomSize::RomBanks8),
            0x3 => Some(CartridgeRomSize::RomBanks16),
            0x4 => Some(CartridgeRomSize::RomBanks32),
            0x5 => Some(CartridgeRomSize::RomBanks64),
            0x6 => Some(CartridgeRomSize::RomBanks128),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CartridgeRamSize {
    NoRam = 0x0,
    Ram2K = 0x1,
    Ram8K = 0x2,
    Ram32K = 0x3,
    Ram128K = 0x4,
}

fn ram_size(size: u8) -> usize {
    match size {
        0x00 => 0,
        0x01 => 1024 * 2,
        0x02 => 1024 * 8,
        0x03 => 1024 * 32,
        0x04 => 1024 * 128,
        0x05 => 1024 * 64,
        n => panic!("Unsupported ram size: 0x{:02x}", n),
    }
}

impl CartridgeRamSize {
    fn from_u8(value: u8) -> Option<CartridgeRamSize> {
        match value {
            0x0 => Some(CartridgeRamSize::NoRam),
            0x1 => Some(CartridgeRamSize::Ram2K),
            0x2 => Some(CartridgeRamSize::Ram8K),
            0x3 => Some(CartridgeRamSize::Ram32K),
            0x4 => Some(CartridgeRamSize::Ram128K),
            _ => None,
        }
    }
}

impl Cartridge {
    pub fn from_buffer(rom: Option<Vec<u8>>) -> Cartridge {
        let rom = rom.expect("Error loading ROM");
        if rom.len() < 0x8000 || rom.len() % 0x4000 != 0 {
            panic!("Invalid length: {} bytes", rom.len());
        }

        let title = {
            let slice = &rom[0x134..0x143];
            let utf8 = str::from_utf8(slice).unwrap_or("ERR: NO TITLE");
            utf8.trim_end_matches('\0').to_string()
        };

        let cartridge_type = CartridgeType::from_u8(rom[0x147]).expect("Incorrect cartridge type");
        let cartridge_rom_size = CartridgeRomSize::from_u8(rom[0x148]).expect("Incorrect ROM size");
        let cartridge_ram_size = CartridgeRamSize::from_u8(rom[0x149]).expect("Incorrect RAM size");

        Cartridge {
            rom,
            ram: vec![0; ram_size(cartridge_ram_size as u8)],
            cartridge_type,
            cartridge_rom_size,
            cartridge_ram_size,
            title,
        }
    }
    pub fn get_license(&self) -> u8 {
        self.rom[0x14b]
    }
    pub fn is_japanese(&self) -> Region {
        if self.rom[0x14a] == 0 {
            Region::JP
        } else {
            Region::NONJP
        }
    }
}

impl Memory for Cartridge {
    fn get(&self, a: u16) -> u8 {
        match self.cartridge_type {
            CartridgeType::ROM { ram: _, battery: _ } => self.rom[a as usize],
            _ => {
                panic!("Not implemented");
            }
        }
    }

    fn set(&mut self, _: u16, _: u8) {
        match self.cartridge_type {
            CartridgeType::ROM { ram: _, battery: _ } => {
                // No support rom only with ram
            }
            _ => {
                panic!("Not implemented");
            }
        }
    }
}
