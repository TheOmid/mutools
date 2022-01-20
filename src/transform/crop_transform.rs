use std::time::Duration;

use super::SoundTransform;
use crate::sound::{Sound, SoundBuffer};

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
    fn transform(input_sound: Sound) -> Sound {
        let start_frame = 0;
        let end_frame = 0;
        let cropped_len = end_frame - start_frame;
        let mut sound = Sound::new();
        for i in start_frame..end_frame {
            sound.push_frame(input_sound.get_frame(i))
        }
        sound
    }
}
