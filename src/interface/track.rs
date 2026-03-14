use crate::interface::{
    note_data::{Note, NoteData},
    sample::Sample,
};

pub struct Track {
    sample: Sample,
    channel: u8,
    notes: Vec<Note>,
    current_time: f32,
}

impl Track {
    pub fn new(sample: Sample, channel: u8) -> Self {
        Self {
            sample,
            channel,
            notes: Vec::new(),
            current_time: 0.0,
        }
    }

    /// Returns the `Sample` object of the track
    pub fn sample(self) -> Sample {
        self.sample
    }

    /// Returns the MIDI channel (0-15) of the track
    pub fn channel(&self) -> u8 {
        self.channel
    }

    /// Returns an array of `Note` objects contained on the track
    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    /// Returns the ending time of the most recent note
    pub fn current_time(&self) -> f32 {
        self.current_time
    }

    pub fn add<T>(&mut self, data: T)
    where
        T: NoteData,
    {
        for n in data.note_data() {
            if n.start + n.duration > self.current_time {
                self.current_time = n.start + n.duration
            }

            self.notes.push(n);
        }
    }
}
