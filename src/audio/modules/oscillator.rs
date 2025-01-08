use std::f32::consts::PI;
use crate::audio::{AudioProcessor, Port, PortType};
use std::sync::mpsc::{channel, Sender, Receiver};

pub enum OscillatorMessage {
    SetFrequency(f32),
    SetVolume(f32),
    SetPan(f32),  // -1.0 = full left, 0.0 = center, 1.0 = full right
}

#[derive(Clone, Copy)]
struct SmoothParameter {
    current: f32,
    target: f32,
    smoothing: f32,
}

impl SmoothParameter {
    fn new(initial: f32, smoothing: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            smoothing,
        }
    }

    fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    fn process(&mut self) {
        if self.current != self.target {
            self.current += (self.target - self.current) * self.smoothing;
            if (self.target - self.current).abs() < 0.0001 {
                self.current = self.target;
            }
        }
    }

    fn get(&self) -> f32 {
        self.current
    }
}

pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
    frequency: SmoothParameter,
    volume: SmoothParameter,
    pan: SmoothParameter,
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    receiver: Receiver<OscillatorMessage>,
    sender: Sender<OscillatorMessage>,
    // DC blocking filter state
    prev_input: f32,
    prev_output: f32,
}

impl Oscillator {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sample_rate: 44100.0,
            phase: 0.0,
            frequency: SmoothParameter::new(440.0, 0.005), // Increased smoothing
            volume: SmoothParameter::new(0.5, 0.01),      // Increased smoothing
            pan: SmoothParameter::new(0.0, 0.005),        // Increased smoothing
            inputs: vec![
                Port::new(PortType::CV),  // Frequency CV input
            ],
            outputs: vec![
                Port::new(PortType::Audio), // Left output
                Port::new(PortType::Audio), // Right output
            ],
            receiver,
            sender,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }

    pub fn get_sender(&self) -> Sender<OscillatorMessage> {
        self.sender.clone()
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency.set_target(freq);
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume.set_target(vol.clamp(0.0, 1.0));
    }

    pub fn set_pan(&mut self, pan: f32) {
        self.pan.set_target(pan.clamp(-1.0, 1.0));
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency.get()
    }

    pub fn get_volume(&self) -> f32 {
        self.volume.get()
    }

    pub fn get_pan(&self) -> f32 {
        self.pan.get()
    }

    fn dc_block(&mut self, input: f32) -> f32 {
        // Simple DC blocking filter
        const R: f32 = 0.995;
        let output = input - self.prev_input + R * self.prev_output;
        self.prev_input = input;
        self.prev_output = output;
        output
    }
}

impl AudioProcessor for Oscillator {
    fn init(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
    
    fn process(&mut self) {
        // Check for parameter updates
        while let Ok(msg) = self.receiver.try_recv() {
            match msg {
                OscillatorMessage::SetFrequency(freq) => self.set_frequency(freq),
                OscillatorMessage::SetVolume(vol) => self.set_volume(vol),
                OscillatorMessage::SetPan(pan) => self.set_pan(pan),
            }
        }

        // Update smoothed parameters
        self.frequency.process();
        self.volume.process();
        self.pan.process();

        // Generate sine wave
        let value = (self.phase * 2.0 * PI).sin() * self.volume.get();
        
        // Apply DC blocking filter
        let value = self.dc_block(value);
        
        // Update phase
        self.phase += self.frequency.get() / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            #[cfg(debug_assertions)]
            println!("Oscillator cycle - Freq: {} Hz", self.frequency.get());
        }
        
        // Calculate left and right channel volumes using equal power panning
        let pan_rad = (self.pan.get() + 1.0) * PI / 4.0;
        let left_gain = (PI/2.0 - pan_rad).cos();
        let right_gain = pan_rad.cos();
        
        // Set stereo outputs
        if let Some(left_output) = self.outputs.get_mut(0) {
            left_output.set_value(value * left_gain);
        }
        if let Some(right_output) = self.outputs.get_mut(1) {
            right_output.set_value(value * right_gain);
        }
    }
    
    fn get_input(&self, index: usize) -> Option<&Port> {
        self.inputs.get(index)
    }
    
    fn get_input_mut(&mut self, index: usize) -> Option<&mut Port> {
        self.inputs.get_mut(index)
    }
    
    fn get_output(&self, index: usize) -> Option<&Port> {
        self.outputs.get(index)
    }
    
    fn get_output_mut(&mut self, index: usize) -> Option<&mut Port> {
        self.outputs.get_mut(index)
    }
    
    fn num_inputs(&self) -> usize {
        self.inputs.len()
    }
    
    fn num_outputs(&self) -> usize {
        self.outputs.len()
    }
    
    fn get_input_type(&self, index: usize) -> Option<PortType> {
        self.inputs.get(index).map(|p| p.port_type)
    }
    
    fn get_output_type(&self, index: usize) -> Option<PortType> {
        self.outputs.get(index).map(|p| p.port_type)
    }
    
    fn reset(&mut self) {
        self.phase = 0.0;
        self.prev_input = 0.0;
        self.prev_output = 0.0;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
} 