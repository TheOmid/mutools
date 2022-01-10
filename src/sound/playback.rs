use std::io::BufReader;
use std::{fs::File, io::BufRead};

use rodio::Decoder;

pub trait Stream<T> {
    fn set_frame(_ : i32) -> ();
    fn get_frame() -> T;
    fn get_num_frames() -> i32;
    fn get_playback_rate() -> i32;
}

pub struct SampleBuffer {
    decoder: Decoder<File>,
}

pub trait AudioBuffer {
    fn transform_speed(&self, ratio: f32) -> SampleBuffer;
    fn transform_pitch(&self, semitones: i32) -> SampleBuffer;
    fn playback(&self) -> ();
}


