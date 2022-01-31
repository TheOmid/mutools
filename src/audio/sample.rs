
// Represents a single audio sample.
// A sequence of audio samples makes up a channel

#[derive(Copy, Clone)]
pub struct AudioSample {
    val: i16,
}

impl AudioSample {
    pub fn new(val: i16) -> Self {
        AudioSample {
            val
        }
    }
}

impl From<i16> for AudioSample {
    fn from(val: i16) -> Self {
        Self::new(val)
    }
}

unsafe impl rodio::cpal::Sample for AudioSample {
    const FORMAT: rodio::cpal::SampleFormat = rodio::cpal::SampleFormat::F32;

    fn to_f32(&self) -> f32 {
        i16::to_f32(&self.val)
    }

    fn to_i16(&self) -> i16 {
        self.val
    }

    fn to_u16(&self) -> u16 {
        i16::to_u16(&self.val)
    }

    fn from<S>(s: &S) -> Self where S: rodio::cpal::Sample {
        <AudioSample as From<i16>>::from(
            rodio::cpal::Sample::to_i16(s)
        )
    }
}

impl rodio::Sample for AudioSample {
    fn lerp(first: Self, second: Self, numerator: u32, denominator: u32) -> Self {
        Self::new(i16::lerp(first.val, second.val, numerator, denominator))
    }

    fn amplify(self, value: f32) -> Self {
        Self::new(i16::amplify(self.val, value))
    }

    fn saturating_add(self, other: Self) -> Self {
        Self::new(i16::saturating_add(self.val, other.val))
    }

    fn zero_value() -> Self {
        Self::new(i16::zero_value())
    }
}
