/// The memory for gameboy
///
pub struct MMU {
    boot_rom: [u8; BOOT_ROM_SIZE],
}

impl MMU {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>) -> MMU {
        let boot_rom = boot_rom_buffer
            .map(|boot_rom_buffer| {
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
            })
            .unwrap();
        Memory { boot_rom }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        1
    }
}
