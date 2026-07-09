use chord_oracle::chord::identify;
use chord_oracle::tuning;

fn std_tuning() -> [u8; 6] {
    tuning::standard().strings
}

// ---------------------------------------------------------------------------
// Major triads
// ---------------------------------------------------------------------------

#[test]
fn c_major_open() {
    let result = identify(&[-1, 3, 2, 0, 1, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "C");
    assert_eq!(result.primary.quality.name, "major");
}

#[test]
fn d_major_open() {
    let result = identify(&[-1, -1, 0, 2, 3, 2], &std_tuning()).unwrap();
    assert_eq!(result.name(), "D");
    assert_eq!(result.primary.quality.name, "major");
}

#[test]
fn e_major_open() {
    let result = identify(&[0, 2, 2, 1, 0, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "E");
    assert_eq!(result.primary.quality.name, "major");
}

#[test]
fn g_major_open() {
    let result = identify(&[3, 2, 0, 0, 0, 3], &std_tuning()).unwrap();
    assert_eq!(result.name(), "G");
    assert_eq!(result.primary.quality.name, "major");
}

#[test]
fn a_major_open() {
    // x 0 2 2 2 0
    let result = identify(&[-1, 0, 2, 2, 2, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "A");
    assert_eq!(result.primary.quality.name, "major");
}

#[test]
fn f_major_barre() {
    // 1 3 3 2 1 1 (low E to high E)
    let result = identify(&[1, 3, 3, 2, 1, 1], &std_tuning()).unwrap();
    assert_eq!(result.name(), "F");
    assert_eq!(result.primary.quality.name, "major");
}

// ---------------------------------------------------------------------------
// Minor triads
// ---------------------------------------------------------------------------

#[test]
fn am_open() {
    let result = identify(&[-1, 0, 2, 2, 1, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "Am");
    assert_eq!(result.primary.quality.name, "minor");
}

#[test]
fn em_open() {
    let result = identify(&[0, 2, 2, 0, 0, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "Em");
    assert_eq!(result.primary.quality.name, "minor");
}

#[test]
fn dm_open() {
    // x x 0 2 3 1
    let result = identify(&[-1, -1, 0, 2, 3, 1], &std_tuning()).unwrap();
    assert_eq!(result.name(), "Dm");
    assert_eq!(result.primary.quality.name, "minor");
}

// ---------------------------------------------------------------------------
// 7th chords
// ---------------------------------------------------------------------------

#[test]
fn a7_open() {
    // x 0 2 0 2 0
    let result = identify(&[-1, 0, 2, 0, 2, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "A7");
    assert_eq!(result.primary.quality.name, "dominant 7th");
}

#[test]
fn e7_open() {
    // 0 2 0 1 0 0
    let result = identify(&[0, 2, 0, 1, 0, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "E7");
    assert_eq!(result.primary.quality.name, "dominant 7th");
}

#[test]
fn am7_open() {
    // x 0 2 0 1 0 -> notes: A, E, G, C, E -> {A, C, E, G}
    let result = identify(&[-1, 0, 2, 0, 1, 0], &std_tuning()).unwrap();
    assert_eq!(result.primary.root.name(), "A");
    assert!(
        result.primary.quality.symbol == "m7",
        "expected Am7 but got {}",
        result.name()
    );
}

// ---------------------------------------------------------------------------
// Power chords
// ---------------------------------------------------------------------------

#[test]
fn e5_power() {
    let result = identify(&[0, 2, 2, -1, -1, -1], &std_tuning()).unwrap();
    assert_eq!(result.name(), "E5");
    assert_eq!(result.primary.quality.name, "power chord");
}

#[test]
fn a5_power() {
    // x 0 2 2 x x
    let result = identify(&[-1, 0, 2, -1, -1, -1], &std_tuning()).unwrap();
    assert_eq!(result.primary.quality.symbol, "5");
}

// ---------------------------------------------------------------------------
// Slash / inversion chords
// ---------------------------------------------------------------------------

#[test]
fn c_over_e() {
    // 0 3 2 0 1 0 (C/E — bass note is E)
    let result = identify(&[0, 3, 2, 0, 1, 0], &std_tuning()).unwrap();
    assert_eq!(result.name(), "C/E");
    assert!(result.primary.is_inversion);
    assert_eq!(result.primary.bass.semitones(), 4); // E
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn single_note_returns_none() {
    assert!(identify(&[0, -1, -1, -1, -1, -1], &std_tuning()).is_none());
}

#[test]
fn all_muted_returns_none() {
    assert!(identify(&[-1, -1, -1, -1, -1, -1], &std_tuning()).is_none());
}

// ---------------------------------------------------------------------------
// Intervals computed correctly
// ---------------------------------------------------------------------------

#[test]
fn intervals_from_root_e_minor() {
    let result = identify(&[0, 2, 2, 0, 0, 0], &std_tuning()).unwrap();
    let semitones: Vec<u8> = result
        .intervals_from_root
        .iter()
        .map(|(_, iv)| iv.semitones)
        .collect();
    assert!(semitones.contains(&0)); // root
    assert!(semitones.contains(&3)); // minor 3rd
    assert!(semitones.contains(&7)); // perfect 5th
}

// ---------------------------------------------------------------------------
// Alternatives
// ---------------------------------------------------------------------------

#[test]
fn alternatives_count_within_bounds() {
    let result = identify(&[-1, 3, 2, 0, 1, 0], &std_tuning()).unwrap();
    assert!(result.alternatives.len() <= 2);
}

// ---------------------------------------------------------------------------
// Alternate tunings
// ---------------------------------------------------------------------------

#[test]
fn drop_d_power_chord() {
    // Drop D tuning: D2(38) A2 D3 G3 B3 E4
    // All open first two strings: D2 + A2 = power chord D5
    let drop_d = tuning::drop_d().strings;
    let result = identify(&[0, 0, -1, -1, -1, -1], &drop_d).unwrap();
    assert_eq!(result.primary.quality.symbol, "5");
    assert_eq!(result.primary.root.name(), "D");
}

// ---------------------------------------------------------------------------
// Suspended chords
// ---------------------------------------------------------------------------

#[test]
fn dsus2() {
    // Dsus2: x x 0 2 3 0
    // D3(50), A3(57), D4(62), E4(64)
    // Actually 50=D, 55+2=57=A, 59+3=62=D, 64+0=64=E
    // That's {D, A, E} = {D, E, A} which is Dsus2 missing nothing (0, 2, 7)
    // Wait: D=2, E=4, A=9. From D: E=(4-2)=2, A=(9-2)=7. Yes, {0,2,7} = sus2.
    let result = identify(&[-1, -1, 0, 2, 3, 0], &std_tuning()).unwrap();
    assert_eq!(result.primary.root.name(), "D");
    assert_eq!(result.primary.quality.symbol, "sus2");
}

#[test]
fn dsus4() {
    // Dsus4: x x 0 2 3 3
    // D3(50), A3(57), D4(62), G4(67)
    // D=2, A=9, G=7. From D: G=(7-2+12)%12=5, A=(9-2)=7.
    // {0, 5, 7} = sus4
    let result = identify(&[-1, -1, 0, 2, 3, 3], &std_tuning()).unwrap();
    assert_eq!(result.primary.root.name(), "D");
    assert_eq!(result.primary.quality.symbol, "sus4");
}
