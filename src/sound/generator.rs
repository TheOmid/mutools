use super::frame::SterioFrame;

pub trait SignalGenerator {
    fn generate_frame(&self, index: usize) -> SterioFrame;
}

// Simple generators

pub struct SineWaveGenerator {}

impl SineWaveGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl SignalGenerator for SineWaveGenerator {
    fn generate_frame(&self, idx: usize) -> SterioFrame {
        let val = ((idx as f32) / 110.0).sin();
        SterioFrame::from_vals(val, val)
    }
}

