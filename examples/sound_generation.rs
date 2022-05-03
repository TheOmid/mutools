use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use mutools::sound::{Sound, SterioSignal, SineWaveGenerator, SquareWaveGenerator};
use rodio::{source::Source, Decoder, OutputStream};

fn generate_sine_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut sine_wave = Sound::new();
    sine_wave.append_signal(SterioSignal::from_generator(SineWaveGenerator::new(220.0, 0.1, 44800), 44800*3));
    stream_handle.play_raw(sine_wave.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(3));


    let mut square_wave = Sound::new();
    square_wave.append_signal(SterioSignal::from_generator(SquareWaveGenerator::new(220.0, 0.1, 32, 44800), 44800*3));
    stream_handle.play_raw(square_wave.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(3));
}

fn main() {
    generate_sine_sound()
}
