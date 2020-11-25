use super::timer::Timer;
use super::Memory;
use crate::cartridge::{load_cartridge, Cartridge};
use crate::cpu::interrupt::InterruptFlags;
use crate::ppu::PPU;
use crate::util::BOOT_ROM_SIZE;
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;

pub struct MMU {
    pub boot_rom: Option<[u8; 256]>,
    pub cartridge: Box<dyn Cartridge>,
    pub ppu: RefCell<PPU>,
    boot_rom_enabled: bool,
    timer: Timer,
    last_serial: u8,
    work_ram: [u8; 0x8000],
    high_ram: [u8; 0x7f],
    work_ram_bank: usize,
    interrupt_flags: Rc<RefCell<InterruptFlags>>,
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
        // The interrupt flag is shared across each component in the gameboy, any component is able
        // to raise an interrupt
        let interrupt_flags = Rc::new(RefCell::new(InterruptFlags::new()));
        Self {
            boot_rom,
            timer: Timer::new(interrupt_flags.clone()),
            ppu: RefCell::new(PPU::new(interrupt_flags.clone())),
            last_serial: 0x00,
            interrupt_flags: interrupt_flags.clone(),
            boot_rom_enabled: boot_rom != None,
            cartridge: load_cartridge(rom),
            high_ram: [0x00; 0x7f],
            work_ram: [0x00; 0x8000],
            work_ram_bank: 0x01,
            interrupt_enabled: 0x00,
        }
    }
    /// Update the MMU cycles, will tick the clock
    pub fn tick(&mut self, cycles: u32) {
        self.timer.tick(cycles);
        self.ppu.borrow_mut().tick(cycles);
    }

    /// DMA oam table to ppu, in order to have sprites on the screen, cartridge will often use DMA
    /// to copy oam table to ppu memory, we use the oam start address to set oam table from the source
    fn oam_dma(&mut self, source_address: u8) {
        trace!("OAM DMA address: ${:04x}", source_address);
        let source_base_address = (source_address as u16) << 8;
        const OAM_START_ADDRESS: u16 = 0xfe00;

        for index in 0x00..0xa0 {
            let source_byte = self.get(source_base_address + index);
            self.set(OAM_START_ADDRESS + index, source_byte);
        }
    }

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
            0x0000...0x7fff => {
                if self.boot_rom_enabled && self.boot_rom != None && address < 0x100 {
                    self.boot_rom.unwrap()[address as usize]
                } else {
                    self.cartridge.get(address)
                }
            }
            0x8000..=0x9fff => self.ppu.borrow().get(address),
            0xa000..=0xbfff => self.cartridge.get(address),
            0xc000..=0xcfff => self.work_ram[address as usize - 0xc000],
            0xd000..=0xdfff => {
                self.work_ram[address as usize - 0xd000 + 0x1000 * self.work_ram_bank]
            }
            0xe000..=0xefff => self.work_ram[address as usize - 0xe000],
            0xf000..=0xfdff => {
                self.work_ram[address as usize - 0xf000 + 0x1000 * self.work_ram_bank]
            }
            0xfe00..=0xfe9f => self.ppu.borrow().get(address),
            0xfea0..=0xfeff => 0x00, // Invalid address
            0xff00 => {
                // IO
                0xff
            }
            0xff01..=0xff02 => {
                // Serial
                0
            }
            0xff04..=0xff07 => {
                // Clock
                self.timer.get(address)
            }
            0xff0f => self.interrupt_flags.borrow_mut().data,
            0xff10..=0xff3f => {
                // APU
                0
            }
            0xf40...0xff4b => self.ppu.borrow().get(address),
            0xff80..=0xfffe => self.high_ram[address as usize - 0xff80],
            0xffff => self.interrupt_enabled,
            _ => 0x0000,
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7fff => self.cartridge.set(address, value),
            0x8000..=0x9fff => self.ppu.borrow_mut().set(address, value),
            0xa000..=0xbfff => self.cartridge.set(address, value),
            0xc000..=0xcfff => self.work_ram[address as usize - 0xc000] = value,
            0xd000..=0xdfff => {
                self.work_ram[address as usize - 0xd000 + 0x1000 * self.work_ram_bank] = value
            }
            0xe000..=0xefff => self.work_ram[address as usize - 0xe000] = value,
            0xf000..=0xfdff => {
                self.work_ram[address as usize - 0xf000 + 0x1000 * self.work_ram_bank] = value
            }
            0xfe00..=0xfe9f => self.ppu.borrow_mut().set(address, value),
            0xfea0..=0xfeff => {
                // Not used
            }
            0xff00 => {
                // Input
            }
            0xff01..=0xff02 => {
                // Serial
                // if address == 0xff01 {
                //     self.last_serial = value;
                // }
                // if address == 0xff02 {
                //     print!("{}", self.last_serial as char);
                //     io::stdout().flush();
                // }
            }
            0xff04..=0xff07 => self.timer.set(address, value),
            0xff0f => self.interrupt_flags.borrow_mut().data = value,
            0xff10..=0xff3f => {
                // Sound
            }
            0xff46 => self.oam_dma(value),
            0xff40..=0xff45 | 0xff47..=0xff4b | 0xff4f => self.ppu.borrow_mut().set(address, value),
            0xff50 => {
                self.boot_rom_enabled = false;
            }
            0xff70 => {
                self.work_ram_bank = match value & 0x7 {
                    0 => 1,
                    n => n as usize,
                };
            }
            0xff80...0xfffe => self.high_ram[address as usize - 0xff80] = value,
            0xffff => self.interrupt_enabled = value,
            _ => {}
        }
    }
}
