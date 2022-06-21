use dasp_ring_buffer as ring_buffer;
use dasp_signal::{self as signal, Signal};
use mutools::player::SoundPlayer;
use mutools::signal::{
    SineWaveGenerator, SquareWaveGenerator, SterioSignal, TriangleWaveGenerator,
};
use mutools::sound::Sound;

use rodio::{source::Source, Decoder, OutputStream};

fn generate_sounds() {
    let player = SoundPlayer::new();

    let mut sine_wave = Sound::new();
    sine_wave.append_signal(SterioSignal::from_generator(
        SineWaveGenerator::new(220.0, 0.3, 44800),
        44800 * 3,
    ));
    player.play_sound(sine_wave.clone()).sleep_until_end();

    let mut square_wave = Sound::new();
    square_wave.append_signal(SterioSignal::from_generator(
        SquareWaveGenerator::new(220.0, 0.05, 32, 44800),
        44800 * 3,
    ));
    player.play_sound(square_wave.clone()).sleep_until_end();

    let mut triangle_wave = Sound::new();
    triangle_wave.append_signal(SterioSignal::from_generator(
        TriangleWaveGenerator::new(220.0, 0.1, 14, 44800),
        44800 * 3,
    ));
    player.play_sound(triangle_wave.clone()).sleep_until_end();

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
    player.play_sound(combined_wave.clone()).sleep_until_end();
}

fn main() {
    generate_sounds()
}
