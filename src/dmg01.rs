use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::memory::Memory;
use crate::memory::MMU;

pub struct dmg01 {
    pub mmu: Rc<RefCell<MMU>>,
    pub cpu: CPU,
}

impl dmg01 {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>, rom_buffer: Option<Vec<u8>>) -> Self {
        let mmu = Rc::new(RefCell::new(MMU::new(boot_rom_buffer, rom_buffer)));
        let cpu = CPU::new(mmu.clone());
        Self { mmu, cpu }
    }
    pub fn tick(&mut self) -> u32 {
        if self.mmu.borrow().get(self.cpu.registers.pc) == 0x10 {
            self.mmu.borrow_mut().switch_speed();
        }
        let cycles = self.cpu.next();
        self.mmu.borrow_mut().next(cycles);
        cycles
    }
}
