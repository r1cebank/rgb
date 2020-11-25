pub mod command;
pub mod debug_logger;
pub mod debug_state;
pub mod debuggable;
pub mod message;

use crate::debug::message::DebugMessage;
use crate::emulator::Emulator;
#[cfg(feature = "debug")]
use cursive::event::Key;
#[cfg(feature = "debug")]
use cursive::menu::MenuTree;
#[cfg(feature = "debug")]
use cursive::traits::*;
#[cfg(feature = "debug")]
use cursive::views::{Dialog, DummyView, LinearLayout, TextView};
#[cfg(feature = "debug")]
use cursive_hexview::{DisplayState, HexView};
use flume::{Receiver, TryRecvError};
use std::thread::{Builder, JoinHandle};

pub const FB_W: usize = 160;
pub const FB_H: usize = 144;

#[cfg(feature = "debug")]
pub fn start_debug_thread(debug_result_receiver: Receiver<DebugMessage>) -> JoinHandle<()> {
    Builder::new()
        .name("debugger".to_string())
        .spawn(move || {
            debug!("Debug thread spawned");
            let mut siv = cursive::default();

            siv.menubar()
                .add_subtree(
                    "emulator",
                    MenuTree::new()
                        .leaf("control", move |s| {})
                        .subtree("view memory", MenuTree::new().leaf("boot", move |s| {
                            match debug_result_receiver.try_recv() {
                                Ok(message) => match message {
                                    DebugMessage::MemoryUpdate(memory) => {
                                        let explanation = TextView::new(
                                            "Use the keys ↑ ↓ ← → to navigate around.\nUse q to exit.",
                                        );
                                        let view = HexView::new_from_iter(memory)
                                            .display_state(DisplayState::Enabled);
                                        s.add_layer(
                                            Dialog::around(
                                                LinearLayout::vertical()
                                                    .child(explanation)
                                                    .child(DummyView)
                                                    .child(view),
                                            )
                                                .title("Memory Viewer"),
                                        );
                                    }
                                    _ => {}
                                },
                                Err(TryRecvError::Empty) => (),
                                Err(TryRecvError::Disconnected) => {
                                    s.quit();
                                }
                            }
                        })),
                )
                .add_subtree(
                    "help",
                    MenuTree::new()
                        .subtree(
                            "help",
                            MenuTree::new()
                                .leaf("general", |s| s.add_layer(Dialog::info("Help message!"))),
                        )
                        .leaf("about", |s| {
                            s.add_layer(Dialog::info(format!("rgb {}", env!("CARGO_PKG_VERSION"))))
                        }),
                )
                .add_delimiter()
                .add_leaf("quit", |s| s.quit());

            siv.add_global_callback(Key::Esc, |s| s.select_menubar());

            siv.add_layer(TextView::new(
                "Welcome to rgb! Press <esc> to show debug menu.",
            ));

            // Starts the event loop.
            siv.run();
            debug!("Debug thread exited");
            std::process::exit(0x00);
        })
        .unwrap()
}
