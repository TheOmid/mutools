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

    fn from_fn<F>(mut from: F) -> Self
        where F: FnMut(usize) -> Self::Sample {
        let v = (from)(0);
        MonoFrame {
            sample: v
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
