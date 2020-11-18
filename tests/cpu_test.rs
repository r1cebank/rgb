use rgb;
use rgb::cpu::ClockedCPU;
use rgb::emulator::Emulator;
use rgb::memory::Memory;
use rgb::util::get_rom;
use std::cell::RefCell;
use std::rc::Rc;

struct TestMemory {
    last_serial: u8,
    memory: [u8; 0xffff],
}

impl TestMemory {
    pub fn new(rom: Vec<u8>) -> TestMemory {
        let mut memory = [0; 0xffff];
        for i in rom {
            memory[i as usize + 0x100] = memory[i as usize];
        }
        Self {
            last_serial: 0,
            memory,
        }
    }
    pub fn simulate_boot_rom(&mut self) {
        self.set(0xff05, 0x00);
        self.set(0xff06, 0x00);
        self.set(0xff07, 0x00);
        self.set(0xff10, 0x80);
        self.set(0xff11, 0xbf);
        self.set(0xff12, 0xf3);
        self.set(0xff14, 0xbf);
        self.set(0xff16, 0x3f);
        self.set(0xff17, 0x00);
        self.set(0xff19, 0xbf);
        self.set(0xff1a, 0x7f);
        self.set(0xff1b, 0xff);
        self.set(0xff1c, 0x9f);
        self.set(0xff1e, 0xbf);
        self.set(0xff20, 0xff);
        self.set(0xff21, 0x00);
        self.set(0xff22, 0x00);
        self.set(0xff23, 0xbf);
        self.set(0xff24, 0x77);
        self.set(0xff25, 0xf3);
        self.set(0xff26, 0xf1);
        self.set(0xff40, 0x91);
        self.set(0xff42, 0x00);
        self.set(0xff43, 0x00);
        self.set(0xff45, 0x00);
        self.set(0xff47, 0xfc);
        self.set(0xff48, 0xff);
        self.set(0xff49, 0xff);
        self.set(0xff4a, 0x00);
        self.set(0xff4b, 0x00);
    }
}

impl Memory for TestMemory {
    fn get(&self, address: u16) -> u8 {
        println!("${:04x}", address);
        match address {
            _ => self.memory[address as usize],
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0xff01 => self.last_serial = value,
            0xff02 => print!("{}", self.last_serial as char),
            _ => self.memory[address as usize] = value,
        }
    }
}

#[test]
fn run_cpu_tests() {
    let rom = get_rom("res/01.gb");
    let mmu = Rc::new(RefCell::new(TestMemory::new(rom)));
    let mut cpu = ClockedCPU::new(mmu.clone());
    mmu.borrow_mut().simulate_boot_rom();
    cpu.simulate_boot_rom();
    loop {
        cpu.tick();
    }
}
