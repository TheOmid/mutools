use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, Ignore};

struct MidiDevice {
}

struct MidiController {
}

impl MidiController {
    pub fn new(device: MidiDevice) -> MidiController {
        Self {
        }
    }
}
