use std::thread::{Builder, JoinHandle};

pub fn start_io_thread() -> JoinHandle<()> {
    Builder::new()
        .name("io".to_string())
        .spawn(move || {
            debug!("IO thread spawned");
        })
        .unwrap()
}
