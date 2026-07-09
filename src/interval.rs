use serde::Serialize;
use std::fmt;

use crate::note::PitchClass;

/// An interval measured in semitones.
///
/// Supports simple intervals (0-11) and compound intervals (12+)
/// for extensions like 9ths, 11ths, and 13ths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Interval {
    pub semitones: u8,
}

impl Interval {
    pub const fn new(semitones: u8) -> Self {
        Interval { semitones }
    }

    // --- Simple intervals ---
    pub const P1: Interval = Interval::new(0);
    pub const MINOR_2ND: Interval = Interval::new(1);
    pub const MAJOR_2ND: Interval = Interval::new(2);
    pub const MINOR_3RD: Interval = Interval::new(3);
    pub const MAJOR_3RD: Interval = Interval::new(4);
    pub const PERFECT_4TH: Interval = Interval::new(5);
    pub const TRITONE: Interval = Interval::new(6);
    pub const PERFECT_5TH: Interval = Interval::new(7);
    pub const MINOR_6TH: Interval = Interval::new(8);
    pub const MAJOR_6TH: Interval = Interval::new(9);
    pub const MINOR_7TH: Interval = Interval::new(10);
    pub const MAJOR_7TH: Interval = Interval::new(11);

    // --- Compound intervals ---
    pub const OCTAVE: Interval = Interval::new(12);
    pub const MINOR_9TH: Interval = Interval::new(13);
    pub const MAJOR_9TH: Interval = Interval::new(14);
    pub const MINOR_10TH: Interval = Interval::new(15);
    pub const MAJOR_10TH: Interval = Interval::new(16);
    pub const PERFECT_11TH: Interval = Interval::new(17);
    pub const AUG_11TH: Interval = Interval::new(18);
    pub const PERFECT_12TH: Interval = Interval::new(19);
    pub const MINOR_13TH: Interval = Interval::new(20);
    pub const MAJOR_13TH: Interval = Interval::new(21);

    /// Compute the interval from `root` to `other` (mod 12, simple interval).
    pub fn between(root: PitchClass, other: PitchClass) -> Interval {
        let diff = (other.semitones() as i8 - root.semitones() as i8).rem_euclid(12) as u8;
        Interval::new(diff)
    }

    /// The simple (mod-12) version of this interval.
    pub fn simple(&self) -> Interval {
        Interval::new(self.semitones % 12)
    }

    /// Whether this is a compound interval (>= octave).
    pub fn is_compound(&self) -> bool {
        self.semitones >= 12
    }

    /// Short display name used in chord info panels.
    pub fn short_name(&self) -> &'static str {
        match self.semitones {
            0 => "R",
            1 => "m2",
            2 => "M2",
            3 => "m3",
            4 => "M3",
            5 => "P4",
            6 => "b5",
            7 => "P5",
            8 => "#5",
            9 => "M6",
            10 => "m7",
            11 => "M7",
            12 => "8",
            13 => "b9",
            14 => "9",
            15 => "#9",
            16 => "M10",
            17 => "11",
            18 => "#11",
            19 => "P12",
            20 => "b13",
            21 => "13",
            _ => "?",
        }
    }

    /// Long display name for human-readable descriptions.
    pub fn long_name(&self) -> &'static str {
        match self.semitones {
            0 => "root",
            1 => "minor 2nd",
            2 => "major 2nd",
            3 => "minor 3rd",
            4 => "major 3rd",
            5 => "perfect 4th",
            6 => "tritone",
            7 => "perfect 5th",
            8 => "augmented 5th",
            9 => "major 6th",
            10 => "minor 7th",
            11 => "major 7th",
            12 => "octave",
            13 => "minor 9th",
            14 => "major 9th",
            15 => "augmented 9th",
            16 => "major 10th",
            17 => "perfect 11th",
            18 => "augmented 11th",
            19 => "perfect 12th",
            20 => "minor 13th",
            21 => "major 13th",
            _ => "unknown",
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::PitchClass;

    #[test]
    fn test_between_same_note() {
        let c = PitchClass::new(0);
        assert_eq!(Interval::between(c, c), Interval::P1);
    }

    #[test]
    fn test_between_ascending() {
        let c = PitchClass::new(0);
        let e = PitchClass::new(4);
        let g = PitchClass::new(7);

        assert_eq!(Interval::between(c, e), Interval::MAJOR_3RD);
        assert_eq!(Interval::between(c, g), Interval::PERFECT_5TH);
    }

    #[test]
    fn test_between_wraps_around() {
        // From G (7) to C (0) should be P4 (5 semitones up)
        let g = PitchClass::new(7);
        let c = PitchClass::new(0);
        assert_eq!(Interval::between(g, c), Interval::PERFECT_4TH);

        // From A (9) to C (0) should be m3 (3 semitones up)
        let a = PitchClass::new(9);
        assert_eq!(Interval::between(a, c), Interval::MINOR_3RD);
    }

    #[test]
    fn test_between_various_roots() {
        // E major triad: E(4) - G#(8) - B(11)
        let e = PitchClass::new(4);
        let gs = PitchClass::new(8);
        let b = PitchClass::new(11);

        assert_eq!(Interval::between(e, gs), Interval::MAJOR_3RD);
        assert_eq!(Interval::between(e, b), Interval::PERFECT_5TH);

        // Bb major triad: Bb(10) - D(2) - F(5)
        let bb = PitchClass::new(10);
        let d = PitchClass::new(2);
        let f = PitchClass::new(5);

        assert_eq!(Interval::between(bb, d), Interval::MAJOR_3RD);
        assert_eq!(Interval::between(bb, f), Interval::PERFECT_5TH);
    }

    #[test]
    fn test_between_tritone() {
        let c = PitchClass::new(0);
        let fs = PitchClass::new(6);
        assert_eq!(Interval::between(c, fs), Interval::TRITONE);
    }

    #[test]
    fn test_short_names_simple() {
        assert_eq!(Interval::P1.short_name(), "R");
        assert_eq!(Interval::MINOR_2ND.short_name(), "m2");
        assert_eq!(Interval::MAJOR_2ND.short_name(), "M2");
        assert_eq!(Interval::MINOR_3RD.short_name(), "m3");
        assert_eq!(Interval::MAJOR_3RD.short_name(), "M3");
        assert_eq!(Interval::PERFECT_4TH.short_name(), "P4");
        assert_eq!(Interval::TRITONE.short_name(), "b5");
        assert_eq!(Interval::PERFECT_5TH.short_name(), "P5");
        assert_eq!(Interval::MINOR_6TH.short_name(), "#5");
        assert_eq!(Interval::MAJOR_6TH.short_name(), "M6");
        assert_eq!(Interval::MINOR_7TH.short_name(), "m7");
        assert_eq!(Interval::MAJOR_7TH.short_name(), "M7");
    }

    #[test]
    fn test_short_names_compound() {
        assert_eq!(Interval::OCTAVE.short_name(), "8");
        assert_eq!(Interval::MINOR_9TH.short_name(), "b9");
        assert_eq!(Interval::MAJOR_9TH.short_name(), "9");
        assert_eq!(Interval::MINOR_10TH.short_name(), "#9");
        assert_eq!(Interval::PERFECT_11TH.short_name(), "11");
        assert_eq!(Interval::AUG_11TH.short_name(), "#11");
        assert_eq!(Interval::MINOR_13TH.short_name(), "b13");
        assert_eq!(Interval::MAJOR_13TH.short_name(), "13");
    }

    #[test]
    fn test_long_names() {
        assert_eq!(Interval::P1.long_name(), "root");
        assert_eq!(Interval::MINOR_3RD.long_name(), "minor 3rd");
        assert_eq!(Interval::MAJOR_3RD.long_name(), "major 3rd");
        assert_eq!(Interval::PERFECT_5TH.long_name(), "perfect 5th");
        assert_eq!(Interval::MINOR_7TH.long_name(), "minor 7th");
        assert_eq!(Interval::MAJOR_7TH.long_name(), "major 7th");
        assert_eq!(Interval::MAJOR_9TH.long_name(), "major 9th");
        assert_eq!(Interval::PERFECT_11TH.long_name(), "perfect 11th");
        assert_eq!(Interval::MAJOR_13TH.long_name(), "major 13th");
    }

    #[test]
    fn test_simple_reduction() {
        assert_eq!(Interval::MAJOR_9TH.simple(), Interval::MAJOR_2ND);
        assert_eq!(Interval::PERFECT_11TH.simple(), Interval::PERFECT_4TH);
        assert_eq!(Interval::MAJOR_13TH.simple(), Interval::MAJOR_6TH);
        assert_eq!(Interval::OCTAVE.simple(), Interval::P1);
    }

    #[test]
    fn test_is_compound() {
        assert!(!Interval::P1.is_compound());
        assert!(!Interval::MAJOR_7TH.is_compound());
        assert!(Interval::OCTAVE.is_compound());
        assert!(Interval::MAJOR_9TH.is_compound());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Interval::PERFECT_5TH), "P5");
        assert_eq!(format!("{}", Interval::MAJOR_9TH), "9");
    }
}
