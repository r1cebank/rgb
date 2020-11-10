use crate::ppu::{PPUFramebuffer, FB_H, FB_W};
use piston_window::*;

pub fn update_game_canvas(
    framebuffer: PPUFramebuffer,
    image_buffer: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
) {
    // Update each pixel from our raw framebuffer to the canvas image
    for y in 0..framebuffer.len() {
        for x in 0..framebuffer[y].len() {
            image_buffer.put_pixel(
                x as u32,
                y as u32,
                im::Rgba([
                    framebuffer[y][x][0],
                    framebuffer[y][x][1],
                    framebuffer[y][x][2],
                    255,
                ]),
            );
        }
    }
}
