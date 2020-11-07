use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

use super::registers::Registers;

pub struct Core {
    pub registers: Registers,
}

impl Core {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Core {
        Self {
            registers: Registers::new(),
        }
    }
    pub fn tick(&mut self) -> u32 {
        0
    }
}
