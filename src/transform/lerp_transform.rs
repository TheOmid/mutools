use rodio::Sample;

use super::SoundTransform;
use crate::sound::{Sound, SoundBuffer, SoundSample};

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
