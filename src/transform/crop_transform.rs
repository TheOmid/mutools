use std::time::Duration;

use rodio::Sample;

use super::SoundTransform;
use crate::sound::{Sound, SoundBuffer, SoundSample};

pub struct CropTransform {
    start_frame: usize,
    end_frame: usize,
}

impl CropTransform {
    pub fn from_start_end(start: Duration, end: Duration, sample_rate: u32) -> Self {
        let start_frame : usize = (start.as_secs_f32()
                            * sample_rate as f32) as usize;
        let end_frame : usize = (end.as_secs_f32()
                            * sample_rate as f32) as usize;
        Self {
            start_frame,
            end_frame,
        }
    }
}

impl SoundTransform for CropTransform {
    type TransformStateT = ();
    fn transform(&self, input_sound: Sound) -> Sound {
        let mut sound = Sound::new();
        for i in self.start_frame..self.end_frame {
            sound.push_sample(
                match input_sound.get_sample(i) {
                    Some(s) => SoundSample::from(s),
                    None => SoundSample::from(0.0),
                }
            )
        }
        sound
    }
}

pub struct LerpTransform {
    sound_operand: Sound,
    numerator: u32,
    denominator: u32,
}

impl LerpTransform {
    pub fn from(sound_operand: Sound, numerator: u32, denominator: u32) -> Self {
        Self {
            sound_operand,
            numerator,
            denominator,
        }
    }
}

impl SoundTransform for LerpTransform {
    type TransformStateT = ();
    fn transform(&self, input_sound: Sound) -> Sound {
        let mut sound = Sound::new();
        for sample_index in 0..input_sound.len() {
            let val = rodio::Sample::lerp(input_sound.get_sample(sample_index).unwrap(),
                                          self.sound_operand.get_sample(sample_index).unwrap(),
                                          self.numerator,
                                          self.denominator);
            println!("Calculated: {}", <SoundSample as Into<f32>>::into(val));
            sound.push_sample(
                val
            )
        }
        sound
    }
}
