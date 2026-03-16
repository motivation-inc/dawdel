use std::fs::File;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::default::{get_codecs, get_probe};

use crate::interface::Effect;

/// A sample object containing sound data, panning, sample rates, and a root note representing the general pitch shift of the sample.
#[derive(Debug, Clone)]
pub struct Sample {
    root_note: u8,
    pan: f32, // -1.0 to +1.0
    sample_rate: u32,
    data: Vec<(f32, f32)>, // L, R
}

impl Sample {
    /// Constructs a new sample object, where `path` is a path to the audio file (`wav`, `mp3`, `.ogg`, `.flac`, `.acc`, and more), and `root_note` is the midi note number (0-127) of the base note.
    ///
    /// # Example
    ///
    /// ```
    /// let sample = Sample::new("my_samples/piano.ogg", 60); // C4 = root note, anything above or below will be pitch shifted
    ///
    /// assert_eq!(sample.root_note, 60);
    /// ```
    pub fn new(path: &str, root_note: u8) -> Self {
        let file = File::open(path).unwrap();
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let hint = symphonia::core::probe::Hint::new();

        let probed = get_probe()
            .format(
                &hint,
                mss,
                &FormatOptions::default(),
                &MetadataOptions::default(),
            )
            .unwrap();

        let mut format = probed.format;
        let track = format.default_track().unwrap();

        let mut decoder = get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())
            .unwrap();

        let sample_rate = track.codec_params.sample_rate.unwrap();

        let mut data = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(_) => break,
            };

            let decoded = decoder.decode(&packet).unwrap();

            match decoded {
                AudioBufferRef::F32(buf) => {
                    let channels = buf.spec().channels.count();

                    for i in 0..buf.frames() {
                        let l = buf.chan(0)[i];
                        let r = if channels > 1 { buf.chan(1)[i] } else { l };
                        data.push((l, r));
                    }
                }

                AudioBufferRef::S16(buf) => {
                    let channels = buf.spec().channels.count();

                    for i in 0..buf.frames() {
                        let l = buf.chan(0)[i] as f32 / i16::MAX as f32;
                        let r = if channels > 1 {
                            buf.chan(1)[i] as f32 / i16::MAX as f32
                        } else {
                            l
                        };

                        data.push((l, r));
                    }
                }

                _ => {}
            }
        }

        Self {
            root_note,
            pan: 0.0,
            sample_rate,
            data,
        }
    }

    /// Adds `effect` to the audio data, modifying it using the `Effect` trait's `modify` method.
    pub fn add_effect<T>(&mut self, effect: T)
    where
        T: Effect,
    {
        self.data = effect.modify(self.sample_rate, &self.data);
    }

    /// Returns the root note of the sample.
    pub fn root_note(&self) -> u8 {
        self.root_note
    }

    /// Returns the panning of the sample.
    pub fn pan(&self) -> f32 {
        self.pan
    }

    /// Returns the audio sample rate of the sample.
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Returns the audio data (left and right stereo channels) of the sample.
    pub fn data(&self) -> &[(f32, f32)] {
        &self.data
    }

    /// Set the root note pitch of the sample (midi numbers 0-127)
    pub fn set_root_note(&mut self, root_note: u8) {
        self.root_note = root_note;
    }

    /// Set the stereo panning of the sample (-1.0 to 1.0, where negative values pan left, and positive values pan right)
    ///
    /// By default, panning is set to 0.0 (centered) when using `Sample::new()`
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan
    }
}
