# `dawdel`
Dawdel (daa·dell) is a programmatic music generation crate for Rust. It provides a "DAW-like" interface for working with samples, tracks, notes, exporting, and more.

```rust
use dawdel::{ExportType, Sample, Song};
use dawdel::{chord, note};

fn main() {
    let mut song = Song::new(120.0); // 120 bpm
    let mut track1 = song.create_track(Sample::new("test.wav", 60), 1); // construct a new track

    for _ in 0..8 {
        track1.note(note!(C, 4), 127, track1.current_beat(), 2.0);
        track1.chord(chord!(min note!(C, 4)), 127, track1.current_beat(), 2.0);
        track1.note(note!(D, 4), 127, track1.current_beat() - 1.0, 1.0);
    }

    song.add_track(track1);
    song.export("output.mid", ExportType::MIDI, false); // export the midi file
    song.export("output.wav", ExportType::WAV(44100), true); // export the wav file at 44100 samples and open it
}
```

## About
Dawdel is both an audio processing engine and MIDI toolkit. Its a simple, yet very powerful toolkit designed for working with:

- **Samples** Create samples + effects from all audio formats supported by [Symphonia](https://github.com/pdeljanov/Symphonia).
- **Tracks** Organize music into seperate tracks and control MIDI channeling. 
- **Exporting** Render both `.wav` and `.mid` formats (`.mp3` support coming in future updates.)

## Notes And Chords
Dawdel supports using standard `u8` types to represent MIDI notes 0-127, but it also includes helper macros like `note!()` and `chord!()` so actual music notation can be used.

```rust
use dawdel::{note, chord};

assert_eq!(note!(C, 4), 60);
assert_eq!(chord!(maj note!(C, 4)), vec![60, 64, 67]);
```

## Custom Effects
Dawdel includes the `Effect` trait, a configurable trait that can be applied to Rust structs to create your own audio effects. 

```rust
use dawdel::Effect;

pub struct SineEffect {
    amount_radians: f32;
}

impl Effect for SineEffect {
    fn modify(&self, sample_rate: u32, data: &[(f32, f32)]) -> Vec<(f32, f32)> {
        let mut new_data = Vec::new();
        
        for (left, right) in data {
            new_data.push(left + amount_radians.sin(), right + amount_radians.cos())
        }
        
        new_data
    }
}
```

# Free & Open-Source

Dawdel is 100% free with no drawbacks or limitations. There is no "premium" version; you get the latest and greatest, all licensed under the GPL-3.0.

All source code is public, to anyone. There is no "hidden mechanism" included in this repository; every reference and used factor exists completely and fully.
