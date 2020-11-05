use gfx::Device;
use glutin::{Event, WindowEvent};
use imgui::*;
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_gfx_renderer::{Renderer, Shaders};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

use crate::cartridge::CartridgeType;
use crate::memory::Memory;

use crate::dmg01::dmg01;

type ColorFormat = gfx::format::Rgba8;

pub struct System {
    pub events_loop: glutin::EventsLoop,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub render_sys: RenderSystem,
    pub font_size: f32,
}

pub fn init(title: &str) -> System {
    let title = match title.rfind('/') {
        Some(idx) => title.split_at(idx + 1).1,
        None => title,
    };
    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title(title.to_owned())
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../res/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let render_sys = RenderSystem::init(&mut imgui, builder, &events_loop);
    platform.attach_window(imgui.io_mut(), render_sys.window(), HiDpiMode::Rounded);
    System {
        events_loop,
        imgui,
        platform,
        render_sys,
        font_size,
    }
}

impl System {
    pub fn main_loop(self, emulator: &mut dmg01) {
        let System {
            mut events_loop,
            mut imgui,
            mut platform,
            mut render_sys,
            ..
        } = self;
        let mut encoder: gfx::Encoder<_, _> = render_sys.factory.create_command_buffer().into();

        let mut last_frame = Instant::now();
        let mut run = true;

        let mut address_value: String = String::from("0");
        let mut address = ImString::new("0");
        let mut boot_rom_toggle: bool = false;

        while run {
            emulator.tick();
            events_loop.poll_events(|event| {
                platform.handle_event(imgui.io_mut(), render_sys.window(), &event);

                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::Resized(size) => render_sys.update_views(size),
                        WindowEvent::CloseRequested => run = false,
                        _ => (),
                    }
                }
            });

            let io = imgui.io_mut();
            platform
                .prepare_frame(io, render_sys.window())
                .expect("Failed to start frame");
            let now = Instant::now();
            io.update_delta_time(now - last_frame);
            last_frame = now;
            let ui = imgui.frame();
            // Run the UI here
            {
                Window::new(&im_str!(
                    "cartridge: [{}]",
                    emulator.mmu.borrow().cartridge.title
                ))
                .size([700.0, 160.0], Condition::FirstUseEver)
                .position([10.0, 10.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!(
                        "cartridge_type: {:?}",
                        emulator.mmu.borrow().cartridge.cartridge_type
                    ));
                    ui.text(im_str!(
                        "cartridge_rom_size: {:?}",
                        emulator.mmu.borrow().cartridge.cartridge_rom_size
                    ));
                    ui.text(im_str!(
                        "cartridge_ram_size: {:?}",
                        emulator.mmu.borrow().cartridge.cartridge_ram_size
                    ));
                    ui.separator();
                    let mbc_state = &emulator.mmu.borrow().cartridge.mbc_state;
                    match emulator.mmu.borrow().cartridge.cartridge_type {
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
                    emulator.cpu.frequency,
                    emulator.cpu.speed,
                    emulator.is_paused()
                ))
                .size([700.0, 160.0], Condition::FirstUseEver)
                .position([10.0, 180.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!(
                        "cartridge_type: {:?}",
                        emulator.mmu.borrow().cartridge.cartridge_type
                    ));
                    ui.text(im_str!(
                        "cycle_time： {}， wait_time: {}, cycle_duration: {}ms",
                        emulator.cpu.cycle_time,
                        emulator.cpu.wait_time,
                        emulator.cpu.cycle_duration,
                    ));
                    ui.text(im_str!(
                        "registers: {}",
                        emulator.cpu.cpu.registers.get_register_overview()
                    ));
                    ui.text(im_str!(
                        "16 bit registers: {}",
                        emulator.cpu.cpu.registers.get_word_register_overview()
                    ));
                    ui.text(im_str!(
                        "flags: {}",
                        emulator.cpu.cpu.registers.get_flag_register_overview()
                    ));
                    ui.text(im_str!(
                        "last instruction: {:?}",
                        emulator.cpu.cpu.last_instruction
                    ));
                    if emulator.is_paused() {
                        if ui.button(im_str!("resume"), [100.0, 20.0]) {
                            emulator.resume();
                        }
                    } else {
                        if ui.button(im_str!("pause"), [100.0, 20.0]) {
                            emulator.pause();
                        }
                    }
                });
                Window::new(&im_str!(
                    "memory : {:?}",
                    emulator.mmu.borrow().cartridge.cartridge_ram_size
                ))
                .size([250.0, 160.0], Condition::FirstUseEver)
                .position([720.0, 10.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.input_text(im_str!("address"), &mut address).build();
                    ui.checkbox(im_str!("bootrom"), &mut boot_rom_toggle);
                    ui.popup(im_str!("overflow_popup"), || {
                        ui.text("address overflow");
                    });
                    let address =
                        u16::from_str_radix(address.to_str().trim_start_matches("0x"), 16)
                            .unwrap_or_default();
                    if ui.button(im_str!("lookup"), [100.0, 20.0]) {
                        if boot_rom_toggle {
                            if address > 0x100 {
                                ui.open_popup(im_str!("overflow_popup"));
                            } else {
                                address_value = format!("{:x}", emulator.mmu.borrow().get(address));
                            }
                        } else {
                            address_value = format!("{:x}", emulator.mmu.borrow().get_mem(address));
                        }
                    }
                    if ui.button(im_str!("lookup word"), [100.0, 20.0]) {
                        if boot_rom_toggle {
                            if address > 0x100 {
                                ui.open_popup(im_str!("overflow_popup"));
                            } else {
                                address_value =
                                    format!("{:x}", emulator.mmu.borrow().get_word(address));
                            }
                        } else {
                            address_value =
                                format!("{:x}", emulator.mmu.borrow().get_mem_word(address));
                        }
                    }
                    ui.text(im_str!("value: {}", address_value.to_uppercase()));
                    ui.text(im_str!("last_op: {}", emulator.mmu.borrow().last_op));
                });
            }

            if let Some(main_color) = render_sys.main_color.as_mut() {
                encoder.clear(main_color, [1.0, 1.0, 1.0, 1.0]);
            }
            platform.prepare_render(&ui, render_sys.window());
            let draw_data = ui.render();
            if let Some(main_color) = render_sys.main_color.as_mut() {
                render_sys
                    .renderer
                    .render(&mut render_sys.factory, &mut encoder, main_color, draw_data)
                    .expect("Rendering failed");
            }
            encoder.flush(&mut render_sys.device);
            render_sys.swap_buffers();
            render_sys.device.cleanup();
        }
    }
}

#[cfg(feature = "opengl")]
mod types {
    pub type Device = gfx_device_gl::Device;
    pub type Factory = gfx_device_gl::Factory;
    pub type Resources = gfx_device_gl::Resources;
}

#[cfg(feature = "opengl")]
pub struct RenderSystem {
    pub renderer: Renderer<ColorFormat, types::Resources>,
    pub windowed_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub device: types::Device,
    pub factory: types::Factory,
    pub main_color: Option<gfx::handle::RenderTargetView<types::Resources, ColorFormat>>,
    pub main_depth: gfx::handle::DepthStencilView<types::Resources, gfx::format::DepthStencil>,
}

#[cfg(feature = "opengl")]
impl RenderSystem {
    pub fn init(
        imgui: &mut Context,
        builder: glutin::WindowBuilder,
        events_loop: &glutin::EventsLoop,
    ) -> RenderSystem {
        {
            // Fix incorrect colors with sRGB framebuffer
            fn imgui_gamma_to_linear(col: [f32; 4]) -> [f32; 4] {
                let x = col[0].powf(2.2);
                let y = col[1].powf(2.2);
                let z = col[2].powf(2.2);
                let w = 1.0 - (1.0 - col[3]).powf(2.2);
                [x, y, z, w]
            }

            let style = imgui.style_mut();
            for col in 0..style.colors.len() {
                style.colors[col] = imgui_gamma_to_linear(style.colors[col]);
            }
        }

        let context = glutin::ContextBuilder::new().with_vsync(true);
        let (windowed_context, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init(builder, context, &events_loop)
                .expect("Failed to initialize graphics");
        let shaders = {
            let version = device.get_info().shading_language;
            if version.is_embedded {
                if version.major >= 3 {
                    Shaders::GlSlEs300
                } else {
                    Shaders::GlSlEs100
                }
            } else if version.major >= 4 {
                Shaders::GlSl400
            } else if version.major >= 3 {
                if version.minor >= 2 {
                    Shaders::GlSl150
                } else {
                    Shaders::GlSl130
                }
            } else {
                Shaders::GlSl110
            }
        };
        let renderer =
            Renderer::init(imgui, &mut factory, shaders).expect("Failed to initialize renderer");
        RenderSystem {
            renderer,
            windowed_context,
            device,
            factory,
            main_color: Some(main_color),
            main_depth,
        }
    }
    pub fn window(&self) -> &glutin::Window {
        self.windowed_context.window()
    }
    pub fn update_views(&mut self, _: glutin::dpi::LogicalSize) {
        if let Some(main_color) = self.main_color.as_mut() {
            gfx_window_glutin::update_views(
                &self.windowed_context,
                main_color,
                &mut self.main_depth,
            );
        }
    }
    pub fn swap_buffers(&mut self) {
        self.windowed_context.swap_buffers().unwrap();
    }
}

#[cfg(feature = "directx")]
mod types {
    pub type Device = gfx_device_dx11::Device;
    pub type Factory = gfx_device_dx11::Factory;
    pub type Resources = gfx_device_dx11::Resources;
}

#[cfg(feature = "directx")]
pub struct RenderSystem {
    pub renderer: Renderer<ColorFormat, types::Resources>,
    pub window: gfx_window_dxgi::Window,
    pub device: types::Device,
    pub factory: types::Factory,
    pub main_color: Option<gfx::handle::RenderTargetView<types::Resources, ColorFormat>>,
}

#[cfg(feature = "directx")]
impl RenderSystem {
    pub fn init(
        imgui: &mut Context,
        builder: glutin::WindowBuilder,
        events_loop: &glutin::EventsLoop,
    ) -> RenderSystem {
        let (window, device, mut factory, main_color) =
            gfx_window_dxgi::init(builder, &events_loop).expect("Failed to initialize graphics");
        let renderer = Renderer::init(imgui, &mut factory, Shaders::HlslSm40)
            .expect("Failed to initialize renderer");
        RenderSystem {
            renderer,
            window,
            device,
            factory,
            main_color: Some(main_color),
        }
    }
    pub fn window(&self) -> &glutin::Window {
        &self.window.inner
    }
    pub fn update_views(&mut self, size: glutin::dpi::LogicalSize) {
        let physical = size.to_physical(self.window().get_hidpi_factor());
        let (width, height): (u32, u32) = physical.into();
        let _ = self.main_color.take(); // we need to drop main_color before calling update_views
        self.main_color = Some(
            gfx_window_dxgi::update_views(
                &mut self.window,
                &mut self.factory,
                &mut self.device,
                width as u16,
                height as u16,
            )
            .expect("Failed to update resize"),
        );
    }
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers(1);
    }
}
