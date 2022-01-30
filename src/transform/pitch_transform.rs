use rodio::Sample;

use super::SoundTransform;
use crate::sound::Sound;

pub struct PitchTransform {
    frequency_shift: i32,
}

impl PitchTransform {
    pub fn from_freq(frequency_shift: i32) -> Self {
        Self {
            frequency_shift
        }
    }
}

impl SoundTransform for PitchTransform {
    type TransformStateT = ();
    fn transform(&self, input_sound: Sound) -> Sound {
        input_sound
    }
}
