use num_traits::Float;

/// Note names for MIDI notes
pub const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// MIDI note names
/// # Example
///
/// ```
/// use music_math::midi;
/// let note = 69;
/// let note_name = midi::MIDI_NOTE_NAMES.get(note as usize).unwrap();
/// assert_eq!(*note_name, "A4");
/// ```
pub const MIDI_NOTE_NAMES: [&str; 128] = [
    "C-1", "C#-1", "D-1", "D#-1", "E-1", "F-1", "F#-1", "G-1", "G#-1", "A-1", "A#-1", "B-1", "C0",
    "C#0", "D0", "D#0", "E0", "F0", "F#0", "G0", "G#0", "A0", "A#0", "B0", "C1", "C#1", "D1",
    "D#1", "E1", "F1", "F#1", "G1", "G#1", "A1", "A#1", "B1", "C2", "C#2", "D2", "D#2", "E2", "F2",
    "F#2", "G2", "G#2", "A2", "A#2", "B2", "C3", "C#3", "D3", "D#3", "E3", "F3", "F#3", "G3",
    "G#3", "A3", "A#3", "B3", "C4", "C#4", "D4", "D#4", "E4", "F4", "F#4", "G4", "G#4", "A4",
    "A#4", "B4", "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5", "A5", "A#5", "B5", "C6",
    "C#6", "D6", "D#6", "E6", "F6", "F#6", "G6", "G#6", "A6", "A#6", "B6", "C7", "C#7", "D7",
    "D#7", "E7", "F7", "F#7", "G7", "G#7", "A7", "A#7", "B7", "C8", "C#8", "D8", "D#8", "E8", "F8",
    "F#8", "G8", "G#8", "A8", "A#8", "B8", "C9", "C#9", "D9", "D#9", "E9", "F9", "F#9", "G9",
];

/// Get the note name for a given midi note
#[must_use]
pub fn get_midi_note_name(note: u8) -> Option<&'static str> {
    MIDI_NOTE_NAMES.get(usize::from(note)).copied()
}

/// Get the midi note for a given note name
/// # Panics
/// If a type conversion fails.
#[must_use]
pub fn get_midinote_from_name(note: &str) -> Option<u8> {
    MIDI_NOTE_NAMES
        .iter()
        .position(|&x| x == note)
        .map(|x| x.try_into().expect("Could not convert usize to u8"))
}

/// Convert a note name to it's position in the octave
/// # Example
/// ```
/// use music_math::midi;
/// let note = "C";
/// let position = midi::note_name_to_octave_position(note).unwrap();
/// assert_eq!(position, 0);
/// ```
#[inline]
#[must_use]
pub fn note_name_to_octave_position(note: &str) -> Option<u8> {
    let note = note.to_uppercase();
    let note = NOTE_NAMES.iter().position(|&x| x == note)?;

    note.try_into().ok()
}

/// Convert an octave position to a note name
/// # Example
/// ```
/// use music_math::midi;
/// let note = 0;
/// let note_name = midi::octave_position_to_note_name(note).unwrap();
/// assert_eq!(note_name, "C");
/// ```
#[inline]
#[must_use]
pub fn octave_position_to_note_name(note: u8) -> Option<&'static str> {
    let index = usize::from(note) % 12;
    NOTE_NAMES.get(index).copied()
}

/// Map a midi note to a frequency
/// # Panics
/// If a type conversion fails.
#[inline]
#[must_use]
pub fn to_frequency<T: Float>(note: u8) -> T {
    let note = T::from(note).unwrap();
    let base: T = T::from(440.0).unwrap();
    let exp_base: T = T::from(2.0).unwrap();
    let divisor: T = T::from(12.0).unwrap();
    let subtractor: T = T::from(69.0).unwrap();

    base.mul(exp_base.powf(note.sub(subtractor).div(divisor)))
}

/// Transpose a midi value, safely keeping it within the bounds of the range 0-127.
///
/// If the resulting value is out of range or the addition operation overflows, it returns a None, otherwise a Some containing the
/// transposed value.
pub fn transpose<T: num_traits::PrimInt>(note: T, semitones: T) -> Option<T> {
    let max = T::from(127);

    max.and_then(|ok_max| {
        let transposed = note.checked_add(&semitones);

        match transposed {
            Some(transposed) if transposed >= T::zero() && transposed <= ok_max => Some(transposed),
            _ => None,
        }
    })
}
