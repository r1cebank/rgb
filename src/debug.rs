use std::thread::{Builder, JoinHandle};

pub fn start_debug_thread() -> JoinHandle<()> {
    Builder::new()
        .name("debugger".to_string())
        .spawn(move || {
            debug!("thread spawned");
        })
        .unwrap()
}
