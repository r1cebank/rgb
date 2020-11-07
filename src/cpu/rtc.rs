use super::CPU;
use crate::memory::Memory;
use std::time;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

// Realtime CPU code from https://github.com/mohanson/gameboy/blob/master/src/cpu.rs
// Comments and changes are made for readability
pub const CLOCK_FREQUENCY: u32 = 4_194_304;
pub const STEP_TIME: u32 = 16;
pub const STEP_CYCLES: u32 = (STEP_TIME as f64 / (1000_f64 / CLOCK_FREQUENCY as f64)) as u32;

// Real time cpu provided to simulate real hardware speed.
/// Because the speed Gameboy is running at, there is no accurate way to time each clock cycle
/// We are slicing the cycles in 16 ms chunks
pub struct ClockedCPU {
    pub cpu: CPU,
    step_cycles: u32,         // How many cycles in the step (around 67108)
    step_zero: time::Instant, // Begin step
    step_flip: bool,          // When updating the step values, skip execution requests when true
}

impl ClockedCPU {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Self {
        let cpu = CPU::new(memory);
        Self {
            cpu,
            step_cycles: 0,
            step_zero: time::Instant::now(),
            step_flip: false,
        }
    }

    // Function next simulates real hardware execution speed, by limiting the frequency of the function cpu.next().
    pub fn tick(&mut self) -> u32 {
        if self.step_cycles > STEP_CYCLES {
            self.step_flip = true;
            self.step_cycles -= STEP_CYCLES;
            let now = time::Instant::now();
            let d = now.duration_since(self.step_zero);
            let s = u64::from(STEP_TIME.saturating_sub(d.as_millis() as u32));
            debug!("CPU: sleep {} millis", s);
            thread::sleep(time::Duration::from_millis(s));
            self.step_zero = self
                .step_zero
                .checked_add(time::Duration::from_millis(u64::from(STEP_TIME)))
                .unwrap();

            // If now is after the just updated target frame time, reset to
            // avoid drift.
            if now.checked_duration_since(self.step_zero).is_some() {
                self.step_zero = now;
            }
        }
        let cycles = self.cpu.tick();
        self.step_cycles += cycles;
        cycles
    }

    pub fn flip(&mut self) -> bool {
        let r = self.step_flip;
        if r {
            self.step_flip = false;
        }
        r
    }
}
