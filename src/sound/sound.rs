use std::time::Duration;
use dasp_signal::Signal;

use super::signal::SterioSignal;
use super::frame::SterioFrame;

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

    pub fn get_raw_mono_frame(&self, index: usize) -> Option<f32> {
        let mut mono_frame: f32 = 0.0;
        for signal in &self.signals {
            if signal.get_raw_frame(index) == None {
                return None
            }
            mono_frame += signal.get_raw_frame(index).unwrap().as_mono_frame()
        }
        Some(mono_frame)
    }

    pub fn append_signal(&mut self, signal: &SterioSignal) -> () {
        &mut self.signals.push(signal.clone());
    }
}

impl IntoIterator for Sound {
    type IntoIter = MonoSoundIterator;
    type Item = <MonoSoundIterator as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        MonoSoundIterator {
            frame_index: 0,
            sound: self.clone()
        }
    }

}

pub struct MonoSoundIterator {
    frame_index: usize,
    sound: Sound
}

impl Iterator for MonoSoundIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.frame_index += 1;
        self.sound.get_raw_mono_frame(self.frame_index - 1)
    }

}

impl rodio::Source for MonoSoundIterator {

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
