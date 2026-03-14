#[derive(Debug, Clone)]
pub struct Sample {
    name: String,
    wav_path: String,
    root_note: u8, // MIDI note number
}

impl Sample {
    pub fn new(name: &str, wav_path: &str, root_note: u8) -> Self {
        Self {
            name: name.to_string(),
            wav_path: wav_path.to_string(),
            root_note,
        }
    }
}
