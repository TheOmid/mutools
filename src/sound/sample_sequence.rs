use super::sample::*;

#[derive(Copy, Clone)]
pub struct SampleSequenceIterator {
    samples: [AudioSample; 8]
}

impl Iterator for SampleSequenceIterator {
    type Item = AudioSample;
    fn next(&mut self) -> Option<Self::Item> {
        self.samples.iter()
    }
}
