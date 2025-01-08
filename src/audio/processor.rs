use super::port::{Port, PortType};
use std::any::Any;

pub trait AudioProcessor: Send + Any {
    /// Initialize the processor with the given sample rate
    fn init(&mut self, sample_rate: f32);
    
    /// Process one sample of audio
    fn process(&mut self);
    
    /// Get a reference to an input port by index
    fn get_input(&self, index: usize) -> Option<&Port>;
    
    /// Get a mutable reference to an input port by index
    fn get_input_mut(&mut self, index: usize) -> Option<&mut Port>;
    
    /// Get a reference to an output port by index
    fn get_output(&self, index: usize) -> Option<&Port>;
    
    /// Get a mutable reference to an output port by index
    fn get_output_mut(&mut self, index: usize) -> Option<&mut Port>;
    
    /// Get the number of input ports
    fn num_inputs(&self) -> usize;
    
    /// Get the number of output ports
    fn num_outputs(&self) -> usize;
    
    /// Get the type of an input port
    fn get_input_type(&self, index: usize) -> Option<PortType>;
    
    /// Get the type of an output port
    fn get_output_type(&self, index: usize) -> Option<PortType>;
    
    /// Reset the processor to its initial state
    fn reset(&mut self);
    
    /// Downcast to concrete type
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
} 