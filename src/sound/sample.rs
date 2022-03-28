use dasp::sample::conv::f32::to_i32;
use dasp::sample::conv::i32::to_f32;
use dasp::sample::{FromSample, Sample, ToSample};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct AudioSample {
    _val: f32,
}

// Creates an audio sample from anything that can be converted to f32
impl<S: Into<f32>> From<S> for AudioSample {
    fn from(sample: S) -> Self {
        Self {
            _val: sample.into()
        }
    }
}

impl FromSample<AudioSample> for f32 {
    fn from_sample_(s: AudioSample) -> Self {
        s._val
    }
}

impl FromSample<AudioSample> for i32 {
    fn from_sample_(s: AudioSample) -> Self {
        to_i32(s._val)
    }
}

impl FromSample<f32> for AudioSample {
    fn from_sample_(_val: f32) -> Self {
        AudioSample {
            _val
        }
    }
}

impl FromSample<i32> for AudioSample {
    fn from_sample_(_val: i32) -> Self {
        AudioSample {
            _val: to_f32(_val)
        }
    }
}

const EQUILIBRIUM_VAL : f32 = 0.0;
impl Sample for AudioSample {
    type Signed = i32;
    type Float = f32;
    const EQUILIBRIUM: Self = AudioSample { _val: EQUILIBRIUM_VAL };
}


