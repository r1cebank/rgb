use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::ClockedCPU;
use crate::memory::Memory;
use crate::memory::MMU;

pub struct dmg01 {
    pub mmu: Rc<RefCell<MMU>>,
    paused: bool,
    pub cpu: ClockedCPU,
}

impl dmg01 {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>, rom_buffer: Option<Vec<u8>>) -> Self {
        let mmu = Rc::new(RefCell::new(MMU::new(boot_rom_buffer, rom_buffer)));
        let cpu = ClockedCPU::new(1, 1.0, mmu.clone());
        Self {
            mmu,
            cpu,
            paused: false,
        }
    }
    pub fn tick(&mut self) -> u32 {
        // if self.mmu.borrow().get(self.cpu.registers.pc) == 0x10 {
        //     self.mmu.borrow_mut().switch_speed();
        // }
        let mut cycles = 0;
        if !self.paused {
            cycles = self.cpu.tick();
            self.mmu.borrow_mut().next(cycles);
        }
        cycles
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }
}
