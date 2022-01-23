
use crate::sound::SoundBuffer;

use rodio::{OutputStream, OutputStreamHandle, Sink};
use rodio::buffer::SamplesBuffer;

pub struct PlaybackManager {
    sink: Sink,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl PlaybackManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        PlaybackManager {
            sink: {
                let sink = Sink::try_new(&stream_handle).unwrap();
                sink.pause();
                sink
            },
            stream: _stream,
            stream_handle,
        }
    }

    pub fn append(&self, sound: SoundBuffer) -> () {
        self.sink.append(sound.into_iter());
    }

    pub fn play(&self) -> () {
        self.sink.play();
        self.sink.sleep_until_end();
    }
}
