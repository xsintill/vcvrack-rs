use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl Waveform {
    pub fn generate_sample(&self, phase: f32) -> f32 {
        match self {
            Waveform::Sine => (phase * 2.0 * PI).sin(),
            Waveform::Square => if phase < 0.5 { 1.0 } else { -1.0 },
            Waveform::Sawtooth => 2.0 * phase - 1.0,
            Waveform::Triangle => {
                if phase < 0.5 {
                    4.0 * phase - 1.0
                } else {
                    3.0 - 4.0 * phase
                }
            }
        }
    }
}
