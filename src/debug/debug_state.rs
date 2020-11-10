use crate::cpu::registers::Registers;

const MAX_LOG: usize = 6;

pub struct DebugState {
    pub registers: Registers,
    pub log_messages: Vec<String>,
}

/// Stores the debugger state, including register state and log messages
impl DebugState {
    pub fn new() -> DebugState {
        Self {
            registers: Registers::new(),
            log_messages: Vec::new(),
        }
    }
    // Append log to the debug state, if it exceed the max log, the oldest log will be purged
    pub fn append_log(&mut self, log: String) {
        if self.log_messages.len() >= MAX_LOG {
            self.log_messages.drain(0..1);
        }
        self.log_messages.push(log);
    }
}
