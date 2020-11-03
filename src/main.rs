#[macro_use]
extern crate conrod;
extern crate clap;
#[macro_use]
extern crate log;

pub mod cartridge;
pub mod cpu;
pub mod dmg01;
pub mod memory;

use clap::{App, Arg};
use conrod::backend::glium::glium::{self, Surface};
use simplelog::*;
use std::io::Read;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;

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

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    loop {
        dmg.tick();
    }
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}
