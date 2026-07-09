use crate::note::PitchClass;

const MAX_FRET: i8 = 24;

/// Convert a string's open MIDI note and a fret number to the resulting MIDI note.
pub fn fret_to_midi(open_string_midi: u8, fret: u8) -> u8 {
    open_string_midi + fret
}

/// Convert a string's open MIDI note and a fret number to a pitch class.
pub fn fret_to_pitch_class(open_string_midi: u8, fret: u8) -> PitchClass {
    PitchClass::from_midi(fret_to_midi(open_string_midi, fret))
}

/// Shift all played notes by `direction` frets (+1 = right/up neck, -1 = left/down neck).
/// Returns None if ANY played note would go below 0 or above 24.
/// Frets of -1 (not played) are left unchanged.
pub fn transpose_fretboard(frets: &[i8; 6], direction: i8) -> Option<[i8; 6]> {
    let mut result = *frets;
    for f in result.iter_mut() {
        if *f >= 0 {
            let new_fret = *f as i16 + direction as i16;
            if new_fret < 0 || new_fret > MAX_FRET as i16 {
                return None;
            }
            *f = new_fret as i8;
        }
    }
    Some(result)
}

/// Shift all played notes to adjacent strings while preserving pitch.
///
/// direction: +1 = shift each note toward higher strings (higher indices, toward high E)
/// direction: -1 = shift each note toward lower strings (lower indices, toward low E)
///
/// For each played string at index i with fret f:
///   1. Compute current MIDI: tuning[i] + f
///   2. New string index: i + direction
///   3. If new index out of bounds, reject entire operation
///   4. New fret on target string: current_midi - tuning[new_index]
///   5. If new fret < 0 or > 24, reject entire operation
///
/// Returns None if the operation cannot be completed.
pub fn transpose_strings(frets: &[i8; 6], tuning: &[u8; 6], direction: i8) -> Option<[i8; 6]> {
    let mut result: [i8; 6] = [-1; 6];

    for (i, &fret) in frets.iter().enumerate() {
        if fret < 0 {
            continue;
        }

        let new_index = i as i8 + direction;
        if new_index < 0 || new_index >= 6 {
            return None;
        }
        let new_index = new_index as usize;

        let current_midi = tuning[i] as i16 + fret as i16;
        let new_fret = current_midi - tuning[new_index] as i16;

        if new_fret < 0 || new_fret > MAX_FRET as i16 {
            return None;
        }

        result[new_index] = new_fret as i8;
    }

    Some(result)
}

/// Find all fretboard positions where any of the given pitch classes can be played.
/// Returns Vec of (string_index, fret_number) pairs.
/// string_index uses our convention: 0 = lowest string (string 6).
pub fn chord_tone_positions(
    pitch_classes: &[PitchClass],
    tuning: &[u8; 6],
    fret_start: u8,
    fret_end: u8,
) -> Vec<(usize, u8)> {
    let mut positions = Vec::new();
    for (string_idx, &open_midi) in tuning.iter().enumerate() {
        for fret in fret_start..=fret_end {
            let pc = fret_to_pitch_class(open_midi, fret);
            if pitch_classes.contains(&pc) {
                positions.push((string_idx, fret));
            }
        }
    }
    positions
}

/// Given a fret array and tuning, extract the played notes.
/// Returns Vec of (string_index, midi_note, pitch_class) for each played string,
/// ordered from lowest string to highest (index 0 to 5).
pub fn played_notes(frets: &[i8; 6], tuning: &[u8; 6]) -> Vec<(usize, u8, PitchClass)> {
    let mut notes = Vec::new();
    for (i, &fret) in frets.iter().enumerate() {
        if fret < 0 {
            continue;
        }
        let midi = tuning[i] + fret as u8;
        let pc = PitchClass::from_midi(midi);
        notes.push((i, midi, pc));
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuning;

    // -----------------------------------------------------------------------
    // fret_to_midi
    // -----------------------------------------------------------------------

    #[test]
    fn fret_to_midi_open_strings_standard() {
        // Standard tuning open strings: E2=40, A2=45, D3=50, G3=55, B3=59, E4=64
        assert_eq!(fret_to_midi(40, 0), 40);
        assert_eq!(fret_to_midi(45, 0), 45);
        assert_eq!(fret_to_midi(64, 0), 64);
    }

    #[test]
    fn fret_to_midi_fretted_notes() {
        // Low E string, fret 5 = A2 = MIDI 45
        assert_eq!(fret_to_midi(40, 5), 45);
        // A string, fret 7 = E3 = MIDI 52
        assert_eq!(fret_to_midi(45, 7), 52);
        // High E string, fret 12 = E5 = MIDI 76
        assert_eq!(fret_to_midi(64, 12), 76);
    }

    #[test]
    fn fret_to_midi_high_frets() {
        // Low E string, fret 24 = E4 = MIDI 64
        assert_eq!(fret_to_midi(40, 24), 64);
        // High E string, fret 24 = E6 = MIDI 88
        assert_eq!(fret_to_midi(64, 24), 88);
    }

    // -----------------------------------------------------------------------
    // fret_to_pitch_class
    // -----------------------------------------------------------------------

    #[test]
    fn fret_to_pitch_class_open_low_e() {
        // E2 (MIDI 40) -> pitch class 4 (E)
        let pc = fret_to_pitch_class(40, 0);
        assert_eq!(pc, PitchClass::new(4));
        assert_eq!(pc.name(), "E");
    }

    #[test]
    fn fret_to_pitch_class_fret_1_low_e() {
        // E string fret 1 = F
        let pc = fret_to_pitch_class(40, 1);
        assert_eq!(pc, PitchClass::new(5));
        assert_eq!(pc.name(), "F");
    }

    #[test]
    fn fret_to_pitch_class_octave_invariance() {
        // Fret 0 and fret 12 on same string should give same pitch class
        let pc0 = fret_to_pitch_class(40, 0);
        let pc12 = fret_to_pitch_class(40, 12);
        assert_eq!(pc0, pc12);
    }

    // -----------------------------------------------------------------------
    // transpose_fretboard
    // -----------------------------------------------------------------------

    #[test]
    fn transpose_fretboard_shift_up() {
        // Open E major: 0 2 2 1 0 0
        let frets: [i8; 6] = [0, 2, 2, 1, 0, 0];
        let result = transpose_fretboard(&frets, 1).unwrap();
        assert_eq!(result, [1, 3, 3, 2, 1, 1]);
    }

    #[test]
    fn transpose_fretboard_shift_down() {
        let frets: [i8; 6] = [3, 3, 5, 5, 5, 3];
        let result = transpose_fretboard(&frets, -1).unwrap();
        assert_eq!(result, [2, 2, 4, 4, 4, 2]);
    }

    #[test]
    fn transpose_fretboard_boundary_fret_0_shift_down() {
        // Has a fret 0, shifting down would make it -1 (invalid)
        let frets: [i8; 6] = [0, 2, 2, 1, 0, 0];
        assert_eq!(transpose_fretboard(&frets, -1), None);
    }

    #[test]
    fn transpose_fretboard_boundary_fret_24_shift_up() {
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, 24];
        assert_eq!(transpose_fretboard(&frets, 1), None);
    }

    #[test]
    fn transpose_fretboard_muted_strings_unchanged() {
        // Only string 2 and 3 played, rest muted
        let frets: [i8; 6] = [-1, -1, 5, 5, -1, -1];
        let result = transpose_fretboard(&frets, 2).unwrap();
        assert_eq!(result, [-1, -1, 7, 7, -1, -1]);
    }

    #[test]
    fn transpose_fretboard_all_muted() {
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, -1];
        let result = transpose_fretboard(&frets, 5).unwrap();
        assert_eq!(result, [-1, -1, -1, -1, -1, -1]);
    }

    #[test]
    fn transpose_fretboard_sequential_shifts() {
        let frets: [i8; 6] = [0, 2, 2, 1, 0, 0];
        // Shift up 2, then up 3 more = total +5
        let step1 = transpose_fretboard(&frets, 2).unwrap();
        let step2 = transpose_fretboard(&step1, 3).unwrap();
        assert_eq!(step2, [5, 7, 7, 6, 5, 5]);
    }

    #[test]
    fn transpose_fretboard_large_shift_rejected() {
        // Fret 20, shift +5 = 25, exceeds max 24
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, 20];
        assert_eq!(transpose_fretboard(&frets, 5), None);
    }

    #[test]
    fn transpose_fretboard_exact_boundary_24() {
        // Fret 20, shift +4 = 24, exactly at max -- should succeed
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, 20];
        let result = transpose_fretboard(&frets, 4).unwrap();
        assert_eq!(result, [-1, -1, -1, -1, -1, 24]);
    }

    #[test]
    fn transpose_fretboard_exact_boundary_0() {
        // Fret 3, shift -3 = 0, exactly at min -- should succeed
        let frets: [i8; 6] = [3, -1, -1, -1, -1, -1];
        let result = transpose_fretboard(&frets, -3).unwrap();
        assert_eq!(result, [0, -1, -1, -1, -1, -1]);
    }

    // -----------------------------------------------------------------------
    // transpose_strings
    // -----------------------------------------------------------------------

    #[test]
    fn transpose_strings_shift_toward_higher_strings() {
        // Standard tuning: [40, 45, 50, 55, 59, 64]
        // Play fret 5 on string 0 (low E) = MIDI 45 (A2)
        // Shift to string 1 (A string, MIDI 45) -> fret 0
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [5, -1, -1, -1, -1, -1];
        let result = transpose_strings(&frets, &std_tuning, 1).unwrap();
        assert_eq!(result, [-1, 0, -1, -1, -1, -1]);
    }

    #[test]
    fn transpose_strings_shift_toward_lower_strings() {
        // Play fret 0 on string 1 (A, MIDI 45)
        // Shift to string 0 (low E, MIDI 40) -> fret 5
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, 0, -1, -1, -1, -1];
        let result = transpose_strings(&frets, &std_tuning, -1).unwrap();
        assert_eq!(result, [5, -1, -1, -1, -1, -1]);
    }

    #[test]
    fn transpose_strings_pitch_preservation() {
        // Play a chord shape and verify MIDI values are preserved after transposition
        // Standard tuning: [40, 45, 50, 55, 59, 64]
        // Use notes high enough on lower strings that they fit on higher strings:
        //   String 0 fret 7 = MIDI 47 -> String 1 (45) fret 2
        //   String 1 fret 7 = MIDI 52 -> String 2 (50) fret 2
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [7, 7, -1, -1, -1, -1];
        let before = played_notes(&frets, &std_tuning);

        let shifted = transpose_strings(&frets, &std_tuning, 1).unwrap();
        let after = played_notes(&shifted, &std_tuning);

        // Same number of notes
        assert_eq!(before.len(), after.len());
        // Same MIDI values (pitch preserved)
        let midi_before: Vec<u8> = before.iter().map(|&(_, m, _)| m).collect();
        let midi_after: Vec<u8> = after.iter().map(|&(_, m, _)| m).collect();
        assert_eq!(midi_before, midi_after);
    }

    #[test]
    fn transpose_strings_boundary_already_on_highest() {
        // Note on string 5 (high E), shift up -> out of bounds
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, 5];
        assert_eq!(transpose_strings(&frets, &std_tuning, 1), None);
    }

    #[test]
    fn transpose_strings_boundary_already_on_lowest() {
        // Note on string 0 (low E), shift down -> out of bounds
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [5, -1, -1, -1, -1, -1];
        assert_eq!(transpose_strings(&frets, &std_tuning, -1), None);
    }

    #[test]
    fn transpose_strings_negative_fret_result() {
        // Play fret 0 on string 2 (D, MIDI 50)
        // Shift down to string 1 (A, MIDI 45) -> fret = 50 - 45 = 5 (OK)
        // But play fret 0 on string 3 (G, MIDI 55)
        // Shift down to string 2 (D, MIDI 50) -> fret = 55 - 50 = 5 (OK)
        // Now test where it would be negative:
        // Play fret 0 on string 1 (A, MIDI 45)
        // Shift up to string 2 (D, MIDI 50) -> fret = 45 - 50 = -5 (INVALID)
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, 0, -1, -1, -1, -1];
        assert_eq!(transpose_strings(&frets, &std_tuning, 1), None);
    }

    #[test]
    fn transpose_strings_fret_exceeds_24() {
        // Play fret 24 on string 1 (A, MIDI 45) = MIDI 69
        // Shift down to string 0 (low E, MIDI 40) -> fret = 69 - 40 = 29 (INVALID)
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, 24, -1, -1, -1, -1];
        assert_eq!(transpose_strings(&frets, &std_tuning, -1), None);
    }

    #[test]
    fn transpose_strings_multiple_notes() {
        // Standard tuning: [40, 45, 50, 55, 59, 64]
        // Play strings 0-2: frets [5, 5, 5] = MIDI [45, 50, 55]
        // Shift up (+1) -> strings 1-3:
        //   MIDI 45 on string 1 (45) -> fret 0
        //   MIDI 50 on string 2 (50) -> fret 0
        //   MIDI 55 on string 3 (55) -> fret 0
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [5, 5, 5, -1, -1, -1];
        let result = transpose_strings(&frets, &std_tuning, 1).unwrap();
        assert_eq!(result, [-1, 0, 0, 0, -1, -1]);
    }

    #[test]
    fn transpose_strings_preserves_pitch_multi_note() {
        // C major open shape: [0, 3, 2, 0, 1, 0] but skip low E for clarity
        // Let's use a simpler shape on inner strings
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, 7, 7, 7, -1, -1];
        let before = played_notes(&frets, &std_tuning);

        let shifted = transpose_strings(&frets, &std_tuning, -1).unwrap();
        let after = played_notes(&shifted, &std_tuning);

        let midi_before: Vec<u8> = before.iter().map(|&(_, m, _)| m).collect();
        let midi_after: Vec<u8> = after.iter().map(|&(_, m, _)| m).collect();
        assert_eq!(midi_before, midi_after);
    }

    #[test]
    fn transpose_strings_non_standard_tuning() {
        // Drop D: [38, 45, 50, 55, 59, 64]
        // Play fret 7 on string 0 (D2, MIDI 38 + 7 = 45)
        // Shift up to string 1 (A, MIDI 45) -> fret 0
        let drop_d = tuning::drop_d().strings;
        let frets: [i8; 6] = [7, -1, -1, -1, -1, -1];
        let result = transpose_strings(&frets, &drop_d, 1).unwrap();
        assert_eq!(result, [-1, 0, -1, -1, -1, -1]);
    }

    // -----------------------------------------------------------------------
    // chord_tone_positions
    // -----------------------------------------------------------------------

    #[test]
    fn chord_tone_positions_c_major_standard_tuning() {
        // C major triad: C(0), E(4), G(7)
        let std_tuning = tuning::standard().strings;
        let targets = [PitchClass::new(0), PitchClass::new(4), PitchClass::new(7)];
        let positions = chord_tone_positions(&targets, &std_tuning, 0, 12);

        // Verify some known positions:
        // String 0 (low E, MIDI 40 = E): E is pitch class 4
        //   fret 0 -> E (match), fret 3 -> G (match), fret 8 -> C (match),
        //   fret 12 -> E (match)
        assert!(positions.contains(&(0, 0)));  // open low E = E
        assert!(positions.contains(&(0, 3)));  // low E fret 3 = G
        assert!(positions.contains(&(0, 8)));  // low E fret 8 = C
        assert!(positions.contains(&(0, 12))); // low E fret 12 = E

        // String 1 (A, MIDI 45 = A): A is pitch class 9, not in triad
        //   fret 3 -> C (match), fret 7 -> E (match), fret 10 -> G (match)
        assert!(positions.contains(&(1, 3)));  // A fret 3 = C
        assert!(positions.contains(&(1, 7)));  // A fret 7 = E
        assert!(positions.contains(&(1, 10))); // A fret 10 = G

        // String 2 (D, MIDI 50 = D): D is pitch class 2, not in triad
        //   fret 2 -> E (match), fret 5 -> G (match), fret 10 -> C (match, but wait: 50+10=60=C)
        assert!(positions.contains(&(2, 2)));  // D fret 2 = E
        assert!(positions.contains(&(2, 5)));  // D fret 5 = G
        assert!(positions.contains(&(2, 10))); // D fret 10 = Bb? No, 50+10=60, 60%12=0 = C. Yes!
        assert!(positions.contains(&(2, 10))); // D fret 10 = C

        // String 4 (B, MIDI 59 = B): B is pitch class 11, not in triad
        //   fret 1 -> C (match), fret 5 -> E (match), fret 8 -> G (match)
        assert!(positions.contains(&(4, 1)));  // B fret 1 = C
        assert!(positions.contains(&(4, 5)));  // B fret 5 = E
        assert!(positions.contains(&(4, 8)));  // B fret 8 = G

        // String 5 (high E, MIDI 64 = E): same as string 0 pattern
        assert!(positions.contains(&(5, 0)));  // open high E = E
        assert!(positions.contains(&(5, 3)));  // high E fret 3 = G
        assert!(positions.contains(&(5, 8)));  // high E fret 8 = C

        // Verify a non-match is excluded
        assert!(!positions.contains(&(0, 1))); // low E fret 1 = F, not in C major
    }

    #[test]
    fn chord_tone_positions_single_pitch_class() {
        // Find all C notes (pitch class 0) in standard tuning, frets 0-5
        let std_tuning = tuning::standard().strings;
        let targets = [PitchClass::new(0)]; // C
        let positions = chord_tone_positions(&targets, &std_tuning, 0, 5);

        // String 0 (E, 40): 40+fret % 12 == 0 -> fret where (40+f)%12==0
        //   40%12=4, need 12-4=8, but 8>5, so no match in 0-5
        // String 1 (A, 45): 45%12=9, need 12-9=3 -> fret 3. Yes!
        assert!(positions.contains(&(1, 3)));
        // String 2 (D, 50): 50%12=2, need 12-2=10, >5, no match
        // String 3 (G, 55): 55%12=7, need 12-7=5 -> fret 5. Yes!
        assert!(positions.contains(&(3, 5)));
        // String 4 (B, 59): 59%12=11, need 12-11=1 -> fret 1. Yes!
        assert!(positions.contains(&(4, 1)));
        // String 5 (E, 64): 64%12=4, need 12-4=8, >5, no match

        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn chord_tone_positions_fret_range() {
        // Only search frets 5-7
        let std_tuning = tuning::standard().strings;
        let targets = [PitchClass::new(0)]; // C
        let positions = chord_tone_positions(&targets, &std_tuning, 5, 7);

        // String 3 (G, 55): fret 5 -> 60%12=0 = C. Match!
        assert!(positions.contains(&(3, 5)));

        // Should not include fret 3 on A string (out of range)
        assert!(!positions.contains(&(1, 3)));
    }

    #[test]
    fn chord_tone_positions_empty_pitch_classes() {
        let std_tuning = tuning::standard().strings;
        let targets: Vec<PitchClass> = vec![];
        let positions = chord_tone_positions(&targets, &std_tuning, 0, 12);
        assert!(positions.is_empty());
    }

    // -----------------------------------------------------------------------
    // played_notes
    // -----------------------------------------------------------------------

    #[test]
    fn played_notes_d_major() {
        // D major: X X 0 2 3 2 (index 0=low E, index 5=high e)
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, -1, 0, 2, 3, 2];
        let notes = played_notes(&frets, &std_tuning);

        assert_eq!(notes.len(), 4);

        // String 2 (D open, MIDI 50) -> D, pitch class 2
        assert_eq!(notes[0], (2, 50, PitchClass::new(2)));
        // String 3 (G fret 2, MIDI 57) -> A, pitch class 9
        assert_eq!(notes[1], (3, 57, PitchClass::new(9)));
        // String 4 (B fret 3, MIDI 62) -> D, pitch class 2
        assert_eq!(notes[2], (4, 62, PitchClass::new(2)));
        // String 5 (E fret 2, MIDI 66) -> F#, pitch class 6
        assert_eq!(notes[3], (5, 66, PitchClass::new(6)));
    }

    #[test]
    fn played_notes_open_e_major() {
        // E major: 0 2 2 1 0 0
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [0, 2, 2, 1, 0, 0];
        let notes = played_notes(&frets, &std_tuning);

        assert_eq!(notes.len(), 6);

        // All strings played
        assert_eq!(notes[0], (0, 40, PitchClass::new(4)));  // E
        assert_eq!(notes[1], (1, 47, PitchClass::new(11))); // B
        assert_eq!(notes[2], (2, 52, PitchClass::new(4)));  // E
        assert_eq!(notes[3], (3, 56, PitchClass::new(8)));  // G#
        assert_eq!(notes[4], (4, 59, PitchClass::new(11))); // B
        assert_eq!(notes[5], (5, 64, PitchClass::new(4)));  // E
    }

    #[test]
    fn played_notes_all_muted() {
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, -1];
        let notes = played_notes(&frets, &std_tuning);
        assert!(notes.is_empty());
    }

    #[test]
    fn played_notes_single_string() {
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, 7];
        let notes = played_notes(&frets, &std_tuning);

        assert_eq!(notes.len(), 1);
        // High E fret 7 = B, MIDI 71, pitch class 11
        assert_eq!(notes[0], (5, 71, PitchClass::new(11)));
    }

    #[test]
    fn played_notes_ordering() {
        // Verify notes are ordered from lowest string index to highest
        let std_tuning = tuning::standard().strings;
        let frets: [i8; 6] = [3, -1, 5, -1, 1, -1];
        let notes = played_notes(&frets, &std_tuning);

        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].0, 0); // string 0 first
        assert_eq!(notes[1].0, 2); // string 2 second
        assert_eq!(notes[2].0, 4); // string 4 third
    }
}
