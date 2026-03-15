# `dawdel`
dawdel (daa·dell) is a programmatic digital audio workstation crate for MIDI, written in Rust.

```rust
use dawdel::interface::{ExportType, Sample, Song};
use dawdel::{chord, note};

fn main() {
    let mut song = Song::new(120.0); // 120 bpm
    let track1 = song.track(Sample::new("test.wav", 60), 1); // add a new track

    for _ in 0..8 {
        track1.note(note!(C, 4), 127, track1.current_beat(), 2.0);
        track1.chord(chord!(min note!(C, 4)), 127, track1.current_beat(), 2.0);
        track1.note(note!(D, 4), 127, track1.current_beat() - 1.0, 1.0);
    }

    song.export("output", ExportType::MIDI, false); // export the midi file
    song.export("output", ExportType::WAV, true); // export the wav audio and open it
}
```
