# Chord Oracle

## Build
```
wasm-pack build --target web --out-dir www/pkg
```

## Test
```
cargo test
```

## Run locally
```
./serve.sh
# Open http://localhost:8000
```

## Architecture
- `src/` — Rust chord engine compiled to WASM
  - `note.rs` — PitchClass, enharmonic spelling
  - `interval.rs` — Interval types and computation
  - `chord.rs` — Chord quality database + identification algorithm
  - `tuning.rs` — Tuning presets and validation
  - `fretboard.rs` — Note positions, transposition logic
  - `lib.rs` — WASM API surface (#[wasm_bindgen] exports)
- `www/` — Vanilla HTML/CSS/JS frontend
- `tests/integration.rs` — Chord identification test matrix

## Conventions
- String index 0 = lowest string (string 6, low E in standard tuning)
- PitchClass: C=0, C#/Db=1, ... B=11
- Fret -1 = string not played, 0 = open string, 1-24 = fretted
