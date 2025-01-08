#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortType {
    Audio,
    CV,
    Gate,
    Trigger,
}

pub struct Port {
    pub port_type: PortType,
    pub value: f32,
    pub connected_to: Option<usize>, // ID of connected port
}

impl Port {
    pub fn new(port_type: PortType) -> Self {
        Self {
            port_type,
            value: 0.0,
            connected_to: None,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn connect(&mut self, port_id: usize) {
        self.connected_to = Some(port_id);
    }

    pub fn disconnect(&mut self) {
        self.connected_to = None;
    }

    pub fn is_connected(&self) -> bool {
        self.connected_to.is_some()
    }
} 