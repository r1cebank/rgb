use crate::ppu::{PPUFramebuffer, SCREEN_H, SCREEN_W};
use flume::{Receiver, TryRecvError};
use std::thread::{Builder, JoinHandle};

pub mod fps;

/// Start the display thread, will be in charge of displaying graphics to screen
pub fn start_display_thread(
    scale_factor: i32,
    rom_name: String,
    framebuffer_receiver: Receiver<PPUFramebuffer>,
) -> JoinHandle<()> {
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

            // Create a new window with the given title
            let mut window = minifb::Window::new(
                format!("rgb [{}] - {} FPS", rom_name, -1).as_str(),
                SCREEN_W,
                SCREEN_H,
                option,
            )
            .unwrap();

            // The window buffer for rendering
            let mut window_buffer = vec![0x00; SCREEN_W * SCREEN_H];
            let mut fps_counter = fps::FPSCounter::new();
            window
                .update_with_buffer(window_buffer.as_slice(), SCREEN_W, SCREEN_H)
                .unwrap();
            'display: loop {
                if !window.is_open() {
                    break;
                }
                // Update framebuffer when the reciver receive new framebuffer
                match framebuffer_receiver.try_recv() {
                    Ok(framebuffer) => {
                        window.set_title(
                            format!("rgb [{}] - {} FPS", rom_name, fps_counter.get_fps()).as_str(),
                        );
                        let mut i: usize = 0;
                        for l in framebuffer.iter() {
                            for w in l.iter() {
                                let b = u32::from(w[0]) << 16;
                                let g = u32::from(w[1]) << 8;
                                let r = u32::from(w[2]);
                                let a = 0xff00_0000;
                                window_buffer[i] = a | b | g | r;
                                i += 1;
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => (),
                    Err(TryRecvError::Disconnected) => break 'display,
                }

                // Refresh window with the new buffers
                window
                    .update_with_buffer(window_buffer.as_slice(), SCREEN_W, SCREEN_H)
                    .unwrap();
            }
        })
        .unwrap()
}
