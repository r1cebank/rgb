use crate::cpu::registers::Flag;
use crate::debug::debug_state::DebugState;
use crate::debug::message::DebugMessage;
use crate::ppu::{Tile, FB_H};
use flume::{Receiver, TryRecvError};
use piston_window::*;

const DEBUG_FONT_SIZE: usize = 13;
const PADDING: usize = DEBUG_FONT_SIZE / 2;

pub fn draw_debug_info(
    e: &Event,
    window: &mut PistonWindow,
    debug_message_receiver: Receiver<DebugMessage>,
    log_message_receiver: Receiver<DebugMessage>,
    tile_update_receiver: Receiver<DebugMessage>,
    tile_image: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
    tile_texture: &mut G2dTexture,
    debug_state: &mut DebugState,
) -> bool {
    // The asset folder
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("res")
        .unwrap();
    // The font for drawing our text
    let mut font = window
        .load_font(assets.join("FiraCode-Regular.ttf"))
        .unwrap();

    let mut draw_result = true;

    match debug_message_receiver.try_recv() {
        Ok(message) => match message {
            DebugMessage::RegisterUpdate(registers) => {
                debug_state.registers = registers;
            }
            DebugMessage::TileUpdate(tiles) => {
                debug_state.tiles = tiles;
            }
            _ => {}
        },
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => {
            draw_result = false;
        }
    }
    match log_message_receiver.try_recv() {
        Ok(message) => match message {
            DebugMessage::LogUpdate(log_message) => {
                debug_state.append_log(log_message);
            }
            _ => {}
        },
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => {
            draw_result = false;
        }
    }
    // Update framebuffer for tiles
    match tile_update_receiver.try_recv() {
        Ok(message) => match message {
            DebugMessage::TileUpdate(tiles) => {
                debug_state.tiles = tiles;
            }
            _ => {}
        },
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => {
            draw_result = false;
        }
    }

    let framebuffer = tile_to_framebuffer(debug_state.tiles.clone());

    // Update each pixel from our raw framebuffer to the canvas image
    for y in 0..framebuffer.len() {
        for x in 0..framebuffer[y].len() {
            tile_image.put_pixel(
                x as u32,
                y as u32,
                im::Rgba([
                    framebuffer[y][x][0],
                    framebuffer[y][x][1],
                    framebuffer[y][x][2],
                    framebuffer[y][x][3],
                ]),
            );
        }
    }

    // Draw the debug info
    window.draw_2d(e, |c, g, device| {
        text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
            .draw(
                "Tileset",
                &mut font,
                &c.draw_state,
                c.transform.trans(640 as f64, 40 as f64),
                g,
            )
            .unwrap();
        image(
            tile_texture,
            c.transform
                .scale(2 as f64, 2 as f64)
                .trans(320 as f64, 40 as f64),
            g,
        );
        text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
            .draw(
                format!("{}", debug_state.registers.get_register_overview()).as_str(),
                &mut font,
                &c.draw_state,
                c.transform
                    .trans(10.0, ((FB_H * 2) + (DEBUG_FONT_SIZE + PADDING)) as f64),
                g,
            )
            .unwrap();
        text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
            .draw(
                format!("{}", debug_state.registers.get_word_register_overview()).as_str(),
                &mut font,
                &c.draw_state,
                c.transform
                    .trans(10.0, ((FB_H * 2) + 2 * (DEBUG_FONT_SIZE + PADDING)) as f64),
                g,
            )
            .unwrap();
        text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
            .draw(
                format!("{}", debug_state.registers.get_flag_register_overview()).as_str(),
                &mut font,
                &c.draw_state,
                c.transform
                    .trans(10.0, ((FB_H * 2) + (DEBUG_FONT_SIZE + PADDING) * 3) as f64),
                g,
            )
            .unwrap();
        for (i, log_message) in debug_state.log_messages.iter().enumerate() {
            text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
                .draw(
                    log_message.as_str(),
                    &mut font,
                    &c.draw_state,
                    c.transform.trans(
                        10.0,
                        ((FB_H * 2)
                            + (DEBUG_FONT_SIZE + PADDING) * 9
                            + i * (DEBUG_FONT_SIZE + PADDING)) as f64,
                    ),
                    g,
                )
                .unwrap();
        }
        font.factory.encoder.flush(device);
    });

    draw_result
}

pub fn update_tile_canvas(
    tile_set: Vec<Tile>,
    image_buffer: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
) {
    let framebuffer = tile_to_framebuffer(tile_set);
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
                    framebuffer[y][x][3],
                ]),
            );
        }
    }
}

pub fn tile_to_framebuffer(tile_set: Vec<Tile>) -> Vec<[[u8; 4]; 128]> {
    const NUM_TILES_ROW: usize = 24;
    const NUM_TILES_COL: usize = 16;
    const TILE_WIDTH: usize = 8;
    const IMAGE_WIDTH: usize = TILE_WIDTH * NUM_TILES_COL;
    const IMAGE_HEIGHT: usize = TILE_WIDTH * NUM_TILES_ROW;
    const PIXEL_COLOUR_STRIDE: usize = 4;
    const PALETTE: [[u8; 4]; 4] = [
        [254, 248, 208, 255],
        [136, 192, 112, 255],
        [39, 80, 70, 255],
        [8, 24, 32, 255],
    ];

    let mut framebuffer = vec![[[0x00 as u8; 4]; IMAGE_WIDTH]; IMAGE_HEIGHT];

    for tile_y in 0..NUM_TILES_ROW {
        for tile_x in 0..NUM_TILES_COL {
            let target_tile = tile_x + (tile_y * NUM_TILES_COL);

            if target_tile >= 384 {
                return framebuffer;
            }

            for y in 0..TILE_WIDTH {
                for x in 0..TILE_WIDTH {
                    let pixel_palette =
                        PALETTE[tile_set[target_tile as usize][y as usize][x as usize] as usize];

                    let point_x = x + (tile_x * TILE_WIDTH);
                    let point_y = y + (tile_y * TILE_WIDTH);

                    framebuffer[point_y][point_x][0] = pixel_palette[0];
                    framebuffer[point_y][point_x][1] = pixel_palette[1];
                    framebuffer[point_y][point_x][2] = pixel_palette[2];
                    framebuffer[point_y][point_x][3] = pixel_palette[3];
                }
            }
        }
    }

    framebuffer
}
