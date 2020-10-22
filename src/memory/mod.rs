pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; 0xFFFF],
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
