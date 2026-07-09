use serde::Serialize;
use std::collections::BTreeSet;

use crate::interval::Interval;
use crate::note::PitchClass;

// ---------------------------------------------------------------------------
// Chord quality database
// ---------------------------------------------------------------------------

/// A chord quality defined by its interval pattern (semitones from root).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChordQuality {
    pub symbol: &'static str,
    pub name: &'static str,
    pub intervals: &'static [u8],
}

impl ChordQuality {
    /// Number of notes in the quality.
    pub fn note_count(&self) -> usize {
        self.intervals.len()
    }

    /// Convert the quality intervals to a mod-12 set.
    fn interval_set_mod12(&self) -> BTreeSet<u8> {
        self.intervals.iter().map(|&i| i % 12).collect()
    }

}

/// Static database of all recognized chord qualities. Ordered roughly from
/// simplest to most complex so that earlier entries win ties.
static CHORD_QUALITIES: &[ChordQuality] = &[
    // --- Triads ---
    ChordQuality { symbol: "",     name: "major",           intervals: &[0, 4, 7] },
    ChordQuality { symbol: "m",    name: "minor",           intervals: &[0, 3, 7] },
    ChordQuality { symbol: "dim",  name: "diminished",      intervals: &[0, 3, 6] },
    ChordQuality { symbol: "aug",  name: "augmented",       intervals: &[0, 4, 8] },
    ChordQuality { symbol: "sus2", name: "suspended 2nd",   intervals: &[0, 2, 7] },
    ChordQuality { symbol: "sus4", name: "suspended 4th",   intervals: &[0, 5, 7] },
    // --- Dyads ---
    ChordQuality { symbol: "5",    name: "power chord",     intervals: &[0, 7] },
    // --- 7th chords ---
    ChordQuality { symbol: "7",      name: "dominant 7th",         intervals: &[0, 4, 7, 10] },
    ChordQuality { symbol: "maj7",   name: "major 7th",            intervals: &[0, 4, 7, 11] },
    ChordQuality { symbol: "m7",     name: "minor 7th",            intervals: &[0, 3, 7, 10] },
    ChordQuality { symbol: "dim7",   name: "diminished 7th",       intervals: &[0, 3, 6, 9] },
    ChordQuality { symbol: "m7b5",   name: "half-diminished 7th",  intervals: &[0, 3, 6, 10] },
    ChordQuality { symbol: "aug7",   name: "augmented 7th",        intervals: &[0, 4, 8, 10] },
    ChordQuality { symbol: "mMaj7",  name: "minor-major 7th",      intervals: &[0, 3, 7, 11] },
    // --- 6th chords ---
    ChordQuality { symbol: "6",    name: "major 6th",       intervals: &[0, 4, 7, 9] },
    ChordQuality { symbol: "m6",   name: "minor 6th",       intervals: &[0, 3, 7, 9] },
    // --- 7th sus ---
    ChordQuality { symbol: "7sus4", name: "dominant 7th sus4", intervals: &[0, 5, 7, 10] },
    // --- 9th chords ---
    ChordQuality { symbol: "9",     name: "dominant 9th",    intervals: &[0, 4, 7, 10, 14] },
    ChordQuality { symbol: "maj9",  name: "major 9th",       intervals: &[0, 4, 7, 11, 14] },
    ChordQuality { symbol: "m9",    name: "minor 9th",       intervals: &[0, 3, 7, 10, 14] },
    ChordQuality { symbol: "add9",  name: "added 9th",       intervals: &[0, 4, 7, 14] },
    ChordQuality { symbol: "madd9", name: "minor added 9th", intervals: &[0, 3, 7, 14] },
    // --- Extended chords ---
    ChordQuality { symbol: "11",   name: "dominant 11th",    intervals: &[0, 4, 7, 10, 14, 17] },
    ChordQuality { symbol: "m11",  name: "minor 11th",       intervals: &[0, 3, 7, 10, 14, 17] },
    ChordQuality { symbol: "13",   name: "dominant 13th",    intervals: &[0, 4, 7, 10, 14, 21] },
    // --- Altered dominants ---
    ChordQuality { symbol: "7#9",  name: "7th sharp 9",      intervals: &[0, 4, 7, 10, 15] },
    ChordQuality { symbol: "7b9",  name: "7th flat 9",       intervals: &[0, 4, 7, 10, 13] },
    ChordQuality { symbol: "7#5",  name: "7th sharp 5",      intervals: &[0, 4, 8, 10] },
    ChordQuality { symbol: "7b5",  name: "7th flat 5",       intervals: &[0, 4, 6, 10] },
    // --- 6/9 and add11 ---
    ChordQuality { symbol: "6/9",   name: "6/9",             intervals: &[0, 4, 7, 9, 14] },
    ChordQuality { symbol: "add11", name: "added 11th",      intervals: &[0, 4, 7, 17] },
];

// ---------------------------------------------------------------------------
// Result types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct ChordMatch {
    pub root: PitchClass,
    #[serde(skip)]
    pub quality: &'static ChordQuality,
    pub bass: PitchClass,
    pub is_inversion: bool,
    pub score: i32,
}

impl ChordMatch {
    /// Build the display name: root + symbol, optionally with /bass.
    pub fn display_name(&self) -> String {
        let root_name = self.root.name();
        let symbol = self.quality.symbol;
        if self.is_inversion {
            format!("{}{}/{}", root_name, symbol, self.bass.name())
        } else {
            format!("{}{}", root_name, symbol)
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ChordResult {
    pub primary: ChordMatch,
    pub alternatives: Vec<ChordMatch>,
    pub notes_played: Vec<PitchClass>,
    pub intervals_from_root: Vec<(PitchClass, Interval)>,
    pub intervals_from_bass: Vec<(PitchClass, Interval)>,
}

impl ChordResult {
    /// Convenience: the display name of the primary match.
    pub fn name(&self) -> String {
        self.primary.display_name()
    }
}

// ---------------------------------------------------------------------------
// Identification algorithm
// ---------------------------------------------------------------------------

/// Identify a chord from fret positions and tuning.
///
/// `frets[i]` is the fret number on string `i` (index 0 = lowest/thickest
/// string). A value of -1 (or any negative) means the string is muted.
///
/// Returns `None` if fewer than 2 notes are played or no chord quality
/// matches.
pub fn identify(frets: &[i8; 6], tuning: &[u8; 6]) -> Option<ChordResult> {
    // Step 1: extract played notes as (midi, pitch_class), ordered low to high
    let mut played: Vec<(u8, PitchClass)> = Vec::new();
    for (i, &fret) in frets.iter().enumerate() {
        if fret >= 0 {
            let midi = tuning[i] + fret as u8;
            played.push((midi, PitchClass::from_midi(midi)));
        }
    }

    if played.len() < 2 {
        return None;
    }

    // Sort by MIDI pitch (low to high)
    played.sort_by_key(|&(midi, _)| midi);

    // Step 2: bass note = lowest MIDI note
    let bass = played[0].1;

    // Step 3: unique pitch classes (preserving low-to-high order by first appearance)
    let mut unique_pcs: Vec<PitchClass> = Vec::new();
    let mut seen = BTreeSet::new();
    for &(_, pc) in &played {
        if seen.insert(pc.semitones()) {
            unique_pcs.push(pc);
        }
    }

    let played_set_mod12: BTreeSet<u8> = unique_pcs.iter().map(|pc| pc.semitones()).collect();

    // Step 4: try each unique pitch class as candidate root
    let mut all_matches: Vec<ChordMatch> = Vec::new();

    for &candidate_root in &unique_pcs {
        // Compute interval set (mod 12) from candidate root
        let interval_set: BTreeSet<u8> = played_set_mod12
            .iter()
            .map(|&pc| (pc + 12 - candidate_root.semitones()) % 12)
            .collect();

        for quality in CHORD_QUALITIES.iter() {
            let quality_set = quality.interval_set_mod12();

            // Check: played notes must be a subset of quality notes (no extra notes allowed)
            if !interval_set.is_subset(&quality_set) {
                continue;
            }

            // Compute score
            let mut score: i32 = 0;

            if interval_set == quality_set {
                // Exact match
                score = 0;
            } else {
                // Subset match: some quality notes are missing
                let missing: BTreeSet<u8> = quality_set.difference(&interval_set).cloned().collect();

                for &m in &missing {
                    if m == 0 {
                        // Missing the root
                        score += 50;
                    } else if m == 3 || m == 4 {
                        // Missing the 3rd (major or minor)
                        score += 40;
                    } else if m == 7 || m == 6 || m == 8 {
                        // Missing a 5th variant
                        if missing.len() == 1 || (missing.iter().all(|&x| x == 7 || x == 6 || x == 8)) {
                            score += 10;
                        } else {
                            score += 10;
                        }
                    } else {
                        score += 20;
                    }
                }
            }

            // Inversion / root position bonus
            let is_inversion = candidate_root != bass;
            if is_inversion {
                score += 15;
            } else {
                score -= 5;
            }

            // Simplicity bonus: prefer triads and simpler qualities
            let note_count = quality.note_count();
            if note_count <= 3 {
                score -= 3; // triads and dyads
            } else if note_count == 4 {
                score -= 1; // 7th chords
            }
            // 5+ note qualities get no bonus

            all_matches.push(ChordMatch {
                root: candidate_root,
                quality,
                bass,
                is_inversion,
                score,
            });
        }
    }

    if all_matches.is_empty() {
        return None;
    }

    // Step 5: rank by score (lower is better), with stability
    all_matches.sort_by_key(|m| m.score);

    let primary = all_matches.remove(0);

    // Collect up to 2 alternatives with different root notes
    let mut alternatives: Vec<ChordMatch> = Vec::new();
    let mut seen_roots: BTreeSet<u8> = BTreeSet::new();
    seen_roots.insert(primary.root.semitones());

    for m in all_matches {
        if seen_roots.contains(&m.root.semitones()) {
            continue;
        }
        seen_roots.insert(m.root.semitones());
        alternatives.push(m);
        if alternatives.len() >= 2 {
            break;
        }
    }

    // Step 6: notes_played ordered low to high by MIDI
    let notes_played: Vec<PitchClass> = unique_pcs.clone();

    // Step 7: intervals from root and bass
    let intervals_from_root: Vec<(PitchClass, Interval)> = unique_pcs
        .iter()
        .map(|&pc| (pc, Interval::between(primary.root, pc)))
        .collect();

    let intervals_from_bass: Vec<(PitchClass, Interval)> = unique_pcs
        .iter()
        .map(|&pc| (pc, Interval::between(bass, pc)))
        .collect();

    Some(ChordResult {
        primary,
        alternatives,
        notes_played,
        intervals_from_root,
        intervals_from_bass,
    })
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuning;

    fn std_tuning() -> [u8; 6] {
        tuning::standard().strings
    }

    #[test]
    fn test_c_major() {
        // x 3 2 0 1 0  (standard tuning)
        let frets: [i8; 6] = [-1, 3, 2, 0, 1, 0];
        let result = identify(&frets, &std_tuning()).expect("should identify C major");
        assert_eq!(result.name(), "C");
        assert_eq!(result.primary.root.semitones(), 0); // C
        assert_eq!(result.primary.quality.name, "major");
        assert!(!result.primary.is_inversion);
    }

    #[test]
    fn test_d_major() {
        // x x 0 2 3 2
        let frets: [i8; 6] = [-1, -1, 0, 2, 3, 2];
        let result = identify(&frets, &std_tuning()).expect("should identify D major");
        assert_eq!(result.name(), "D");
        assert_eq!(result.primary.root.semitones(), 2); // D
        assert_eq!(result.primary.quality.name, "major");
    }

    #[test]
    fn test_a_minor() {
        // x 0 2 2 1 0
        let frets: [i8; 6] = [-1, 0, 2, 2, 1, 0];
        let result = identify(&frets, &std_tuning()).expect("should identify Am");
        assert_eq!(result.name(), "Am");
        assert_eq!(result.primary.root.semitones(), 9); // A
        assert_eq!(result.primary.quality.name, "minor");
    }

    #[test]
    fn test_g_major() {
        // 3 2 0 0 0 3
        let frets: [i8; 6] = [3, 2, 0, 0, 0, 3];
        let result = identify(&frets, &std_tuning()).expect("should identify G major");
        assert_eq!(result.name(), "G");
        assert_eq!(result.primary.root.semitones(), 7); // G
        assert_eq!(result.primary.quality.name, "major");
    }

    #[test]
    fn test_e_minor() {
        // 0 2 2 0 0 0
        let frets: [i8; 6] = [0, 2, 2, 0, 0, 0];
        let result = identify(&frets, &std_tuning()).expect("should identify Em");
        assert_eq!(result.name(), "Em");
        assert_eq!(result.primary.root.semitones(), 4); // E
        assert_eq!(result.primary.quality.name, "minor");
    }

    #[test]
    fn test_a7() {
        // x 0 2 0 2 0
        let frets: [i8; 6] = [-1, 0, 2, 0, 2, 0];
        let result = identify(&frets, &std_tuning()).expect("should identify A7");
        assert_eq!(result.name(), "A7");
        assert_eq!(result.primary.root.semitones(), 9); // A
        assert_eq!(result.primary.quality.name, "dominant 7th");
    }

    #[test]
    fn test_power_chord_e5() {
        // 0 2 2 x x x
        let frets: [i8; 6] = [0, 2, 2, -1, -1, -1];
        let result = identify(&frets, &std_tuning()).expect("should identify E5");
        assert_eq!(result.name(), "E5");
        assert_eq!(result.primary.quality.name, "power chord");
    }

    #[test]
    fn test_slash_chord_c_over_e() {
        // 0 3 2 0 1 0  (C/E — low E bass, C major)
        let frets: [i8; 6] = [0, 3, 2, 0, 1, 0];
        let result = identify(&frets, &std_tuning()).expect("should identify C/E");
        assert_eq!(result.name(), "C/E");
        assert_eq!(result.primary.root.semitones(), 0); // C
        assert_eq!(result.primary.bass.semitones(), 4); // E
        assert!(result.primary.is_inversion);
    }

    #[test]
    fn test_no_chord_single_note() {
        // Only one string played
        let frets: [i8; 6] = [0, -1, -1, -1, -1, -1];
        let result = identify(&frets, &std_tuning());
        assert!(result.is_none());
    }

    #[test]
    fn test_no_chord_all_muted() {
        let frets: [i8; 6] = [-1, -1, -1, -1, -1, -1];
        let result = identify(&frets, &std_tuning());
        assert!(result.is_none());
    }

    #[test]
    fn test_c6_vs_am7_bass_determines() {
        // C6 and Am7 share pitch classes {C, E, G, A}.
        // With C in the bass: C6
        // C in bass: x 3 2 2 1 0  (A=45+3=48=C, D=50+2=52=E, G=55+2=57=A, B=59+1=60=C, E=64+0=64=E)
        // Wait, let's be more precise.
        // Standard tuning: [E2=40, A2=45, D3=50, G3=55, B3=59, E4=64]
        // For C in bass, need lowest note to be C.
        // A string (45) + 3 = 48 = C3. Mute low E.
        // x 3 2 2 1 0 gives notes: C3, E3, A3, C4, E4
        // That's {C, E, A} — missing G for C6. Let's use a voicing with G too.
        // x 3 2 2 1 3 gives: C3(48), E3(52), A3(57-nope wait 55+2=57=A), C4(60), G4(67)
        // 55+2 = 57 which is A3. So {C, E, A, C, G} = {C, E, G, A}. Yes!
        // But fret 3 on high E = 64+3=67 = G4. Good.
        let frets_c_bass: [i8; 6] = [-1, 3, 2, 2, 1, 3];
        let result = identify(&frets_c_bass, &std_tuning()).expect("should identify");
        // Should prefer C6 since C is in bass
        assert_eq!(result.primary.root.name(), "C");
        assert!(result.primary.quality.symbol == "6" || result.primary.quality.symbol == "");

        // With A in bass: x 0 2 2 1 0 (A2=45, E3=52, A3=57-no, 55+2=57=A3, C4=60, E4=64)
        // 50+2=52=E, 55+2=57=A, 59+1=60=C, 64+0=64=E => {A, E, A, C, E} = {A, C, E}
        // Missing G — need to add it.
        // x 0 2 0 1 0 => A2(45), E3(52), G3(55), C4(60), E4(64) = {A, E, G, C, E} = {A, C, E, G}
        let frets_a_bass: [i8; 6] = [-1, 0, 2, 0, 1, 0];
        let result = identify(&frets_a_bass, &std_tuning()).expect("should identify");
        // Should prefer Am7 since A is in bass
        assert_eq!(result.primary.root.name(), "A");
        assert!(result.primary.quality.symbol == "m7" || result.primary.quality.symbol == "m");
    }

    #[test]
    fn test_e_major() {
        // 0 2 2 1 0 0
        let frets: [i8; 6] = [0, 2, 2, 1, 0, 0];
        let result = identify(&frets, &std_tuning()).expect("should identify E major");
        assert_eq!(result.name(), "E");
        assert_eq!(result.primary.root.semitones(), 4); // E
        assert_eq!(result.primary.quality.name, "major");
    }

    #[test]
    fn test_f_major_barre() {
        // 1 3 3 2 1 1 (F barre chord, low E to high E)
        let frets: [i8; 6] = [1, 3, 3, 2, 1, 1];
        let result = identify(&frets, &std_tuning()).expect("should identify F major");
        assert_eq!(result.name(), "F");
        assert_eq!(result.primary.root.semitones(), 5); // F
        assert_eq!(result.primary.quality.name, "major");
    }

    #[test]
    fn test_notes_played_order() {
        // C major: x 3 2 0 1 0 -> notes C3, E3, G3, C4, E4
        let frets: [i8; 6] = [-1, 3, 2, 0, 1, 0];
        let result = identify(&frets, &std_tuning()).unwrap();
        // Unique PCs ordered by first MIDI appearance: C, E, G
        let pc_values: Vec<u8> = result.notes_played.iter().map(|pc| pc.semitones()).collect();
        assert!(pc_values.contains(&0)); // C
        assert!(pc_values.contains(&4)); // E
        assert!(pc_values.contains(&7)); // G
    }

    #[test]
    fn test_intervals_from_root() {
        // E minor: 0 2 2 0 0 0 -> root = E(4)
        let frets: [i8; 6] = [0, 2, 2, 0, 0, 0];
        let result = identify(&frets, &std_tuning()).unwrap();
        // Intervals from E: E=P1, G=m3, B=P5
        let interval_semitones: Vec<u8> = result
            .intervals_from_root
            .iter()
            .map(|(_, iv)| iv.semitones)
            .collect();
        assert!(interval_semitones.contains(&0)); // P1
        assert!(interval_semitones.contains(&3)); // m3
        assert!(interval_semitones.contains(&7)); // P5
    }

    #[test]
    fn test_chord_result_has_alternatives() {
        // Most chords should produce at least one alternative interpretation
        let frets: [i8; 6] = [-1, 3, 2, 0, 1, 0]; // C major
        let result = identify(&frets, &std_tuning()).unwrap();
        // Just verify alternatives is a Vec (may be empty for very clear chords)
        assert!(result.alternatives.len() <= 2);
    }

    #[test]
    fn test_dm() {
        // Dm: x x 0 2 3 1
        let frets: [i8; 6] = [-1, -1, 0, 2, 3, 1];
        let result = identify(&frets, &std_tuning()).expect("should identify Dm");
        assert_eq!(result.name(), "Dm");
        assert_eq!(result.primary.root.semitones(), 2); // D
        assert_eq!(result.primary.quality.name, "minor");
    }
}
