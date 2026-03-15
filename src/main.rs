use dawdel::interface::{ExportType, Sample, Song};
use dawdel::{chord, note};

fn main() {
    let mut song = Song::new(120.0);
    let track1 = song.track(Sample::new("test.wav", 60), 1);

    for _ in 0..8 {
        track1.chord(
            chord!(maj note!(C, 4)),
            127,
            track1.current_beat() + 1.0,
            2.0,
        );
        track1.chord(
            chord!(min note!(C, 4)),
            127,
            track1.current_beat() + 1.0,
            2.0,
        );
    }

    song.export("output", ExportType::WAV, true);
}
