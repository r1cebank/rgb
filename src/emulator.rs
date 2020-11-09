use crate::cartridge::load_cartridge;
use crate::cpu::ClockedCPU;
use crate::ppu::{random_framebuffer, PPUFramebuffer, SCREEN_H, SCREEN_W};
use flume::{Sender, TrySendError};
use std::thread::{Builder, JoinHandle};

pub fn start_emulator_thread(
    bootrom: Vec<u8>,
    rom: Vec<u8>,
    framebuffer_sender: Sender<PPUFramebuffer>,
) -> JoinHandle<()> {
    Builder::new()
        .name("emulator".to_string())
        .spawn(move || {
            debug!("thread spawned");
            let cartridge = load_cartridge(rom);
            'emulator: loop {
                std::thread::sleep(std::time::Duration::from_millis(130));
                let mut gpu_framebuffer = random_framebuffer();
                match framebuffer_sender.try_send(gpu_framebuffer) {
                    Ok(_) => {}
                    Err(TrySendError::Full(_)) => {}
                    Err(TrySendError::Disconnected(_)) => break 'emulator,
                }
            }
        })
        .unwrap()
}
