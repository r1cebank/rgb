extern crate clap;
#[macro_use]
extern crate log;
#[cfg(feature = "debug")]
extern crate cursive_hexview;
extern crate find_folder;
extern crate image as im;
extern crate piston_window;

mod apu;
mod cartridge;
mod cpu;
mod debug;
mod display;
mod emulator;
mod input;
mod memory;
mod ppu;
mod save;
mod util;

use apu::start_apu_thread;
use cartridge::load_cartridge;
use clap::{App, Arg};
use debug::debug_logger::DebugLogger;
#[cfg(feature = "debug")]
use debug::start_debug_thread;
use display::start_display_thread;
use emulator::start_emulator_thread;
use input::start_io_thread;
use simplelog::*;
use std::fs::File;
use util::{get_boot_rom, get_rom};

fn main() {
    /////////////////flume sender receivers////////////////////////
    let (framebuffer_sender, framebuffer_receiver) = flume::bounded(1);
    // Debug channels
    // let (debug_command_sender, debug_command_receiver) = flume::unbounded();
    let (debug_message_sender, debug_message_receiver) = flume::bounded(1);
    let (tile_update_sender, tile_update_receiver) = flume::bounded(1);
    let (log_message_sender, log_message_receiver) = flume::unbounded();
    let (input_message_sender, input_message_receiver) = flume::unbounded();

    // Logger configurations
    let mut debug_logger =
        DebugLogger::new(LevelFilter::Debug, Config::default(), log_message_sender);
    debug_logger.add_filter_allow(format!("{}", "rgb"));
    let mut config = ConfigBuilder::new();
    config.add_filter_allow(format!("{}", "rgb"));
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, config.build(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create("warnings.log").unwrap(),
        ),
        debug_logger
        // WriteLogger::new(
        //     LevelFilter::Trace,
        //     Config::default(),
        //     File::create("trace.log").unwrap(),
        // ),
    ])
    .unwrap();

    let matches = App::new("rgb")
        .author("Siyuan Gao <rbnk@elica.input>")
        .arg(
            Arg::with_name("boot")
                .short("b")
                .required(false)
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

    let boot_rom = matches.value_of("boot").map(|path| get_boot_rom(path));
    let rom = get_rom(matches.value_of("rom").unwrap());

    let emulator_thread = start_emulator_thread(
        boot_rom,
        rom.clone(),
        input_message_receiver.clone(),
        framebuffer_sender.clone(),
        debug_message_sender.clone(),
        tile_update_sender.clone(),
    );
    let io_thread = start_io_thread(input_message_sender.clone());
    let display_thread = start_display_thread(
        matches.value_of("scale").unwrap().parse::<u32>().unwrap(),
        load_cartridge(rom.clone()).title(),
        input_message_sender.clone(),
        framebuffer_receiver.clone(),
        debug_message_receiver.clone(),
        log_message_receiver.clone(),
        tile_update_receiver.clone(),
    );
    let apu_thread = start_apu_thread();

    #[cfg(feature = "debug")]
    let debug_thread = start_debug_thread(debug_message_receiver.clone());

    emulator_thread.join().unwrap();
    io_thread.join().unwrap();
    apu_thread.join().unwrap();
    display_thread.join().unwrap();

    // Optional features
    #[cfg(feature = "debug")]
    debug_thread.join().unwrap();
}
