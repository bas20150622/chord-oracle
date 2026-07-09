# Chord Oracle

A visual chord identification tool for guitar. Click finger positions on an interactive fretboard and get immediate chord identification with intervals, alternative names, and music theory details.

Runs entirely in the browser — no server-side processing. The chord engine is written in Rust and compiled to WebAssembly.

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (includes `cargo`)
- `wasm-pack` — install with `cargo install wasm-pack`
- Python 3 (for the local dev server, or use any static file server)

### Build and Run

```sh
wasm-pack build --target web --out-dir www/pkg
./serve.sh
```

Open http://localhost:8000.

### Run Tests

```sh
cargo test
```

## Usage

### Selecting Notes

- **Click a fret** to place a note on that string
- **Click the nut** (left edge) to select an open string
- **Click a selected note** to deselect it
- Each string can have at most one note

### Text Input

Type a fingering in the input field using the format `X,X,0,2,3,2` (low E to high e):
- `X` = string not played
- `0` = open string
- `1`-`24` = fret number

Press **Enter** to apply, or **Enter** again to exit text mode.

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| E | Toggle expanded note view (shows all chord tone positions) |
| C | Clear all selected notes |
| Left/Right | Transpose chord shape up/down the neck |
| Up/Down | Shift notes to adjacent strings (preserving pitch) |
| Tab | Cycle through alternative chord name interpretations |
| Enter | Focus text input / confirm |

### Chord Information

When notes are selected, the panel below the fretboard shows:
- **Chord name** with alternative root-note interpretations (click or Tab to cycle)
- **Notes played** and **intervals** relative to both the chord root and the bass note
- **Fingering** and fret position

### Tuning

Use the dropdown in the top-right to switch between tuning presets: Standard, Drop D, Half-step Down, Open G, Open D, Open E.
