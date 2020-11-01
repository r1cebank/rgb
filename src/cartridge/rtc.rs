use crate::memory::Memory;

use std::time::SystemTime;

// The Clock Counter Registers
//  08h  RTC S   Seconds   0-59 (0-3Bh)
//  09h  RTC M   Minutes   0-59 (0-3Bh)
//  0Ah  RTC H   Hours     0-23 (0-17h)
//  0Bh  RTC DL  Lower 8 bits of Day Counter (0-FFh)
//  0Ch  RTC DH  Upper 1 bit of Day Counter, Carry Bit, Halt Flag
//        Bit 0  Most significant bit of Day Counter (Bit 8)
//        Bit 6  Halt (0=Active, 1=Stop Timer)
//        Bit 7  Day Counter Carry Bit (1=Counter Overflow)
// The Halt Flag is supposed to be set before <writing> to the RTC Registers.
#[derive(Debug)]
struct ClockRegisters {
    S: u8,
    M: u8,
    H: u8,
    DL: u8,
    DH: u8,
}

#[derive(Debug)]
pub struct RealTimeClock {
    epoch: u64,
    reg: ClockRegisters,
}

impl RealTimeClock {
    pub fn new() -> Self {
        let epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            epoch,
            reg: ClockRegisters {
                S: 0x00,
                M: 0x00,
                H: 0x00,
                DL: 0x00,
                DH: 0x00,
            },
        }
    }

    pub fn tick(&mut self) {
        let delta = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - self.epoch;

        self.reg.S = (delta % 60) as u8;
        self.reg.M = (delta / 60 % 60) as u8;
        self.reg.H = (delta / 3600 % 24) as u8;
        let days = (delta / 3600 / 24) as u16;
        self.reg.DL = (days % 256) as u8;
        match days {
            0x0000..=0x00ff => {}
            0x0100..=0x01ff => {
                self.reg.DH |= 0x01;
            }
            _ => {
                self.reg.DH |= 0x01;
                self.reg.DH |= 0x80;
            }
        }
    }
}

impl Memory for RealTimeClock {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x08 => self.reg.S,
            0x09 => self.reg.M,
            0x0a => self.reg.H,
            0x0b => self.reg.DL,
            0x0c => self.reg.DH,
            _ => panic!("Invalid access on RTC"),
        }
    }

    fn set(&mut self, a: u16, v: u8) {
        match a {
            0x08 => self.reg.S = v,
            0x09 => self.reg.M = v,
            0x0a => self.reg.H = v,
            0x0b => self.reg.DL = v,
            0x0c => self.reg.DH = v,
            _ => panic!("Invalid access on RTC"),
        }
    }
}
