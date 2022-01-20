use crate::sound::Sound;

pub trait SoundTransform {
    fn transform(_: Sound) -> Sound;
}

mod crop_transform;
pub use crop_transform::CropTransform;

