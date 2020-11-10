pub mod mmu;
mod timer;

pub trait Memory {
    fn get(&self, address: u16) -> u8;

    fn set(&mut self, address: u16, value: u8);

    fn get_word(&self, address: u16) -> u16 {
        u16::from(self.get(address)) | (u16::from(self.get(address + 1)) << 8)
    }

    fn set_word(&mut self, address: u16, value: u16) {
        self.set(address, (value & 0xFF) as u8);
        self.set(address + 1, (value >> 8) as u8)
    }
}
