mod sound;
mod midi;


#[cfg(test)]
mod tests {
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
}

