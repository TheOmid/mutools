use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use mutools::sound::{
    SineWaveGenerator, Sound, SquareWaveGenerator, SterioSignal, TriangleWaveGenerator,
};
use rodio::{source::Source, Decoder, OutputStream};

fn generate_sine_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut sine_wave = Sound::new();
    sine_wave.append_signal(SterioSignal::from_generator(
        SineWaveGenerator::new(220.0, 0.3, 44800),
        44800 * 3,
    ));
    stream_handle.play_raw(sine_wave.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut square_wave = Sound::new();
    square_wave.append_signal(SterioSignal::from_generator(
        SquareWaveGenerator::new(220.0, 0.05, 32, 44800),
        44800 * 3,
    ));
    stream_handle.play_raw(square_wave.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut triangle_wave = Sound::new();
    triangle_wave.append_signal(SterioSignal::from_generator(
        TriangleWaveGenerator::new(220.0, 0.1, 14, 44800),
        44800 * 3,
    ));
    stream_handle.play_raw(triangle_wave.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(3));

    let mut combined_wave = Sound::new();
    combined_wave.append_signal(SterioSignal::from_generator(
        SineWaveGenerator::new(554.37, 0.3, 44800),
        44800 * 5,
    ));
    combined_wave.append_signal(SterioSignal::from_generator(
        SquareWaveGenerator::new(277.18, 0.1, 32, 44800),
        44800 * 5,
    ));
    combined_wave.append_signal(SterioSignal::from_generator(
        TriangleWaveGenerator::new(138.59, 0.2, 14, 44800),
        44800 * 5,
    ));
    stream_handle.play_raw(combined_wave.into_iter());
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn main() {
    generate_sine_sound()
}
