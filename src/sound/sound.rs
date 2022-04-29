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

    pub fn add_frames(&self, index: usize) -> Option<SterioFrame> {
        let mut res = SterioSignal::EQUILIBRIUM.clone();
        for signal in self.signals {
            res = res.add_amp(signal.get_channel(index))
        }
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
        let frame = self.sound.add_frames(self.frame_index-1)
                    .unwrap_or(SterioFrame::EQUILIBRIUM);
        return frame[0] + frame[1]
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
