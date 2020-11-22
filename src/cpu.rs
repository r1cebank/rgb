pub mod instruction;
pub mod interrupt;
pub mod registers;
pub mod sm80;

use crate::memory::Memory;
use std::time::{Duration, Instant};

use crate::cpu::instruction::InstructionSet;
use crate::cpu::registers::Flag;
use sm80::Core;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

// Realtime CPU code from https://github.com/mohanson/gameboy/blob/master/src/cpu.rs
// Comments and changes are made for readability
pub const CLOCK_FREQUENCY: u32 = 4_194_304;
pub const STEP_TIME: u32 = 16;
pub const STEP_CYCLES: u32 = (STEP_TIME as f64 / (1000_f64 / CLOCK_FREQUENCY as f64)) as u32;

const INTERRUPT_ENABLE_REG: u16 = 0xFFFF;
const INTERRUPT_FLAG_REG: u16 = 0xFF0F;

const V_BLANK: u8 = 0x01;
const LCD_STAT: u8 = 0x02;
const TIMER: u8 = 0x04;
const SERIAL: u8 = 0x08;
const D_PAD: u8 = 0x10;

// Nintendo documents describe the CPU & instructions speed in machine cycles while this document describes them in
// clock cycles. Here is the translation:
//   1 machine cycle = 4 clock cycles
//                   GB CPU Speed    NOP Instruction
// Machine Cycles    1.05MHz         1 cycle
// Clock Cycles      4.19MHz         4 cycles
//
//  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
const OP_CYCLES: [u32; 256] = [
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 2
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 3
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6
    2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // a
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // b
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, // c
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, // d
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // e
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // f
];

//  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
const CB_CYCLES: [u32; 256] = [
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 1
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 2
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 3
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 4
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 5
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 6
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 7
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 8
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 9
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // a
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // b
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // c
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // d
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // e
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // f
];

// Real time cpu provided to simulate real hardware speed.
/// Because the speed Gameboy is running at, there is no accurate way to time each clock cycle
/// We are slicing the cycles in 16 ms chunks
pub struct ClockedCPU {
    // The sm83 core
    pub core: Core,
    // The instruction set mappers
    instruction_set: InstructionSet,
    // How many cycles in the step (around 67108)
    step_cycles: u32,
    // last time when the last step is finished
    step_zero: Instant,
    // Begin step
    step_flip: bool, // When this is set to true, we want to handle events
}

impl ClockedCPU {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Self {
        let core = Core::new(memory);
        Self {
            core,
            instruction_set: InstructionSet::new(),
            step_cycles: 0,
            step_zero: Instant::now(),
            step_flip: false,
        }
    }
    fn execute_next_instruction(&mut self) -> u32 {
        let executable_instruction = self
            .instruction_set
            .get_next_executable_instruction(&mut self.core)
            .expect("Error decoding next instruction");

        let (instruction, operand, prefixed, opcode) = executable_instruction;

        // Some instructions have operands, for those we need to push the pc register and get the operand from memory
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

        // Execute the instruction
        (instruction.exec)(&mut self.core, operand);

        trace!("{}", self.core.registers.get_flag_register_overview());
        trace!("{}", self.core.registers.get_register_overview());
        trace!("{}", self.core.registers.get_word_register_overview());

        // When we branch, there are extra machine cycles needed for (reading, setting) pc, we are
        // adding those there
        let branch_cycle = match opcode {
            0x20 | 0x30 => {
                if self.core.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x01
                }
            }
            0x28 | 0x38 => {
                if self.core.registers.get_flag(Flag::Z) {
                    0x01
                } else {
                    0x00
                }
            }
            0xc0 | 0xd0 => {
                if self.core.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x03
                }
            }
            0xc8 | 0xcc | 0xd8 | 0xdc => {
                if self.core.registers.get_flag(Flag::Z) {
                    0x03
                } else {
                    0x00
                }
            }
            0xc2 | 0xd2 => {
                if self.core.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x01
                }
            }
            0xca | 0xda => {
                if self.core.registers.get_flag(Flag::Z) {
                    0x01
                } else {
                    0x00
                }
            }
            0xc4 | 0xd4 => {
                if self.core.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x03
                }
            }
            _ => 0x00,
        };

        // Based on the type of the instruction, cycle is mapped with the cycle map
        if prefixed {
            CB_CYCLES[opcode as usize]
        } else {
            OP_CYCLES[opcode as usize] + branch_cycle
        }
    }

    // Function next simulates real hardware execution speed, by limiting the frequency of the function cpu.next().
    pub fn tick(&mut self) -> u32 {
        // When we ran all the cycles in this step, we enter the count and wait period
        if self.step_cycles > STEP_CYCLES {
            // Set the step flip flag so events will be handled at the end of the step
            self.step_flip = true;
            self.step_cycles -= STEP_CYCLES;
            let now = Instant::now();

            // Time passed since last run time
            let time_passed = now.duration_since(self.step_zero);

            // Subtract the time passed from the expected step time to get the time thread needs to sleep
            let sleep_time = u64::from(STEP_TIME.saturating_sub(time_passed.as_millis() as u32));

            trace!("CPU: sleep {} millis", sleep_time);
            thread::sleep(Duration::from_millis(sleep_time));

            // Update the last run zero time with the last time + step time
            self.step_zero = self
                .step_zero
                .checked_add(Duration::from_millis(u64::from(STEP_TIME)))
                .unwrap();

            // If now is after the just updated target frame time, reset to
            // avoid drift.
            if now.checked_duration_since(self.step_zero).is_some() {
                self.step_zero = now;
            }
        }

        // Run the CPU and get the machine cycles, handle interrupts if there is any
        let cycles = {
            let interrupt_cycles = self.core.handle_interrupt();
            if interrupt_cycles != 0 {
                interrupt_cycles
            } else if self.core.halted {
                OP_CYCLES[0]
            } else {
                self.execute_next_instruction()
            }
        } * 4; // We time this by 4 since up till now, the cycles we are referring to is machine cycles. 1 machine cycle = 4 t-cycle

        // Increment the step cycles with the cpu tick cycles
        self.step_cycles += cycles;
        cycles
    }

    pub fn simulate_boot_rom(&mut self) {
        self.core.simulate_boot_rom();
    }

    pub fn flip(&mut self) -> bool {
        // If true return and flip the flag, otherwise false
        let r = self.step_flip;
        self.step_flip = false;
        r
    }
}
