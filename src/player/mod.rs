use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};

pub struct SoundPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle
}

impl SoundPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            stream,
            stream_handle,
        }
    }
}

