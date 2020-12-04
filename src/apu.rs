use std::thread;
use std::thread::{Builder, JoinHandle};

pub fn start_apu_thread() -> JoinHandle<()> {
    Builder::new()
        .name("apu".to_string())
        .spawn(move || {
            debug!("Audio thread spawned");
            let device = cpal::default_output_device().unwrap();
            let format = device.default_output_format().unwrap();
            let format = cpal::Format {
                channels: 2,
                sample_rate: format.sample_rate,
                data_type: cpal::SampleFormat::F32,
            };
            debug!("Open the audio player: {}", device.name());
            thread::yield_now();
        })
        .unwrap()
}
