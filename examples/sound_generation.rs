
use mutools::sound::{MonoSignal, MonoFrame};
use dasp_signal::{self as signal, Signal};
use rodio::{Decoder, OutputStream, source::Source};

fn generate_sine_sound() {

    let sine_wave = signal::rate(4.0).const_hz(1.0).sine();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(sine_wave);

}

fn main() {
    generate_sine_sound()
}
