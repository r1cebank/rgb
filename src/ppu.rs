use crate::cpu::interrupt::{Flag, InterruptFlags};
use crate::memory::Memory;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub const FB_W: usize = 160;
pub const FB_H: usize = 144;
pub const VRAM_SIZE: usize = 0x2000;
pub const OAM_SIZE: usize = 0xa0;
pub const TILE_MAP_SIZE: usize = 384;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    HBlank,
    // Mode 0
    VBlank,
    // Mode 1
    OAMRead,
    // Mode 2
    VRAMRead, // Mode 3
}

impl std::convert::From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OAMRead => 2,
            Mode::VRAMRead => 3,
        }
    }
}

type Tile = [[u8; 8]; 8];

// Digital image with mode RGB. Size = 144 * 160 * 3.
// 3---------
// ----------
// ----------
// ---------- 160
//        144
pub type PPUFramebuffer = [[[u8; 3]; FB_W]; FB_H];

pub struct PPU {
    pub interrupt_flags: Rc<RefCell<InterruptFlags>>,
    pub framebuffer: PPUFramebuffer,
    pub tile_set: [Tile; TILE_MAP_SIZE],
    pub vram: [u8; VRAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub mode: Mode,
    scx: u8,
    scy: u8,
    ly: u8,
    lyc: u8,
    cycles: u16,
}

impl PPU {
    pub fn new(interrupt_flags: Rc<RefCell<InterruptFlags>>) -> PPU {
        Self {
            interrupt_flags,
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            tile_set: [[[0x00; 8]; 8]; TILE_MAP_SIZE],
            framebuffer: [[[0x00; 3]; FB_W]; FB_H],
            mode: Mode::HBlank,
            scx: 0,
            scy: 0,
            ly: 144,
            lyc: 0xff,
            cycles: 0,
        }
    }
    pub fn update_tile(&mut self, address: u16, value: u8) {
        // Get the "base address" for this tile row
        let base_address = address & 0x1ffe;

        if value != 0x00 {
            // Nothing but zeros being written, in infinite loop.
            debug!("Writing data to VRAM!: {:#X}", value);
        }

        // Work out which tile and row was updated
        let tile = (base_address >> 4) & 511;
        let y = (base_address >> 1) & 7;

        for x in 0..8 {
            // Find bit index for this pixel
            let sx = 1 << (7 - x);

            // Update tile set
            let pixel_colour = if self.vram[base_address as usize] & sx != 0 {
                1
            } else {
                0
            } + if self.vram[(base_address + 1) as usize] & sx != 0 {
                2
            } else {
                0
            };

            self.tile_set[tile as usize][y as usize][x as usize] = pixel_colour;
        }
    }
    pub fn tick(&mut self, cycles: u32) {
        self.cycles += cycles as u16;

        match self.mode {
            Mode::HBlank => {
                if self.cycles >= 200 {
                    self.cycles = self.cycles % 200;
                    self.ly += 1;

                    if self.ly >= 144 {
                        self.mode = Mode::VBlank;
                        self.interrupt_flags.borrow_mut().hi(Flag::VBlank);
                        // TODO: I am here
                    }
                }
            }
            Mode::VBlank => {}
            Mode::OAMRead => {}
            Mode::VRAMRead => {}
        }
    }
}

impl Memory for PPU {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9fff => self.vram[address as usize - 0x8000],
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            _ => panic!("Read not implemented for address: ${:04x}", address),
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9fff => {
                self.vram[address as usize - 0x8000] = value;

                // Tile Data is stored in VRAM at addresses 8000h-97FFh, this area defines the Bitmaps for 192 Tiles.
                if address <= 0x97ff {
                    self.update_tile(address, value);
                }
            }
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff44 => {} // ly is changed by scanline
            0xff45 => self.lyc = value,
            _ => panic!("Write not implemented for address: ${:04x}", address),
        }
    }
}

pub fn random_framebuffer() -> PPUFramebuffer {
    let mut framebuffer = [[[0x00; 3]; FB_W]; FB_H];
    let mut rng = rand::thread_rng();
    for i in 0..framebuffer.len() {
        let random_color = [
            rng.gen_range(0, 256),
            rng.gen_range(0, 256),
            rng.gen_range(0, 256),
        ];
        for j in 0..framebuffer[i].len() {
            framebuffer[i][j] = [
                random_color[0] as u8,
                random_color[1] as u8,
                random_color[2] as u8,
            ];
        }
    }

    framebuffer
}
