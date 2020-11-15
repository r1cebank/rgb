use crate::cartridge::load_cartridge;
use crate::cpu::instruction::InstructionSet;
use crate::cpu::ClockedCPU;
use crate::debug::message::DebugMessage;
use crate::memory::mmu::MMU;
use crate::ppu::{random_framebuffer, PPUFramebuffer};
use flume::{Sender, TrySendError};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::{Builder, JoinHandle};

pub struct Emulator {
    pub mmu: Rc<RefCell<MMU>>,
    instruction_set: InstructionSet,
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

        Self {
            cpu,
            mmu,
            instruction_set: InstructionSet::new(),
        }
    }
    fn execute_next_instruction(&mut self) -> u8 {
        let executable_instruction = self
            .instruction_set
            .get_next_executable_instruction(&mut self.cpu.cpu)
            .unwrap();

        let (instruction, operand) = executable_instruction;

        match instruction.operand_length {
            0 => {
                trace!("{}", instruction.name);
            }
            1 => {
                trace!("{}, ${:02x}", instruction.name, operand.unwrap().byte);
            }
            2 => {
                trace!("{}, ${:04x}", instruction.name, operand.unwrap().word);
            }
            _ => {}
        }

        (instruction.exec)(&mut self.cpu.cpu, operand);

        instruction.cycles
    }
    pub fn tick(&mut self) -> u32 {
        let cycles = self.execute_next_instruction() as u32;
        self.mmu.borrow_mut().tick(cycles);
        cycles
    }
}

pub fn start_emulator_thread(
    boot_rom: Option<Vec<u8>>,
    rom: Vec<u8>,
    framebuffer_sender: Sender<PPUFramebuffer>,
    debug_result_sender: Sender<DebugMessage>,
) -> JoinHandle<()> {
    Builder::new()
        .name("emulator".to_string())
        .spawn(move || {
            debug!("thread spawned");
            let mut emulator = Emulator::new(boot_rom, rom);
            'emulator: loop {
                std::thread::sleep(std::time::Duration::from_millis(10));
                emulator.tick();
                let mut gpu_framebuffer = random_framebuffer();
                match framebuffer_sender.try_send(gpu_framebuffer) {
                    Ok(_) => {}
                    Err(TrySendError::Full(_)) => {}
                    Err(TrySendError::Disconnected(_)) => break 'emulator,
                }
                match debug_result_sender
                    .try_send(DebugMessage::RegisterUpdate(emulator.cpu.cpu.registers))
                {
                    Ok(_) => {}
                    Err(TrySendError::Full(_)) => {}
                    Err(TrySendError::Disconnected(_)) => break 'emulator,
                }
            }
        })
        .unwrap()
}
