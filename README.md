# `dawdel`
dawdel (daa·dell) is a programmatic digital audio workstation crate for MIDI, written in Rust.

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
    song.export("output", ExportType::MIDI, false); // export the midi file
    song.export("output", ExportType::WAV(44100), true); // export the wav audio at 44100 samples and open it
}
```
