use std::io::{BufReader};
use std::fs::{File};

use rodio::buffer::SamplesBuffer;
use rodio::{Decoder};
use rodio::source::{Source, SamplesConverter};

pub struct SoundBuffer {
    buffer: Vec<f32>,
}

impl<T: Copy> From<&Vec<T>> for SoundBuffer where f32: From<T> {
    fn from(src: &Vec<T>) -> Self {
        SoundBuffer {
            buffer: {
                let mut vec : Vec<f32> = Vec::new();
                vec.reserve(src.len());
                for v in src {
                    let fv : f32 = f32::from(*v);
                    vec.push(fv);
                }
                vec
            },
        }
    }
}

pub enum FileDescriptor {
    WavFile(String),
}


const SAMPLE_RATE : u32 = 48000;
const NUM_CHANNELS : u16 = 2;
impl Into<SamplesBuffer<f32>> for SoundBuffer {
    fn into(self) -> SamplesBuffer<f32> {
        SamplesBuffer::<f32>::new(NUM_CHANNELS, SAMPLE_RATE, self.buffer.clone())
    }
}

impl SoundBuffer {
    pub fn from_file(file_descriptor: FileDescriptor) -> Self {
        let filename = {
            match file_descriptor {
                FileDescriptor::WavFile(s) => s,
            }
        };

        let file = BufReader::new(File::open(filename).unwrap());
        let decoder = Decoder::new(file).unwrap();
        let vec = Vec::<f32>::from_iter(decoder.convert_samples());
        SoundBuffer::from(&vec)
    }

    pub fn clone(&self) -> SoundBuffer {
        SoundBuffer {
            buffer: self.buffer.clone(),
        }
    }

    pub fn repitch(&mut self, pitch_factor: u16) -> () {
        /*
        let mut aligned_buffer = align_buffer(raw_buffer);
        println!("{:?}", aligned_buffer);
        for i in 0..aligned_buffer.len() {
            aligned_buffer[i] = Sample::amplify(aligned_buffer[i], 0.001);
            //aligned_buffer[i] = (aligned_buffer[i] / 2);
        }
        let new_source = unalign_buffer(&aligned_buffer);
        println!("{:?}", new_source);
        self.source = new_source;
        */
    }
}


