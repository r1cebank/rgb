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

#[derive(Debug, PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const WHITE: Color = Color {
    r: 254,
    g: 248,
    b: 208,
    a: 255,
};
const LIGHT_GRAY: Color = Color {
    r: 136,
    g: 192,
    b: 112,
    a: 255,
};
const DARK_GRAY: Color = Color {
    r: 39,
    g: 80,
    b: 70,
    a: 255,
};
const BLACK: Color = Color {
    r: 8,
    g: 24,
    b: 32,
    a: 255,
};

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
    ly: u8,
    scroll_x: u8,
    scroll_y: u8,
    lcdc_display_enabled: bool,
    lcdc_window_tilemap: bool,
    lcdc_window_enabled: bool,
    lcdc_bg_and_window_tile_base: bool,
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
            mode_clock: 0,
            ly: 0,
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
            lcdc_bg_and_window_tile_base: true,
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
                self.ly = (self.ly + 1) % 154;
                if self.ly_coincidence_interrupt_enabled && self.ly == self.ly_coincidence {
                    self.interrupt_flags.borrow_mut().hi(Flag::LCDStat);
                }

                if self.ly >= 144 && self.mode != Mode::VBlank {
                    self.change_mode(Mode::VBlank);
                }
            }

            if self.ly < 144 {
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
        let scanline = self.ly;

        let scroll_y = self.scroll_y;
        let scroll_x = self.scroll_x;
        let window_x = self.wx.wrapping_sub(7);
        let window_y = self.wy;

        let use_window = self.lcdc_window_enabled && window_y <= scanline;

        let (tile_data, unsigned): (u16, bool) = if self.lcdc_bg_and_window_tile_base {
            (0x8000, true)
        } else {
            (0x8800, false)
        };

        let background_mem = if use_window {
            if self.lcdc_window_tilemap {
                0x9c00
            } else {
                0x9800
            }
        } else {
            if self.lcdc_bg_tilemap_base {
                0x9c00
            } else {
                0x9800
            }
        };

        let y_pos = if use_window {
            scanline.wrapping_sub(window_y)
        } else {
            scroll_y.wrapping_add(scanline)
        };

        let tile_row: u16 = (y_pos / 8) as u16 * 32;

        for pixel in 0..160 {
            let pixel = pixel as u8;
            let x_pos = if use_window && pixel >= window_x {
                pixel.wrapping_sub(window_x)
            } else {
                pixel.wrapping_add(scroll_x)
            };

            let tile_col = (x_pos / 8) as u16;

            let tile_address = background_mem + tile_row + tile_col;

            let tile_num: i16 = if unsigned {
                self.get(tile_address) as u16 as i16
            } else {
                self.get(tile_address) as i8 as i16
            };

            let tile_location: u16 = if unsigned {
                tile_data + (tile_num as u16 * 16)
            } else {
                tile_data + ((tile_num + 128) * 16) as u16
            };

            let line = (y_pos as u16 % 8) * 2;
            let data1 = self.get(tile_location + line);
            let data2 = self.get(tile_location + line + 1);

            let color_bit = ((x_pos as i32 % 8) - 7) * -1;

            let color_num = ((data2 >> color_bit) & 0b1) << 1;
            let color_num = color_num | ((data1 >> color_bit) & 0b1);

            let color = self.get_color(color_num, self.bgp);

            self.framebuffer[scanline as usize][pixel as usize] = [color.r, color.g, color.b];
        }
    }

    fn render_sprites(&mut self) {
        let use_8x16 = self.lcdc_obj_sprite_size;
        for sprite in self.sprites.iter() {
            let y_pos = sprite.y_pos as u8;
            let x_pos = sprite.x_pos as u8;
            let tile_location = sprite.tile as u16;
            let y_flip = sprite.y_flip;
            let x_flip = sprite.x_flip;
            let scanline = self.ly;

            let y_size = if use_8x16 { 16 } else { 8 };

            if scanline >= y_pos && scanline < (y_pos.wrapping_add(y_size)) {
                let line: i32 = scanline as i32 - y_pos as i32;

                let line = if y_flip {
                    (line - y_size as i32) * -1
                } else {
                    line
                };

                let line = line * 2;

                let data_address = 0x8000 + (tile_location * 16) + line as u16;

                let data1 = self.get(data_address);
                let data2 = self.get(data_address + 1);

                for tile_pixel in (0..8).rev() {
                    let color_bit = tile_pixel as i32;
                    let color_bit = if x_flip {
                        (color_bit - 7) * -1
                    } else {
                        color_bit
                    };

                    let color_num = ((data2 >> color_bit) & 0b1) << 1;
                    let color_num = color_num | ((data1 >> color_bit) & 0b1);

                    let palette_num = if sprite.use_palette_1 {
                        self.op1
                    } else {
                        self.op0
                    };

                    if color_num == 0 {
                        continue;
                    }
                    let color = self.get_color(color_num, palette_num);

                    let x_pix = (0 as u8).wrapping_sub(tile_pixel as u8);
                    let x_pix = x_pix.wrapping_add(7);

                    let pixel = x_pos.wrapping_add(x_pix);

                    if scanline > 143 || pixel > 159 {
                        continue;
                    }

                    if self.framebuffer[scanline as usize][pixel as usize]
                        != [WHITE.r, WHITE.g, WHITE.b]
                    {
                        if sprite.priority_behind_bg {
                            continue;
                        }
                    }

                    self.framebuffer[scanline as usize][pixel as usize] =
                        [color.r, color.g, color.b];
                }
            }
        }
    }

    fn get_color(&self, color_id: u8, palette_num: u8) -> Color {
        let (hi, lo) = match color_id {
            0 => (1, 0),
            1 => (3, 2),
            2 => (5, 4),
            3 => (7, 6),
            _ => panic!("Invalid color id: 0x{:x}", color_id),
        };

        let color = ((palette_num >> hi) & 0b1) << 1;
        let color = color | ((palette_num >> lo) & 0b1);

        match color {
            0 => WHITE,
            1 => LIGHT_GRAY,
            2 => DARK_GRAY,
            3 => BLACK,
            _ => panic!("Invalid color: 0x{:x}", color),
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
                }) | (if self.lcdc_bg_and_window_tile_base {
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
                }) | (if self.ly == self.ly_coincidence {
                    0x04
                } else {
                    0
                }) | self.mode as u8;

                ff41_val
            }
            0xff42 => self.scroll_y,
            0xff43 => self.scroll_x,
            0xff44 => self.ly,
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
                self.lcdc_bg_and_window_tile_base = value & 0b0001_0000 != 0;
                self.lcdc_bg_tilemap_base = value & 0b0000_1000 != 0;
                self.lcdc_obj_sprite_size = value & 0b0000_0100 != 0;
                self.lcdc_obj_sprite_display_enabled = value & 0b0000_0010 != 0;
                self.lcdc_bg_enabled = value & 0b0000_0001 != 0;

                if previous_lcdc_display_enabled && !self.lcdc_display_enabled {
                    self.mode_clock = 0;
                    self.ly = 0;
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
            }
            0xff48 => {
                self.op0 = value;
            }
            0xff49 => {
                self.op1 = value;
            }
            0xff4a => {
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
