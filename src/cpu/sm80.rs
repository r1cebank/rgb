use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

use super::registers::Registers;

pub struct sm80 {
    pub registers: Registers,
}

impl sm80 {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> sm80 {
        Self {
            registers: Registers::new(),
        }
    }
    pub fn tick(&mut self) -> u32 {
        0
    }
}
