pub mod registers;

pub struct CPU {
    pub registers: registers::Registers,
    pc: u16,
    sp: u16,
    pub bus: MemoryBus,
    is_halted: bool,
    interrupts_enabled: bool,
}
