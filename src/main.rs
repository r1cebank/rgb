extern crate clap;
#[macro_use]
extern crate log;

mod apu;
mod cpu;
mod display;
mod emulator;
mod io;
mod memory;
mod ppu;
mod util;

use apu::start_apu_thread;
use clap::{App, Arg};
use display::start_display_thread;
use emulator::start_emulator_thread;
use io::start_io_thread;
use simplelog::*;
use std::fs::File;
use util::{get_bootrom, get_rom};

fn main() {
    let mut config = ConfigBuilder::new();
    config.add_filter_ignore(format!("{}", "rustyline"));
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, config.build(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create("warnings.log").unwrap(),
        ),
        // WriteLogger::new(
        //     LevelFilter::Trace,
        //     Config::default(),
        //     File::create("trace.log").unwrap(),
        // ),
    ])
    .unwrap();

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
        .arg(
            Arg::with_name("audio")
                .long("audio")
                .short("a")
                .required(false)
                .takes_value(false)
                .help("Enable audio"),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .takes_value(true)
                .required(false)
                .default_value("2")
                .help("UI scale factor"),
        )
        .get_matches();

    let bootrom = get_bootrom(matches.value_of("boot").unwrap());
    let rom = get_rom(matches.value_of("rom").unwrap());

    let emulator_thread = start_emulator_thread();
    let io_thread = start_io_thread();
    let display_thread = start_display_thread(
        matches.value_of("scale").unwrap().parse::<i32>().unwrap(),
        String::from("test rom"),
    );
    let apu_thread = start_apu_thread();

    emulator_thread.join().unwrap();
    io_thread.join().unwrap();
    apu_thread.join().unwrap();
    display_thread.join().unwrap();
}
