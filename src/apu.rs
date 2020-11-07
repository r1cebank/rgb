use std::thread::{Builder, JoinHandle};

pub fn start_apu_thread() -> JoinHandle<()> {
    Builder::new()
        .name("emulator".to_string())
        .spawn(move || {
            debug!("thread spawned");
        })
        .unwrap()
}
