use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use mutools::sound::{Sound};
use rodio::{source::Source, Decoder, OutputStream};

fn generate_sine_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let empty_sound = Sound::new();
    stream_handle.play_raw(empty_sound.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    generate_sine_sound()
}
