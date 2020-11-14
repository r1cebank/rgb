use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

use super::registers::{Flag, Registers};

pub struct Core {
    pub memory: Rc<RefCell<dyn Memory>>,
    pub registers: Registers,
    halted: bool,
    ei: bool,
}

impl Core {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Core {
        Self {
            memory,
            registers: Registers::new(),
            ei: true,
            halted: false,
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
    pub fn get_next(&mut self) -> u8 {
        let value = self.memory.borrow().get(self.registers.pc);
        self.registers.pc += 1;
        value
    }
    /// Get the next work in the next memory location
    pub fn get_next_word(&mut self) -> u16 {
        let value = self.memory.borrow().get_word(self.registers.pc);
        self.registers.pc += 2;
        value
    }
    /// Push value to the stack and update the stack pointer
    pub fn stack_push(&mut self, value: u16) {
        self.registers.sp -= 2;
        self.memory.borrow_mut().set_word(self.registers.sp, value);
    }
    /// Pop the current value on the stack
    pub fn stack_pop(&mut self) -> u16 {
        let value = self.memory.borrow_mut().get_word(self.registers.sp);
        self.registers.sp += 2;
        value
    }
    // Complement carry flag. If C flag is set, then reset it. If C flag is reset, then set it.
    pub fn alu_ccf(&mut self) {
        let carry = !self.registers.get_flag(Flag::C);
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
    }
    // Set Carry flag.
    pub fn alu_scf(&mut self) {
        self.registers.set_flag(Flag::C, true);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
    }
    // Complement A register. (Flip all bits.)
    pub fn alu_cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.set_flag(Flag::H, true);
        self.registers.set_flag(Flag::N, true);
    }
    // Add n to current address and jump to it.
    // n = one byte signed immediate value
    pub fn alu_jr(&mut self, n: u8) {
        let n = n as i8;
        self.registers.pc = ((u32::from(self.registers.pc) as i32) + i32::from(n)) as u16;
    }
    // Rotate n right through Carry flag.
    pub fn alu_rr(&mut self, n: u8) -> u8 {
        let carry = n & 0x01 == 0x01;
        let result = if self.registers.get_flag(Flag::C) {
            0x80 | (n >> 1)
        } else {
            n >> 1
        };
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Decimal adjust register A. This instruction adjusts register A so that the correct representation of Binary
    // Coded Decimal (BCD) is obtained.
    pub fn alu_daa(&mut self) {
        let mut a = self.registers.a;
        let mut adjust = if self.registers.get_flag(Flag::C) {
            0x60
        } else {
            0x00
        };
        if self.registers.get_flag(Flag::H) {
            adjust |= 0x06;
        };
        if !self.registers.get_flag(Flag::N) {
            if a & 0x0f > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }
        self.registers.set_flag(Flag::C, adjust >= 0x60);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::Z, a == 0x00);
        self.registers.a = a;
    }
    // Shift n right into Carry. MSB doesn't change.
    pub fn alu_sra(&mut self, n: u8) -> u8 {
        let carry = n & 0x01 == 0x01;
        let result = (n >> 1) | (n & 0x80);
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Shift n left into Carry. LSB of n set to 0.
    pub fn alu_sla(&mut self, n: u8) -> u8 {
        let carry = (n & 0x80) >> 7 == 0x01;
        let result = n << 1;
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Rotate n right. Old bit 0 to Carry flag.
    pub fn alu_rrc(&mut self, n: u8) -> u8 {
        let carry = n & 0x01 == 0x01;
        let result = if carry { 0x80 | (n >> 1) } else { n >> 1 };
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Rotate n left through Carry flag.
    pub fn alu_rl(&mut self, n: u8) -> u8 {
        let carry = (n & 0x80) >> 7 == 0x01;
        let result = (n << 1) + u8::from(self.registers.get_flag(Flag::C));
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Rotate n left. Old bit 7 to Carry flag.
    pub fn alu_rlc(&mut self, n: u8) -> u8 {
        let carry = (n & 0x80) >> 7 == 0x01;
        let result = (n << 1) | u8::from(carry);
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Decrement number
    pub fn alu_dec(&mut self, n: u8) -> u8 {
        let result = n.wrapping_sub(1);
        self.registers.set_flag(Flag::H, n.trailing_zeros() >= 4);
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::Z, result == 0);
        result
    }
    // Increment number
    pub fn alu_inc(&mut self, n: u8) -> u8 {
        let result = n.wrapping_add(1);
        self.registers.set_flag(Flag::H, (n & 0x0f) + 0x01 > 0x0f);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Subtract n from A.
    pub fn alu_sub(&mut self, n: u8) {
        let a = self.registers.a;
        let result = a.wrapping_sub(n);
        self.registers
            .set_flag(Flag::C, u16::from(a) < u16::from(n));
        self.registers.set_flag(Flag::H, (a & 0x0f) < (n & 0x0f));
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Subtract n + Carry flag from A.
    pub fn alu_sbc(&mut self, n: u8) {
        let a = self.registers.a;
        let carry = u8::from(self.registers.get_flag(Flag::C));
        let result = a.wrapping_sub(n).wrapping_sub(carry);
        self.registers
            .set_flag(Flag::C, u16::from(a) < u16::from(n) + u16::from(carry));
        self.registers
            .set_flag(Flag::H, (a & 0x0f) < (n & 0x0f) + carry);
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Add n + Carry flag to A.
    pub fn alu_adc(&mut self, n: u8) {
        let a = self.registers.a;
        let carry = u8::from(self.registers.get_flag(Flag::C));
        let result = a.wrapping_add(n).wrapping_add(carry);
        self.registers.set_flag(
            Flag::C,
            u16::from(a) + u16::from(n) + u16::from(carry) > 0xff,
        );
        self.registers
            .set_flag(Flag::H, (a & 0x0f) + (n & 0x0f) + (carry & 0x0f) > 0x0f);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Add n to A.
    pub fn alu_add(&mut self, n: u8) {
        let a = self.registers.a;
        let result = a.wrapping_add(n);
        self.registers
            .set_flag(Flag::C, u16::from(a) + u16::from(n) > 0xff);
        self.registers
            .set_flag(Flag::H, (a & 0x0f) + (n & 0x0f) > 0x0f);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Add n to HL
    pub fn alu_add_hl(&mut self, n: u16) {
        let value = self.registers.get_hl();
        let result = value.wrapping_add(n);
        self.registers.set_flag(Flag::C, value > 0xffff - n);
        self.registers
            .set_flag(Flag::H, (value & 0x0fff) + (n & 0x0fff) > 0x0fff);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_hl(result);
    }
    // Logically AND n with A, result in A.
    pub fn alu_and(&mut self, n: u8) {
        let result = self.registers.a & n;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, true);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Logical OR n with register A, result in A.
    pub fn alu_or(&mut self, n: u8) {
        let result = self.registers.a | n;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Logical exclusive OR n with register A, result in A.
    pub fn alu_xor(&mut self, n: u8) {
        let result = self.registers.a ^ n;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Compare A with n. This is basically an A - n subtraction instruction but the results are thrown away.
    pub fn alu_cp(&mut self, n: u8) {
        let r = self.registers.a;
        self.alu_sub(n);
        self.registers.a = r;
    }
    // Swap upper & lower nibles of n.
    pub fn alu_swap(&mut self, n: u8) -> u8 {
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, n == 0x00);
        (n >> 4) | (n << 4)
    }
    // Shift n right into Carry. MSB set to 0.
    pub fn alu_srl(&mut self, n: u8) -> u8 {
        let carry = n & 0x01 == 0x01;
        let result = n >> 1;
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Test bit b in register r.
    pub fn alu_bit(&mut self, a: u8, b: u8) {
        let result = a & (1 << b) == 0x00;
        self.registers.set_flag(Flag::H, true);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result);
    }
    // Reset bit b in register r.
    pub fn alu_res(&mut self, a: u8, b: u8) -> u8 {
        a & !(1 << b)
    }
    // Set bit b in register r.
    pub fn alu_set(&mut self, a: u8, b: u8) -> u8 {
        a | (1 << b)
    }
}
