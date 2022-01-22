use std::io::{BufReader};
use std::fs::{File};
use std::slice::SliceIndex;

use rodio::buffer::SamplesBuffer;
use rodio::{Decoder};
use rodio::source::{Source, SamplesConverter};

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

pub struct SamplesVector {
    _buffer: Vec<SoundSample>,
}

impl SamplesVector {
    pub fn new() -> Self {
        SamplesVec {
            _buffer: Vec::new(),
        }
    }

    pub fn get_sample(&self, index: usize) -> Option<SoundSample> {
        match self._buffer.get(index) {
            Some(s) => Some(*s),
            None => None,
        }
    }

    pub fn push(&mut self, sample: SoundSample) -> () {
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
        SamplesVec {
            _buffer: src.into_iter()
                     .map(|val| val.into() )
                     .collect(),
        }
    }
}

impl<T: Into<SoundSample>> Into<Vec<T>> for SamplesVector {
    fn into(self) -> Vec<T> {
        self.into
    }
}

struct SamplesVectorIterator {
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

impl<T: Into<SoundSample>> From<Vec<T>> for SoundBuffer where f32: From<T> {
    fn from(src: Vec<T>) -> Self {
        SoundBuffer {
            _buffer: SamplesVector::from(src),
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

    pub fn clone(&self) -> SoundBuffer {
        SoundBuffer {
            _buffer: self._buffer.clone(),
        }
    }

    pub fn push(&mut self, frame: f32) -> () {
        self._buffer.push(SoundSample::from(frame));
    }

    pub fn get_sample(&self, index: usize) -> Option<SoundSample> {
        self._buffer.get_sample(index)
    }
}

impl Into<SamplesBuffer<f32>> for SoundBuffer {
    fn into(self) -> SamplesBuffer<f32> {
        let _sample_vec : Vec<f32> = self._buffer.clone().into();
        SamplesBuffer::<f32>::new(SoundBuffer::NUM_CHANNELS, SoundBuffer::SAMPLE_RATE, _sample_vec)
    }
}

