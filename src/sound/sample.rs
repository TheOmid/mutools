use dasp::sample::{Sample, FromSample, ToSample};
use dasp::sample::conv::f32::to_i32;
use dasp::sample::conv::i32::to_f32;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct RawSample {
    _val: f32,
}

impl FromSample<RawSample> for f32 {
    fn from_sample_(s: RawSample) -> Self {
        s._val
    }
}

impl FromSample<RawSample> for i32 {
    fn from_sample_(s: RawSample) -> Self {
        to_i32(s._val)
    }
}

impl FromSample<f32> for RawSample {
    fn from_sample_(_val: f32) -> Self {
        RawSample {
            _val
        }
    }
}

impl FromSample<i32> for RawSample {
    fn from_sample_(_val: i32) -> Self {
        RawSample {
            _val: to_f32(_val)
        }
    }
}

const EQUILIBRIUM_VAL : f32 = 0.0;
impl Sample for RawSample {
    type Signed = i32;
    type Float = f32;
    const EQUILIBRIUM: Self = RawSample { _val: EQUILIBRIUM_VAL };
}
