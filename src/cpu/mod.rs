pub mod instruction;
pub mod opcodes;
pub mod registers;

pub use self::instruction::{ArithmeticTarget, Instruction, JumpCondition};
use self::registers::Registers;
use crate::memory::MemoryBus;

pub struct CPU {
    pub registers: Registers,
    bus: MemoryBus,
    pc: u16,
    sp: u16,
    is_halted: bool,
    interrupts_enabled: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            bus: MemoryBus::new(),
            pc: 0x0,
            sp: 0x0,
            is_halted: true,
            interrupts_enabled: true,
        }
    }
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        let next_pc = if instruction != Instruction::NAI {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unkown instruction found for: {}", description)
        };
        self.pc = next_pc;
    }
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        /* TODO: support more targets */
                        panic!("ADD ({:?}) not yet implemented", target);
                    }
                }
            }
            Instruction::JP(condition) => {
                let jump_condition = match condition {
                    JumpCondition::NotZero => !self.registers.f.zero,
                    JumpCondition::NotCarry => !self.registers.f.carry,
                    JumpCondition::Zero => self.registers.f.zero,
                    JumpCondition::Carry => self.registers.f.carry,
                    JumpCondition::Always => true,
                };
                self.jump(jump_condition)
            }
            _ => {
                /* TODO: support more instructions */
                panic!("{:?} is not yet implemented", instruction);
            }
        }
    }
    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }
    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);

        // Update flags
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}
