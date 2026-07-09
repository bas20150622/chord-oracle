pub mod note;
pub mod interval;
pub mod chord;
pub mod tuning;
pub mod fretboard;

use wasm_bindgen::prelude::*;
use serde::Serialize;

use interval::Interval;
use note::PitchClass;

// ---------------------------------------------------------------------------
// JS-friendly response structs
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct JsChordMatch {
    root_name: String,
    root_pc: u8,
    quality_symbol: String,
    quality_name: String,
    bass_name: String,
    bass_pc: u8,
    is_inversion: bool,
    display_name: String,
}

#[derive(Serialize)]
struct JsInterval {
    note_name: String,
    note_pc: u8,
    semitones: u8,
    short_name: String,
    long_name: String,
}

#[derive(Serialize)]
struct JsChordResult {
    primary: JsChordMatch,
    alternatives: Vec<JsChordMatch>,
    notes_played: Vec<String>,
    intervals_from_root: Vec<JsInterval>,
    intervals_from_bass: Vec<JsInterval>,
}

#[derive(Serialize)]
struct JsTransposeResult {
    success: bool,
    frets: Vec<i8>,
}

#[derive(Serialize)]
struct JsValidationResult {
    valid: bool,
    frets: Option<Vec<i8>>,
    error: Option<String>,
}

#[derive(Serialize)]
struct JsChordPosition {
    string_index: usize,
    fret: u8,
    note_name: String,
    interval_short: String,
}

#[derive(Serialize)]
struct JsError {
    error: String,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn to_js_chord_match(m: &chord::ChordMatch) -> JsChordMatch {
    JsChordMatch {
        root_name: m.root.name().to_string(),
        root_pc: m.root.semitones(),
        quality_symbol: m.quality.symbol.to_string(),
        quality_name: m.quality.name.to_string(),
        bass_name: m.bass.name().to_string(),
        bass_pc: m.bass.semitones(),
        is_inversion: m.is_inversion,
        display_name: m.display_name(),
    }
}

fn to_js_interval(pc: PitchClass, iv: Interval) -> JsInterval {
    JsInterval {
        note_name: pc.name().to_string(),
        note_pc: pc.semitones(),
        semitones: iv.semitones,
        short_name: iv.short_name().to_string(),
        long_name: iv.long_name().to_string(),
    }
}

fn to_js_chord_result(result: &chord::ChordResult) -> JsChordResult {
    JsChordResult {
        primary: to_js_chord_match(&result.primary),
        alternatives: result.alternatives.iter().map(to_js_chord_match).collect(),
        notes_played: result.notes_played.iter().map(|pc| pc.name().to_string()).collect(),
        intervals_from_root: result
            .intervals_from_root
            .iter()
            .map(|&(pc, iv)| to_js_interval(pc, iv))
            .collect(),
        intervals_from_bass: result
            .intervals_from_bass
            .iter()
            .map(|&(pc, iv)| to_js_interval(pc, iv))
            .collect(),
    }
}

fn error_value(message: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&JsError {
        error: message.to_string(),
    })
    .unwrap_or(JsValue::NULL)
}

// ---------------------------------------------------------------------------
// Exported functions
// ---------------------------------------------------------------------------

#[wasm_bindgen]
pub fn identify_chord(frets: &[i8], tuning: &[u8]) -> JsValue {
    if frets.len() != 6 {
        return error_value("frets must have exactly 6 entries");
    }
    if tuning.len() != 6 {
        return error_value("tuning must have exactly 6 entries");
    }

    let mut frets_arr = [0i8; 6];
    frets_arr.copy_from_slice(frets);
    let mut tuning_arr = [0u8; 6];
    tuning_arr.copy_from_slice(tuning);

    match chord::identify(&frets_arr, &tuning_arr) {
        Some(result) => {
            let js_result = to_js_chord_result(&result);
            serde_wasm_bindgen::to_value(&js_result).unwrap_or_else(|e| error_value(&e.to_string()))
        }
        None => error_value("No chord found"),
    }
}

#[wasm_bindgen]
pub fn transpose_frets(frets: &[i8], direction: i8) -> JsValue {
    if frets.len() != 6 {
        return error_value("frets must have exactly 6 entries");
    }

    let mut frets_arr = [0i8; 6];
    frets_arr.copy_from_slice(frets);

    let result = match fretboard::transpose_fretboard(&frets_arr, direction) {
        Some(new_frets) => JsTransposeResult {
            success: true,
            frets: new_frets.to_vec(),
        },
        None => JsTransposeResult {
            success: false,
            frets: frets_arr.to_vec(),
        },
    };

    serde_wasm_bindgen::to_value(&result).unwrap_or_else(|e| error_value(&e.to_string()))
}

#[wasm_bindgen]
pub fn transpose_strings_wasm(frets: &[i8], tuning: &[u8], direction: i8) -> JsValue {
    if frets.len() != 6 {
        return error_value("frets must have exactly 6 entries");
    }
    if tuning.len() != 6 {
        return error_value("tuning must have exactly 6 entries");
    }

    let mut frets_arr = [0i8; 6];
    frets_arr.copy_from_slice(frets);
    let mut tuning_arr = [0u8; 6];
    tuning_arr.copy_from_slice(tuning);

    let result = match fretboard::transpose_strings(&frets_arr, &tuning_arr, direction) {
        Some(new_frets) => JsTransposeResult {
            success: true,
            frets: new_frets.to_vec(),
        },
        None => JsTransposeResult {
            success: false,
            frets: frets_arr.to_vec(),
        },
    };

    serde_wasm_bindgen::to_value(&result).unwrap_or_else(|e| error_value(&e.to_string()))
}

#[wasm_bindgen]
pub fn get_chord_positions(
    root_pc: u8,
    quality_intervals: &[u8],
    tuning: &[u8],
    fret_start: u8,
    fret_end: u8,
) -> JsValue {
    if tuning.len() != 6 {
        return error_value("tuning must have exactly 6 entries");
    }

    let mut tuning_arr = [0u8; 6];
    tuning_arr.copy_from_slice(tuning);

    let root = PitchClass::new(root_pc);

    // Build the set of pitch classes in the chord (root + each interval), deduped.
    let mut pitch_classes: Vec<PitchClass> = Vec::new();
    for &semitones in quality_intervals {
        let pc = PitchClass::new(root.semitones() + (semitones % 12));
        if !pitch_classes.contains(&pc) {
            pitch_classes.push(pc);
        }
    }
    if pitch_classes.is_empty() {
        pitch_classes.push(root);
    }

    let positions = fretboard::chord_tone_positions(&pitch_classes, &tuning_arr, fret_start, fret_end);

    let js_positions: Vec<JsChordPosition> = positions
        .into_iter()
        .map(|(string_index, fret)| {
            let open_midi = tuning_arr[string_index];
            let note_pc = fretboard::fret_to_pitch_class(open_midi, fret);
            let iv = Interval::between(root, note_pc);
            JsChordPosition {
                string_index,
                fret,
                note_name: note_pc.name().to_string(),
                interval_short: iv.short_name().to_string(),
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&js_positions).unwrap_or_else(|e| error_value(&e.to_string()))
}

#[wasm_bindgen]
pub fn get_tuning_presets() -> JsValue {
    let presets = tuning::presets();
    serde_wasm_bindgen::to_value(&presets).unwrap_or_else(|e| error_value(&e.to_string()))
}

#[wasm_bindgen]
pub fn validate_text_input(input: &str) -> JsValue {
    let tokens: Vec<&str> = input.split(',').map(|t| t.trim()).collect();

    if tokens.len() != 6 {
        let result = JsValidationResult {
            valid: false,
            frets: None,
            error: Some(format!("Expected 6 comma-separated values, got {}", tokens.len())),
        };
        return serde_wasm_bindgen::to_value(&result).unwrap_or_else(|e| error_value(&e.to_string()));
    }

    let mut frets: Vec<i8> = Vec::with_capacity(6);
    for token in tokens {
        if token.eq_ignore_ascii_case("x") {
            frets.push(-1);
            continue;
        }
        match token.parse::<i32>() {
            Ok(n) if (0..=24).contains(&n) => frets.push(n as i8),
            Ok(n) => {
                let result = JsValidationResult {
                    valid: false,
                    frets: None,
                    error: Some(format!("Fret value {} out of range (0-24)", n)),
                };
                return serde_wasm_bindgen::to_value(&result)
                    .unwrap_or_else(|e| error_value(&e.to_string()));
            }
            Err(_) => {
                let result = JsValidationResult {
                    valid: false,
                    frets: None,
                    error: Some(format!("Invalid token '{}': expected 'x' or a number 0-24", token)),
                };
                return serde_wasm_bindgen::to_value(&result)
                    .unwrap_or_else(|e| error_value(&e.to_string()));
            }
        }
    }

    let result = JsValidationResult {
        valid: true,
        frets: Some(frets),
        error: None,
    };
    serde_wasm_bindgen::to_value(&result).unwrap_or_else(|e| error_value(&e.to_string()))
}
