// https://github.com/mohanson/gameboy/blob/06672b1f1c/src/timer.rs
use crate::cpu::interrupt::{Flag, InterruptFlags};
use std::cell::RefCell;
use std::rc::Rc;

// Sometimes it's useful to have a timer that interrupts at regular intervals for routines that require periodic or
// precise updates. The timer in the GameBoy has a selectable frequency of 4096, 16384, 65536, or 262144 Hertz.
// This frequency increments the Timer Counter (TIMA). When it overflows, it generates an interrupt. It is then loaded
// with the contents of Timer Modulo (TMA).
//
// See: http://gbdev.gg8.se/wiki/articles/Timer_and_Divider_Registers
// Clock is ticked 1 cycle every N cycles.
pub struct Clock {
    pub period: u32,
    pub n: u32,
}

impl Clock {
    pub fn new(period: u32) -> Self {
        Self { period, n: 0x00 }
    }

    pub fn next(&mut self, cycles: u32) -> u32 {
        self.n += cycles;
        let rs = self.n / self.period;
        self.n = self.n % self.period;
        rs
    }
}

// FF0F - IF - Interrupt Flag (R/W)
// Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
// Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
// Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)
// Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)
// Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)
#[derive(Default)]
struct Register {
    // This register is incremented at rate of 16384Hz (~16779Hz on SGB). Writing any value to this register resets it
    // to 00h.
    // Note: The divider is affected by CGB double speed mode, and will increment at 32768Hz in double speed.
    div: u8,
    // This timer is incremented by a clock frequency specified by the TAC register ($FF07). When the value overflows
    // (gets bigger than FFh) then it will be reset to the value specified in TMA (FF06), and an interrupt will be
    // requested, as described below.
    tima: u8,
    // When the TIMA overflows, this data will be loaded.
    tma: u8,
    //  Bit  2   - Timer Enable
    //  Bits 1-0 - Input Clock Select
    //             00: CPU Clock / 1024 (DMG, CGB:   4096 Hz, SGB:   ~4194 Hz)
    //             01: CPU Clock / 16   (DMG, CGB: 262144 Hz, SGB: ~268400 Hz)
    //             10: CPU Clock / 64   (DMG, CGB:  65536 Hz, SGB:  ~67110 Hz)
    //             11: CPU Clock / 256  (DMG, CGB:  16384 Hz, SGB:  ~16780 Hz)
    tac: u8,
}

// Each time when the timer overflows (ie. when TIMA gets bigger than FFh), then an interrupt is requested by
// setting Bit 2 in the IF Register (FF0F). When that interrupt is enabled, then the CPU will execute it by calling
// the timer interrupt vector at 0050h.
pub struct Timer {
    interrupt_flag: Rc<RefCell<InterruptFlags>>,
    register: Register,
    div_clock: Clock,
    tma_clock: Clock,
}

/// The timer is used for keeping track of operations in the gameboy, we update the clock at each
/// cycle (end of instruction currently) to make sure clock interrupts is working as expected
impl Timer {
    pub fn new(interrupt_flag: Rc<RefCell<InterruptFlags>>) -> Self {
        Timer {
            interrupt_flag,
            register: Register::default(),
            div_clock: Clock::new(256),
            tma_clock: Clock::new(1024),
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        match address {
            0xff04 => self.register.div,
            0xff05 => self.register.tima,
            0xff06 => self.register.tma,
            0xff07 => self.register.tac,
            _ => panic!("Unsupported address"),
        }
    }

    pub fn set(&mut self, address: u16, value: u8) {
        match address {
            0xff04 => {
                self.register.div = 0x00;
                self.div_clock.n = 0x00;
            }
            0xff05 => self.register.tima = value,
            0xff06 => self.register.tma = value,
            0xff07 => {
                if (self.register.tac & 0x03) != (value & 0x03) {
                    self.tma_clock.n = 0x00;
                    self.tma_clock.period = match value & 0x03 {
                        0x00 => 1024,
                        0x01 => 16,
                        0x02 => 64,
                        0x03 => 256,
                        _ => panic!("Setting invalid clock"),
                    };
                    self.register.tima = self.register.tma;
                }
                self.register.tac = value;
            }
            _ => panic!("Unsupported address"),
        }
    }

    pub fn tick(&mut self, cycles: u32) {
        // Increment div at rate of 16384Hz. Because the clock cycles is 4194304, so div increment every 256 cycles.
        self.register.div = self
            .register
            .div
            .wrapping_add(self.div_clock.next(cycles) as u8);

        // Increment tima at rate of Clock / freq
        // Timer Enable
        if (self.register.tac & 0x04) != 0x00 {
            let n = self.tma_clock.next(cycles);
            for _ in 0..n {
                self.register.tima = self.register.tima.wrapping_add(1);
                if self.register.tima == 0x00 {
                    self.register.tima = self.register.tma;
                    self.interrupt_flag.borrow_mut().hi(Flag::Timer);
                }
            }
        }
    }
}
