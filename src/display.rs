use crate::ppu::{SCREEN_H, SCREEN_W};
use std::thread::{Builder, JoinHandle};

pub fn start_display_thread(scale_factor: i32, rom_name: String) -> JoinHandle<()> {
    Builder::new()
        .name("display".to_string())
        .spawn(move || {
            debug!("thread spawned");
            let mut option = minifb::WindowOptions::default();
            option.resize = true;
            option.scale = match scale_factor {
                1 => minifb::Scale::X1,
                2 => minifb::Scale::X2,
                4 => minifb::Scale::X4,
                8 => minifb::Scale::X8,
                _ => panic!("Supported scale: 1, 2, 4 or 8"),
            };
            let mut window = minifb::Window::new(
                format!("Gameboy - {}", rom_name).as_str(),
                SCREEN_W,
                SCREEN_H,
                option,
            )
            .unwrap();
            let mut window_buffer = vec![0x00; SCREEN_W * SCREEN_H];
            window
                .update_with_buffer(window_buffer.as_slice(), SCREEN_W, SCREEN_H)
                .unwrap();
            loop {
                window
                    .update_with_buffer(window_buffer.as_slice(), SCREEN_W, SCREEN_H)
                    .unwrap();
            }
        })
        .unwrap()
}
