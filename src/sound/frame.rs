use std::{slice::SliceIndex, slice::Iter, borrow::BorrowMut};

use super::sample::*;
use dasp::frame::*;

#[derive(Copy, Clone, PartialEq)]
pub struct MonoFrame {
    val: [i32; 1],
}

impl MonoFrame {
    pub fn new() -> Self {
        Self::EQUILIBRIUM
    }
}

impl Frame for MonoFrame {
    type Sample = i32;
    type NumChannels = N1;
    type Channels = Iterator<Item = i32>;
    type Signed = i32;
    type Float = f32;

    const EQUILIBRIUM: Self = MonoFrame { val: [0] };
    const CHANNELS: usize = 1;

    fn from_fn<F>(from: F) -> Self
    where
        F: FnMut(usize) -> Self::Sample,
    {
        MonoFrame { val: [from(0)] }
    }

    fn from_samples<I>(samples: &mut I) -> Option<Self>
    where
        I: Iterator<Item = Self::Sample>,
    {
        Some(MonoFrame {
            val: [{
                match samples.next() {
                    Some(x) => x,
                    _ => 0,
                }
            }],
        })
    }

    fn channels(self) -> Self::Channels {
        self.val.iter()
    }

    fn channel(&self, idx: usize) -> Option<&Self::Sample> {
        self.val.get(idx)
    }

    unsafe fn channel_unchecked(&self, idx: usize) -> &Self::Sample {
        self.val.get(idx).unwrap()
    }

    fn map<F, M>(self, map: M) -> F
    where
        F: Frame<NumChannels = Self::NumChannels>,
        M: FnMut(Self::Sample) -> <F as Frame>::Sample,
    {
        let arr : [<F as Frame>::Sample; 1] = [map(self.val[0])];
        F::from_samples(&mut arr.into_iter()).unwrap()
    }

    fn zip_map<O, F, M>(self, other: O, zip_map: M) -> F
    where
        F: Frame<NumChannels = Self::NumChannels>,
        M: FnMut(Self::Sample, <O as Frame>::Sample) -> <F as Frame>::Sample,
        O: Frame<NumChannels = Self::NumChannels>,
    {
        let arr : [<F as Frame>::Sample; 1] = [zip_map(self.val[0], *other.channel_unchecked(0))];
        F::from_samples(&mut arr.into_iter()).unwrap()
    }

    fn to_signed_frame(self) -> Self::Signed {
        Self::Signed::from(self.val[0])
    }

    fn to_float_frame(self) -> Self::Float {
        0.0
    }
}
