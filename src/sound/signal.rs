use dasp::frame::*;
use dasp_signal::Signal;

use super::frame::*;

#[derive(Clone)]
pub struct MonoSignal {
    frames: Vec<MonoFrame>
}

impl Signal for MonoSignal {

    type Frame = MonoFrame;
    fn next(&mut self) -> Self::Frame {
        let v = self.frames.clone().into_iter().next().unwrap_or(Self::Frame::EQUILIBRIUM);
        v
    }

}

