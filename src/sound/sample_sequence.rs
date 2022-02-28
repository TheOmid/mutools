use super::sample::*;

#[derive(Copy, Clone)]
pub struct SampleSequence<NumSamples: dasp::frame::NumChannels> {
    samples: std::array<AudioSample, NumSamples>
}

impl Iterator for SampleSequence {
    type Item = AudioSample;
    fn next(&mut self) -> Option<Self::Item> {
        self.samples.iter()
    }
}
