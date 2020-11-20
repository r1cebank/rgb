/// The interrupt flags that can be raised in the system
#[derive(Clone)]
pub enum Flag {
    VBlank = 0,
    LCDStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

// The interrupt flags that is shared across the system.
pub struct InterruptFlags {
    pub data: u8,
}

impl InterruptFlags {
    pub fn new() -> Self {
        Self { data: 0x00 }
    }

    /// Raise an interrupt
    pub fn hi(&mut self, flag: Flag) {
        self.data |= 1 << flag as u8;
    }
}
