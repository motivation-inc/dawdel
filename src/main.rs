use dawdel::interface::{Chord, ExportType, Note, Sample, Song, Track};
use dawdel::{chord, note};

fn main() {
    let mut song = Song::new(120.0);
    let mut track1 = Track::new(Sample::new("piano", "test.wav", 60), 10);
    track1.add(Chord::new(chord!(maj note!(C, 4)), 0.0, 2.0, 127));

    song.add_track(track1);
    song.export("output", ExportType::MIDI);
}
