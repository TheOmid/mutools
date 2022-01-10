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
    fn test_wav_playback() {
        //sound::wav_playback();
    }

    #[test]
    fn test_artificial_playback() {
        sound::artificial_playback()
    }
}

