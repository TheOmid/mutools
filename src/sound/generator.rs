use super::frame::SterioFrame;

pub trait SignalGenerator {
    fn generate_frame(&self, index: usize) -> SterioFrame;
}

// Simple generators

pub struct SineWaveGenerator {
    frequency: f32,
    amplitude: f32,
    sample_rate: i32,
}

impl SineWaveGenerator {
    pub fn new(frequency: f32, amplitude: f32, sample_rate: i32) -> Self {
        Self {
            frequency,
            amplitude,
            sample_rate,
        }
    }

    pub fn compute_frame(&self, idx: usize) -> f32 {
        let sample_rate = self.sample_rate as f32;
        let i = idx as f32;
        self.amplitude * (i * (2.0 * std::f32::consts::PI / sample_rate * self.frequency)).sin()
    }
}

impl SignalGenerator for SineWaveGenerator {
    fn generate_frame(&self, idx: usize) -> SterioFrame {
        let val = self.compute_frame(idx);
        SterioFrame::from_vals(val, val)
    }
}

pub struct SquareWaveGenerator {
    frequency: f32,
    amplitude: f32,
    num_components: i32,
    sample_rate: i32,
}

impl SquareWaveGenerator {
    pub fn new(frequency: f32, amplitude: f32, num_components: i32, sample_rate: i32) -> Self {
        Self {
            frequency,
            amplitude,
            num_components,
            sample_rate,
        }
    }

    pub fn compute_frame(&self, idx: usize) -> f32 {
        let sample_rate = self.sample_rate as f32;
        let i = idx as f32;
        let mut res = 0.0;
        for n in 0..self.num_components {
            let b = (n * 2 + 1) as f32;
            let t = 2.0 * std::f32::consts::PI / sample_rate * self.frequency;
            res += self.amplitude * (4.0 / std::f32::consts::PI) * (1.0 / b) * (b * t * i).sin()
        }
        res
    }
}

impl SignalGenerator for SquareWaveGenerator {
    fn generate_frame(&self, idx: usize) -> SterioFrame {
        let val = self.compute_frame(idx);
        SterioFrame::from_vals(val, val)
    }
}

pub struct TriangleWaveGenerator {
    frequency: f32,
    amplitude: f32,
    num_components: i32,
    sample_rate: i32,
}

impl TriangleWaveGenerator {
    pub fn new(frequency: f32, amplitude: f32, num_components: i32, sample_rate: i32) -> Self {
        Self {
            frequency,
            amplitude,
            num_components,
            sample_rate,
        }
    }

    pub fn compute_frame(&self, idx: usize) -> f32 {
        let sample_rate = self.sample_rate as f32;
        let i = idx as f32;
        let mut res = 0.0;
        for n in 0..self.num_components {
            let b = (n + 1) as f32;
            let t = 2.0 * std::f32::consts::PI / sample_rate * self.frequency;
            res += (1.0 / std::f32::consts::PI) * (1.0 / b) * (b * t * i).sin()
        }
        self.amplitude * (1.0 / 2.0 - res)
    }
}

impl SignalGenerator for TriangleWaveGenerator {
    fn generate_frame(&self, idx: usize) -> SterioFrame {
        let val = self.compute_frame(idx);
        SterioFrame::from_vals(val, val)
    }
}
