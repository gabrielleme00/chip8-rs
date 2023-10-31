#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod chip8;

use std::sync::Arc;

use chip8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};
use error_iter::ErrorIter as _;
use game_loop::game_loop;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const WIDTH: u32 = SCREEN_WIDTH as u32;
const HEIGHT: u32 = SCREEN_HEIGHT as u32;
const SCALE: f64 = 12.0;

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

    let window = Arc::new(window);
    game_loop(
        event_loop,
        window,
        chip8,
        600,
        0.1,
        move |g| {
            // Update
            g.game.run_cycle();
        },
        move |g| {
            // Render
            g.game.render();
            if let Err(err) = g.game.pixels.render() {
                log_error("pixels.render", err);
                g.exit();
            }
        },
        |g, event| {
            // Events
            if g.game.input.update(event) {
                // Update controls
                g.game.update_input();
                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape)
                    || g.game.input.close_requested()
                {
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
            }
        },
    );
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
