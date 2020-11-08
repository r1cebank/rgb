mod mbc1;
mod mbc3;
mod rom;

use super::memory::Memory;
use super::save::Savable;

pub trait Cartridge: Memory + Savable {
    fn title(&self) -> String {
        let mut title = String::new();
        let title_start = 0x134;
        let mut title_end = 0x143;
        // Using the cartridge type to determine how far to read the data
        let is_cgb = match self.get(0x143) {
            0x80 => true,
            _ => false,
        };
        if is_cgb {
            title_end = 0x13e;
        }
        for address in title_start..title_end {
            match self.get(address) {
                0 => break,
                c => title.push(c as char),
            }
        }
        title
    }
}
