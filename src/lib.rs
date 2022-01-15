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
    }

    #[test]
    fn test_input_stream() {
        let midi_control = midi::read_midi_input();
        println!("Done!!!");
    }

    #[test]
    fn test_freq_playback() {
        let sound_buffer = sound::SoundBuffer::from_frequency(50, 1500);
        let mut sound = sound::Sound::from(sound_buffer);
        //sound.wait_play(1500).unwrap();
    }

    #[test]
    fn test_sound_playback() {
        let sound_buffer = sound::SoundBuffer::from_filename(
            String::from("/home/axtya/Projects/mutools/bin/FS_001/BASS/bass_triangle_1.wav")
        );
        let mut sound = sound::Sound::from(sound_buffer);
        sound.wait_play(1500).unwrap();
    }

}

