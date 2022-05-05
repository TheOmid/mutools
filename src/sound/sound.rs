use dasp_signal::Signal;
use std::time::Duration;

use super::frame::SterioFrame;
use super::signal::SterioSignal;

#[derive(Clone)]
pub struct Sound {
    signals: Vec<SterioSignal>,
}

impl Sound {
    pub fn new() -> Self {
        Sound {
            signals: Vec::<SterioSignal>::new(),
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

    pub fn get_signals(&self) -> &Vec<SterioSignal> {
        &self.signals
    }

    pub fn get_num_signals(&self) -> usize {
        self.signals.len()
    }

    pub fn get_num_frames(&self) -> usize {
        let mut len: usize = 0;
        for signal in &self.signals {
            if signal.get_num_frames() > len {
                len = signal.get_num_frames()
            }
        }
        len
    }

    pub fn append_sound(&mut self, sound: &Sound) -> () {
        for ns in sound.get_signals().clone().into_iter() {
            self.signals.insert(0, ns);
        }
    }

    pub fn append_signal(&mut self, signal: SterioSignal) -> () {
        self.signals.insert(0, signal);
    }
}

impl IntoIterator for Sound {
    type IntoIter = MonoSoundIterator;
    type Item = <MonoSoundIterator as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        MonoSoundIterator {
            frame_index: 0,
            sound: self.clone(),
        }
    }
}

pub struct MonoSoundIterator {
    frame_index: usize,
    sound: Sound,
}

impl Iterator for MonoSoundIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.frame_index += 1;
        if self.frame_index >= self.sound.get_num_frames() {
            return None;
        }
        //println!("frame: {}", self.sound.get_raw_mono_frame(self.frame_index-1).unwrap_or(0.0));
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

/*
pub struct InterleavedSoundIterator {
    frame_index: usize,
    signal_index: usize,
    sound: Sound,
}

impl Iterator for InterleavedSoundIterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.signal_index += 1;
        if self.signal_index == sound.get_num_signals() {
            self.signal_index = 0;
            self.frame_index += 1;
        }
        self.sound.get_raw_mono_frame(self.frame_index - 1)
    }
}

impl rodio::Source for InterleavedSoundIterator {
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
    ..}
}
*/
