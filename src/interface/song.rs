use crate::engine::{export_midi, export_wav};
use crate::interface::Sample;
use crate::interface::track::Track;
use midly::{MidiMessage, Smf, Timing, TrackEventKind};
use std::collections::HashMap;
use std::fs::{self};

/// Options for exporting song objects.
///
/// - `MIDI` describes a `.mid` file.
/// - `WAV(u32)` describes a `.wav` file with the specified sample rate `u32`.
pub enum ExportType {
    MIDI,
    WAV(u32),
}

/// A song object containing all track objects and a BPM variable.
#[derive(Debug, Clone)]
pub struct Song {
    bpm: f32,
    tracks: Vec<Track>,
}

impl Song {
    /// Constructs a new song object, where `bpm` is beats per minute.
    pub fn new(bpm: f32) -> Self {
        Self {
            bpm,
            tracks: Vec::new(),
        }
    }

    /// Loads the midi `file` into seperated tracks using `sample`, loading the midi starting at `starting_beat`.
    ///
    /// # Example
    ///
    /// ```
    /// let mut song = Song::new(120);
    /// song.load_midi("Bach.mid", Sample::new("my_samples/piano.wav", 60), 2.0); // load the midi data at 2 beats with a piano sound sample.
    /// ```
    pub fn load_midi(&mut self, file: &str, sample: Sample, starting_beat: f32) {
        let data = fs::read(file).expect("Failed to read MIDI file");
        let smf = Smf::parse(&data).expect("Invalid MIDI");

        let ticks_per_beat = match smf.header.timing {
            Timing::Metrical(t) => t.as_int() as f32,
            _ => panic!("Unsupported MIDI timing format"),
        };

        // active notes: (channel, pitch) -> (start_tick, velocity)
        let mut active_notes: HashMap<(u8, u8), (u32, u8)> = HashMap::new();

        for midi_track in smf.tracks {
            let mut absolute_tick: u32 = 0;

            for event in midi_track {
                absolute_tick += event.delta.as_int();

                if let TrackEventKind::Midi { channel, message } = event.kind {
                    match message {
                        MidiMessage::NoteOn { key, vel } if vel > 0 => {
                            active_notes.insert(
                                (channel.as_int(), key.as_int()),
                                (absolute_tick, vel.as_int()),
                            );
                        }

                        MidiMessage::NoteOff { key, vel } | MidiMessage::NoteOn { key, vel }
                            if vel == 0 =>
                        {
                            if let Some((start_tick, velocity)) =
                                active_notes.remove(&(channel.as_int(), key.as_int()))
                            {
                                let duration_ticks = absolute_tick - start_tick;

                                let start_beats =
                                    (start_tick as f32 / ticks_per_beat) + starting_beat;
                                let duration_beats =
                                    (duration_ticks as f32 / ticks_per_beat) + starting_beat;

                                let velocity_norm = velocity as f32 / 127.0;

                                if let Some(track) = self
                                    .tracks
                                    .iter_mut()
                                    .find(|t| t.channel() == channel.as_int())
                                {
                                    track.note(
                                        key.as_int(),
                                        (velocity_norm * 127.0) as u8,
                                        start_beats,
                                        duration_beats,
                                    );
                                } else {
                                    let mut track =
                                        Track::new(sample.clone(), channel.as_int(), self.bpm);

                                    track.note(
                                        key.as_int(),
                                        (velocity_norm * 127.0) as u8,
                                        start_beats,
                                        duration_beats,
                                    );

                                    self.tracks.push(track);
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }

    /// Constructs a new track object and appends it to the song.
    ///
    /// # Example
    ///
    /// ```
    /// let mut song = Song::new(120);
    /// let track1 = song.track(Sample::new("my_samples/piano.wav", 60), 0);
    ///
    /// assert_eq!(track1.channel(), 0);
    /// ```
    pub fn track(&mut self, sample: Sample, channel: u8) -> &mut Track {
        self.tracks.push(Track::new(sample, channel, self.bpm));
        self.tracks.last_mut().unwrap()
    }

    /// Returns the beats per minute (bpm) of the song.
    ///
    /// # Example
    ///
    /// ```
    /// let song = Song::new(120);
    ///
    /// assert_eq!(song.bpm(), 120);
    /// ```
    pub fn bpm(&self) -> f32 {
        self.bpm
    }

    /// Returns all track objects contained in the song.
    ///
    /// # Example
    ///
    /// ```
    /// let mut song = Song::new(120);
    /// let track1 = song.track(Sample::new("my_samples/piano.wav", 60), 0);
    ///
    /// assert_eq!(song.tracks()[0].channel(), 0);
    /// ```
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    pub fn export(&self, name: &str, export_type: ExportType, open_in_default_app: bool) {
        match export_type {
            ExportType::MIDI => {
                export_midi(name, self);
                let file_name = format!("{}.mid", name);

                if open_in_default_app {
                    open::that(&file_name).expect("Error opening export");
                }
            }
            ExportType::WAV(sample_rate) => {
                export_wav(name, sample_rate, self);
                let file_name = format!("{}.wav", name);

                if open_in_default_app {
                    open::that(&file_name).expect("Error opening export");
                }
            }
            _ => {}
        }
    }
}
