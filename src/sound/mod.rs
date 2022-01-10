use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

// TODOS:
// - 'Sample' abstraction
// - 'Note' abstraction
// - 'Tuned' trait, to be implemented by a sample when a note has been imposed

mod streams;
pub use streams::*;

mod playback;
pub use playback::*;

pub fn wav_playback() -> () {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(
        File::open(
            "/home/axtya/Projects/mutools/bin/FS_001/SYNTH/synth_air_chord.wav"
        ).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn get_tuned_buffer(file: File) -> BufReader<File> {
    BufReader::new(file)
}

pub fn artificial_playback() -> () {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("/home/axtya/Projects/mutools/bin/FS_001/SYNTH/synth_air_chord.wav").unwrap();
    let beep1 = stream_handle.play_once(get_tuned_buffer(file)).unwrap();
    beep1.set_volume(0.9);
    println!("Started beep1");
    std::thread::sleep(std::time::Duration::from_millis(1500));
}


