#![forbid(unsafe_code)]

mod chip8;

use chip8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};
use error_iter::ErrorIter as _;
use game_loop::game_loop;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const WIDTH: u32 = SCREEN_WIDTH as u32;
const HEIGHT: u32 = SCREEN_HEIGHT as u32;
const SCALE: f64 = 12.0;
const UPS: u32 = 500;
const MAX_FRAME_TIME: f64 = 0.1;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let logical_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size: PhysicalSize<f64> = logical_size.to_physical(SCALE);
        WindowBuilder::new()
            .with_title("CHIP-8")
            .with_inner_size(scaled_size)
            .with_min_inner_size(scaled_size)
            .build(&event_loop)
            .unwrap()
    };
    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut chip8 = Chip8::new(pixels);
    chip8.load_file("roms/particle.ch8").unwrap();

    type Game = game_loop::GameLoop<Chip8, game_loop::Time, Arc<winit::window::Window>>;
    type GameEvent<'a> = winit::event::Event<'a, ()>;
    let update = |g: &mut Game| g.game.run_cycle();
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
