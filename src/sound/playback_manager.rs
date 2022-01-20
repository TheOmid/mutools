use crate::sound::{SoundBuffer,
                   OutputStreamHandle,
                   OutputStream,
                   Sink};
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
                let mut sink = Sink::try_new(&stream_handle).unwrap();
                sink.pause();
                sink
            },
            stream: _stream,
            stream_handle,
        }
    }

    pub fn append(&self, sound: SoundBuffer) -> () {
        self.sink.append(<SoundBuffer as Into<SamplesBuffer<f32>>>::into(sound));
    }

    pub fn play(&self) -> () {
        self.sink.play();
        self.sink.sleep_until_end();
    }
}
