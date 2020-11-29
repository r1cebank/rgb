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
    // Mode 0
    HBlank,
    // Mode 1
    VBlank,
    // Mode 2
    OAMRead,
    // Mode 3
    VRAMRead,
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

pub type Tile = [[u8; 8]; 8];

// Digital image with mode RGB. Size = 144 * 160 * 3.
// 3---------
// ----------
// ----------
// ---------- 160
//        144
pub type PPUFramebuffer = [[[u8; 3]; FB_W]; FB_H];

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    y_pos: i32,
    x_pos: i32,
    tile: u8,
    priority_behind_bg: bool,
    y_flip: bool,
    x_flip: bool,
    use_palette_1: bool,
    index: usize,
}

impl Sprite {
    fn new() -> Sprite {
        Sprite {
            y_pos: 0x00,
            x_pos: 0x00,
            tile: 0x00,

            priority_behind_bg: false,
            y_flip: false,
            x_flip: false,
            use_palette_1: false,

            index: 0,
        }
    }
}

pub struct PPU {
    pub interrupt_flags: Rc<RefCell<InterruptFlags>>,
    pub framebuffer: PPUFramebuffer,
    pub tile_set: [Tile; TILE_MAP_SIZE],
    pub video_ram: [u8; VRAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub mode: Mode,
    sprites: [Sprite; 40],
    palette: [[u8; 3]; 4],

    bgp: u8, // Background sprite

    // This register assigns gray shades for sprite palette 0. It works exactly as BGP (FF47), except that the lower
    // two bits aren't used because sprite data 00 is transparent.
    op0: u8,
    // This register assigns gray shades for sprite palette 1. It works exactly as BGP (FF47), except that the lower
    // two bits aren't used because sprite data 00 is transparent.
    op1: u8,

    // Window Y Position (R/W), Window X Position minus 7 (R/W)
    wy: u8,
    wx: u8,

    mode_clock: u32,
    line: u8,
    scroll_x: u8,
    scroll_y: u8,
    lcdc_display_enabled: bool,
    lcdc_window_tilemap: bool,
    lcdc_window_enabled: bool,
    lcdc_bg_and_windown_tile_base: bool,
    lcdc_bg_tilemap_base: bool,
    lcdc_obj_sprite_size: bool,
    lcdc_obj_sprite_display_enabled: bool,
    lcdc_bg_enabled: bool,
    ly_coincidence: u8,
    ly_coincidence_interrupt_enabled: bool,
    mode_0_interrupt_enabled: bool,
    mode_1_interrupt_enabled: bool,
    mode_2_interrupt_enabled: bool,
    horiz_blanking: bool,
    tick_counter: u64,
}

impl PPU {
    pub fn new(interrupt_flags: Rc<RefCell<InterruptFlags>>) -> PPU {
        Self {
            interrupt_flags,
            video_ram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            tile_set: [[[0x00; 8]; 8]; TILE_MAP_SIZE],
            framebuffer: [[[0x00; 3]; FB_W]; FB_H],
            sprites: [Sprite::new(); 40],
            palette: [[254, 248, 208], [136, 192, 112], [39, 80, 70], [8, 24, 32]],
            mode_clock: 0,
            line: 0,
            scroll_x: 0,
            scroll_y: 0,

            bgp: 0x00,

            op0: 0x00,
            op1: 0x01,
            wx: 0x00,
            wy: 0x00,

            lcdc_display_enabled: false,
            lcdc_window_tilemap: true,
            lcdc_window_enabled: false,
            lcdc_bg_and_windown_tile_base: true,
            lcdc_bg_tilemap_base: true,
            lcdc_obj_sprite_size: false,
            lcdc_obj_sprite_display_enabled: false,
            lcdc_bg_enabled: false,

            ly_coincidence: 0x00,
            ly_coincidence_interrupt_enabled: false,
            mode_0_interrupt_enabled: false,
            mode_1_interrupt_enabled: false,
            mode_2_interrupt_enabled: false,

            horiz_blanking: false,
            mode: Mode::HBlank,
            tick_counter: 0,
        }
    }
    pub fn update_tile(&mut self, address: u16, value: u8) {
        // Get the "base address" for this tile row
        let base_address = address & 0x1ffe;

        if value != 0x00 {
            // Nothing but zeros being written, in infinite loop.
            trace!("VRAM:TILE {:#x}", value);
        }

        // Work out which tile and row was updated
        let tile = (base_address >> 4) & 511;
        let y = (base_address >> 1) & 7;

        for x in 0..8 {
            // Find bit index for this pixel
            let sx = 1 << (7 - x);

            // Update tile set
            let pixel_colour = if self.video_ram[base_address as usize] & sx != 0 {
                1
            } else {
                0
            } + if self.video_ram[(base_address + 1) as usize] & sx != 0 {
                2
            } else {
                0
            };

            self.tile_set[tile as usize][y as usize][x as usize] = pixel_colour;
        }
    }
    pub fn tick(&mut self, cycles: u32) {
        self.tick_counter += 1;

        if !self.lcdc_display_enabled {
            return;
        }

        self.horiz_blanking = false;
        let mut ticks_remaining = cycles;

        while ticks_remaining > 0 {
            let current_ticks = if ticks_remaining >= 80 {
                80
            } else {
                ticks_remaining
            };
            self.mode_clock += current_ticks;
            ticks_remaining -= current_ticks;

            if self.mode_clock >= 456 {
                self.mode_clock -= 456;
                self.line = (self.line + 1) % 154;
                if self.ly_coincidence_interrupt_enabled && self.line == self.ly_coincidence {
                    self.interrupt_flags.borrow_mut().hi(Flag::LCDStat);
                }

                if self.line >= 144 && self.mode != Mode::VBlank {
                    self.change_mode(Mode::VBlank);
                }
            }

            if self.line < 144 {
                if self.mode_clock <= 80 {
                    if self.mode != Mode::OAMRead {
                        self.change_mode(Mode::OAMRead);
                    }
                } else if self.mode_clock <= (80 + 172) {
                    if self.mode != Mode::VRAMRead {
                        self.change_mode(Mode::VRAMRead);
                    }
                } else {
                    if self.mode != Mode::HBlank {
                        self.change_mode(Mode::HBlank);
                    }
                }
            }
        }
    }

    fn render_background(&mut self) {
        // tiles: 8x8 pixels
        // two maps: 32x32 each

        let mapbase: usize = if self.lcdc_bg_tilemap_base {
            0x1c00
        } else {
            0x1800
        };
        let line = self.line as usize + self.scroll_y as usize;
        let mapbase = mapbase + ((line % 256) >> 3) * 32;
        let y = (self.line.wrapping_add(self.scroll_y)) % 8;
        let mut x = self.scroll_x % 8;
        let mut canvas_offset = (self.line as usize) * 160;
        let mut i = 0;
        let tilebase = if !self.lcdc_bg_and_windown_tile_base {
            256
        } else {
            0
        };
        while i < 160 {
            let mapoff = ((i as usize + self.scroll_x as usize) % 256) >> 3;
            let tilei = self.video_ram[mapbase + mapoff];

            let tilebase = if self.lcdc_bg_and_windown_tile_base {
                tilebase + tilei as usize
            } else {
                (tilebase as isize + (tilei as i8 as isize)) as usize
            };

            let row;
            row = self.tile_set[tilebase as usize][y as usize];

            while x < 8 && i < 160 as u8 {
                let palette_index = row[x as usize];
                let colour = self.palette[palette_index as usize];

                let pixel_y = canvas_offset / FB_W;
                let pixel_x = canvas_offset % FB_W;

                // println!("offset: {}, x: {}, y: {}", canvas_offset, pixel_x, pixel_y);

                self.framebuffer[pixel_y][pixel_x][0] = colour[0];
                self.framebuffer[pixel_y][pixel_x][1] = colour[1];
                self.framebuffer[pixel_y][pixel_x][2] = colour[2];

                x += 1;
                i += 1;
                canvas_offset += 1;
            }

            x = 0;
        }
    }

    fn render_sprites(&mut self) {
        for sprite in self.sprites.iter() {
            let line = self.line as i32;

            if self.lcdc_obj_sprite_size {
                panic!("Double-sized sprites not yet supported");
            }

            // If the sprite falls within the scanline
            if sprite.y_pos <= line && (sprite.y_pos + 8) > line {
                let mut canvas_offset = ((line * 160) + sprite.x_pos) as usize;
                let tile_row;

                if sprite.y_flip {
                    tile_row =
                        self.tile_set[sprite.tile as usize][7 - (line - sprite.y_pos) as usize];
                } else {
                    tile_row = self.tile_set[sprite.tile as usize][(line - sprite.y_pos) as usize];
                }

                let mut color;

                for x in 0..8 {
                    if sprite.x_pos + x >= 0 && sprite.x_pos + x < 160 {
                        let palette_index = if sprite.x_flip {
                            7 - x as usize
                        } else {
                            x as usize
                        };

                        let color_index = tile_row[palette_index];
                        color = self.palette[color_index as usize];

                        let pixel_y = canvas_offset / FB_W;
                        let pixel_x = canvas_offset % FB_W;

                        canvas_offset += 1;

                        if color_index == 0 {
                            continue;
                        }

                        if self.framebuffer[pixel_y][pixel_x] != self.palette[0] {
                            if sprite.priority_behind_bg {
                                continue;
                            }
                        }

                        self.framebuffer[pixel_y][pixel_x][0] = color[0];
                        self.framebuffer[pixel_y][pixel_x][1] = color[1];
                        self.framebuffer[pixel_y][pixel_x][2] = color[2];
                    }
                }
            }
        }
    }

    pub fn update_sprite_object(&mut self, sprite_addr: usize, value: u8) {
        let sprite_index = sprite_addr >> 2;
        let byte = sprite_addr & 3;

        match byte {
            0x00 => self.sprites[sprite_index].y_pos = value as i32 - 16,
            0x01 => self.sprites[sprite_index].x_pos = value as i32 - 8,
            0x02 => self.sprites[sprite_index].tile = value,
            0x03 => {
                self.sprites[sprite_index].priority_behind_bg = (value & 0b1000_0000) != 0;
                self.sprites[sprite_index].y_flip = (value & 0b0100_0000) != 0;
                self.sprites[sprite_index].x_flip = (value & 0b0010_0000) != 0;
                self.sprites[sprite_index].use_palette_1 = (value & 0b0001_0000) != 0;
            }
            _ => panic!("Invalid byte in update_sprite_object"),
        }
    }

    fn render_scanline(&mut self) {
        trace!("Rendering scanline, {:?}", self.mode);
        self.render_background();
        self.render_sprites();
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;
        if match self.mode {
            Mode::HBlank => {
                self.render_scanline();
                self.horiz_blanking = true;
                self.mode_0_interrupt_enabled
            }
            Mode::VBlank => {
                self.interrupt_flags.borrow_mut().hi(Flag::VBlank);
                self.mode_1_interrupt_enabled
            }
            Mode::OAMRead => self.mode_2_interrupt_enabled,
            Mode::VRAMRead => false,
        } {
            self.interrupt_flags.borrow_mut().hi(Flag::LCDStat);
        }
    }
}

impl Memory for PPU {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9fff => self.video_ram[address as usize - 0x8000],
            0xfe00...0xfe9f => self.oam[address as usize - 0xfe00],
            0xff40 => {
                (if self.lcdc_display_enabled {
                    0b1000_0000
                } else {
                    0
                }) | (if self.lcdc_window_tilemap {
                    0b0100_0000
                } else {
                    0
                }) | (if self.lcdc_window_enabled {
                    0b0010_0000
                } else {
                    0
                }) | (if self.lcdc_bg_and_windown_tile_base {
                    0b0001_0000
                } else {
                    0
                }) | (if self.lcdc_bg_tilemap_base {
                    0b0000_1000
                } else {
                    0
                }) | (if self.lcdc_obj_sprite_size {
                    0b0000_0100
                } else {
                    0
                }) | (if self.lcdc_obj_sprite_display_enabled {
                    0b0000_0010
                } else {
                    0
                }) | (if self.lcdc_bg_enabled { 0b0000_0001 } else { 0 })
            }
            0xff41 => {
                let ff41_val = (if self.ly_coincidence_interrupt_enabled {
                    0x40
                } else {
                    0
                }) | (if self.mode_2_interrupt_enabled {
                    0x20
                } else {
                    0
                }) | (if self.mode_1_interrupt_enabled {
                    0x10
                } else {
                    0
                }) | (if self.mode_0_interrupt_enabled {
                    0x08
                } else {
                    0
                }) | (if self.line == self.ly_coincidence {
                    0x04
                } else {
                    0
                }) | self.mode as u8;

                ff41_val
            }
            0xff42 => self.scroll_y,
            0xff43 => self.scroll_x,
            0xff44 => self.line,
            0xff45 => self.ly_coincidence,
            0xff47 => self.bgp,
            0xff48 => self.op0,
            0xff49 => self.op1,
            0xff4a => self.wy,
            0xff4b => self.wx,
            _ => panic!("Read not implemented for address: ${:04x}", address),
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9fff => {
                self.video_ram[address as usize - 0x8000] = value;

                // Tile Data is stored in VRAM at addresses 8000h-97FFh, this area defines the Bitmaps for 192 Tiles.
                if address <= 0x97ff {
                    self.update_tile(address, value);
                }
            }
            0xfe00..=0xfe9f => {
                let sprite_addr = address as usize - 0xfe00;
                self.oam[sprite_addr] = value;

                self.update_sprite_object(sprite_addr, value);
            }
            0xff40 => {
                let previous_lcdc_display_enabled = self.lcdc_display_enabled;

                self.lcdc_display_enabled = value & 0b1000_0000 != 0;
                self.lcdc_window_tilemap = value & 0b0100_0000 != 0;
                self.lcdc_window_enabled = value & 0b0010_0000 != 0;
                self.lcdc_bg_and_windown_tile_base = value & 0b0001_0000 != 0;
                self.lcdc_bg_tilemap_base = value & 0b0000_1000 != 0;
                self.lcdc_obj_sprite_size = value & 0b0000_0100 != 0;
                self.lcdc_obj_sprite_display_enabled = value & 0b0000_0010 != 0;
                self.lcdc_bg_enabled = value & 0b0000_0001 != 0;

                if previous_lcdc_display_enabled && !self.lcdc_display_enabled {
                    self.mode_clock = 0;
                    self.line = 0;
                    self.change_mode(Mode::HBlank);
                }
            }
            0xff41 => {
                self.ly_coincidence_interrupt_enabled = value & 0x40 == 0x40;
                self.mode_2_interrupt_enabled = value & 0x20 == 0x20;
                self.mode_1_interrupt_enabled = value & 0x10 == 0x10;
                self.mode_0_interrupt_enabled = value & 0x08 == 0x08;
            }
            0xff42 => self.scroll_y = value,
            0xff43 => self.scroll_x = value,
            0xff44 => {} // ly is changed by scanline
            0xff45 => self.ly_coincidence = value,
            0xff47 => {
                self.bgp = value;
                for i in 0..4 {
                    match (value >> (i * 2)) & 3 {
                        0 => self.palette[i] = [254, 248, 208],
                        1 => self.palette[i] = [136, 192, 112],
                        2 => self.palette[i] = [39, 80, 70],
                        3 => self.palette[i] = [8, 24, 32],
                        _ => {
                            panic!("Unexpected background palette value: {:#X}", i);
                        }
                    }
                }
            }
            0xff48 => {
                self.op0 = value;
            }
            0xff49 => {
                self.op1 = value;
            }
            0xff4A => {
                self.wy = value;
            }
            0xff4b => self.wx = value,
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
