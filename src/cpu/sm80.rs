use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

use super::cycles::{CB_CYCLES, OP_CYCLES};
use super::registers::Registers;
use crate::cpu::instruction::Instruction;

pub struct Core {
    pub memory: Rc<RefCell<dyn Memory>>,
    pub registers: Registers,
}

impl Core {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Core {
        Self {
            memory,
            registers: Registers::new(),
        }
    }
    /// When not boot rom is supplied, we call this to make sure the following state is set
    pub fn simulate_boot_rom(&mut self) {
        self.registers.a = 0x01;
        self.registers.f = 0xb0;
        self.registers.b = 0x00;
        self.registers.c = 0x13;
        self.registers.d = 0x00;
        self.registers.e = 0xd8;
        self.registers.h = 0x01;
        self.registers.l = 0x4d;
        self.registers.pc = 0x0100;
        self.registers.sp = 0xfffe;
    }
    /// Get the next byte in the memory location
    fn get_next(&mut self) -> u8 {
        let value = self.memory.borrow().get(self.registers.pc);
        self.registers.pc += 1;
        value
    }
    /// Get the next work in the next memory location
    fn get_next_word(&mut self) -> u16 {
        let value = self.memory.borrow().get_word(self.registers.pc);
        self.registers.pc += 2;
        value
    }
    /// Push value to the stack and update the stack pointer
    fn stack_push(&mut self, value: u16) {
        self.registers.sp -= 2;
        self.memory.borrow_mut().set_word(self.registers.sp, value);
    }
    /// Pop the current value on the stack
    fn stack_pop(&mut self) -> u16 {
        let value = self.memory.borrow_mut().get_word(self.registers.sp);
        self.registers.sp += 2;
        value
    }
    /// Execute the next instruction
    pub fn execute_next(&mut self) -> u32 {
        // Grab the next memory location and parse it as opcode
        let mut instruction_opcode = self.get_next();
        let is_prefixed = instruction_opcode == 0xcb;
        if is_prefixed {
            instruction_opcode = self.get_next();
        }

        let instruction = Instruction::from_byte(instruction_opcode, is_prefixed);

        debug!(
            "HEX: {:04x} Decoded: {:?} Prefixed: {}",
            instruction_opcode, instruction, is_prefixed
        );

        match instruction {
            Instruction::NAI => {
                panic!("Not supposed to run the NAI instruction");
            }
            _ => unimplemented!(),
        }
        0
    }
    pub fn tick(&mut self) -> u32 {
        // TODO: Handle interrupts here
        let machine_cycle = self.execute_next();

        // Gameboy 1 machine cycle = 4 clock cycle
        machine_cycle * 4
    }
}
