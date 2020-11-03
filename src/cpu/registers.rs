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
    Z,
    // Subtract Flag. This bit is set if a subtraction was performed in the last math instruction.
    N,
    // Half Carry Flag. This bit is set if a carry occurred from the lowernibble in the last math operation.
    H,
    // Carry Flag. This bit is set if a carry occurred from the last math operation or if register A is the smaller
    // valuewhen executing the CP instruction.
    C,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl FlagsRegister {
    pub fn new() -> FlagsRegister {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
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
    pub f: FlagsRegister,
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
            f: FlagsRegister::new(),
            h: 0x00,
            l: 0x00,
            pc: 0x00,
            sp: 0x00,
        }
    }

    pub fn get_register_overview(&self) -> String {
        format!(
            "A: {:x}, B: {:x}, C: {:x}, D: {:x}, E: {:x}, H: {:x}, L: {:x}",
            self.a, self.b, self.c, self.d, self.e, self.h, self.l
        )
    }

    pub fn get_word_register_overview(&self) -> String {
        format!(
            "AF: {:x}, BC: {:x}, DE: {:x}, HL: {:x}, AF: {:x}",
            self.get_af(),
            self.get_bc(),
            self.get_de(),
            self.get_hl(),
            self.get_af(),
        )
    }

    pub fn get_flag_register_overview(&self) -> String {
        format!("{:?}", self.f)
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
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = FlagsRegister::from((value & 0x00f0) as u8);
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

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::C => self.f.zero,
            Flag::H => self.f.half_carry,
            Flag::N => self.f.half_carry,
            Flag::Z => self.f.zero,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        match flag {
            Flag::C => self.f.zero = value,
            Flag::H => self.f.half_carry = value,
            Flag::N => self.f.half_carry = value,
            Flag::Z => self.f.zero = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_to_u8() {
        let mut flags = FlagsRegister::new();
        flags.zero = true;
        flags.carry = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b1001_0000u8);
    }

    #[test]
    fn can_be_converted_from_u8() {
        let result: FlagsRegister = 0b1001_0000.into();
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
        assert_eq!(result.half_carry, false);
        assert_eq!(result.subtract, false);
    }
}
