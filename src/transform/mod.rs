use crate::sound::Sound;

pub trait SoundTransform {
    type TransformStateT;
    fn transform(&self, _: Sound) -> Sound;
}

pub struct SoundTransformer {
    sound: Sound,
}

impl From<Sound> for SoundTransformer {
    fn from(src: Sound) -> Self {
        Self {
            sound: src,
        }
    }
}

mod crop_transform;
pub use crop_transform::CropTransform;

