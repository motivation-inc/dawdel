pub trait NoteData {
    fn note_data(&self) -> Vec<Note>;
}

#[macro_export]
macro_rules! note {
    (C, $oct:literal) => {
        (($oct + 1) * 12 + 0)
    };
    (Cs, $oct:literal) => {
        (($oct + 1) * 12 + 1)
    };
    (D, $oct:literal) => {
        (($oct + 1) * 12 + 2)
    };
    (Ds, $oct:literal) => {
        (($oct + 1) * 12 + 3)
    };
    (E, $oct:literal) => {
        (($oct + 1) * 12 + 4)
    };
    (F, $oct:literal) => {
        (($oct + 1) * 12 + 5)
    };
    (Fs, $oct:literal) => {
        (($oct + 1) * 12 + 6)
    };
    (G, $oct:literal) => {
        (($oct + 1) * 12 + 7)
    };
    (Gs, $oct:literal) => {
        (($oct + 1) * 12 + 8)
    };
    (A, $oct:literal) => {
        (($oct + 1) * 12 + 9)
    };
    (As, $oct:literal) => {
        (($oct + 1) * 12 + 10)
    };
    (B, $oct:literal) => {
        (($oct + 1) * 12 + 11)
    };
}

#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: u8,
    pub start: f32,
    pub duration: f32,
    pub velocity: f32,
}

impl Note {
    pub fn new(pitch: u8, start: f32, duration: f32, velocity: u8) -> Self {
        Self {
            pitch,
            start,
            duration,
            velocity: velocity as f32 / 127.0,
        }
    }
}

impl NoteData for Note {
    fn note_data(&self) -> Vec<Note> {
        let mut notes = Vec::new();
        notes.push(self.clone());

        notes
    }
}

#[macro_export]
macro_rules! chord {
    (maj $root:expr) => {
        vec![$root, $root + 4, $root + 7]
    };

    (min $root:expr) => {
        vec![$root, $root + 3, $root + 7]
    };

    (maj7 $root:expr) => {
        vec![$root, $root + 4, $root + 7, $root + 11]
    };

    (min7 $root:expr) => {
        vec![$root, $root + 3, $root + 7, $root + 10]
    };
}

#[derive(Debug, Clone)]
pub struct Chord {
    pub notes: Vec<u8>,
    pub start: f32,
    pub duration: f32,
    pub velocity: f32,
}

impl Chord {
    pub fn new(notes: Vec<u8>, start: f32, duration: f32, velocity: u8) -> Self {
        Self {
            notes,
            start,
            duration,
            velocity: velocity as f32 / 127.0,
        }
    }
}

impl NoteData for Chord {
    fn note_data(&self) -> Vec<Note> {
        let mut notes = Vec::new();

        for i in &self.notes {
            notes.push(Note {
                pitch: i.clone(),
                start: self.start,
                duration: self.duration,
                velocity: self.velocity,
            });
        }

        notes
    }
}
