use std::{slice::SliceIndex, slice::Iter, borrow::BorrowMut};
use dasp::*;

use super::sample::*;

#[derive(Copy, Clone, PartialEq)]
pub struct SterioFrame {
    pub data: [AudioSample; 2],
}

impl IntoIterator for SterioFrame {
    type Item = AudioSample;
    type IntoIter = SterioFrameIterator;

    fn into_iter(self) -> Self::IntoIterator {
        SterioFrameIterator {
            index: 0,
            frame: self
        }
    }
}

pub struct SterioFrameIterator {
    index: usize,
    frame: SterioFrame
}

impl Iterator for SterioFrameIterator {
    type Item = AudioSample;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 1 {
            None
        } else {
            self.frame.val[self.index]
        }
    }
}

impl SterioFrame {
    pub fn new() -> Self {
        Self::EQUILIBRIUM
    }
}

impl dasp::frame::Frame for SterioFrame {
    type Sample = AudioSample;
    type NumChannels = dasp::frame::N2;
    type Channels = SterioFrameIterator;
    type Signed = SterioFrame;
    type Float = SterioFrame;

    const EQUILIBRIUM: Self = SterioFrame { frame: [0, 0] };
    const CHANNELS: usize = 1;

    fn from_fn<F>(from: F) -> Self
    where
        F: FnMut(usize) -> Self::Sample,
    {
        SterioFrame {
            data: {
                let tmp: [AudioSample; 2];
                for i in 0..2 {
                    tmp[i] = from(i)
                }
                tmp
            }
        }
    }

    fn from_samples<I>(samples: &mut I) -> Option<Self>
    where
        I: Iterator<Item = Self::Sample>,
    {
        Some(SterioFrame {
            data: samples.data.copy()
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
        Self::Signed::from(self)
    }

    fn to_float_frame(self) -> Self::Float {
        Self::Float::from(self)
    }
}
