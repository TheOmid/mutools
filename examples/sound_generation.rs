
use mutools::sound::{MonoSignal, MonoFrame};

use dasp::Signal::Sine;

fn generate_sine_sound() {

    let mysound: MonoSignal = MonoSignal::from_samples(Sine::new());

}

fn main() {
    generate_sine_sound()
}
