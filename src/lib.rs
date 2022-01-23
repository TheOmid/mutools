mod sound;
mod transform;
mod midi;

#[cfg(test)]
mod tests {
    use super::sound::{Sound,
                       FileDescriptor,
                       PlaybackManager};
    use super::midi;


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sound_playback() {
        let sound = Box::from(Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_triangle_1.wav")
        )));
        let player = Box::from(PlaybackManager::new());
        let sound_buffer = sound.clone_buffer();
        player.append(sound_buffer);
        player.play();
    }

    #[test]
    fn test_sound_crop() {
        let sound = Box::from(Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_triangle_1.wav")
        )));
        let transformer = SoundTransformer::from(sound);
        //transformer.push(transforms::Crop::from(transforms::Crop::ArgsT{std::time::duration....)

        // Next - Implement SoundTransformer and have it apply a crop transformation to the sound
    }
}

