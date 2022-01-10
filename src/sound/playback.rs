pub trait Playable {
    fn set_frame(_ : i32) -> ();
    fn get_frame() -> ();
    fn get_num_frames() -> i32;
    fn get_playback_rate() -> i32;
}
