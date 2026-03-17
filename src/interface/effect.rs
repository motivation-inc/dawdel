/// Trait for audio effect implementations.
///
/// `modify` takes `input` (left and right audio channels) and outputs `Vec<(f32, f32)>`
/// left and right audio channels. Anything that happens between is described as an "audio effect".
///
/// # Example
///
/// ```no_run
/// use dawdel::Effect;
///
/// pub struct RandomEffect {
///     amount: f32,
/// }
///
/// impl Effect for RandomEffect {
///     fn modify(&self, sample_rate: u32, input: &[(f32, f32)]) -> Vec<(f32, f32)> {
///         let mut new_data = Vec::new();
///
///         for (l, r) in input {
///             new_data.push((l + self.amount, r + self.amount))
///         }
///
///         new_data
///     }
/// }
/// ```
pub trait Effect {
    fn modify(&self, sample_rate: u32, input: &[(f32, f32)]) -> Vec<(f32, f32)>;
}

/// A reverb effect for audio samples.
pub struct ReverbEffect {
    room_size: f32,
    wet: f32,
    dry: f32,
}

impl ReverbEffect {
    /// Constructs a new reverb effect, where `room_size` represents the simulated "room", `wet` is the echo volume, and `dry` is the original signal volume.
    ///
    /// All parameters are defined by the limit `x > 0`, and `x <= 1`.
    pub fn new(room_size: f32, wet: f32, dry: f32) -> Self {
        Self {
            room_size,
            wet,
            dry,
        }
    }
}

impl Effect for ReverbEffect {
    fn modify(&self, sample_rate: u32, input: &[(f32, f32)]) -> Vec<(f32, f32)> {
        let reflections = [
            (0.012, 0.6),
            (0.017, 0.5),
            (0.023, 0.4),
            (0.031, 0.3),
            (0.045, 0.2),
        ];

        let mut output = input.to_vec();

        for &(delay_sec, decay) in &reflections {
            let delay_samples = (delay_sec * self.room_size * sample_rate as f32) as usize;

            for i in delay_samples..output.len() {
                let (dl, dr) = output[i - delay_samples];

                output[i].0 += dl * decay * self.wet;
                output[i].1 += dr * decay * self.wet;
            }
        }

        for i in 0..output.len() {
            output[i].0 = output[i].0 * self.wet + input[i].0 * self.dry;

            output[i].1 = output[i].1 * self.wet + input[i].1 * self.dry;
        }

        output
    }
}

/// A delay effect for audio samples.
pub struct DelayEffect {
    delay_time: f32,
    feedback: f32,
    wet: f32,
    dry: f32,
}

impl DelayEffect {
    /// Constructs a new delay effect, where `delay_time` represents the distance between echoes,
    /// `feedback` controls how many repeats. `wet` is the echo volume, and `dry` is the original signal volume.
    ///
    /// All parameters are defined by the limit `x > 0`, and `x <= 1`.
    pub fn new(delay_time: f32, feedback: f32, wet: f32, dry: f32) -> Self {
        Self {
            delay_time,
            feedback,
            wet,
            dry,
        }
    }
}

impl Effect for DelayEffect {
    fn modify(&self, sample_rate: u32, input: &[(f32, f32)]) -> Vec<(f32, f32)> {
        let delay_samples = (self.delay_time * sample_rate as f32) as usize;

        let mut output = input.to_vec();

        for i in delay_samples..output.len() {
            let (delayed_l, delayed_r) = output[i - delay_samples];

            let echo_l = delayed_l * self.feedback;
            let echo_r = delayed_r * self.feedback;

            output[i].0 = input[i].0 * self.dry + echo_l * self.wet;
            output[i].1 = input[i].1 * self.dry + echo_r * self.wet;
        }

        output
    }
}
