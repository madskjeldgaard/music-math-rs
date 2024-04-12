use approx::assert_relative_eq;
use music_math::binaryops::*;
use music_math::interpolation::*;
use music_math::midi;
use music_math::scaling::*;
use pretty_assertions::assert_eq;

#[test]
fn test_linear_interpolation() {
    // Test with floating point numbers
    assert_eq!(linear(0.0, 1.0, 0.5), 0.5);
    assert_eq!(linear(0.0, 1.0, 0.0), 0.0);
    assert_eq!(linear(0.0, 1.0, 1.0), 1.0);

    // Test with negative numbers
    assert_eq!(linear(-1.0, 1.0, 0.5), 0.0);
    assert_eq!(linear(-1.0, 1.0, 0.0), -1.0);
    assert_eq!(linear(-1.0, 1.0, 1.0), 1.0);
}

#[test]
fn test_hermite_interpolation() {
    // Test with floating point numbers
    assert_eq!(hermite(0.0, 1.0, 2.0, 3.0, 0.5), 1.5);
    assert_eq!(hermite(0.0, 1.0, 2.0, 3.0, 0.0), 1.0);
    assert_eq!(hermite(0.0, 1.0, 2.0, 3.0, 1.0), 2.0);

    // Test with negative numbers
    assert_eq!(hermite(-1.0, 0.0, 1.0, 2.0, 0.5), 0.5);
    assert_eq!(hermite(-1.0, 0.0, 1.0, 2.0, 0.0), 0.0);
    assert_eq!(hermite(-1.0, 0.0, 1.0, 2.0, 1.0), 1.0);
}

#[test]
fn note_names_array() {
    let note_names = midi::NOTE_NAMES;
    assert_eq!(note_names.len(), 12);
    assert_eq!(note_names[0], "C");
    assert_eq!(note_names[1], "C#");
    assert_eq!(note_names[2], "D");
    assert_eq!(note_names[3], "D#");
    assert_eq!(note_names[4], "E");
    assert_eq!(note_names[5], "F");
    assert_eq!(note_names[6], "F#");
    assert_eq!(note_names[7], "G");
    assert_eq!(note_names[8], "G#");
    assert_eq!(note_names[9], "A");
    assert_eq!(note_names[10], "A#");
    assert_eq!(note_names[11], "B");
}

#[test]
fn midi_note_names_array() {
    let midi_note_names = midi::MIDI_NOTE_NAMES;
    assert_eq!(midi_note_names.len(), 128);
    assert_eq!(midi_note_names[69], "A4");
    assert_eq!(midi_note_names[0], "C-1");
    assert_eq!(midi_note_names[127], "G9");
}

#[test]
fn get_midi_note_name() {
    let note = 69;
    let note_name = midi::get_midi_note_name(note).unwrap();
    assert_eq!(note_name, "A4");

    // Try an out of bounds note
    let note = 128;
    let note_name = midi::get_midi_note_name(note);
    assert_eq!(note_name, None);
}

#[test]
fn get_midi_note_num_from_name() {
    let note_name = "A4";
    let note = midi::get_midinote_from_name(note_name).unwrap();
    assert_eq!(note, 69);

    // Try an out of bounds note
    let note_name = "A#9";
    let note = midi::get_midinote_from_name(note_name);
    assert_eq!(note, None);
}

#[test]
fn test_note_name_to_oct_pos() {
    let note = "C";
    let position = midi::note_name_to_octave_position(note).unwrap();
    assert_eq!(position, 0);

    let note = "D";
    let position = midi::note_name_to_octave_position(note).unwrap();
    assert_eq!(position, 2);
}

#[test]
fn test_oct_pos_to_note_name() {
    let note = 0;
    let note_name = midi::octave_position_to_note_name(note).unwrap();
    assert_eq!(note_name, "C");

    let note = 2;
    let note_name = midi::octave_position_to_note_name(note).unwrap();
    assert_eq!(note_name, "D");
}

#[test]
fn test_midi_to_frequency() {
    assert_relative_eq!(midi::to_frequency::<f32>(69), 440.0, epsilon = 0.1);
    assert_relative_eq!(midi::to_frequency::<f32>(44), 103.83, epsilon = 0.1);
    assert_relative_eq!(midi::to_frequency::<f32>(1), 8.66, epsilon = 0.1);
    assert_relative_eq!(midi::to_frequency::<f32>(100), 2637.02, epsilon = 0.1);
}

#[test]
fn test_clip() {
    assert_eq!(clip(5, 0, 10), 5);
    assert_eq!(clip(5, 10, 20), 10);
    assert_eq!(clip(5, 0, 4), 4);
    assert_eq!(clip(-10, 0, 4), 0);
    assert_eq!(clip(0.25, 0.1, 0.125), 0.125);
    assert_eq!(clip(-0.5, 0.1, 0.125), 0.1);
}

#[test]
fn test_wrap() {
    assert_eq!(wrap::<f32>(-1.0, 0.0, 10.0), 9.0);
    assert_eq!(wrap::<f32>(11.0, 0.0, 10.0), 1.0);
    assert_eq!(wrap::<f32>(5.0, 0.0, 10.0), 5.0);
}

#[test]
fn test_fold() {
    assert_eq!(fold::<f32>(-1.0, 0.0, 10.0), 1.0);
    assert_eq!(fold::<f32>(11.0, 0.0, 10.0), 9.0);
    assert_eq!(fold::<f32>(5.0, 0.0, 10.0), 5.0);
}

#[test]
fn test_linlin() {
    assert_relative_eq!(linlin(0.5, 0.0, 1.0, 0.0, 10.0), 5.0, epsilon = 0.1);
    assert_relative_eq!(linlin(0.5, 0.0, 1.0, 0.0, 100.0), 50.0, epsilon = 0.1);
}

#[test]
fn test_linexp_1() {
    let result = linexp(0.5f32, 0.0f32, 1.0f32, 1.0f32, 10.0f32, 2.0f32);
    assert_eq!(result, 3.25);
}

#[test]
fn test_linexp_2() {
    let result = linexp(0.2f32, 0.0f32, 1.0f32, 1.0f32, 10.0f32, 2.0f32);
    assert_eq!(result, 1.36);
}

#[test]
fn test_linexp_3() {
    let result = linexp(0.8f32, 0.0f32, 1.0f32, 1.0f32, 10.0f32, 2.0f32);
    assert_eq!(result, 6.76);
}

#[test]
fn test_transpose_midi() {
    assert_eq!(
        midi::transpose(69, 12).expect("Could not transpose midi"),
        81
    );
    assert_eq!(
        midi::transpose(69, -12).expect("Could not transpose midi"),
        57
    );
    assert_eq!(
        midi::transpose(69, 0).expect("Could not transpose midi"),
        69
    );
    assert_eq!(midi::transpose(69, 100).is_none(), true);
}

#[test]
fn test_ampdb() {
    assert_relative_eq!(ampdb(0.5), -6.02, epsilon = 0.1);
    assert_relative_eq!(ampdb(0.25), -12.04, epsilon = 0.1);
    assert_relative_eq!(ampdb(0.125), -18.06, epsilon = 0.1);
}

#[test]
fn test_dbamp() {
    assert_relative_eq!(dbamp(-6.02), 0.5, epsilon = 0.1);
    assert_relative_eq!(dbamp(-12.04), 0.25, epsilon = 0.1);
    assert_relative_eq!(dbamp(-18.06), 0.125, epsilon = 0.1);
}
