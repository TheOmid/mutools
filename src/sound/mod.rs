mod sound_buffer;
pub use sound_buffer::{SoundBuffer, SoundSample, SamplesVector, FileDescriptor};
mod playback_manager;
pub use playback_manager::PlaybackManager;

pub struct Sound {
    sound_buffer: SoundBuffer,
}

impl From<SoundBuffer> for Sound {
    fn from(buffer: SoundBuffer) -> Self {
        Sound {
            sound_buffer: buffer,
        }
    }
}

impl Sound {
    pub fn new() -> Self {
        Sound {
            sound_buffer: SoundBuffer::new(),
        }
    }

    pub fn from_file(file_descriptor: FileDescriptor) -> Self {
        Sound {
            sound_buffer: SoundBuffer::from_file(file_descriptor),
        }
    }

    pub fn clone_buffer(&self) -> SoundBuffer {
        self.sound_buffer.clone()
    }

    pub fn get_buffer(self) -> SoundBuffer {
        self.sound_buffer
    }

    pub fn len(&self) -> usize {
        self.sound_buffer.len()
    }

    pub fn push_sample<T: Into<SoundSample>>(&mut self, frame: T) -> () {
        self.sound_buffer.push_sample(frame);
    }

    pub fn get_sample(&self, index: usize) -> Option<SoundSample> {
        self.sound_buffer.get_sample(index)
    }
}

impl Clone for Sound {
    fn clone(&self) -> Self {
        Sound::from(self.clone_buffer())
    }
}

