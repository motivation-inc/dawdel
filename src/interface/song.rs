use crate::engine::{export_midi, export_wav};
use crate::interface::Sample;
use crate::interface::track::Track;
use midly::{MidiMessage, Smf, Timing, TrackEventKind};
use std::collections::HashMap;
use std::fs::{self};

/// Describes the song export type.
pub enum ExportType {
    /// The MIDI file format.
    MIDI,
    /// The WAV file format.
    WAV(u32),
}

/// A song object.
#[derive(Debug, Clone)]
pub struct Song {
    bpm: f32,
    tracks: Vec<Track>,
}

impl Song {
    /// Constructs a new song object, where `bpm` is beats per minute.
    ///
    /// # Example
    ///
    /// ```
    /// use dawdel::Song;
    ///
    /// let song = Song::new(120.0); // constructs a new song at 120 bpm
    /// ```
    pub fn new(bpm: f32) -> Self {
        Self {
            bpm,
            tracks: Vec::new(),
        }
    }

    /// Loads the midi `file` into seperated tracks.
    ///
    /// - `sample`: the `Sample` object to use during loading
    /// - `starting_beat`: the starting beat (where the track loads in at)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dawdel::{Song, Sample};
    ///
    /// let mut song = Song::new(120.0);
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

    /// Constructs a new track object using the song bpm.
    ///
    /// - `sample`: the track's `Sample` object
    /// - `channel`: the MIDI channel (0-15) of the track
    ///
    /// _This function will not append the track object to the song; use `Song::add_track` to
    /// append the track to the song._
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dawdel::{Song, Sample};
    ///
    /// let mut song = Song::new(120.0);
    /// let track1 = song.create_track(Sample::new("my_samples/piano.wav", 60), 0);
    ///
    /// assert_eq!(track1.bpm(), 120.0);
    /// ```
    pub fn create_track(&self, sample: Sample, channel: u8) -> Track {
        Track::new(sample, channel, self.bpm)
    }

    /// Appends a track to the song.
    ///
    /// - `track`: the `Track` object to append
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dawdel::{Song, Sample};
    ///
    /// let mut song = Song::new(120.0);
    /// let track1 = song.create_track(Sample::new("my_samples/piano.wav", 60), 0);
    ///
    /// song.add_track(track1);
    /// assert_eq!(song.tracks()[0].bpm(), 120.0);
    /// ```
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Returns the beats per minute (bpm) of the song.
    ///
    /// # Example
    ///
    /// ```
    /// use dawdel::Song;
    ///
    /// let song = Song::new(120.0);
    /// assert_eq!(song.bpm(), 120.0);
    /// ```
    pub fn bpm(&self) -> f32 {
        self.bpm
    }

    /// Returns all track objects contained in the song.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dawdel::{Song, Sample};
    ///
    /// let mut song = Song::new(120.0);
    /// let track1 = song.create_track(Sample::new("my_samples/piano.wav", 60), 0);
    /// song.add_track(track1);
    ///
    /// assert_eq!(song.tracks()[0].channel(), 0);
    /// ```
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// Export all tracks contained in the song.
    ///
    /// - `filename`: the file name to write to
    /// - `export_type`: the audio format to export in
    /// - `open_in_default_app`: open the exported file in the system's default app for that file type
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dawdel::{Song, Sample, ExportType, note};
    ///
    /// let mut song = Song::new(120.0);
    /// let mut track1 = song.create_track(Sample::new("my_samples/piano.wav", 60), 0);
    /// track1.note(note!(C, 4), 127, track1.current_beat(), 2.0);
    ///
    /// song.add_track(track1);
    /// song.export("my_song.mid", ExportType::MIDI, true); // exports `my_song.mid` and opens it in a default application
    /// ```
    pub fn export(&self, filename: &str, export_type: ExportType, open_in_default_app: bool) {
        match export_type {
            ExportType::MIDI => {
                export_midi(filename, self);

                if open_in_default_app {
                    open::that(&filename).expect("Error opening export");
                }
            }
            ExportType::WAV(sample_rate) => {
                export_wav(filename, sample_rate, self);

                if open_in_default_app {
                    open::that(&filename).expect("Error opening export");
                }
            }
        }
    }
}
