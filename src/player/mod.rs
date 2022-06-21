/*
The 'player' module provides convenient abstractions for playing sounds through hardware devices.

Since many sounds are composed of multiple mono or sterio signals, this module also serves to create and cache sounds into device-friendly buffers.
    Eg. A sound may be composed of a dozen mono signals (sequences of mono frames), so the player takes care of caching the raw interleaved copy for efficient playback.

    NOTE: Is this even necessary?
*/

use super::sound::Sound;
use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};

pub struct PlaybackHandle {
    sink: Sink,
}

impl PlaybackHandle {
    pub fn new(stream_handle: &OutputStreamHandle) -> Self {
        Self {
            sink: Sink::try_new(&stream_handle).unwrap(),
        }
    }

    pub fn load_sound(&mut self, sound: Sound) -> () {
        self.sink.append(sound.into_iter());
    }

    pub fn sleep_until_end(&self) -> () {
        self.sink.sleep_until_end();
    }
}

pub struct SoundPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl SoundPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            stream,
            stream_handle,
        }
    }

    pub fn play_sound(&self, sound: Sound) -> PlaybackHandle {
        let mut playback_handle = PlaybackHandle::new(&self.stream_handle);
        playback_handle.load_sound(sound);
        playback_handle
    }
}
