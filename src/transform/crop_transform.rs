use std::time::Duration;

use super::SoundTransform;
use crate::sound::{Sound, SoundBuffer, SoundSample};

pub struct CropTransform {
    duration: Duration,
}

impl From<Duration> for CropTransform {
    fn from(duration: Duration) -> Self {
        CropTransform {
            duration,
        }
    }
}

impl SoundTransform for CropTransform {
    type TransformStateT = ();
    fn transform(state: &Self::TransformStateT, input_sound: Sound) -> Sound {
        let start_frame = 3000;
        let end_frame = 100000;
        let cropped_len = end_frame - start_frame;
        let mut sound = Sound::new();
        for i in start_frame..end_frame {
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
