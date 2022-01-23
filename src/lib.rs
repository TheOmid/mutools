mod sound;
mod transform;
mod midi;

#[cfg(test)]
mod tests {
    use crate::transform::{CropTransform, SoundTransform};

    use super::sound::{Sound,
                       FileDescriptor,
                       PlaybackManager};
    use super::midi;

    use super::transform::SoundTransformer;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sound_playback() {
        let sound = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_triangle_1.wav")
        ));
        let player = PlaybackManager::new();
        let sound_buffer = sound.clone_buffer();
        player.append(sound_buffer);
        //player.play();
    }

    #[test]
    fn test_sound_crop() {
        let player = PlaybackManager::new();

        let sound = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_sub_0.wav")
        ));
        let crop_transform = CropTransform::from_start_end(
                                std::time::Duration::from_millis(500),
                                std::time::Duration::from_millis(2500),
                                44100
                            );
        let cropped_sound = crop_transform.transform(sound);

        player.append(cropped_sound.clone_buffer());
        player.play()
    }
}

