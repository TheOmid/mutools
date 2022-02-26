use dasp::frame::*;
use dasp::sample::*;

use super::sample::*;

use std::array;

#[derive(Copy, Clone, PartialEq)]
pub struct MonoFrame {
    sample: RawSample
}


impl Frame for MonoFrame {

    type Sample = RawSample;
    type NumChannels = N1;
    type Channels = array::IntoIter<RawSample,1>;
    type Signed = i32;
    type Float = f32;

    const EQUILIBRIUM : Self = MonoFrame { sample: RawSample::EQUILIBRIUM };
    const CHANNELS : usize = 1;

    fn from_fn<F>(from: F) -> Self
        where F: FnMut(usize) -> Self::Sample {
        MonoFrame {
            sample: from(0)
        }
    }

    fn from_samples<I>(samples: &mut I) -> Option<MonoFrame> where I: Iterator<Item = Self::Sample> {
            Some(MonoFrame {
                sample: samples.next().unwrap_or(RawSample::EQUILIBRIUM)
            })
    }

    fn channels(self) -> Self::Channels {
        array::IntoIter::<RawSample, 1>::new([self.sample])
    }

    fn channel(&self, idx: usize) -> Option<&Self::Sample> {
        match idx {
            0 => Some(&self.sample),
            _ => None
        }
    }

    unsafe fn

}

