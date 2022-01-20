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
        let sound = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_triangle_1.wav")
        ));

        let player = PlaybackManager::new();
        player.append(sound.clone_buffer());
        player.play();
    }

    fn test_sound_crop() {
        let sound = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_triangle_1.wav")
        ));
        //let transformer = SoundTransformer::from(sound);
        //transformer.push(transforms::Crop::from(transforms::Crop::ArgsT{std::time::duration....)
    }
}

