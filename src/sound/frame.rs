use dasp::frame::Frame;

use super::sample::*;

#[derive(Copy, Clone, PartialEq)]
pub struct MonoFrame {
    val: i8
}


impl Frame for MonoFrame {

    type Sample = RawSample;
    type NumChannels = NumChannels::N1;
    type Channels = ITERATOR;
    type Signed = i32;
    type Float = f32;

}

