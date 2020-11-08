pub mod cycles;
pub mod instruction;
pub mod opcodes;
pub mod registers;
pub mod sm80;
use crate::memory::Memory;
use std::time::{Duration, Instant};

use sm80::Core;
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
    pub cpu: Core,
    step_cycles: u32,   // How many cycles in the step (around 67108)
    step_zero: Instant, // Begin step
    step_flip: bool,    // When this is set to true, we want to handle events
}

impl ClockedCPU {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Self {
        let cpu = Core::new(memory);
        Self {
            cpu,
            step_cycles: 0,
            step_zero: Instant::now(),
            step_flip: false,
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

            debug!("CPU: sleep {} millis", sleep_time);
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

        // Run the CPU and get the machine cycles
        let cycles = self.cpu.tick();

        // Increment the step cycles with the cpu tick cycles
        self.step_cycles += cycles;
        cycles
    }

    pub fn flip(&mut self) -> bool {
        // If true return and flip the flag, otherwise false
        let r = self.step_flip;
        self.step_flip = false;
        r
    }
}