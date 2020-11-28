use crate::cpu::interrupt::{Flag, InterruptFlags};
use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum JoyPadKey {
    Right = 0b0000_0001,
    Left = 0b0000_0010,
    Up = 0b0000_0100,
    Down = 0b0000_1000,
    A = 0b0001_0000,
    B = 0b0010_0000,
    Select = 0b0100_0000,
    Start = 0b1000_0000,
    Invalid = 0b1111_1111,
}

pub struct JoyPad {
    pub interrupt_flags: Rc<RefCell<InterruptFlags>>,
    matrix: u8,
    select: u8,
}

impl JoyPad {
    pub fn new(interrupt_flags: Rc<RefCell<InterruptFlags>>) -> Self {
        Self {
            interrupt_flags,
            matrix: 0xff,
            select: 0x00,
        }
    }
}

impl JoyPad {
    pub fn key_down(&mut self, key: JoyPadKey) {
        if key == JoyPadKey::Invalid {
            return;
        }
        self.matrix &= !(key as u8);
        self.interrupt_flags.borrow_mut().hi(Flag::Joypad);
    }

    pub fn key_up(&mut self, key: JoyPadKey) {
        if key == JoyPadKey::Invalid {
            return;
        }
        self.matrix |= key as u8;
    }
}

impl Memory for JoyPad {
    fn get(&self, a: u16) -> u8 {
        assert_eq!(a, 0xff00);
        if (self.select & 0b0001_0000) == 0x00 {
            return self.select | (self.matrix & 0x0f);
        }
        if (self.select & 0b0010_0000) == 0x00 {
            return self.select | (self.matrix >> 4);
        }
        self.select
    }

    fn set(&mut self, a: u16, v: u8) {
        assert_eq!(a, 0xff00);
        self.select = v;
    }
}
