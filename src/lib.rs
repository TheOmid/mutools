mod sound;
mod midi;

#[cfg(test)]
mod tests {
    use crate::sound::wav_playback;

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
    fn test_sound_playback() {
        let mut snd = &mut sound::Sound::from_filename(
            String::from("/home/axtya/Projects/mutools/bin/FS_001/SYNTH/synth_air_chord.wav")
        );
        snd.wait_play(1500);
        snd.repitch(5);
        snd.wait_play(1500);
    }

}

