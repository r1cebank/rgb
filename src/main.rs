extern crate clap;
#[macro_use]
extern crate log;

pub mod cartridge;
pub mod cpu;
pub mod debug;
pub mod dmg01;
pub mod memory;
pub mod ppu;
pub mod timer;

use clap::{App, Arg};
use ppu::{FB_H, FB_W};

#[cfg(feature = "repl")]
use debug::{setup_repl, DebugResult};
#[cfg(feature = "tui")]
use debug::{setup_tui, DebugCommand, DebugResult};
use minifb::{Key, Window, WindowOptions};
use simplelog::*;
use std::fs::File;
use std::io::Read;
use std::process;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[cfg(feature = "audio")]
fn initialize_audio() {
    let device = cpal::default_output_device().unwrap();
    info!("Open the audio player: {}", device.name());
    let format = device.default_output_format().unwrap();
    let format = cpal::Format {
        channels: 2,
        sample_rate: format.sample_rate,
        data_type: cpal::SampleFormat::F32,
    };

    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);

    // Setup APU here

    std::thread::spawn(move || {
        event_loop.run(move |_, stream_data| {
            // Audio thread
        })
    });
}

#[cfg(feature = "gui")]
fn main() {
    let mut config = ConfigBuilder::new();
    config.add_filter_ignore(format!("{}", "rustyline"));
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, config.build(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create("warnings.log").unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("trace.log").unwrap(),
        ),
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

    // Initialize audio related
    if matches.is_present("audio") {
        info!("Audio is enabled");
        initialize_audio();
    }

    #[cfg(feature = "repl")]
    // repl consumer producer
    let (repl_sender, repl_receiver) = flume::unbounded();
    // repl results channel
    #[cfg(feature = "repl")]
    let (ui_sender, ui_receiver) = flume::unbounded();

    #[cfg(feature = "tui")]
    // repl consumer producer
    let (tui_sender, tui_receiver) = flume::unbounded();
    // repl results channel
    #[cfg(feature = "tui")]
    let (ui_sender, ui_receiver) = flume::unbounded();

    #[cfg(feature = "tui")]
    setup_tui(tui_sender, ui_receiver);
    #[cfg(feature = "repl")]
    setup_repl(repl_sender, ui_receiver);

    let boot_buffer = matches.value_of("boot").map(|path| buffer_from_file(path));
    let rom_buffer = matches.value_of("rom").map(|path| buffer_from_file(path));

    // let test_cpu = cpu::CPU::new(boot_buffer);
    let mut emulator = dmg01::Dmg01::new(boot_buffer, rom_buffer);

    let mut option = minifb::WindowOptions::default();
    option.resize = true;
    let scale_factor = matches.value_of("scale").unwrap().parse::<i32>().unwrap();
    option.scale = match scale_factor {
        1 => minifb::Scale::X1,
        2 => minifb::Scale::X2,
        4 => minifb::Scale::X4,
        8 => minifb::Scale::X8,
        _ => panic!("Supported scale: 1, 2, 4 or 8"),
    };
    info!("Scale factor is set to: {:?}", option.scale);

    let mut window = minifb::Window::new(
        format!("Gameboy - {}", emulator.mmu.borrow().cartridge.title).as_str(),
        FB_W,
        FB_H,
        option,
    )
    .unwrap();

    run(emulator, window);

    // let mut tiles_window = minifb::Window::new(
    //     format!("Tiles - {}", emulator.mmu.borrow().cartridge.title).as_str(),
    //     FB_W,
    //     FB_H,
    //     option,
    // )
    // .unwrap();

    // let mut window_buffer = vec![0x00; FB_W * FB_H];
    // window
    //     .update_with_buffer(window_buffer.as_slice(), FB_W, FB_H)
    //     .unwrap();

    // loop {
    //     // Get repl results
    //     #[cfg(feature = "repl")]
    //     {
    //         match repl_receiver.try_recv() {
    //             Ok(command) => match command.as_str() {
    //                 "r" => {
    //                     ui_sender
    //                         .send(DebugResult::Registers(emulator.cpu.registers))
    //                         .unwrap();
    //                 }
    //                 "pause" => {
    //                     emulator.pause();
    //                 }
    //                 "resume" => {
    //                     emulator.resume();
    //                 }
    //                 _ => {
    //                     ui_sender.send(DebugResult::NotACommand).unwrap();
    //                 }
    //             },
    //             Err(_) => {}
    //         }
    //     }
    //     #[cfg(feature = "tui")]
    //     {
    //         match tui_receiver.try_recv() {
    //             Ok(command) => match command {
    //                 DebugCommand::ShowRegister => {
    //                     ui_sender.send(DebugResult::Registers(emulator.cpu.registers));
    //                 }
    //             },
    //             Err(_) => {}
    //         }
    //     }
    //     // Stop the program, if the GUI is closed by the user
    //     if !window.is_open() {
    //         break;
    //     }
    //     if window.is_key_down(minifb::Key::Escape) {
    //         break;
    //     }

    //     emulator.cpu.tick();
    //     window
    //         .update_with_buffer(window_buffer.as_slice(), FB_W, FB_H)
    //         .unwrap();
    // }

    // process::exit(0x000);
}

const ONE_SECOND_IN_MICROS: usize = 1000000000;
const ONE_SECOND_IN_CYCLES: usize = 4190000;
const ONE_FRAME_IN_CYCLES: usize = 70224;
const NUMBER_OF_PIXELS: usize = 23040;

fn run(mut emulator: dmg01::Dmg01, mut window: Window) {
    let mut buffer = [0; NUMBER_OF_PIXELS];
    let mut cycles_elapsed_in_frame = 0usize;
    let mut now = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time_delta = now.elapsed().subsec_nanos();
        now = Instant::now();
        let delta = time_delta as f64 / ONE_SECOND_IN_MICROS as f64;
        let cycles_to_run = delta * ONE_SECOND_IN_CYCLES as f64;

        info!("running {} cycles", cycles_to_run);

        let mut cycles_elapsed = 0;
        while cycles_elapsed <= cycles_to_run as usize {
            cycles_elapsed += emulator.tick() as usize;
        }
        cycles_elapsed_in_frame += cycles_elapsed;

        // TODO: Consider updating buffer after every line is rendered.
        if cycles_elapsed_in_frame >= ONE_FRAME_IN_CYCLES {
            for (i, pixel) in emulator
                .mmu
                .borrow()
                .ppu
                .borrow()
                .framebuffer
                .chunks(4)
                .enumerate()
            {
                buffer[i] = (pixel[3] as u32) << 24
                    | (pixel[2] as u32) << 16
                    | (pixel[1] as u32) << 8
                    | (pixel[0] as u32)
            }
            window.update_with_buffer(&buffer, FB_W, FB_H).unwrap();
            cycles_elapsed_in_frame = 0;
        } else {
            sleep(Duration::from_nanos(2))
        }
    }
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}
