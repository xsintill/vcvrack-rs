pub mod engine;
pub mod processor;
pub mod port;
pub mod io;
pub mod modules;
pub mod waveform;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample};
use std::sync::{Arc, Mutex};
use waveform::Waveform;

pub use processor::AudioProcessor;
pub use port::{Port, PortType};
pub use io::AudioIO;
pub use modules::oscillator::Oscillator;

pub struct AudioEngine {
    stream: Option<cpal::Stream>,
    phase: Arc<Mutex<f32>>,
    waveform: Arc<Mutex<Waveform>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            stream: None,
            phase: Arc::new(Mutex::new(0.0)),
            waveform: Arc::new(Mutex::new(Waveform::Sine)),
        }
    }

    pub fn set_waveform(&self, waveform: Waveform) {
        if let Ok(mut w) = self.waveform.lock() {
            *w = waveform;
        }
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| anyhow::Error::msg("No output device available"))?;
        let config = device.default_output_config()?;

        println!("Default output config: {:?}", config);

        let phase = Arc::clone(&self.phase);
        let waveform = Arc::clone(&self.waveform);

        let stream = match config.sample_format() {
            SampleFormat::F32 => self.build_stream::<f32>(&device, &config.into(), phase, waveform)?,
            SampleFormat::I16 => self.build_stream::<i16>(&device, &config.into(), phase, waveform)?,
            SampleFormat::U16 => self.build_stream::<u16>(&device, &config.into(), phase, waveform)?,
            sample_format => return Err(anyhow::Error::msg(format!("Unsupported sample format '{sample_format}'"))),
        };

        stream.play()?;
        self.stream = Some(stream);

        Ok(())
    }

    fn build_stream<T>(
        &self,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        phase: Arc<Mutex<f32>>,
        waveform: Arc<Mutex<Waveform>>,
    ) -> Result<cpal::Stream, anyhow::Error>
    where
        T: Sample + SizedSample + FromSample<f32>,
    {
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;
        let freq = 440.0;

        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let mut phase = phase.lock().unwrap();
                    let waveform = waveform.lock().unwrap();
                    
                    let value = waveform.generate_sample(*phase);
                    
                    // Advance phase
                    *phase += freq / sample_rate;
                    if *phase >= 1.0 {
                        *phase -= 1.0;
                    }

                    // Convert f32 sample to target format and write to all channels
                    let value = T::from_sample(value);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            |err| eprintln!("Error in audio stream: {}", err),
            None,
        )?;

        Ok(stream)
    }

    pub fn stop(&mut self) {
        self.stream = None;
    }
}