use crate::cpu::registers::Flag;
use crate::debug::debug_state::DebugState;
use crate::debug::message::DebugMessage;
use crate::ppu::FB_H;
use flume::{Receiver, TryRecvError};
use piston_window::*;

const DEBUG_FONT_SIZE: usize = 15;
const PADDING: usize = DEBUG_FONT_SIZE / 2;

pub fn draw_debug_info(
    e: &Event,
    window: &mut PistonWindow,
    debug_message_receiver: Receiver<DebugMessage>,
    log_message_receiver: Receiver<DebugMessage>,
    debug_state: &mut DebugState,
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

    match debug_message_receiver.try_recv() {
        Ok(message) => match message {
            DebugMessage::RegisterUpdate(registers) => {
                debug_state.registers = registers;
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

    // Draw the debug info
    window.draw_2d(e, |c, g, device| {
        text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
            .draw(
                format!(
                    "Z: {}, N: {}, H: {}, C: {}",
                    debug_state.registers.get_flag(Flag::Z),
                    debug_state.registers.get_flag(Flag::N),
                    debug_state.registers.get_flag(Flag::H),
                    debug_state.registers.get_flag(Flag::C),
                )
                .as_str(),
                &mut font,
                &c.draw_state,
                c.transform
                    .trans(10.0, ((FB_H * 2) + 2 * (DEBUG_FONT_SIZE + PADDING)) as f64),
                g,
            )
            .unwrap();
        text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
            .draw(
                format!(
                    "A: {:04x}, B: {:04x}, C: {:04x}, D: {:04x}, E: {:04x}",
                    debug_state.registers.a,
                    debug_state.registers.b,
                    debug_state.registers.c,
                    debug_state.registers.d,
                    debug_state.registers.e,
                )
                .as_str(),
                &mut font,
                &c.draw_state,
                c.transform
                    .trans(10.0, ((FB_H * 2) + (DEBUG_FONT_SIZE + PADDING)) as f64),
                g,
            )
            .unwrap();
        text::Text::new_color([1.0; 4], DEBUG_FONT_SIZE as u32)
            .draw(
                format!(
                    "AF: {:04x}, BC: {:04x}, DE: {:04x}, HL: {:04x}",
                    debug_state.registers.get_af(),
                    debug_state.registers.get_bc(),
                    debug_state.registers.get_de(),
                    debug_state.registers.get_hl(),
                )
                .as_str(),
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
                            + (DEBUG_FONT_SIZE + PADDING) * 7
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
