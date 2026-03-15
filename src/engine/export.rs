use std::fs::File;

use crate::interface::{Song, Track};
use midly::num::{u4, u7, u15, u24, u28};
use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};

fn beats_to_ticks(beats: f32, tpq: u16) -> u32 {
    (beats * tpq as f32) as u32
}

/// Renders all track samples + notes into two left and right master buffers
fn render_tracks_wav(tracks: &Vec<Track>) -> (u32, Vec<f32>, Vec<f32>) {
    if tracks.is_empty() {
        return (44100, Vec::new(), Vec::new());
    }

    // assume same sample rate across samples
    let bpm = tracks[0].bpm();
    let sample_rate = tracks[0].sample().sample_rate;
    let song_duration = tracks
        .iter()
        .map(|t| t.current_beat() * (60.0 / bpm))
        .fold(0.0, f32::max);
    let total_samples = (song_duration * (sample_rate) as f32) as usize;

    // mix buffers
    let mut left = vec![0.0f32; total_samples];
    let mut right = vec![0.0f32; total_samples];

    for track in tracks {
        let sample = track.sample();
        let sample_data = &sample.data;
        let root_note = sample.root_note;
        let pan = sample.pan;

        for note in track.notes() {
            let start_index = (note.start * (60.0 / bpm) * sample_rate as f32) as usize;

            let pitch_ratio = 2f32.powf((note.pitch as f32 - root_note as f32) / 12.0);

            let velocity = note.velocity;

            let max_samples = (note.duration * (60.0 / bpm) * sample_rate as f32) as usize;

            let mut i = 0usize;

            while i < max_samples {
                let source_index = (i as f32 * pitch_ratio) as usize;

                if source_index >= sample_data.len() {
                    break;
                }

                let dest_index = start_index + i;

                if dest_index >= total_samples {
                    break;
                }

                let s = sample_data[source_index] * velocity;

                // constant power panning
                let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
                let l = s * angle.cos();
                let r = s * angle.sin();

                left[dest_index] += l;
                right[dest_index] += r;

                i += 1;
            }
        }
    }

    (sample_rate, left, right)
}

pub fn export_wav(name: &str, tracks: &Vec<Track>) {
    let (sample_rate, left, right) = render_tracks_wav(tracks);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec).unwrap();

    for i in 0..left.len() {
        let l = (left[i] * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;

        let r = (right[i] * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;

        writer.write_sample(l).unwrap();
        writer.write_sample(r).unwrap();
    }

    writer.finalize().unwrap()
}

pub fn export_midi(name: &str, song: &Song) {
    let ticks_per_beat: u16 = 480;

    let header = Header {
        format: Format::Parallel,
        timing: Timing::Metrical(u15::from(ticks_per_beat)),
    };

    let mut midi_tracks = Vec::new();

    for track in song.clone().tracks() {
        let mut events: Vec<(u32, TrackEventKind)> = Vec::new();

        for note in track.notes() {
            let start = beats_to_ticks(note.start, ticks_per_beat);
            let end = beats_to_ticks(note.start + note.duration, ticks_per_beat);

            events.push((
                start,
                TrackEventKind::Meta(MetaMessage::Tempo(u24::new(
                    (60_000_000.0 / track.bpm()) as u32,
                ))),
            ));

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

    let file_name = format!("{}.mid", name);
    let mut file = File::create(&file_name).unwrap();
    smf.write_std(&mut file).unwrap();
}
