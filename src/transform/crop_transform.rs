use std::time::Duration;

use super::SoundTransform;
use crate::sound::Sound;

pub struct CropTransform {
    duration: Duration,
}

impl From<Duration> for CropTransform {
    fn from(duration: Duration) -> Self {
        CropTransform {
            duration: Duration::from_millis(1000),
        }
    }
}

impl SoundTransform for CropTransform {
    fn transform(input_sound: Sound) -> Sound {
        input_sound.clone()
    }
}
