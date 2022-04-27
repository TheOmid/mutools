
use mutools::sound::{SterioSignal, SterioFrame};
use dasp_signal::{self as signal, Signal};
use dasp_ring_buffer as ring_buffer;
use rodio::{Decoder, OutputStream, source::Source};

fn generate_sine_sound() {
    let sine_wave = signal::rate(4.0).const_hz(1.0).sine();
    let interleaved_samples = sine_wave.into_interleaved_samples();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(interleaved_samples.into_iter());
}

fn main() {
    generate_sine_sound()
}
