use dawdel::{ExportType, Sample, Song};

fn main() {
    let mut song = Song::new(80.0); // 120 bpm

    // load a midi file with a specific audio sample
    song.load_midi(
        "examples/test.mid",
        Sample::new("examples/test.wav", 60),
        0.0,
    );

    song.export("output.wav", ExportType::WAV(44100), true); // export the wav audio at 44100 samples and open it
}
