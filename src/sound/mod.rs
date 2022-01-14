use std::{fs::File, io::BufRead};
use std::time::Duration;
use std::io::{BufReader, Read, Cursor};
use rodio::{Sample, Decoder, OutputStream, OutputStreamHandle, source::Source};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, BigEndian}; // 1.3.4
use rodio::source::{SineWave};

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


pub struct Sound {
    source: ,
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

impl Sound {

    pub fn from(filename: String) -> Sound {
        Sound::from_filename(filename)
    }

    pub fn from_filename(filename: String) -> Sound {
        Sound {
            source: reader_to_vec_buffer(
                &mut BufReader::new(File::open(filename).expect("Could not open file!"))
            ),
        }
    }

    pub fn wait_play(&self, millis: u64) -> () {
        let (_stream, stream_handle) = OutputStream::try_default()
                                        .expect("Failed to acquire stream handle!");
        let cursor = Cursor::new( Vec::clone(&self.source));
        let sink = stream_handle
                    .play_once(cursor)
                    .expect("Error while trying to play sound!");
        sink.play();
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }


    pub fn repitch(&mut self, pitch_factor: u16) -> () {
        let raw_buffer = &self.source.clone();
        let mut buf = SineWave::new(440);
        let nbuf = buf.take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
        let buf_iter = ;
        let raw_buf_iter = raw_buffer.into_iter();
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

impl Clone for Sound {
    fn clone(&self) -> Self {
        Sound {
            source: self.source.clone(),
        }
    }
}

