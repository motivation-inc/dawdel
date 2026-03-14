pub trait NoteData {
    fn note_data(&self) -> Vec<Note>;
}

/// Converts a note name into the corresponding midi note number.
///
/// # Examples
///
/// ```rust
/// let note_num = note!(C, 4);
///
/// assert_eq!(note_num, 60)
/// ```
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

/// A note object, containing pitch (0-127), start time, duration, and velocity.
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: u8,
    pub start: f32,
    pub duration: f32,
    pub velocity: f32,
}

impl Note {
    /// Constructs a new note object, where `pitch` is numbers 0-127, `start` and `duration` are in seconds, and `velocity` is numbers 0-127.
    ///
    /// # Examples
    ///
    /// ```
    /// let note = Note::new(60, 0.0, 2.0, 127); // C4 at 2 seconds long with full velocity
    /// ```
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

/// Converts a chord name into the corresponding midi note numbers.
///
/// # Examples
///
/// ```rust
/// let note_nums = chord!(maj note!(C, 4));
///
/// assert_eq!(note_nums, vec![60, 64, 67])
/// ```
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

/// A chord object, containing a Vec of note pitches (the chord), start time, duration, and velocity.
#[derive(Debug, Clone)]
pub struct Chord {
    pub notes: Vec<u8>,
    pub start: f32,
    pub duration: f32,
    pub velocity: f32,
}

impl Chord {
    /// Constructs a new chord object, where `notes` is a Vec of numbers each index in the range 0-127, `start` and `duration` are in seconds, and `velocity` is numbers 0-127.
    ///
    /// # Examples
    ///
    /// ```
    /// let chord = Chord::new(vec![60, 64, 67], 0.0, 2.0, 127); // Csus4 chord at 2 seconds long with full velocity
    /// ```
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
