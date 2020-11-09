use crate::cartridge::load_cartridge;
use crate::cpu::ClockedCPU;
use crate::memory::mmu::MMU;
use crate::ppu::{random_framebuffer, PPUFramebuffer, SCREEN_H, SCREEN_W};
use flume::{Sender, TrySendError};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::{Builder, JoinHandle};

pub struct Emulator {
    pub mmu: Rc<RefCell<MMU>>,
    pub cpu: ClockedCPU,
}

impl Emulator {
    pub fn new(boot_rom: Option<Vec<u8>>, rom: Vec<u8>) -> Emulator {
        let has_bootrom = match boot_rom {
            None => false,
            _ => { true }
        };
        let mmu = Rc::new(RefCell::new(MMU::new(boot_rom, rom)));
        let mut cpu = ClockedCPU::new(mmu.clone());

        // If no boot rom is set, we simulate the boot rom states on the mmu and cpu
        if !has_bootrom {
            mmu.borrow_mut().simulate_boot_rom();
            cpu.simulate_boot_rom();
        }

        Self { cpu, mmu }
    }
    pub fn tick(&mut self) -> u32 {
        let cycles = self.cpu.tick();
        self.mmu.borrow_mut().tick(cycles);
        cycles
    }
}

pub fn start_emulator_thread(
    boot_rom: Option<Vec<u8>>,
    rom: Vec<u8>,
    framebuffer_sender: Sender<PPUFramebuffer>,
) -> JoinHandle<()> {
    Builder::new()
        .name("emulator".to_string())
        .spawn(move || {
            debug!("thread spawned");
            let mut emulator = Emulator::new(boot_rom, rom);
            'emulator: loop {
                std::thread::sleep(std::time::Duration::from_millis(130));
                // emulator.tick();
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
