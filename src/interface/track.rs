use crate::interface::{note::Note, sample::Sample};

/// A track object, containing an audio sample, midi channel, and a Vec of note objects.
#[derive(Debug, Clone)]
pub struct Track {
    bpm: f32,
    sample: Sample,
    channel: u8,
    notes: Vec<Note>,
    current_beat: f32,
}

impl Track {
    /// Constructs a new track object with the specified `sample`, `channel` being midi channels 0-15, and `bpm` being the beats per minute of the track.
    ///
    /// # Example
    ///
    /// ```
    /// let mut song = Song::new(120);
    /// let track1 = Track::new(Sample::new("my_samples/piano.wav", 60), 0, 120);
    ///
    /// assert_eq!(track1.channel(), 0);
    /// ```
    pub fn new(sample: Sample, channel: u8, bpm: f32) -> Self {
        Self {
            bpm,
            sample,
            channel,
            notes: Vec::new(),
            current_beat: 0.0,
        }
    }

    /// Returns the beats per minute of the track (bpm)
    pub fn bpm(&self) -> f32 {
        self.bpm
    }

    /// Returns a reference to the `Sample` object of the track
    pub fn sample(&self) -> &Sample {
        &self.sample
    }

    /// Returns the MIDI channel (0-15) of the track.
    pub fn channel(&self) -> u8 {
        self.channel
    }

    /// Returns a Vec of `Note` objects contained on the track.
    pub fn notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    /// Returns the ending beat of the most recent note.
    pub fn current_beat(&self) -> f32 {
        self.current_beat
    }

    /// Appends a note to the track, with `pitch` midi numbers 1-127, `velocity` midi numbers 1-127, `start` and `duration` in beats.
    ///
    /// # Example
    ///
    /// ```
    /// let mut song = Song::new(120);
    /// let mut track1 = song.track(Sample::new("my_samples/piano.wav", 60), 0);
    ///
    /// track1.note(60, 127, 0.0, 2.0); // C4 at full velocity, played for 2 beats
    /// ```
    pub fn note(&mut self, pitch: u8, velocity: u8, start: f32, duration: f32) {
        if start + duration > self.current_beat {
            self.current_beat = start + duration
        }

        self.notes.push(Note {
            pitch,
            velocity: velocity as f32 / 127.0,
            start,
            duration,
        });
    }

    /// Appends a chord to the track, with `notes` a Vec of indexes midi numbers 1-127, `velocity` midi numbers 1-127, `start` and `duration` in beats.
    ///
    /// # Example
    ///
    /// ```
    /// let mut song = Song::new(120);
    /// let mut track1 = song.track(Sample::new("my_samples/piano.wav", 60), 0);
    ///
    /// track1.chord(vec![60, 64, 67], 127, 0.0, 2.0); // Csus4 at full velocity, played for 2 beats
    /// ```
    pub fn chord(&mut self, notes: Vec<u8>, velocity: u8, start: f32, duration: f32) {
        if start + duration > self.current_beat {
            self.current_beat = start + duration
        }

        for n in notes {
            self.notes.push(Note {
                pitch: n,
                velocity: velocity as f32 / 127.0,
                start,
                duration,
            });
        }
    }
}
