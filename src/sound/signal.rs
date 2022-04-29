use dasp::frame::*;
use dasp_signal::Signal;

use super::frame::*;

#[derive(Clone)]
pub struct SterioSignal {
    frames: Vec<SterioFrame>,
}

impl SterioSignal {
    pub fn new() -> SterioSignal {
        SterioSignal {
            frames: Vec::new()
        }
    }

    pub fn get_raw_frame(&self, idx: usize) -> Option<&SterioFrame> {
        self.frames.get(idx).clone()
    }
}

impl Signal for SterioSignal {
    type Frame = SterioFrame;
    fn next(&mut self) -> Self::Frame {
        self.frames
            .clone()
            .into_iter()
            .next()
            .unwrap_or(Self::Frame::EQUILIBRIUM)
    }
}
