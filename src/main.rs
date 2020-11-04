extern crate clap;
#[macro_use]
extern crate log;

pub mod cartridge;
pub mod cpu;
pub mod dmg01;
pub mod memory;

use cartridge::CartridgeType;
use clap::{App, Arg};
use imgui::*;
use memory::Memory;
use simplelog::*;
use std::io::Read;

mod support;

fn main() {
    let matches = App::new("rgb")
        .author("Siyuan Gao <rbnk@elica.io>")
        .arg(
            Arg::with_name("boot")
                .short("b")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("rom")
                .short("r")
                .required(false)
                .value_name("FILE"),
        )
        .get_matches();

    let _ = SimpleLogger::init(LevelFilter::Trace, Config::default());

    let boot_buffer = matches.value_of("boot").map(|path| buffer_from_file(path));
    let rom_buffer = matches.value_of("rom").map(|path| buffer_from_file(path));

    // let test_cpu = cpu::CPU::new(boot_buffer);
    let mut dmg = dmg01::dmg01::new(boot_buffer, rom_buffer);

    let system = support::init(file!());
    let mut address_value: String = String::from("0");
    let mut address = ImString::with_capacity(32);

    system.main_loop(move |_, ui| {
        dmg.tick();
        let cartridge = &dmg.mmu.borrow().cartridge;
        let cpu = &dmg.cpu;
        Window::new(&im_str!(
            "cartridge: [{}]",
            dmg.mmu.borrow().cartridge.title
        ))
        .size([700.0, 160.0], Condition::FirstUseEver)
        .position([10.0, 10.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("cartridge_type: {:?}", cartridge.cartridge_type));
            ui.text(im_str!(
                "cartridge_rom_size: {:?}",
                cartridge.cartridge_rom_size
            ));
            ui.text(im_str!(
                "cartridge_ram_size: {:?}",
                cartridge.cartridge_ram_size
            ));
            ui.separator();
            let mbc_state = &cartridge.mbc_state;
            match cartridge.cartridge_type {
                CartridgeType::MBC3 {
                    ram: _,
                    battery: _,
                    rtc: _,
                } => {
                    ui.text(im_str!("rtc: {:?}", mbc_state.mbc3.rtc));
                    ui.text(im_str!("rom_bank: {:x}", mbc_state.mbc3.rom_bank));
                    ui.text(im_str!("ram_bank: {:x}", mbc_state.mbc3.ram_bank));
                    ui.text(im_str!("ram_enable: {:?}", mbc_state.mbc3.ram_enable));
                }
                _ => {}
            }
        });

        Window::new(&im_str!("cpu: {}hz, speed: {}x", cpu.frequency, cpu.speed))
            .size([700.0, 150.0], Condition::FirstUseEver)
            .position([10.0, 180.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("cartridge_type: {:?}", cartridge.cartridge_type));
                ui.text(im_str!(
                    "wait_time: {}, cycle_duration: {}ms",
                    cpu.wait_time,
                    cpu.cycle_duration,
                ));
                ui.text(im_str!(
                    "registers: {}",
                    cpu.cpu.registers.get_register_overview()
                ));
                ui.text(im_str!(
                    "16 bit registers: {}",
                    cpu.cpu.registers.get_word_register_overview()
                ));
                ui.text(im_str!(
                    "flags: {}",
                    cpu.cpu.registers.get_flag_register_overview()
                ));
                ui.text(im_str!("last instruction: {:?}", cpu.cpu.last_instruction));
            });
        Window::new(&im_str!("memory : {:?}", cartridge.cartridge_ram_size))
            .size([250.0, 160.0], Condition::FirstUseEver)
            .position([720.0, 10.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.input_text(im_str!("address"), &mut address).build();
                if ui.button(im_str!("lookup"), [100.0, 20.0]) {
                    let address =
                        u16::from_str_radix(address.to_str().trim_start_matches("0x"), 16).unwrap();
                    address_value = format!("{:x}", dmg.mmu.borrow().get(address));
                }
                if ui.button(im_str!("lookup word"), [100.0, 20.0]) {
                    let address =
                        u16::from_str_radix(address.to_str().trim_start_matches("0x"), 16).unwrap();
                    address_value = format!("{:x}", dmg.mmu.borrow().get_word(address));
                }
                ui.text(im_str!("value: {}", address_value.to_uppercase()));
            });
    });
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}
