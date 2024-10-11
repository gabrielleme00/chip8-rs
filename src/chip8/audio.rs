use cpal::{
    traits::{DeviceTrait, HostTrait},
    BuildStreamError, Stream,
};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

const VOLUME: f32 = 0.05;
const WAVE_FREQUENCY: f32 = 440.0;

pub struct Audio {
    _stream: cpal::Stream,
    active: Arc<Mutex<bool>>,
}

impl Audio {
    pub fn new() -> Self {
        // Initialize audio config
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("[Audio] Failed to get default output device");
        let config = device
            .default_output_config()
            .expect("[Audio] Failed to get default output config");

        let active = Arc::new(Mutex::new(true));

        // Start audio stream
        let _stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                Self::create_stream::<f32>(&device, &config.into(), Arc::clone(&active))
            }
            _ => panic!("[Audio] Unsupported sample format"),
        }
        .expect("[Audio] Failed to build output audio stream");

        Self { _stream, active }
    }

    pub fn set_active(&self, active: bool) {
        let mut active_lock = self.active.lock().unwrap();
        *active_lock = active;
    }

    fn create_stream<T>(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        active: Arc<Mutex<bool>>,
    ) -> Result<Stream, BuildStreamError> {
        let sample_rate = config.sample_rate.0 as f32;
        let mut phase = 0.0;

        device.build_output_stream(
            config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Generate a sinusoidal wave of maximum amplitude
                let active = *active.lock().unwrap();
                let omega = 2.0 * PI * WAVE_FREQUENCY / sample_rate;
                for sample in data.iter_mut() {
                    *sample = if active {
                        VOLUME * (omega * phase).sin()
                    } else {
                        0.0
                    };
                    phase = (phase + 0.5) % sample_rate;
                }
            },
            |err| eprintln!("[Audio] Error occurred on output audio stream: {}", err),
            None,
        )
    }
}
