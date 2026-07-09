use serde::Serialize;

/// A guitar tuning: six strings, each with a MIDI note number.
/// Index 0 is the lowest-pitched string (string 6, typically low E).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Tuning {
    pub name: String,
    pub strings: [u8; 6],
}

const MIDI_MIN: u8 = 20;
const MIDI_MAX: u8 = 100;

/// Standard tuning: E2 A2 D3 G3 B3 E4.
pub fn standard() -> Tuning {
    Tuning {
        name: "Standard".to_string(),
        strings: [40, 45, 50, 55, 59, 64],
    }
}

/// Drop D tuning: D2 A2 D3 G3 B3 E4.
pub fn drop_d() -> Tuning {
    Tuning {
        name: "Drop D".to_string(),
        strings: [38, 45, 50, 55, 59, 64],
    }
}

/// Half-step down tuning: Eb2 Ab2 Db3 Gb3 Bb3 Eb4.
pub fn half_step_down() -> Tuning {
    Tuning {
        name: "Half-step Down".to_string(),
        strings: [39, 44, 49, 54, 58, 63],
    }
}

/// Open G tuning: D2 G2 D3 G3 B3 D4.
pub fn open_g() -> Tuning {
    Tuning {
        name: "Open G".to_string(),
        strings: [38, 43, 50, 55, 59, 62],
    }
}

/// Open D tuning: D2 A2 D3 F#3 A3 D4.
pub fn open_d() -> Tuning {
    Tuning {
        name: "Open D".to_string(),
        strings: [38, 45, 50, 54, 57, 62],
    }
}

/// Open E tuning: E2 B2 E3 G#3 B3 E4.
pub fn open_e() -> Tuning {
    Tuning {
        name: "Open E".to_string(),
        strings: [40, 47, 52, 56, 59, 64],
    }
}

/// Returns all preset tunings.
pub fn presets() -> Vec<Tuning> {
    vec![
        standard(),
        drop_d(),
        half_step_down(),
        open_g(),
        open_d(),
        open_e(),
    ]
}

/// Builds a custom tuning, validating that every MIDI note number falls
/// within a reasonable range (20-100 inclusive).
///
/// The `strings` parameter is already fixed at length 6 by its type
/// ([u8; 6]), so only the value range is checked.
pub fn custom(name: &str, strings: [u8; 6]) -> Result<Tuning, String> {
    for &note in strings.iter() {
        if note < MIDI_MIN || note > MIDI_MAX {
            return Err(format!(
                "MIDI note {} out of range ({}-{})",
                note, MIDI_MIN, MIDI_MAX
            ));
        }
    }

    Ok(Tuning {
        name: name.to_string(),
        strings,
    })
}

/// Looks up a preset tuning by name, case-insensitively.
pub fn preset_by_name(name: &str) -> Option<Tuning> {
    presets()
        .into_iter()
        .find(|t| t.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_tuning_values() {
        let t = standard();
        assert_eq!(t.name, "Standard");
        assert_eq!(t.strings, [40, 45, 50, 55, 59, 64]);
    }

    #[test]
    fn all_presets_have_correct_midi_values() {
        let presets = presets();
        assert_eq!(presets.len(), 6);

        assert_eq!(presets[0].strings, [40, 45, 50, 55, 59, 64]); // Standard
        assert_eq!(presets[1].strings, [38, 45, 50, 55, 59, 64]); // Drop D
        assert_eq!(presets[2].strings, [39, 44, 49, 54, 58, 63]); // Half-step Down
        assert_eq!(presets[3].strings, [38, 43, 50, 55, 59, 62]); // Open G
        assert_eq!(presets[4].strings, [38, 45, 50, 54, 57, 62]); // Open D
        assert_eq!(presets[5].strings, [40, 47, 52, 56, 59, 64]); // Open E

        let names: Vec<&str> = presets.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "Standard",
                "Drop D",
                "Half-step Down",
                "Open G",
                "Open D",
                "Open E",
            ]
        );
    }

    #[test]
    fn custom_tuning_valid() {
        let t = custom("My Tuning", [40, 45, 50, 55, 59, 64]).unwrap();
        assert_eq!(t.name, "My Tuning");
        assert_eq!(t.strings, [40, 45, 50, 55, 59, 64]);
    }

    #[test]
    fn custom_tuning_boundary_values_valid() {
        let t = custom("Boundary", [20, 20, 20, 20, 20, 100]).unwrap();
        assert_eq!(t.strings, [20, 20, 20, 20, 20, 100]);
    }

    #[test]
    fn custom_tuning_below_min_invalid() {
        let result = custom("Too Low", [19, 45, 50, 55, 59, 64]);
        assert!(result.is_err());
    }

    #[test]
    fn custom_tuning_above_max_invalid() {
        let result = custom("Too High", [40, 45, 50, 55, 59, 101]);
        assert!(result.is_err());
    }

    #[test]
    fn preset_by_name_case_insensitive() {
        assert_eq!(preset_by_name("standard"), Some(standard()));
        assert_eq!(preset_by_name("STANDARD"), Some(standard()));
        assert_eq!(preset_by_name("Drop D"), Some(drop_d()));
        assert_eq!(preset_by_name("drop d"), Some(drop_d()));
        assert_eq!(preset_by_name("open g"), Some(open_g()));
    }

    #[test]
    fn preset_by_name_not_found() {
        assert_eq!(preset_by_name("Nonexistent Tuning"), None);
    }
}
