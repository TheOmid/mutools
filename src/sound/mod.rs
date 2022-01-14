use std::borrow::{Borrow, BorrowMut};
use std::vec::IntoIter;
use std::{fs::File, io::BufRead};
use std::time::Duration;
use std::io::{BufReader, Read, Cursor};
use std::iter::{Iterator, Cloned};
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



pub struct SoundBuffer<'a> {
    src: &'a SamplesBuffer<f32>,
}


impl<'a> SoundBuffer<'a> {

    pub fn from_f32_vec(vec: Vec<f32>) -> SoundBuffer<'a> {
        SoundBuffer {
            src: &SamplesBuffer::<f32>::new(1, 44100, vec.clone()),
        }
    }

    pub fn from_sample_vec<T: Sample>(vec: Vec<T>) -> SoundBuffer<'a> {
        let mut formatted_vec : Vec<f32> = Vec::new();
        for v in vec {
            formatted_vec.push(f32::from(v.to_f32()));
        }
        SoundBuffer::from_f32_vec(formatted_vec)
    }

    pub fn from_filename(filename: String) -> SoundBuffer<'a> {
        let file = BufReader::new(File::open("examples/music.ogg").unwrap());
        let source = Decoder::new(file).unwrap();
        let src_vec = Vec::<i16>::from_iter(source.buffered());
        SoundBuffer::from_sample_vec(src_vec)
    }

    pub fn from_frequency(freq: u32) -> SoundBuffer<'a> {
        let source = SineWave::new(freq)
                        .take_duration(Duration::from_secs_f32(0.25));
        SoundBuffer::from_f32_vec(Vec::<f32>::from_iter(source.buffered()))
    }

    pub fn clone(self) -> SoundBuffer<'a> {
        let mut temp_buf : Vec<f32> = Vec::new();
        let src = self.get_source().clone();
        let nsrc = Vec::<f32>::new();
        for v in src.into_iter() {

        }
        SoundBuffer::from_f32_vec(nsrc)
    }

    fn get_source(&self) -> &'a SamplesBuffer<f32> {
        self.src
    }

    fn get_source_clone(self) -> SamplesBuffer<f32> {
        let buf_vec = Vec::<f32>::new();
        for (i, v) in self.src.enumerate() {

        }
        let n_sample_buffer = SamplesBuffer::<f32>::new(1, 44100, buf_vec);
        n_sample_buffer
    }

    pub fn play(source : &mut SoundBuffer, output_stream: &mut OutputStreamHandle) -> () {
        let src = source.clone()
                    .get_source()
                    .buffered()
                    .to_owned()
                    .into_iter();
        output_stream
            .play_raw(src)
            .expect("Error when trying to play SoundBuffer");
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

pub struct Sound<'a> {
    buffer: SoundBuffer<'a>,
}

impl<'a> Sound<'a> {

    pub fn from(filename: String) -> Sound<'a> {
        Sound::from_filename(filename)
    }

    pub fn from_filename(filename: String) -> Sound<'a> {
        Sound {
            buffer: SoundBuffer::from_filename(filename),
        }
    }

    pub fn wait_play(&mut self, millis: u64) -> () {
        let (_stream, mut stream_handle) = OutputStream::try_default()
                                        .expect("Failed to acquire stream handle!");
        SoundBuffer::play(&mut self.buffer.clone(), &mut stream_handle);
        std::thread::sleep(std::time::Duration::from_millis(millis));
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

