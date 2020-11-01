use crate::memory::Memory;

use std::str;
pub mod rtc;

#[derive(Debug)]
pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub cartridge_rom_size: CartridgeRomSize,
    pub cartridge_ram_size: CartridgeRamSize,
    pub mbc_state: MBCState,
}

#[derive(Debug)]
pub enum Region {
    JP,
    NONJP,
}

#[derive(Debug)]
pub enum BankMode {
    RAM,
    ROM,
}

#[derive(Debug)]
pub struct MBC1State {
    pub bank_mode: BankMode,
    pub bank: usize,
    pub ram_enable: bool,
}

#[derive(Debug)]
pub struct MBC2State {
    pub bank: usize,
    pub ram_enable: bool,
}

#[derive(Debug)]
pub struct MBC3State {
    pub rtc: rtc::RealTimeClock,
    pub rom_bank: usize,
    pub ram_bank: usize,
    pub ram_enable: bool,
}

#[derive(Debug)]
pub struct MBC5State {
    rom_bank: usize,
    ram_bank: usize,
    ram_enable: bool,
}

#[derive(Debug)]
pub struct MBCState {
    pub mbc1: MBC1State,
    pub mbc2: MBC2State,
    pub mbc3: MBC3State,
    pub mbc5: MBC5State,
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
            0x00 => Some(CartridgeType::ROM {
                ram: false,
                battery: false,
            }),
            0x01 => Some(CartridgeType::MBC1 {
                ram: false,
                battery: false,
            }),
            0x02 => Some(CartridgeType::MBC1 {
                ram: true,
                battery: false,
            }),
            0x03 => Some(CartridgeType::MBC1 {
                ram: true,
                battery: true,
            }),
            0x05 => Some(CartridgeType::MBC2 { battery: false }),
            0x06 => Some(CartridgeType::MBC2 { battery: true }),
            0x08 => Some(CartridgeType::ROM {
                ram: true,
                battery: false,
            }),
            0x09 => Some(CartridgeType::ROM {
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
    RomBanks2 = 0x00,
    RomBanks4 = 0x01,
    RomBanks8 = 0x02,
    RomBanks16 = 0x03,
    RomBanks32 = 0x04,
    RomBanks64 = 0x05,
    RomBanks128 = 0x06,
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
    NoRam = 0x00,
    Ram2K = 0x01,
    Ram8K = 0x02,
    Ram32K = 0x03,
    Ram128K = 0x04,
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
            0x00 => Some(CartridgeRamSize::NoRam),
            0x01 => Some(CartridgeRamSize::Ram2K),
            0x02 => Some(CartridgeRamSize::Ram8K),
            0x03 => Some(CartridgeRamSize::Ram32K),
            0x04 => Some(CartridgeRamSize::Ram128K),
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

        let mbc_state = MBCState {
            mbc1: MBC1State {
                bank_mode: BankMode::ROM,
                bank: 0x01,
                ram_enable: false,
            },
            mbc2: MBC2State {
                bank: 0x01,
                ram_enable: false,
            },
            mbc3: MBC3State {
                rtc: rtc::RealTimeClock::new(),
                rom_bank: 0x01,
                ram_bank: 0x00,
                ram_enable: false,
            },
            mbc5: MBC5State {
                rom_bank: 0x01,
                ram_bank: 0x00,
                ram_enable: false,
            },
        };

        Cartridge {
            rom,
            ram: vec![0; ram_size(cartridge_ram_size as u8)],
            cartridge_type,
            cartridge_rom_size,
            cartridge_ram_size,
            title,
            mbc_state,
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

    pub fn print_rom_info(&self) {
        println!("Title: {}", self.title);
        println!("Region: {:?}", self.is_japanese());
        println!("Type: {:?}", self.cartridge_type);
        println!("Rom Size: {:?}", self.cartridge_rom_size);
        println!("Ram Size: {:?}", self.cartridge_ram_size);
        println!("MBC State: {:?}", self.mbc_state);
    }
}

// https://problemkaputt.de/pandocs.htm#mbc3max2mbyteromandor32kbyteramandtimer
impl Memory for Cartridge {
    fn get(&self, address: u16) -> u8 {
        match self.cartridge_type {
            CartridgeType::ROM { ram: _, battery: _ } => self.rom[address as usize],
            CartridgeType::MBC3 {
                ram: _,
                battery: _,
                rtc: _,
            } => match address {
                0x0000..=0x3fff => self.rom[address as usize], // ROM bank 00
                0x4000..=0x7fff => {
                    // ROM bank 01-7F
                    let i = self.mbc_state.mbc3.rom_bank * 0x4000 + address as usize - 0x4000;
                    self.rom[i]
                }
                0xa000..=0xbfff => {
                    // RAM Bank read and write upper address is for RTC
                    if self.mbc_state.mbc3.ram_enable {
                        if self.mbc_state.mbc3.ram_bank <= 0x03 {
                            let i =
                                self.mbc_state.mbc3.ram_bank * 0x2000 + address as usize - 0xa000;
                            self.ram[i]
                        } else {
                            self.mbc_state
                                .mbc3
                                .rtc
                                .get(self.mbc_state.mbc3.ram_bank as u16)
                        }
                    } else {
                        0x00
                    }
                }
                _ => 0x00,
            },
            // TODO: Implement more MBC types, currently focusing in MBC3
            _ => {
                panic!("Not implemented");
            }
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match self.cartridge_type {
            CartridgeType::ROM { ram: _, battery: _ } => {
                // No support rom only with ram
            }
            CartridgeType::MBC3 {
                ram: _,
                battery: _,
                rtc: _,
            } => match address {
                0xa000..=0xbfff => {
                    // RAM bank for R/W
                    if self.mbc_state.mbc3.ram_enable {
                        if self.mbc_state.mbc3.ram_bank <= 0x03 {
                            let i =
                                self.mbc_state.mbc3.ram_bank * 0x2000 + address as usize - 0xa000;
                            self.ram[i] = value;
                        } else {
                            self.mbc_state
                                .mbc3
                                .rtc
                                .set(self.mbc_state.mbc3.ram_bank as u16, value)
                        }
                    }
                }
                0x0000..=0x1fff => {
                    // RAM enable
                    self.mbc_state.mbc3.ram_enable = value & 0x0f == 0x0a;
                }
                0x2000..=0x3fff => {
                    // Bank number select
                    // Same as for MBC1, except that the whole 7 bits of the RAM Bank Number are written directly to this address.
                    // As for the MBC1, writing a value of 00h, will select Bank 01h instead.
                    // All other values 01-7Fh select the corresponding ROM Banks.
                    let rom_bank = (value & 0x7f) as usize;
                    let rom_bank = match rom_bank {
                        0x00 => 0x01,
                        _ => rom_bank,
                    };
                    self.mbc_state.mbc3.rom_bank = rom_bank;
                }
                0x4000..=0x5fff => {
                    let ram_bank = (value & 0x0f) as usize;
                    self.mbc_state.mbc3.ram_bank = ram_bank;
                }
                0x6000..=0x7fff => {
                    if value & 0x01 != 0 {
                        self.mbc_state.mbc3.rtc.tick();
                    }
                }
                _ => {}
            },
            _ => {
                panic!("Not implemented");
            }
        }
    }
}
