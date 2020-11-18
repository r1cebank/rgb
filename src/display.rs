use crate::debug;
use crate::debug::message::DebugMessage;
use crate::ppu::{PPUFramebuffer, FB_H, FB_W};
use debug::debug_state::DebugState;
use flume::{Receiver, TryRecvError};
use piston_window::*;
use std::thread::{Builder, JoinHandle};

mod debug_canvas;
mod draw_logs;
pub mod fps;
mod game_canvas;

/// The actual window size factored in scaling and debug windows
pub fn get_actual_window_size(scale: u32) -> (u32, u32) {
    return (FB_W as u32 * scale * 2, FB_H as u32 * scale * 2);
}

/// Start the display thread, will be in charge of displaying graphics to screen
pub fn start_display_thread(
    scale_factor: u32,
    rom_name: String,
    framebuffer_receiver: Receiver<PPUFramebuffer>,
    debug_result_receiver: Receiver<DebugMessage>,
    log_message_receiver: Receiver<DebugMessage>,
) -> JoinHandle<()> {
    Builder::new()
        .name("display".to_string())
        .spawn(move || {
            debug!("Display thread spawned");
            // Grab the actual screen size to draw our window in
            let (screen_width, screen_height) = get_actual_window_size(scale_factor);

            // Set the debug state, this is persisted
            let mut debug_state = DebugState::new();

            // Create the piston window, this will be the window to to draw everything in our emulator
            let mut window: PistonWindow = WindowSettings::new(
                format!("rgb [{}] - {} FPS", rom_name, -1).as_str(),
                (screen_width, screen_height),
            )
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build window: {}", e));

            // The canvas to draw our emulator framebuffer
            let mut game_image = im::ImageBuffer::new(FB_W as u32, FB_H as u32);

            // Create a texture context for our texture
            let mut game_texture_context = TextureContext {
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into(),
            };

            // Create a texture from the image that stores our framebuffer
            let mut game_texture: G2dTexture = Texture::from_image(
                &mut game_texture_context,
                &game_image,
                &TextureSettings::new(),
            )
            .unwrap();

            // Our super inaccurate FPS counter
            let mut fps_counter = fps::FPSCounter::new();

            // Create a texture context for our texture
            let mut game_texture_context = TextureContext {
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into(),
            };

            // Create a texture from the image that stores our framebuffer
            let mut game_texture: G2dTexture = Texture::from_image(
                &mut game_texture_context,
                &game_image,
                &TextureSettings::new(),
            )
            .unwrap();

            // Our display loop
            'display: while let Some(e) = window.next() {
                if let Some(_) = e.render_args() {
                    window.draw_2d(&e, |_, g, _| {
                        clear([0.0, 0.0, 0.0, 1.0], g);
                    });
                    // Update the game texture with the game image
                    game_texture
                        .update(&mut game_texture_context, &game_image)
                        .unwrap();

                    // Draw the debug info, if this function returns false, break the loop
                    if !debug_canvas::draw_debug_info(
                        &e,
                        &mut window,
                        debug_result_receiver.clone(),
                        log_message_receiver.clone(),
                        &mut debug_state,
                    ) {
                        break 'display;
                    }

                    // Draw on the window.
                    window.draw_2d(&e, |c, g, device| {
                        // Update texture before rendering.
                        game_texture_context.encoder.flush(device);
                        image(
                            &game_texture,
                            c.transform.scale(scale_factor as f64, scale_factor as f64),
                            g,
                        );
                    });
                }
                // Update framebuffer when the receiver receive new framebuffer
                match framebuffer_receiver.try_recv() {
                    Ok(framebuffer) => {
                        window.set_title(format!(
                            "rgb [{}] - {} FPS",
                            rom_name,
                            fps_counter.get_fps()
                        ));
                        game_canvas::update_game_canvas(framebuffer, &mut game_image);
                    }
                    Err(TryRecvError::Empty) => (),
                    Err(TryRecvError::Disconnected) => break 'display,
                }
            }
        })
        .unwrap()
}
