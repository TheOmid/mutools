use std::io::BufReader;

pub trait Stream<T> {
    fn set_frame(_ : i32) -> ();
    fn get_frame() -> T;
    fn get_num_frames() -> i32;
    fn get_playback_rate() -> i32;
}

pub trait Note {
    fn tone() -> i32;
}

pub trait Sample<T: Stream<U>, U> {
    fn buffer() -> T;
}

pub trait Sound<T> {
}
