use std::collections::HashMap;
use super::processor::AudioProcessor;

pub struct AudioEngine {
    sample_rate: f32,
    pub processors: HashMap<usize, Box<dyn AudioProcessor>>,
    next_processor_id: usize,
    running: bool,
}

impl AudioEngine {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            processors: HashMap::new(),
            next_processor_id: 0,
            running: false,
        }
    }

    /// Add a new processor to the engine
    pub fn add_processor(&mut self, mut processor: Box<dyn AudioProcessor>) -> usize {
        let id = self.next_processor_id;
        processor.init(self.sample_rate);
        self.processors.insert(id, processor);
        self.next_processor_id += 1;
        id
    }

    /// Remove a processor from the engine
    pub fn remove_processor(&mut self, id: usize) -> Option<Box<dyn AudioProcessor>> {
        self.processors.remove(&id)
    }

    /// Connect an output port of one processor to an input port of another
    pub fn connect_ports(
        &mut self,
        source_id: usize,
        source_port: usize,
        dest_id: usize,
        dest_port: usize,
    ) -> Result<(), String> {
        if source_id == dest_id {
            return Err("Cannot connect a processor to itself".to_string());
        }

        // Verify ports exist and are compatible
        let source_type = self.processors.get(&source_id)
            .ok_or_else(|| "Source processor not found".to_string())?
            .get_output_type(source_port)
            .ok_or_else(|| "Source port not found".to_string())?;

        let dest_type = self.processors.get(&dest_id)
            .ok_or_else(|| "Destination processor not found".to_string())?
            .get_input_type(dest_port)
            .ok_or_else(|| "Destination port not found".to_string())?;

        if source_type != dest_type {
            return Err("Port types are incompatible".to_string());
        }

        // Now we can safely get mutable access to both processors
        if let Some(source) = self.processors.get_mut(&source_id) {
            if let Some(source_port) = source.get_output_mut(source_port) {
                source_port.connect(dest_id);
            }
        }

        if let Some(dest) = self.processors.get_mut(&dest_id) {
            if let Some(dest_port) = dest.get_input_mut(dest_port) {
                dest_port.connect(source_id);
            }
        }

        Ok(())
    }

    /// Disconnect two ports
    pub fn disconnect_ports(
        &mut self,
        source_id: usize,
        source_port: usize,
        dest_id: usize,
        dest_port: usize,
    ) -> Result<(), String> {
        if let Some(source) = self.processors.get_mut(&source_id) {
            if let Some(source_port) = source.get_output_mut(source_port) {
                source_port.disconnect();
            }
        }

        if let Some(dest) = self.processors.get_mut(&dest_id) {
            if let Some(dest_port) = dest.get_input_mut(dest_port) {
                dest_port.disconnect();
            }
        }

        Ok(())
    }

    /// Process one sample of audio through all processors and return stereo output
    pub fn process(&mut self) -> (f32, f32) {
        if !self.running {
            return (0.0, 0.0);
        }

        // Process each processor in order
        for processor in self.processors.values_mut() {
            processor.process();
        }

        // Get output from the last processor (assuming it's our oscillator for now)
        if let Some(processor) = self.processors.values().next() {
            let left = processor.get_output(0)
                .map(|p| p.get_value())
                .unwrap_or(0.0);
            let right = processor.get_output(1)
                .map(|p| p.get_value())
                .unwrap_or(0.0);
            return (left, right);
        }

        (0.0, 0.0)
    }

    /// Start audio processing
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop audio processing
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Reset all processors
    pub fn reset(&mut self) {
        for processor in self.processors.values_mut() {
            processor.reset();
        }
    }

    /// Get the current sample rate
    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Set a new sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        // Reinitialize all processors with new sample rate
        for processor in self.processors.values_mut() {
            processor.init(sample_rate);
        }
    }
} 