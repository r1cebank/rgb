use super::input::input_message::InputMessage;
use crate::cartridge::load_cartridge;
use crate::cpu::instruction::InstructionSet;
use crate::cpu::ClockedCPU;
use crate::debug::message::DebugMessage;
use crate::memory::mmu::MMU;
use crate::ppu::{random_framebuffer, Mode, PPUFramebuffer};
use flume::{Receiver, Sender, TryRecvError, TrySendError};
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
            _ => true,
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
        // Execute one cpu cycle
        let cycles = self.cpu.tick();
        // Update the mmu with the cycles
        self.mmu.borrow_mut().tick(cycles);
        cycles
    }

    pub fn should_refresh_screen(&self) -> bool {
        self.mmu.borrow().ppu.borrow().mode == Mode::VBlank
    }
}

pub fn start_emulator_thread(
    boot_rom: Option<Vec<u8>>,
    rom: Vec<u8>,
    input_message_receiver: Receiver<InputMessage>,
    framebuffer_sender: Sender<PPUFramebuffer>,
    debug_result_sender: Sender<DebugMessage>,
    tile_update_sender: Sender<DebugMessage>,
) -> JoinHandle<()> {
    Builder::new()
        .name("emulator".to_string())
        .spawn(move || {
            debug!("Emulator Thread spawned");
            let mut emulator = Emulator::new(boot_rom, rom);
            'emulator: loop {
                // std::thread::sleep(std::time::Duration::from_millis(10));
                emulator.tick();
                match input_message_receiver.try_recv() {
                    Ok(inputMessage) => match inputMessage {
                        InputMessage::KeyDown(key) => {
                            emulator.mmu.borrow_mut().joypad.key_down(key)
                        }
                        InputMessage::KeyUp(key) => emulator.mmu.borrow_mut().joypad.key_up(key),
                    },
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => break 'emulator,
                }
                if emulator.should_refresh_screen() {
                    match framebuffer_sender
                        .try_send(emulator.mmu.borrow().ppu.borrow().framebuffer)
                    {
                        Ok(_) => {}
                        Err(TrySendError::Full(_)) => {}
                        Err(TrySendError::Disconnected(_)) => break 'emulator,
                    }
                    match debug_result_sender
                        .try_send(DebugMessage::RegisterUpdate(emulator.cpu.core.registers))
                    {
                        Ok(_) => {}
                        Err(TrySendError::Full(_)) => {}
                        Err(TrySendError::Disconnected(_)) => break 'emulator,
                    }
                    match tile_update_sender.try_send(DebugMessage::TileUpdate(Vec::from(
                        emulator.mmu.borrow().ppu.borrow().tile_set,
                    ))) {
                        Ok(_) => {}
                        Err(TrySendError::Full(_)) => {}
                        Err(TrySendError::Disconnected(_)) => break 'emulator,
                    }
                    match debug_result_sender.try_send(DebugMessage::MemoryUpdate(
                        emulator
                            .mmu
                            .borrow()
                            .boot_rom
                            .unwrap_or([0 as u8; 256])
                            .to_vec(),
                    )) {
                        Ok(_) => {}
                        Err(TrySendError::Full(_)) => {}
                        Err(TrySendError::Disconnected(_)) => break 'emulator,
                    }
                }
            }
            debug!("Emulator loop exited");
            std::process::exit(0x00);
        })
        .unwrap()
}
