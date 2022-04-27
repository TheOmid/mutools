
use std::{slice::SliceIndex, slice::Iter, borrow::BorrowMut, iter::FromIterator};
use dasp::*;

use super::sample::*;

#[derive(Copy, Clone, PartialEq)]
pub struct SterioFrame {
    pub data: [f32; 2],
}

impl SterioFrame {
    pub fn new() -> Self {
        Self::EQUILIBRIUM
    }

    pub fn get_channel(&self, idx: usize) -> Option<&f32> {
        if idx <= 1 {
            Some(&self.data[idx])
        } else {
            None
        }
    }

    pub fn get_channel_val(&self, idx: usize) -> Option<f32> {
        if idx <= 1 {
            Some(self.data[idx])
        } else {
            None
        }
    }
}

impl IntoIterator for SterioFrame {
    type Item = f32;
    type IntoIter = SterioFrameIterator;

    fn into_iter(self) -> Self::IntoIter {
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
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.frame.get_channel_val(self.index-1)
    }
}

impl dasp::frame::Frame for SterioFrame {
    type Sample = f32;
    type NumChannels = dasp::frame::N2;
    type Channels = SterioFrameIterator;
    type Signed = SterioFrame;
    type Float = SterioFrame;

    const EQUILIBRIUM: Self = SterioFrame {
        data: [0.0, 0.0]
    };
    const CHANNELS: usize = 1;

    fn from_fn<F>(from: F) -> Self
    where
        F: FnMut(usize) -> Self::Sample,
    {
        SterioFrame {
            data: {
                let tmp: &mut [f32; 2] = &mut Self::EQUILIBRIUM.data.clone();
                let f = &mut F::from(from);
                for i in 0..2 {
                    tmp[i] = f(i)
                }
                *tmp
            }
        }
    }

    fn from_samples<I>(samples: &mut I) -> Option<Self>
    where
        I: Iterator<Item = Self::Sample>,
    {
        Some(Self {
            data: [samples.next().unwrap_or(0.0), samples.next().unwrap_or(0.0)],
        })
    }

    fn channels(self) -> Self::Channels {
        self.into_iter()
    }

    fn channel(&self, idx: usize) -> Option<&Self::Sample> {
        self.get_channel(idx)
    }

    unsafe fn channel_unchecked(&self, idx: usize) -> &Self::Sample {
        self.get_channel(idx).unwrap()
    }

    fn map<F, M>(self, map_fn: M) -> F
    where
        F: Frame<NumChannels = Self::NumChannels>,
        M: FnMut(Self::Sample) -> <F as Frame>::Sample,
    {
        F::from_samples(&mut self.channels()
                            .map(map_fn)).unwrap_or(F::EQUILIBRIUM)
    }

    fn zip_map<O, F, M>(self, other: O, zip_map_fn: M) -> F
    where
        F: Frame<NumChannels = Self::NumChannels>,
        M: FnMut(Self::Sample, <O as Frame>::Sample) -> <F as Frame>::Sample,
        O: Frame<NumChannels = Self::NumChannels>,
    {
        let f = &mut M::from(zip_map_fn);
        let samples = &mut self.channels()
                        .zip(other.channels())
                        .map(|(a, b)| { f(a, b) });
        F::from_samples(samples).unwrap_or(F::EQUILIBRIUM)
    }

    fn to_signed_frame(self) -> Self::Signed {
        Self::Signed::from(self)
    }

    fn to_float_frame(self) -> Self::Float {
        Self::Float::from(self)
    }
}
