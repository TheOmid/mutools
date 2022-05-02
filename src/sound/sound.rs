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
        let mut all_none = true;
        for signal in &self.signals {
            if signal.get_raw_frame(index) != None {
                mono_frame += signal.get_raw_frame(index).unwrap().as_mono_frame();
                all_none = false;
            }
        }
        if all_none {
            return None;
        }
        Some(mono_frame)
    }

    pub fn get_num_signals(&self) -> usize {
        signals.size()
    }

    pub fn get_num_frames(&self) -> usize {
        let len: usize = 0;
        for signal in &self.signals {
            if signal.get_num_frames() > len {
                len = signal.get_num_frames()
            }
        }
        len
    }

    pub fn append_sound(&mut self, sound: &Sound) -> () {
        let new_signals : Vec<SterioSignal> = Vec::new();
        for i in 0..sound.get_num_frames() {
            for j in 0..sound.get_num_signals() {
                //new_signals[j][i] = sound.get_sound_frame(i)[j];
                new_signals[j][i] = 0.0;
            }
        }
        signals.insert(0, new_signals.into_iter());
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
