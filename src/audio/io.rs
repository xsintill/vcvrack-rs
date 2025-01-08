use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig, SampleFormat, BufferSize};
use std::sync::{Arc, Mutex};
use super::engine::AudioEngine;

pub struct AudioIO {
    stream: Option<Stream>,
    engine: Arc<Mutex<AudioEngine>>,
}

impl AudioIO {
    pub fn new(engine: AudioEngine) -> Self {
        Self {
            stream: None,
            engine: Arc::new(Mutex::new(engine)),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let host = cpal::default_host();
        
        let device = host.default_output_device()
            .ok_or_else(|| "No output device found".to_string())?;

        let mut supported_configs = device.supported_output_configs()
            .map_err(|e| format!("Error getting supported configs: {}", e))?;

        let supported_config = supported_configs.find(|config| {
            config.channels() == 2 && config.sample_format() == SampleFormat::F32
        }).ok_or_else(|| "No suitable audio config found".to_string())?;

        let mut config = supported_config.with_max_sample_rate().config();
        
        // Set a smaller buffer size for lower latency
        config.buffer_size = BufferSize::Fixed(1024);  // Or try 256 for even lower latency
        
        let sample_rate = config.sample_rate.0 as f32;
        
        // Update engine's sample rate
        if let Ok(mut engine) = self.engine.lock() {
            engine.set_sample_rate(sample_rate);
            engine.start();
        }

        let engine = Arc::clone(&self.engine);

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Process audio in chunks of stereo samples
                if let Ok(mut engine) = engine.lock() {
                    for chunk in data.chunks_mut(2) {
                        if chunk.len() >= 2 {
                            let (left, right) = engine.process();
                            chunk[0] = left;
                            chunk[1] = right;
                        }
                    }
                }
            },
            move |err| eprintln!("Audio output error: {}", err),
            None
        ).map_err(|e| format!("Stream build error: {}", e))?;

        stream.play().map_err(|e| format!("Stream play error: {}", e))?;
        
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Ok(mut engine) = self.engine.lock() {
            engine.stop();
        }
        self.stream.take();
    }

    pub fn is_active(&self) -> bool {
        self.stream.is_some()
    }

    pub fn get_engine(&self) -> std::sync::LockResult<std::sync::MutexGuard<'_, AudioEngine>> {
        self.engine.lock()
    }
} 