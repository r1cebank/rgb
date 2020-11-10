use crate::debug::message::DebugMessage;
use crate::ppu::FB_H;
use flume::{Receiver, TryRecvError};
use piston_window::*;

pub fn draw_debug_info(
    e: &Event,
    window: &mut PistonWindow,
    debug_result_receiver: Receiver<DebugMessage>,
) -> bool {
    // The asset folder
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("res")
        .unwrap();
    // The font for drawing our text
    let mut font = window
        .load_font(assets.join("mplus-1p-regular.ttf"))
        .unwrap();

    let mut draw_result = true;

    // Draw the debug info
    window.draw_2d(e, |c, g, device| {
        match debug_result_receiver.try_recv() {
            Ok(message) => {
                println!("{:?}", message);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                draw_result = false;
            }
        }
        text::Text::new_color([1.0; 4], 20)
            .draw(
                format!("A: {:04x}, B: 0x00, C: 0x00, D: 0x00, E: 0x00, H: 0x00", 1).as_str(),
                &mut font,
                &c.draw_state,
                c.transform.trans(10.0, ((FB_H * 2) + 20) as f64),
                g,
            )
            .unwrap();
        font.factory.encoder.flush(device);
    });

    println!("{}", draw_result);

    draw_result
}
