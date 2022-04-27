use dasp::frame::*;
use dasp_signal::Signal;

use super::frame::*;

#[derive(Clone)]
pub struct SterioSignal {
    frames: Vec<SterioFrame>
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

