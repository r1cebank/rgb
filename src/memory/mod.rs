// General Memory Map
// 0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
// 4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
// 8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
// A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
// C000-CFFF   4KB Work RAM Bank 0 (WRAM)
// D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
// E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
// FE00-FE9F   Sprite Attribute Table (OAM)
// FEA0-FEFF   Not Usable
// FF00-FF7F   I/O Ports
// FF80-FFFE   High RAM (HRAM)
// FFFF        Interrupt Enable Register
//
// See: http://bgb.bircd.org/pandocs.htm#cgbregisters

const BOOT_ROM_SIZE: usize = 0x100;

/// The memory for gameboy
///
pub struct Memory {
    boot_rom: [u8; BOOT_ROM_SIZE],
}

impl Memory {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>) -> Memory {
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
