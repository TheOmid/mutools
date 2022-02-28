use dasp::frame::*;

use super::sample::*;
use std::array;

#[derive(Copy, Clone, PartialEq)]
pub struct AudioFrame<const NChannels: usize> {
    samples: [AudioSample; NChannels]
}

impl<N: NumChannels> Frame for AudioFrame<N> {
    type Sample = AudioSample;
    type NumChannels = N;
    type Channels = Channels<Self>;
    type Signed = i32;
    type Float = f32;

    const EQUILIBRIUM : Self = AudioFrame { samples: AudioSample::EQUILIBRIUM };
    const CHANNELS : usize = Self::N;

    fn from_fn<F>(mut from: F) -> Self where F: FnMut(usize) -> Self::Sample {
        let samples : [AudioSample; N];
        for i in Self::NumChannels {
            let v = (from)(i);
        }
        Self {
            samples: v
        }
    }

    fn from_samples<I: Iterator<Item = Self::Sample>>(samples: &mut I) -> Option<AudioFrame<Self::NumChannels>> {
        Some(AudioFrame::<Self::NumChannels> {
            samples: Vec::<AudioSample>::from_iter(samples)
        })
    }

    fn channels(self) -> Self::Channels {
        self.samples.into_iter()
    }

    fn channel(&self, idx: usize) -> Option<&Self::Sample> {
        match idx {
            0 => Some(&self.sample),
            _ => None
        }
    }

    unsafe fn channel_unchecked(&self, idx: usize) -> &Self::Sample {
        match idx {
            0 => &self.sample,
            _ => panic!("Out of bounds!")
        }
    }

    fn map<F, M>(self, mut map: M) -> F
        where F: Frame<NumChannels = Self::NumChannels>,
              M: FnMut(Self::Sample) -> <F as Frame>::Sample {
        let res = map(self.sample);
        F::from_samples(&mut vec![res].into_iter()).unwrap()
    }


    fn zip_map<O, F, M>(self, other: O, mut zip_map: M) -> F where
                F: Frame<NumChannels = Self::NumChannels>,
                M: FnMut(Self::Sample, <O as Frame>::Sample) -> <F as Frame>::Sample,
                O: Frame<NumChannels = Self::NumChannels> {

        let res = zip_map(self.sample, *other.channel(0).unwrap_or(&O::Sample::EQUILIBRIUM));
        F::from_samples(&mut vec![res].into_iter()).unwrap()
    }

    fn to_signed_frame(self) -> Self::Signed {
        self.sample.to_signed_sample()
    }

    fn to_float_frame(self) -> Self::Float {
        self.sample.to_float_sample()
    }

}
