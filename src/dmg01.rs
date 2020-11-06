use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::memory::MMU;

pub struct Dmg01 {
    pub mmu: Rc<RefCell<MMU>>,
    paused: bool,
    pub cpu: CPU,
}

impl Dmg01 {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>, rom_buffer: Option<Vec<u8>>) -> Self {
        let mmu = Rc::new(RefCell::new(MMU::new(boot_rom_buffer, rom_buffer)));
        let cpu = CPU::new(mmu.clone());
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

            // Update the mmu and rest with cycles
            // self.mmu.borrow_mut().timer.tick(cycles);

            // Update interrupt flags
            // self.mmu.borrow_mut().interrupt_flags |= self.mmu.input.interrupt_flags;
            // self.mmu.borrow_mut().input.interrupt_flags = 0x00;

            // Update ppu
            // self.mmu.borrow().ppu.borrow_mut().tick(cycles);
            // self.mmu.borrow_mut().interrupt_flags |=
            //     self.mmu.borrow().ppu.borrow_mut().interrupt_flags;
            // self.mmu.borrow().ppu.borrow_mut().interrupt_flags = 0x00;
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
