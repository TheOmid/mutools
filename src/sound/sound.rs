use std::time::Duration;
use rand::Rng;

use super::signal::{SterioSignal};

#[derive(Clone)]
pub struct Sound {
    signals: Vec<SterioSignal>
}

impl Sound {
    pub fn new() -> Self {
        Sound {
            signals: Vec::<SterioSignal>::new()
        }
    }

    pub fn interpolate_frame(&self, index: usize) -> Option<f32> {
        let mut rng = rand::thread_rng();
        //let val = rng.gen::<f32>();
        let val1 = (f32::from(index as f32) / 16.0).sin() * 4.0;
        let val2 = (f32::from(index as f32) / 8.0).sin() * 4.0;
        let val2 = (f32::from(index as f32) / 32.0).sin() * 4.0;
        let val2 = (f32::from(index as f32) / 64.0).sin() * 4.0;
        let res = val1 + val2;
        println!("{}", res);
        Some(res)
    }
}

impl IntoIterator for Sound {
    type IntoIter = SoundIterator;
    type Item = <SoundIterator as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        SoundIterator {
            frame_index: 0,
            sound: self.clone()
        }
    }

}

pub struct SoundIterator {
    frame_index: usize,
    sound: Sound
}

impl Iterator for SoundIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.frame_index += 1;
        self.sound.interpolate_frame(self.frame_index-1)
    }

}

impl rodio::Source for SoundIterator {

    fn current_frame_len(&self) -> Option<usize> {
        //Some(self.frame_index % )
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44800
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs(5))
    }

}
