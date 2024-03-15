#![forbid(unsafe_code)]

mod chip8;

use chip8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};
use clap::Parser;
use error_iter::ErrorIter as _;
use game_loop::game_loop;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

const WIDTH: u32 = SCREEN_WIDTH as u32;
const HEIGHT: u32 = SCREEN_HEIGHT as u32;
const SCALE: f64 = 12.0;
const UPS: u32 = 500;
const MAX_FRAME_TIME: f64 = 0.1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ROM file
    #[arg(required = true, index = 1)]
    rom: String,

    /// Enable debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<(), Error> {
    // Init logging
    env_logger::init();

    // Init winit and pixels
    let event_loop = EventLoop::new();
    let window = build_window(&event_loop);
    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    // Parse args
    let args = Args::parse();
    let rom_path = args.rom;
    let debug = args.debug;

    // Load ROM
    let mut chip8 = Chip8::new(pixels);
    chip8.load_file(&rom_path).unwrap();

    type Game = game_loop::GameLoop<Chip8, game_loop::Time, Arc<winit::window::Window>>;
    type GameEvent<'a> = winit::event::Event<'a, ()>;
    let update = move |g: &mut Game| g.game.run_cycle(debug);
    let render = |g: &mut Game| {
        g.game.render();
        if let Err(err) = g.game.pixels.render() {
            log_error("pixels.render", err);
            g.exit();
        }
    };
    let handle_events = |g: &mut Game, event: &GameEvent| {
        if !g.game.input.update(event) {
            return;
        }
        g.game.update_controls();
        if g.game.should_close() {
            g.exit();
            return;
        }
        // Resize the window
        if let Some(size) = g.game.input.window_resized() {
            if let Err(err) = g.game.pixels.resize_surface(size.width, size.height) {
                log_error("pixels.resize_surface", err);
                g.exit();
            }
        }
    };

    game_loop(
        event_loop,
        Arc::new(window),
        chip8,
        UPS,
        MAX_FRAME_TIME,
        update,
        render,
        handle_events,
    );
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn build_window(event_loop: &EventLoop<()>) -> Window {
    let logical_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    let scaled_size: PhysicalSize<f64> = logical_size.to_physical(SCALE);
    WindowBuilder::new()
        .with_title("CHIP-8")
        .with_inner_size(scaled_size)
        .with_min_inner_size(scaled_size)
        .build(event_loop)
        .unwrap()
}
