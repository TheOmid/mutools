mod streams;
pub use streams::*;

mod playback;
pub use playback::*;

mod sound_buffer;
pub use sound_buffer::{SoundBuffer, FileDescriptor};

mod playback_manager;
pub use playback_manager::{PlaybackManager};

pub struct Sound {
    sound_buffer: SoundBuffer,
}

impl Sound {
}

pub trait SoundTransform<Sound> {
    fn transform(_: Sound) -> Sound;
}

