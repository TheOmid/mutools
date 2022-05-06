use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, Ignore};

struct MidiDevice {
    input: MidiInput,
    port: MidiInputPort,
    connection: MidiInputConnection,
}

impl MidiDevice {
    pub fn new() -> Self {
        let midi_in = MidiInput::new("midi input");
        midi_in.ignore(Ignore::None);
        let in_port = match in_ports.len() {
            0 => panic!("No midi inputs found!"),
            _ => {
                &in_ports[0]
            }
        };
        let connection = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        }, ())?;

        Self {
            input: midi_in,
            port: in_port,
            connection
        }
    }
}

struct MidiController {
    device: MidiDevice
}

impl MidiController {
    pub fn new(device: MidiDevice) -> Self {
        Self {
            device
        }
    }
}
