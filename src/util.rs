pub mod file;
use std::str;

const BOOT_ROM_SIZE: usize = 0x100;

pub fn get_bootrom(path: &str) -> [u8; BOOT_ROM_SIZE] {
    let boot_rom_buffer = file::buffer_from_file(path);
    let boot_rom = {
        if boot_rom_buffer.len() > BOOT_ROM_SIZE {
            panic!(
                "Bootroom size mismatch, expected {}, got {}",
                BOOT_ROM_SIZE,
                boot_rom_buffer.len()
            );
        }
        let mut boot_rom = [0; BOOT_ROM_SIZE];
        boot_rom.copy_from_slice(&boot_rom_buffer);
        boot_rom
    };
    boot_rom
}

pub fn get_rom(path: &str) -> Vec<u8> {
    let rom_buffer = file::buffer_from_file(path);
    if rom_buffer.len() < 0x8000 || rom_buffer.len() % 0x4000 != 0 {
        panic!("Invalid length: {} bytes", rom_buffer.len());
    }
    rom_buffer
}
