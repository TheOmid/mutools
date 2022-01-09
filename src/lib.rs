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
    fn test_example() {
       midi::read_midi_input()
    }
}
