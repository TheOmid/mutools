mod sound;
mod midi;

#[cfg(test)]
mod tests {
    use super::sound;
    use super::midi;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(result, 4);
    }

    /*
    #[test]
    fn test_input_stream() {
        let midi_control = midi::read_midi_input();
        println!("Done!!!");
    }
    */

    #[test]
    fn test_sound_playback() {
        let sound_buffer = sound::SoundBuffer::from_file(sound::FileDescriptor::WavFile(
            String::from("/home/axtya/Projects/sampler-rs/bin/FS_001/BASS/bass_triangle_1.wav")
        ));

        let player = sound::PlaybackManager::new();
        player.append(sound_buffer);
        player.play();
    }

}

