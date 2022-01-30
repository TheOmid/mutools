use rodio::Sample;

use super::SoundTransform;
use crate::sound::{Sound, SamplesVector, SoundBuffer, SoundSample};
use rustfft::{FftPlanner, num_complex::Complex};

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

        let mut sound_buffer = input_sound.clone_buffer();
        let mut planner = FftPlanner::<f32>::new();
        let forward_fft = planner.plan_fft_forward(sound_buffer.len()-1);
        let inverse_fft = planner.plan_fft_inverse(sound_buffer.len()-1);

        let mut samples_vec: Vec<SoundSample> = Vec::from_iter(sound_buffer.into_iter());
        let mut complex_samples : Vec<Complex<f32>> = samples_vec
                                    .into_iter()
                                    .map(|e| e.into())
                                    .collect();
        let mut samples_slice : &mut [Complex<f32>] = &mut complex_samples[0..];
        forward_fft.process(samples_slice);
        inverse_fft.process(samples_slice);

        let v = Vec::from(samples_slice);
        let vs : Vec<SoundSample> = v.into_iter().map(|e| e.into()).collect();
        let s_buff : SoundBuffer = vs.into();
        let n_sound : Sound = s_buff.into();

        n_sound
    }
}
