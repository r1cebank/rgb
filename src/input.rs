pub mod input_message;
pub mod joypad;

use crate::input::input_message::InputMessage;
use flume::Sender;
use std::thread::{Builder, JoinHandle};

pub fn start_io_thread(_input_message_sender: Sender<InputMessage>) -> JoinHandle<()> {
    Builder::new()
        .name("input".to_string())
        .spawn(move || {
            debug!("IO thread spawned");
            // This place is reserved, if we need a separate producer for input commands
            // ex. external server
        })
        .unwrap()
}
