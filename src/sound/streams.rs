
use super::playback;

pub struct Note {
    tone: u8,
}

pub trait AudioStream {
}

pub struct SampleStream {}

pub struct Sample {
    source_stream: SampleStream,
}

pub struct Sound {

}
