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
    let mut address = ImString::new("0");
    let mut boot_rom_toggle: bool = false;

    system.main_loop(move |_, ui| {
        dmg.tick();
        Window::new(&im_str!(
            "cartridge: [{}]",
            dmg.mmu.borrow().cartridge.title
        ))
        .size([700.0, 160.0], Condition::FirstUseEver)
        .position([10.0, 10.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!(
                "cartridge_type: {:?}",
                dmg.mmu.borrow().cartridge.cartridge_type
            ));
            ui.text(im_str!(
                "cartridge_rom_size: {:?}",
                dmg.mmu.borrow().cartridge.cartridge_rom_size
            ));
            ui.text(im_str!(
                "cartridge_ram_size: {:?}",
                dmg.mmu.borrow().cartridge.cartridge_ram_size
            ));
            ui.separator();
            let mbc_state = &dmg.mmu.borrow().cartridge.mbc_state;
            match dmg.mmu.borrow().cartridge.cartridge_type {
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

        Window::new(&im_str!(
            "cpu: {}hz, speed: {}x paused: {}",
            dmg.cpu.frequency,
            dmg.cpu.speed,
            dmg.is_paused()
        ))
        .size([700.0, 160.0], Condition::FirstUseEver)
        .position([10.0, 180.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!(
                "cartridge_type: {:?}",
                dmg.mmu.borrow().cartridge.cartridge_type
            ));
            ui.text(im_str!(
                "wait_time: {}, cycle_duration: {}ms",
                dmg.cpu.wait_time,
                dmg.cpu.cycle_duration,
            ));
            ui.text(im_str!(
                "registers: {}",
                dmg.cpu.cpu.registers.get_register_overview()
            ));
            ui.text(im_str!(
                "16 bit registers: {}",
                dmg.cpu.cpu.registers.get_word_register_overview()
            ));
            ui.text(im_str!(
                "flags: {}",
                dmg.cpu.cpu.registers.get_flag_register_overview()
            ));
            ui.text(im_str!(
                "last instruction: {:?}",
                dmg.cpu.cpu.last_instruction
            ));
            if dmg.is_paused() {
                if ui.button(im_str!("resume"), [100.0, 20.0]) {
                    dmg.resume();
                }
            } else {
                if ui.button(im_str!("pause"), [100.0, 20.0]) {
                    dmg.pause();
                }
            }
        });
        Window::new(&im_str!(
            "memory : {:?}",
            dmg.mmu.borrow().cartridge.cartridge_ram_size
        ))
        .size([250.0, 160.0], Condition::FirstUseEver)
        .position([720.0, 10.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.input_text(im_str!("address"), &mut address).build();
            ui.checkbox(im_str!("bootrom"), &mut boot_rom_toggle);
            ui.popup(im_str!("overflow_popup"), || {
                ui.text("address overflow");
            });
            let address = u16::from_str_radix(address.to_str().trim_start_matches("0x"), 16)
                .unwrap_or_default();
            if ui.button(im_str!("lookup"), [100.0, 20.0]) {
                if boot_rom_toggle {
                    if address > 0x100 {
                        ui.open_popup(im_str!("overflow_popup"));
                    } else {
                        address_value = format!("{:x}", dmg.mmu.borrow().get(address));
                    }
                } else {
                    address_value = format!("{:x}", dmg.mmu.borrow().get_mem(address));
                }
            }
            if ui.button(im_str!("lookup word"), [100.0, 20.0]) {
                if boot_rom_toggle {
                    if address > 0x100 {
                        ui.open_popup(im_str!("overflow_popup"));
                    } else {
                        address_value = format!("{:x}", dmg.mmu.borrow().get_word(address));
                    }
                } else {
                    address_value = format!("{:x}", dmg.mmu.borrow().get_mem_word(address));
                }
            }
            ui.text(im_str!("value: {}", address_value.to_uppercase()));
            ui.text(im_str!("last_op: {}", dmg.mmu.borrow().last_op));
        });
    });
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}
