mod mbc1;
mod mbc3;
mod rom;

use rom::Rom;

use super::memory::Memory;
use super::save::Savable;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CartridgeRomSize {
    Rom256K = 0x00,
    Rom512K = 0x01,
    Rom1M = 0x02,
    Rom2M = 0x03,
    Rom4M = 0x04,
    Rom8M = 0x05,
    Rom16M = 0x06,
}

impl CartridgeRomSize {
    fn from_u8(value: u8) -> Option<CartridgeRomSize> {
        match value {
            0x0 => Some(CartridgeRomSize::Rom256K),
            0x1 => Some(CartridgeRomSize::Rom512K),
            0x2 => Some(CartridgeRomSize::Rom1M),
            0x3 => Some(CartridgeRomSize::Rom2M),
            0x4 => Some(CartridgeRomSize::Rom4M),
            0x5 => Some(CartridgeRomSize::Rom8M),
            0x6 => Some(CartridgeRomSize::Rom16M),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CartridgeRamSize {
    NoRam = 0x00,
    Ram2K = 0x01,
    Ram8K = 0x02,
    Ram32K = 0x03,
    Ram128K = 0x04,
}

impl CartridgeRamSize {
    fn from_u8(value: u8) -> Option<CartridgeRamSize> {
        match value {
            0x00 => Some(CartridgeRamSize::NoRam),
            0x01 => Some(CartridgeRamSize::Ram2K),
            0x02 => Some(CartridgeRamSize::Ram8K),
            0x03 => Some(CartridgeRamSize::Ram32K),
            0x04 => Some(CartridgeRamSize::Ram128K),
            _ => None,
        }
    }
}

pub trait Cartridge: Memory + Savable {
    fn title(&self) -> String {
        let mut title = String::new();
        let title_start = 0x134;
        let mut title_end = 0x143;
        // Using the cartridge type to determine how far to read the data
        let is_cgb = match self.get(0x143) {
            0x80 => true,
            _ => false,
        };
        if is_cgb {
            title_end = 0x13e;
        }
        for address in title_start..title_end {
            match self.get(address) {
                0 => break,
                c => title.push(c as char),
            }
        }
        title
    }
    fn get_rom_size(&self) -> CartridgeRomSize {
        CartridgeRomSize::from_u8(self.get(0x148)).expect("Incorrect ROM size")
    }
    fn get_ram_size(&self) -> CartridgeRamSize {
        CartridgeRamSize::from_u8(self.get(0x149)).expect("Incorrect ROM size")
    }
    fn get_cart_info(&self) -> String {
        String::from(match self.get(0x147) {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0b => "MMM01",
            0x0c => "MMM01+RAM",
            0x0d => "MMM01+RAM+BATTERY",
            0x0f => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM",
            0x13 => "MBC3+RAM+BATTERY",
            0x15 => "MBC4",
            0x16 => "MBC4+RAM",
            0x17 => "MBC4+RAM+BATTERY",
            0x19 => "MBC5",
            0x1a => "MBC5+RAM",
            0x1b => "MBC5+RAM+BATTERY",
            0x1c => "MBC5+RUMBLE",
            0x1d => "MBC5+RUMBLE+RAM",
            0x1e => "MBC5+RUMBLE+RAM+BATTERY",
            0xfc => "POCKET CAMERA",
            0xfd => "BANDAI TAMA5",
            0xfe => "HuC3",
            0x1f => "HuC1+RAM+BATTERY",
            n => panic!("Unknown cartridge type: 0x{:02x}", n),
        })
    }
}

pub fn get_cartridge(rom: Vec<u8>) -> Box<dyn Cartridge> {
    let cartridge: Box<dyn Cartridge> = match rom[0x147] {
        0x00 => Box::new(Rom::new(rom)),
        _ => unimplemented!(),
    };

    debug!("Loaded cartridge: {}", cartridge.title());
    debug!("Cartridge type is: {}", cartridge.get_cart_info());
    debug!("Cartridge rom size is: {}", cartridge.get_rom_size());
    debug!("Cartridge ram size is: {}", cartridge.get_ram_size());

    cartridge
}
