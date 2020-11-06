#[derive(Copy, Clone, Debug)]
pub struct FlagsRegister {
    // set if the the last operation produced a result of 0
    pub zero: bool,
    // set if the last operation was a subtraction
    pub subtract: bool,
    // set if lower half of the result overflowed
    pub half_carry: bool,
    // set if the result overflowed
    pub carry: bool,
}

// The Fleg Register consists of the following bits: Z, N, H, C, 0, 0, 0, 0.
pub enum Flag {
    // Zero Flag. This bit is set when the result of a math operationis zero or two values match when using the CP
    // instruction.
    Z = 0b1000_0000,
    // Subtract Flag. This bit is set if a subtraction was performed in the last math instruction.
    N = 0b0100_0000,
    // Half Carry Flag. This bit is set if a carry occurred from the lowernibble in the last math operation.
    H = 0b0010_0000,
    // Carry Flag. This bit is set if a carry occurred from the last math operation or if register A is the smaller
    // valuewhen executing the CP instruction.
    C = 0b0001_0000,
}

impl Flag {
    pub fn og(self) -> u8 {
        self as u8
    }

    pub fn bw(self) -> u8 {
        !self.og()
    }
}

/// Registers in the Gameboy include
/// general use a, b, c, d, e, h, l
/// flag register f
/// pc for pc counter  16-bit
/// sp for stack pointer  16-bit
#[derive(Copy, Clone, Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x00,
            sp: 0x00,
        }
    }

    pub fn get_register_overview(&self) -> String {
        format!(
            "A: {:04x}, B: {:04x}, C: {:04x}, D: {:04x}, E: {:04x}, H: {:04x}, L: {:04x}",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l
        )
    }

    pub fn get_word_register_overview(&self) -> String {
        format!(
            "AF: {:04x}, BC: {:04x}, DE: {:04x}, HL: {:04x}, SP: {:04x}",
            self.get_af(),
            self.get_bc(),
            self.get_de(),
            self.get_hl(),
            self.sp
        )
    }

    pub fn get_flag_register_overview(&self) -> String {
        format!(
            "Z: {}, N: {}, H: {}, C: {}",
            self.get_flag(Flag::Z),
            self.get_flag(Flag::N),
            self.get_flag(Flag::H),
            self.get_flag(Flag::C)
        )
    }

    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(u8::from(self.f))
    }

    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn get_hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as u8;
        self.f = (v & 0x00f0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00ff) as u8;
    }

    pub fn get_flag(&self, f: Flag) -> bool {
        (self.f & f as u8) != 0
    }

    pub fn set_flag(&mut self, f: Flag, v: bool) {
        if v {
            self.f |= f.og();
        } else {
            self.f &= f.bw();
        }
    }
}
