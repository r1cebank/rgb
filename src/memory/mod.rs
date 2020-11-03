// General Memory Map
// 0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
// 4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
// 8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
// A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
// C000-CFFF   4KB Work RAM Bank 0 (WRAM)
// D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
// E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
// FE00-FE9F   Sprite Attribute Table (OAM)
// FEA0-FEFF   Not Usable
// FF00-FF7F   I/O Ports
// FF80-FFFE   High RAM (HRAM)
// FFFF        Interrupt Enable Register
//
// See: http://bgb.bircd.org/pandocs.htm#cgbregisters
use crate::cartridge::Cartridge;

pub trait Memory {
    fn get(&self, a: u16) -> u8;

    fn set(&mut self, a: u16, v: u8);

    fn get_word(&self, a: u16) -> u16 {
        u16::from(self.get(a)) | (u16::from(self.get(a + 1)) << 8)
    }

    fn set_word(&mut self, a: u16, v: u16) {
        self.set(a, (v & 0xFF) as u8);
        self.set(a + 1, (v >> 8) as u8)
    }
}

/// The memory for gameboy
const BOOT_ROM_SIZE: usize = 0x100;

pub struct MMU {
    pub cartridge: Cartridge,
    boot_rom: [u8; BOOT_ROM_SIZE],
    interrupt_enable: u8,
    work_ram: [u8; 0x8000],
    high_ram: [u8; 0x7f],
    work_ram_bank: usize,
    boot_rom_enabled: bool,
}

impl MMU {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>, rom_buffer: Option<Vec<u8>>) -> MMU {
        let boot_rom = boot_rom_buffer
            .map(|boot_rom_buffer| {
                if boot_rom_buffer.len() > BOOT_ROM_SIZE {
                    panic!(
                        "Bootroom size mismatch, expected {}, got {}",
                        BOOT_ROM_SIZE,
                        boot_rom_buffer.len()
                    );
                }
                let mut boot_rom = [0; BOOT_ROM_SIZE];
                boot_rom.copy_from_slice(&boot_rom_buffer);
                boot_rom
            })
            .unwrap();
        let cartridge = Cartridge::from_buffer(rom_buffer);
        MMU {
            boot_rom,
            cartridge,
            interrupt_enable: 0x00,
            high_ram: [0x00; 0x7f],
            work_ram: [0x00; 0x8000],
            work_ram_bank: 0x01,
            boot_rom_enabled: true,
        }
    }

    pub fn switch_speed(&mut self) {
        // Switching speed for CGB
    }

    pub fn next(&mut self, cycles: u32) -> u32 {
        1
    }
}

impl Memory for MMU {
    fn get(&self, address: u16) -> u8 {
        if self.boot_rom_enabled {
            return self.boot_rom[address as usize];
        }
        match address {
            0x0000..=0x7fff => self.cartridge.get(address),
            0x8000..=0x9fff => 1,
            0xa000..=0xbfff => self.cartridge.get(address),
            0xc000..=0xcfff => self.work_ram[address as usize - 0xc000],
            0xd000..=0xdfff => {
                self.work_ram[address as usize - 0xd000 + 0x1000 * self.work_ram_bank]
            }
            0xe000..=0xefff => self.work_ram[address as usize - 0xe000],
            0xf000..=0xfdff => {
                self.work_ram[address as usize - 0xf000 + 0x1000 * self.work_ram_bank]
            }
            0xfe00..=0xfe9f => 1,
            0xfea0..=0xfeff => 0x00,
            0xff00 => 1,
            0xff01..=0xff02 => 1,
            0xff04..=0xff07 => 1,
            0xff0f => 1,
            0xff10..=0xff3f => 1,
            0xff4d => 1,
            0xff40..=0xff45 | 0xff47..=0xff4b | 0xff4f => 1,
            0xff51..=0xff55 => 1,
            0xff68..=0xff6b => 1,
            0xff70 => self.work_ram_bank as u8,
            0xff80..=0xfffe => self.high_ram[address as usize - 0xff80],
            0xffff => self.interrupt_enable,
            _ => 0x00,
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7fff => self.cartridge.set(address, value),
            0x8000..=0x9fff => {}
            0xa000..=0xbfff => self.cartridge.set(address, value),
            0xc000..=0xcfff => self.work_ram[address as usize - 0xc000] = value,
            0xd000..=0xdfff => {
                self.work_ram[address as usize - 0xd000 + 0x1000 * self.work_ram_bank] = value
            }
            0xe000..=0xefff => self.work_ram[address as usize - 0xe000] = value,
            0xf000..=0xfdff => {
                self.work_ram[address as usize - 0xf000 + 0x1000 * self.work_ram_bank] = value
            }
            0xfe00..=0xfe9f => {}
            0xfea0..=0xfeff => {}
            0xff00 => {}
            0xff01..=0xff02 => {}
            0xff04..=0xff07 => {}
            0xff10..=0xff3f => {}
            0xff46 => {}
            0xff4d => {}
            0xff40..=0xff45 | 0xff47..=0xff4b | 0xff4f => {}
            0xff51..=0xff55 => {}
            0xff68..=0xff6b => {}
            0xff0f => {}
            0xff70 => {}
            0xff80..=0xfffe => self.high_ram[address as usize - 0xff80] = value,
            0xffff => self.interrupt_enable = value,
            _ => {}
        }
    }
}
