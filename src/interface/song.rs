use crate::interface::track::Track;
use midly::num::{u4, u7, u15, u28};
use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};
use std::fs::File;

pub enum ExportType {
    MIDI,
    WAV,
}

pub struct Song {
    bpm: f32,
    tracks: Vec<Track>,
}

fn beats_to_ticks(beats: f32, tpq: u16) -> u32 {
    (beats * tpq as f32) as u32
}

impl Song {
    pub fn new(bpm: f32) -> Self {
        Self {
            bpm,
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track)
    }

    pub fn export(&self, name: &str, export_type: ExportType) {
        match export_type {
            ExportType::MIDI => self.export_midi(name),
            _ => {}
        }
    }

    fn export_midi(&self, name: &str) {
        let ticks_per_beat: u16 = 480;

        let header = Header {
            format: Format::Parallel,
            timing: Timing::Metrical(u15::from(ticks_per_beat)),
        };

        let mut midi_tracks = Vec::new();

        for track in &self.tracks {
            let mut events: Vec<(u32, TrackEventKind)> = Vec::new();

            for note in track.notes() {
                let start = beats_to_ticks(note.start, ticks_per_beat);
                let end = beats_to_ticks(note.start + note.duration, ticks_per_beat);

                let velocity = (note.velocity * 127.0) as u8;

                // NOTE ON
                events.push((
                    start,
                    TrackEventKind::Midi {
                        channel: u4::from(track.channel()),
                        message: MidiMessage::NoteOn {
                            key: u7::from(note.pitch),
                            vel: u7::from(velocity),
                        },
                    },
                ));

                // NOTE OFF
                events.push((
                    end,
                    TrackEventKind::Midi {
                        channel: u4::from(track.channel()),
                        message: MidiMessage::NoteOff {
                            key: u7::from(note.pitch),
                            vel: u7::from(0),
                        },
                    },
                ));
            }

            // Sort events by time
            events.sort_by_key(|e| e.0);

            // Convert absolute time → delta time
            let mut last_time = 0;
            let mut track_events = Vec::new();

            for (time, kind) in events {
                let delta = time - last_time;
                last_time = time;

                track_events.push(TrackEvent {
                    delta: u28::from(delta),
                    kind,
                });
            }

            // End of track event
            track_events.push(TrackEvent {
                delta: u28::from(0),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            });

            midi_tracks.push(track_events);
        }

        let smf = Smf {
            header,
            tracks: midi_tracks,
        };

        let mut file = File::create(format!("{}.mid", name)).unwrap();
        smf.write_std(&mut file).unwrap();
    }
}
