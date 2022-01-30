use std::io::{BufReader};
use std::fs::{File};
use std::slice::SliceIndex;

use rodio::buffer::SamplesBuffer;
use rodio::Decoder;
use rodio::source::{Source, SamplesConverter};

use rustfft::num_complex::Complex;

pub struct SoundSample {
    _val: f32,
}

impl Clone for SoundSample { 
    fn clone(&self) -> Self {
        Self {
            _val: self._val,
        }
    }
}
impl Copy for SoundSample { }

impl From<f32> for SoundSample {
    fn from(_val: f32) -> Self {
        SoundSample {
            _val
        }
    }
}

impl Into<f32> for SoundSample {
    fn into(self) -> f32 {
        self._val
    }
}

impl From<Complex<f32>> for SoundSample {
    fn from(complex_num: Complex<f32>) -> Self {
        //Self::from(complex_num.l1_norm())
        Self::from(complex_num.norm())
    }
}

impl Into<Complex<f32>> for SoundSample {
    fn into(self) -> Complex<f32> {
        //Complex::new(<Self as Into<f32>>::into(self) / 88100.0, 0.0)
        Complex::new(0.0, <Self as Into<f32>>::into(self) / 44100.0)
    }
}

pub struct SamplesVector {
    _buffer: Vec<SoundSample>,
}

impl SamplesVector {
    pub fn new() -> Self {
        Self {
            _buffer: Vec::<SoundSample>::new(),
        }
    }

    pub fn len(&self) -> usize {
        self._buffer.len()
    }

    pub fn get_sample(&self, index: usize) -> Option<SoundSample> {
        match self._buffer.get(index) {
            Some(s) => Some(*s),
            None => None,
        }
    }

    pub fn push_sample(&mut self, sample: SoundSample) -> () {
        &mut self._buffer.push(sample);
    }

}

impl Clone for SamplesVector {
    fn clone(&self) -> Self {
        Self {
            _buffer: self._buffer.clone(),
        }
    }
}

impl<T: Into<SoundSample>> From<Vec<T>> for SamplesVector {
    fn from(src: Vec<T>) -> Self {
        Self {
            _buffer: src.into_iter()
                     .map(|val| val.into() )
                     .collect(),
        }
    }
}

impl<T: Into<SoundSample>> Into<Vec<T>> for SamplesVector {
    fn into(self) -> Vec<T> {
        self.into()
    }
}

pub struct SamplesVectorIterator {
    _samples_vector: SamplesVector,
    _index: usize,
}

impl std::iter::Iterator for SamplesVectorIterator {
    type Item = SoundSample;

    fn next(&mut self) -> Option<Self::Item> {
        self._index += 1;
        self._samples_vector.get_sample(self._index)
    }
}

impl std::iter::IntoIterator for SoundBuffer {
    type Item = SoundSample;
    type IntoIter = SamplesVectorIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            _index: 0,
            _samples_vector: self._buffer,
        }
    }
}

impl SoundSample {
}

pub struct SoundBuffer {
    _buffer: SamplesVector,
}

/*
impl<T: Into<SoundSample>> From<Vec<T>> for SoundBuffer where f32: From<T> {
    fn from(src: Vec<T>) -> Self {
        SoundBuffer {
            _buffer: SamplesVector::from(src),
        }
    }
}
*/

impl <T: Into<SoundSample>> From<Vec<T>> for SoundBuffer {
    fn from(src: Vec<T>) -> Self {
        Self {
            _buffer: SamplesVector::from(src.into_iter().map(|e| <T as Into<SoundSample>>::into(e)).collect::<Vec<SoundSample>>()),
        }
    }
}

pub enum FileDescriptor {
    WavFile(String),
}

impl SoundBuffer {

    const SAMPLE_RATE : u32 = 48000;
    const NUM_CHANNELS : u16 = 2;

    pub fn new() -> Self {
        SoundBuffer {
            _buffer: SamplesVector::new(),
        }
    }

    pub fn from_file(file_descriptor: FileDescriptor) -> Self {
        let filename = {
            match file_descriptor {
                FileDescriptor::WavFile(s) => s,
            }
        };

        let file = BufReader::new(File::open(filename).unwrap());
        let decoder = Decoder::new(file).unwrap();
        let vec = Vec::<f32>::from_iter(decoder.convert_samples());
        SoundBuffer::from(vec)
    }

    pub fn get_samples(self) -> SamplesVector {
        self._buffer
    }

    pub fn len(&self) -> usize {
        self._buffer.len()
    }

    pub fn clone(&self) -> SoundBuffer {
        SoundBuffer {
            _buffer: self._buffer.clone(),
        }
    }

    pub fn push_sample<T: Into<SoundSample>>(&mut self, frame: T) -> () {
        self._buffer.push_sample(frame.into());
    }

    pub fn get_sample(&self, index: usize) -> Option<SoundSample> {
        self._buffer.get_sample(index)
    }
}

pub struct RawSamplesIterator {
    _samples_vector: Vec<f32>,
    _index: usize,
}

impl SoundBuffer {
    pub fn into_raw_samples_iter(self) -> RawSamplesIterator {
        RawSamplesIterator {
            _index: 0,
            _samples_vector: self._buffer.into(),
        }
    }
}

impl Into<SamplesBuffer<f32>> for SoundBuffer {
    fn into(self) -> SamplesBuffer<f32> {
        let _sample_vec : Vec<f32> = self._buffer.clone().into();
        SamplesBuffer::<f32>::new(SoundBuffer::NUM_CHANNELS, SoundBuffer::SAMPLE_RATE, _sample_vec)
    }
}

impl rodio::Sample for SoundSample {
    fn lerp(first: Self, second: Self, numerator: u32, denominator: u32) -> Self {
        Self::from(f32::lerp(first.into(), second.into(), numerator, denominator))
    }

    fn amplify(self, value: f32) -> Self {
        Self::from(f32::amplify(self.into(), value))
    }

    fn saturating_add(self, other: Self) -> Self {
        Self::from(f32::saturating_add(self.into(), other.into()))
    }

    fn zero_value() -> Self {
        Self::from(f32::zero_value())
    }
}

unsafe impl rodio::cpal::Sample for SoundSample {
    const FORMAT: rodio::cpal::SampleFormat = rodio::cpal::SampleFormat::F32;
    fn to_f32(&self) -> f32 {
        self._val
    }

    fn to_i16(&self) -> i16 {
        f32::to_i16(&self.to_f32())
    }

    fn to_u16(&self) -> u16 {
        f32::to_u16(&self.to_f32())
    }

    fn from<S>(s: &S) -> Self
    where
            S: rodio::cpal::Sample {
        <SoundSample as From<f32>>::from(
            rodio::cpal::Sample::to_f32(s)
        )
    }
}


impl rodio::Source for SamplesVectorIterator {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        SoundBuffer::NUM_CHANNELS
    }

    fn sample_rate(&self) -> u32 {
        SoundBuffer::SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}


