use crate::memory::Memory;

pub const FB_W: usize = 160;
pub const FB_H: usize = 144;

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
    pub framebuffer: Vec<u8>,
    pub video_ram: [u8; 0x4000],
    pub interrupt_flags: u8,
    pub oam: [u8; 0xa0],

    // This register assigns gray shades for sprite palette 0. It works exactly as BGP (FF47), except that the lower
    // two bits aren't used because sprite data 00 is transparent.
    op0: u8,
    // This register assigns gray shades for sprite palette 1. It works exactly as BGP (FF47), except that the lower
    // two bits aren't used because sprite data 00 is transparent.
    op1: u8,

    // Window Y Position (R/W), Window X Position minus 7 (R/W)
    wy: u8,
    wx: u8,

    tileset: [[[u8; 8]; 8]; 384],
    sprites: [Sprite; 40],
    palette: [[u8; 4]; 4],
    mode: u8,
    mode_clock: i32,
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
    pub fn new() -> PPU {
        PPU {
            framebuffer: vec![10; FB_W * FB_H * 4],
            video_ram: [0x00; 0x4000],
            oam: [0x00; 0xa0],
            tileset: [[[0x00; 8]; 8]; 384],
            palette: [
                [255, 255, 255, 255],
                [192, 192, 192, 255],
                [96, 96, 96, 255],
                [0, 0, 0, 255],
            ],

            sprites: [Sprite::new(); 40],

            mode: 0,
            mode_clock: 0,
            line: 0,
            scroll_x: 0,
            scroll_y: 0,

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
            interrupt_flags: 0x00,

            tick_counter: 0,
        }
    }

    pub fn update_tile(&mut self, address: u16, value: u8) {
        // Get the "base address" for this tile row
        let base_address = address & 0x1FFE;
        if value != 0x00 {
            // Nothing but zeros being written, in infinite loop.
            trace!("Writing data to VRAM!: {:#X}", value);
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

            self.tileset[tile as usize][y as usize][x as usize] = pixel_colour;
        }
    }

    pub fn tick(&mut self, cycles: u32) {}
}

impl Memory for PPU {
    fn get(&self, address: u16) -> u8 {
        match address {
            0xFF40 => {
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
            0xFF41 => {
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
                }) | self.mode;

                ff41_val
            }
            0xFF42 => self.scroll_y as u8,
            0xFF43 => self.scroll_x,
            0xFF44 => self.line,
            0xFF45 => self.ly_coincidence,
            _ => {
                panic!("Unexpected address in PPU#read: {:#X}", address);
            }
        }
    }
    fn set(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => {
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
                    self.mode = 0;
                }
            }
            0xFF41 => {
                self.ly_coincidence_interrupt_enabled = value & 0x40 == 0x40;
                self.mode_2_interrupt_enabled = value & 0x20 == 0x20;
                self.mode_1_interrupt_enabled = value & 0x10 == 0x10;
                self.mode_0_interrupt_enabled = value & 0x08 == 0x08;
            }
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => {
                panic!("Writing to LY in PPU#write: {:#X}", address);
            }
            0xFF45 => self.ly_coincidence = value,
            0xFF47 => {
                for i in 0..4 {
                    match (value >> (i * 2)) & 3 {
                        0 => self.palette[i] = [255, 255, 255, 255],
                        1 => self.palette[i] = [192, 192, 192, 255],
                        2 => self.palette[i] = [96, 96, 96, 255],
                        3 => self.palette[i] = [0, 0, 0, 255],
                        _ => {
                            panic!("Unexpected background palette value: {:#X}", i);
                        }
                    }
                }
            }
            0xFF48 => {
                self.op0 = value;
            }
            0xFF49 => {
                self.op1 = value;
            }
            0xFF4A => {
                self.wy = value;
            }
            0xFF4B => self.wx = value,
            _ => {
                panic!("Writing to PPU {:#x}", address);
            }
        }
    }
}
