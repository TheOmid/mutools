
/*
The signal module provides structures for storing, generating, and performing computations on:
- Samples individual sound samples
- Frames: collections of samples
- Signals: sequences of frames

These utilities are used by other modules, particularly 'sound', to conveniently work with raw audio data.
*/

pub mod sample;
pub use sample::*;

pub mod frame;
pub use frame::*;

pub mod signal;
pub use signal::*;

pub mod generator;
pub use generator::*;
