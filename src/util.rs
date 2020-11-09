pub mod file;

use std::io::Read;
use std::str;

pub const BOOT_ROM_SIZE: usize = 0x100;

pub fn get_boot_rom(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File does not exist");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("File read error");
    buffer
}

pub fn get_rom(path: &str) -> Vec<u8> {
    let rom_buffer = file::buffer_from_file(path);
    if rom_buffer.len() < 0x8000 || rom_buffer.len() % 0x4000 != 0 {
        panic!("Invalid length: {} bytes", rom_buffer.len());
    }
    rom_buffer
}
