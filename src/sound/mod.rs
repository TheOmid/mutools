use std::{fs::File, io::BufRead};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

// TODOS:
// - 'Sample' abstraction
// - 'Note' abstraction
// - 'Tuned' trait, to be implemented by a sample when a note has been imposed

// IMPORTANT
// - Abstract audio streams out into a single class
// - The class should store an audio buffer and implement all traits necessary

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

fn get_buffer(file: File) -> BufReader<File> {
    BufReader::new(file)
}

pub fn artificial_playback() -> () {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("/home/axtya/Projects/mutools/bin/FS_001/SYNTH/synth_air_chord.wav").unwrap();
    let mut source = Decoder::new(get_buffer(file)).unwrap();
    {
        let f1 = File::open("/home/axtya/Projects/mutools/bin/FS_001/SYNTH/synth_air_chord.wav").unwrap();
        let art_src = Decoder::new(get_buffer(f1))
                    .unwrap()
                    .buffered();
                    //.speed(0.8);
        let sound = stream_handle.play_raw(art_src.clone().convert_samples()).unwrap();
        println!("Started sound");
        std::thread::sleep(std::time::Duration::from_millis(1500));
        for frame in art_src.clone() {
            println!("playing: {}", frame);
        }
    };
}


