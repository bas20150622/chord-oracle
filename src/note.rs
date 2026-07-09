use serde::Serialize;
use std::fmt;

/// Two spellings per pitch class: [preferred, alternate].
/// Index 0 is the default display name.
const SPELLINGS: [[&str; 2]; 12] = [
    ["C", "C"],
    ["C#", "Db"],
    ["D", "D"],
    ["Eb", "D#"],
    ["E", "E"],
    ["F", "F"],
    ["F#", "Gb"],
    ["G", "G"],
    ["Ab", "G#"],
    ["A", "A"],
    ["Bb", "A#"],
    ["B", "B"],
];

/// A pitch class (0-11), where C=0, C#/Db=1, D=2, ... B=11.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub struct PitchClass(u8);

impl PitchClass {
    /// Create a new PitchClass, wrapping the value mod 12.
    pub fn new(value: u8) -> Self {
        PitchClass(value % 12)
    }

    /// Create a PitchClass from a MIDI note number.
    pub fn from_midi(midi: u8) -> Self {
        PitchClass(midi % 12)
    }

    /// The number of semitones above C (0-11).
    pub fn semitones(&self) -> u8 {
        self.0
    }

    /// The preferred (default) spelling for this pitch class.
    pub fn name(&self) -> &'static str {
        SPELLINGS[self.0 as usize][0]
    }

    /// The alternate enharmonic spelling for this pitch class.
    pub fn alt_name(&self) -> &'static str {
        SPELLINGS[self.0 as usize][1]
    }

    /// Return the sharp-preferring spelling.
    /// For pitch classes that have both a sharp and flat name, this returns
    /// whichever uses a sharp (or the natural name if no accidental).
    pub fn name_preferring_sharp(&self) -> &'static str {
        match self.0 {
            1 => "C#",
            3 => "D#",
            6 => "F#",
            8 => "G#",
            10 => "A#",
            _ => self.name(),
        }
    }

    /// Return the flat-preferring spelling.
    /// For pitch classes that have both a sharp and flat name, this returns
    /// whichever uses a flat (or the natural name if no accidental).
    pub fn name_preferring_flat(&self) -> &'static str {
        match self.0 {
            1 => "Db",
            3 => "Eb",
            6 => "Gb",
            8 => "Ab",
            10 => "Bb",
            _ => self.name(),
        }
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Parse a note name (e.g. "C#", "Db", "e", "Bb") into a PitchClass.
/// Case-insensitive. Returns None if the name is not recognized.
pub fn pitch_class_from_name(name: &str) -> Option<PitchClass> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }

    let mut chars = name.chars();
    let letter = chars.next()?.to_ascii_uppercase();

    let base = match letter {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };

    let rest: String = chars.collect();
    let modifier: i8 = match rest.as_str() {
        "" => 0,
        "#" | "♯" => 1,
        "b" | "♭" => -1,
        "##" | "♯♯" => 2,
        "bb" | "♭♭" => -2,
        _ => return None,
    };

    let value = ((base as i8 + modifier).rem_euclid(12)) as u8;
    Some(PitchClass::new(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wraps_mod_12() {
        assert_eq!(PitchClass::new(0).semitones(), 0);
        assert_eq!(PitchClass::new(12).semitones(), 0);
        assert_eq!(PitchClass::new(13).semitones(), 1);
        assert_eq!(PitchClass::new(24).semitones(), 0);
    }

    #[test]
    fn test_from_midi() {
        assert_eq!(PitchClass::from_midi(60).semitones(), 0); // Middle C
        assert_eq!(PitchClass::from_midi(69).semitones(), 9); // A4
        assert_eq!(PitchClass::from_midi(40).semitones(), 4); // E2
    }

    #[test]
    fn test_names() {
        assert_eq!(PitchClass::new(0).name(), "C");
        assert_eq!(PitchClass::new(1).name(), "C#");
        assert_eq!(PitchClass::new(1).alt_name(), "Db");
        assert_eq!(PitchClass::new(3).name(), "Eb");
        assert_eq!(PitchClass::new(3).alt_name(), "D#");
        assert_eq!(PitchClass::new(6).name(), "F#");
        assert_eq!(PitchClass::new(6).alt_name(), "Gb");
        assert_eq!(PitchClass::new(8).name(), "Ab");
        assert_eq!(PitchClass::new(8).alt_name(), "G#");
        assert_eq!(PitchClass::new(10).name(), "Bb");
        assert_eq!(PitchClass::new(10).alt_name(), "A#");
    }

    #[test]
    fn test_name_preferring_sharp() {
        assert_eq!(PitchClass::new(1).name_preferring_sharp(), "C#");
        assert_eq!(PitchClass::new(3).name_preferring_sharp(), "D#");
        assert_eq!(PitchClass::new(8).name_preferring_sharp(), "G#");
        assert_eq!(PitchClass::new(0).name_preferring_sharp(), "C"); // no accidental
    }

    #[test]
    fn test_name_preferring_flat() {
        assert_eq!(PitchClass::new(1).name_preferring_flat(), "Db");
        assert_eq!(PitchClass::new(3).name_preferring_flat(), "Eb");
        assert_eq!(PitchClass::new(8).name_preferring_flat(), "Ab");
        assert_eq!(PitchClass::new(0).name_preferring_flat(), "C"); // no accidental
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PitchClass::new(0)), "C");
        assert_eq!(format!("{}", PitchClass::new(1)), "C#");
        assert_eq!(format!("{}", PitchClass::new(3)), "Eb");
    }

    #[test]
    fn test_pitch_class_from_name() {
        // Basic naturals
        assert_eq!(pitch_class_from_name("C"), Some(PitchClass::new(0)));
        assert_eq!(pitch_class_from_name("D"), Some(PitchClass::new(2)));
        assert_eq!(pitch_class_from_name("E"), Some(PitchClass::new(4)));
        assert_eq!(pitch_class_from_name("F"), Some(PitchClass::new(5)));
        assert_eq!(pitch_class_from_name("G"), Some(PitchClass::new(7)));
        assert_eq!(pitch_class_from_name("A"), Some(PitchClass::new(9)));
        assert_eq!(pitch_class_from_name("B"), Some(PitchClass::new(11)));

        // Sharps
        assert_eq!(pitch_class_from_name("C#"), Some(PitchClass::new(1)));
        assert_eq!(pitch_class_from_name("F#"), Some(PitchClass::new(6)));

        // Flats
        assert_eq!(pitch_class_from_name("Db"), Some(PitchClass::new(1)));
        assert_eq!(pitch_class_from_name("Eb"), Some(PitchClass::new(3)));
        assert_eq!(pitch_class_from_name("Bb"), Some(PitchClass::new(10)));
        assert_eq!(pitch_class_from_name("Ab"), Some(PitchClass::new(8)));

        // Case-insensitive
        assert_eq!(pitch_class_from_name("c"), Some(PitchClass::new(0)));
        assert_eq!(pitch_class_from_name("c#"), Some(PitchClass::new(1)));
        assert_eq!(pitch_class_from_name("eb"), Some(PitchClass::new(3)));
        assert_eq!(pitch_class_from_name("gb"), Some(PitchClass::new(6)));

        // Invalid
        assert_eq!(pitch_class_from_name("H"), None);
        assert_eq!(pitch_class_from_name(""), None);
        assert_eq!(pitch_class_from_name("X#"), None);
    }

    #[test]
    fn test_equality_and_ord() {
        assert_eq!(PitchClass::new(0), PitchClass::new(12));
        assert!(PitchClass::new(0) < PitchClass::new(1));
    }
}
