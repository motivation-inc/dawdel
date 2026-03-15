use hound::WavReader;

/// A sample object based on a `.wav` file and root note representing the general pitch shift of the sound sample.
#[derive(Debug, Clone)]
pub struct Sample {
    pub root_note: u8,
    pub pan: f32, // -1.0 to +1.0
    pub sample_rate: u32,
    pub data: Vec<f32>,
}

impl Sample {
    /// Constructs a new sample object, where `wav_path` is the path to the WAV file, and `root_note` is the midi note number (0-127) of the base note.
    ///
    /// # Example
    ///
    /// ```
    /// let sample = Sample::new("my_samples/piano.wav", 60); // C4 = root note, anything above or below will be pitch shifted
    /// ```
    pub fn new(wav_path: &str, root_note: u8) -> Self {
        let mut reader = WavReader::open(wav_path).unwrap();
        let sample_rate = reader.spec().sample_rate;
        let data: Vec<f32> = reader
            .samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect();

        Self {
            root_note,
            pan: 0.0,
            sample_rate: sample_rate,
            data,
        }
    }

    /// Set the stereo panning of the sample (-1.0 to 1.0, where negative values pan left, and positive values pan right)
    ///
    /// By default, panning is set to 0.0 (centered) when using `Sample::new()`
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan
    }
}
