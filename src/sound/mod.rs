use std::borrow::{Borrow, BorrowMut};
use std::vec::IntoIter;
use std::{fs::File, io::BufRead};
use std::time::Duration;
use std::io::{BufReader, Read, Seek, Cursor};
use std::iter::{Iterator, Cloned};
use rodio::PlayError;
use rodio::buffer::SamplesBuffer;
use rodio::static_buffer::StaticSamplesBuffer;
use rodio::{Sample, Decoder, OutputStream, OutputStreamHandle, source::Source};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, BigEndian}; // 1.3.4
use rodio::source::{SineWave, Buffered};

// TODOS:
// - 'Sample' abstraction
// - 'Note' abstraction
// - 'Tuned' trait, to be implemented by a sample when a note has been imposed

// IMPORTANT
// - Abstract audio streams out into a single class
// - The class should store an audio buffer and implement all traits necessary

mod streams;
pub use streams::*;

mod playback;
pub use playback::*;

const SAMPLE_RATE : u32 = 48000;
const NUM_CHANNELS : u16 = 2;
pub struct SoundBuffer {
    src: Vec<f32>,
}

impl SoundBuffer {

    pub fn from_f32_vec(vec: Vec<f32>) -> SoundBuffer {
        SoundBuffer {
            src: vec,
        }
    }

    pub fn from_sample_vec<T: Sample>(vec: Vec<T>) -> SoundBuffer {
        let mut formatted_vec : Vec<f32> = Vec::new();
        for v in vec {
            formatted_vec.push(f32::from(v.to_f32()));
        }
        SoundBuffer::from_f32_vec(formatted_vec)
    }

    fn vec_to_sample_buffer(vec: &Vec<f32>) -> SamplesBuffer<f32> {
        SamplesBuffer::<f32>::new(NUM_CHANNELS, SAMPLE_RATE, vec.clone())
    }

    fn source_to_vec<T: Seek + Read>(source: Decoder<T>) -> Vec<f32> {
        let vec = Vec::from_iter(source.convert_samples());
        vec
    }

    pub fn from_filename(filename: String) -> SoundBuffer {
        let file = BufReader::new(File::open(filename).unwrap());
        let decoder = Decoder::new(file).unwrap();
        SoundBuffer::from_f32_vec(SoundBuffer::source_to_vec(decoder))
    }

    pub fn from_frequency(freq: u32, duration: u64) -> SoundBuffer {
        let source = SineWave::new(freq)
                        .take_duration(Duration::from_millis(duration));
        SoundBuffer::from_f32_vec(Vec::<f32>::from_iter(source.buffered()))
    }

    pub fn clone(&self) -> SoundBuffer {
        SoundBuffer {
            src: self.src.clone(),
        }
    }

    fn play_on_stream(&self, output_stream: &mut OutputStreamHandle) -> Result<(), PlayError> {
        output_stream.play_raw(SoundBuffer::vec_to_sample_buffer(&self.src))
    }

    pub fn play(source: &mut SoundBuffer, output_stream: &mut OutputStreamHandle) -> Result<(), PlayError> {
        let src = SoundBuffer::clone(source);
        src.play_on_stream(output_stream)
    }

    /*pub fn clone(&mut self) -> Self {
        let mut src_vec : Vec<f32> = Vec::new();
        {
            src_vec = Vec::<f32>::from_iter(&mut self.src.into_iter());
        }
        SoundBuffer::from_f32_vec(&src_vec)
    }*/

}


fn reader_to_vec_buffer<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<u8> {
    let mut buf : Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf);
    buf
}

fn align_buffer(raw_buffer: &Vec<u8>) -> Vec<u16> {
    let scale_factor = std::mem::size_of::<u16>();
    let mut aligned_buffer = Vec::<u16>::new();
    let aligned_size : usize = raw_buffer.len() / scale_factor;
    aligned_buffer.resize(aligned_size, 0);
    for i in 0..aligned_size {
        let mut block : u16 = 0;
        for s in (i * scale_factor)..((i + 1) * scale_factor) {
            block = block << 8;
            block = block + u16::from(raw_buffer[s]);
        }
        println!("assigning to {} <- {}", i, block);
        aligned_buffer[i] = block;
    }
    aligned_buffer
}

fn unalign_buffer(aligned_buffer: &Vec<u16>) -> Vec<u8> {
    let mut raw_buffer : Vec<u8> = Vec::new();
    let scale_factor = std::mem::size_of::<u16>();
    raw_buffer.resize(scale_factor * aligned_buffer.len(), 0);
    for i in (0..raw_buffer.len()).step_by(scale_factor) {
        let mut block : &[u8] = &aligned_buffer[i / scale_factor].to_be_bytes();
        for j in 0..scale_factor {
            raw_buffer[i + j] = block[j];
        }
    }
    raw_buffer
}

pub struct Sound {
    buffer: SoundBuffer,
}

impl From<SoundBuffer> for Sound {
    fn from(buffer: SoundBuffer) -> Sound {
        Sound {
            buffer,
        }
    }
}

impl Sound {
    pub fn wait_play(&mut self, millis: u64) -> Result<(), PlayError> {
        let (_stream, mut stream_handle) = OutputStream::try_default()
                                        .expect("Failed to acquire stream handle!");
        let res = SoundBuffer::play(&mut SoundBuffer::clone(&self.buffer), &mut stream_handle);
        std::thread::sleep(std::time::Duration::from_millis(millis));
        res
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

