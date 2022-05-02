use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use mutools::sound::{Sound, SterioSignal, SineWaveGenerator};
use rodio::{source::Source, Decoder, OutputStream};

fn generate_sine_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut simple_sound = Sound::new();
    simple_sound.append_signal(SterioSignal::from_generator(SineWaveGenerator::new(), 44800*5));

    stream_handle.play_raw(simple_sound.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    generate_sine_sound()
}
