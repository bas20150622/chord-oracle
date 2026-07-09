# Chord Oracle - Specification

## 1. Project Overview

**Chord Oracle** is a visual chord identification tool that allows users to draw finger positions on a guitar neck and receive immediate chord identification with detailed information.

**Core Value Proposition:** Enable musicians to quickly identify chords by clicking on frets rather than manually analyzing finger positions.

---

## 2. Core Features

### 2.1 Guitar Neck Visualization
- Display an interactive guitar neck with 6 strings
- Support three zoom levels/views:
  - **Zoom View:** 5 frets wide (for detailed positioning)
  - **Octave View:** 12 frets wide (for common chord shapes)
  - **Full View:** 24 frets wide (for complete fretboard overview)
- **Z Key:** Cycle through views (Zoom -> Octave -> Full -> Zoom)
- Always display the current view as the primary interface element

### 2.2 Finger Positioning Input
- **Mouse Click Input:**
  - Click on a fret to select a note on that string
  - Click on the nut to select an open string (fret 0)
  - Click on a selected fret to deselect it
  - Each string can have at most one note selected
  - Strings with no selection are simply not played (no separate "muted" concept)
  - Visual feedback for selected frets (highlight/color change)
- **Text Input Mode:**
  - Input field always visible on screen
  - Toggle text input mode with **Enter key**
  - Format: `##,##,##,##,##,##` (six values, one per string from lowest to highest)
  - `0` = open string, `X` = not played, `1-24` = fret number
  - Example: `X,X,0,2,3,2` for a D major chord
  - Confirm text input with **Enter key** again
  - Invalid input displays error message; chord remains unchanged

### 2.3 Tuning System Support
- Default to **Standard Tuning** (E-A-D-G-B-E from lowest to highest string)
- Support multiple alternative tuning systems:
  - Half-step down
  - Drop D
  - Open tunings (user-selectable presets)
  - Custom tuning (user-defined string notes)
- Allow users to switch tunings and see neck update accordingly
- Persist tuning selection during the session

### 2.4 Chord Identification
- Analyze the drawn finger positions and identify the chord
- **Fretboard Display:**
  - Label each note in the selected fingering with its interval number (1, 3, 5, etc.)
  - When Expanded Note View (E key) is enabled, label ALL fretboard locations of chord notes with their interval numbers
- **Display comprehensive chord information below the guitar neck graphic:**
  - **Chord Names with Alternatives:**
    - Display primary chord name plus up to 2 alternative names (max 3 total)
    - Automatically select a sensible default (e.g., most common enharmonic spelling)
    - Allow user to cycle through alternatives with **Tab key**
    - Visually indicate which alternative is currently selected (highlight/border/etc.)
  - **Notes Played:** List of actual notes produced (e.g., "C, E, G")
  - **Intervals (Chord Root):** Interval sequence relative to chord root (e.g., "root-maj3-perf5")
  - **Intervals (Bass Relative):** In adjacent info box, show interval sequence relative to bass note (lowest note played) (e.g., "maj3-perf5-root" if first inversion)
  - **Fingering Position Info:** 
    - String numbers and corresponding frets
    - Fret number relative to the view (e.g., "starting at fret 3")
  - **Difficulty/Position:** Difficulty assessment (e.g., "Beginner", "Intermediate", "Advanced")

### 2.5 Chord Transposition

#### Fretboard Transposition (Left/Right Arrows)
- **Right Arrow:** Move all selected notes up the neck (increase all fret numbers by 1)
- **Left Arrow:** Move all selected notes down the neck (decrease all fret numbers by 1)
- Maintains the same string positions and chord shape
- Movement stops when any selected note reaches a fretboard boundary (fret 0 or fret 24)
- Chord identification updates automatically after transposition

#### String Transposition (Up/Down Arrows)
- **Up Arrow:** Shift all selected notes to higher strings (towards String 1 / high E)
  - All notes maintain exact same pitch
  - Fret positions automatically adjust to play the same notes on higher strings
  - Movement stops when any note reaches a fretboard boundary (fret 0 or fret 24) or String 1
- **Down Arrow:** Shift all selected notes to lower strings (towards String 6 / low E)
  - All notes maintain exact same pitch
  - Fret positions automatically adjust to play the same notes on lower strings
  - Movement stops when any note reaches a fretboard boundary (fret 0 or fret 24) or String 6
- Works regardless of current view (Zoom/Octave/Full)

### 2.6 Expanded Note View
- **Toggle with "E" key:** Show all possible fretboard locations of the chord's notes
- When enabled:
  - Highlights all frets where each note of the current chord can be played
  - Uses a visually secondary color (less prominent than the primary fingering)
  - Displays within the current view (Zoom/Octave/Full)
  - Allows users to explore alternative chord voicings and inversions
  - Updates in real-time as chord is transposed or modified
- When disabled:
  - Returns to standard view showing only the primary fingering
- Helps users understand chord structure across the fretboard

### 2.7 Error Handling
- If the drawn fingering does not match any known chord:
  - Display **"No chord found"** message
  - Optionally suggest closest/similar chord shapes (if determinable)
  - Allow user to continue adjusting the fingering

---

## 3. User Interface Requirements

### 3.1 Layout
- Guitar neck visualization occupies the primary area of the interface
- Chord information display positioned directly below the neck
- Controls for view selection (Zoom/Octave/Full) clearly accessible
- Tuning system selector prominent and easy to change
- Clear visual distinction between the interactive neck and information display

### 3.2 Interactivity

**Mouse Controls:**
- Click on a fret to select a note on that string
- Click on the nut to select an open string (fret 0)
- Click on a selected fret to deselect it

**Hotkey Reference:**

| Key | Action |
|---|---|
| **Right Arrow** | Fretboard transposition up (higher frets) |
| **Left Arrow** | Fretboard transposition down (lower frets) |
| **Up Arrow** | String transposition up (higher strings, same pitches) |
| **Down Arrow** | String transposition down (lower strings, same pitches) |
| **E** | Toggle Expanded Note View |
| **Z** | Cycle view (Zoom -> Octave -> Full) |
| **Tab** | Cycle through chord name alternatives |
| **Enter** | Toggle text input mode / confirm text input |
| **C** | Clear all selected notes |

**Real-time Updates:**
- Chord identification updates as user clicks, types, or transposes

### 3.3 Accessibility
- Keyboard navigation support for clearing/selecting fingers
- Clear visual indicators for marked positions (color, highlighting)
- Responsive design that works across different screen sizes

---

## 4. Functional Requirements

| Requirement | Priority | Status |
|---|---|---|
| Display 3-view guitar neck (Zoom/Octave/Full) | MUST | Pending |
| Cycle views with Z key | MUST | Pending |
| Click fret to select/deselect notes | MUST | Pending |
| Click nut for open string (fret 0) | MUST | Pending |
| Text input for fingering (##,##,##,##,##,##) | MUST | Pending |
| Toggle text input mode (Enter key) | MUST | Pending |
| Validate text input (0, X, 1-24) | MUST | Pending |
| Identify chord from selected notes | MUST | Pending |
| Display chord name | MUST | Pending |
| Display alternative chord names (up to 3) | MUST | Pending |
| Auto-select default alternative | MUST | Pending |
| Cycle through alternatives (Tab key) | MUST | Pending |
| Visually indicate selected alternative | MUST | Pending |
| Label fretboard notes with interval numbers | MUST | Pending |
| Display interval numbers on all notes in Expanded View | MUST | Pending |
| Display notes played | MUST | Pending |
| Display intervals relative to chord root | MUST | Pending |
| Display intervals relative to bass note | MUST | Pending |
| Display fingering position info | MUST | Pending |
| Support standard tuning | MUST | Pending |
| Support multiple alternative tunings | MUST | Pending |
| Allow tuning system switching | MUST | Pending |
| Handle unrecognized chords ("No chord found") | MUST | Pending |
| Fretboard transposition up (Right Arrow) | MUST | Pending |
| Fretboard transposition down (Left Arrow) | MUST | Pending |
| String transposition to higher strings (Up Arrow) | MUST | Pending |
| String transposition to lower strings (Down Arrow) | MUST | Pending |
| Toggle Expanded Note View (E key) | MUST | Pending |
| Clear all notes (C key) | MUST | Pending |
| Responsive layout | SHOULD | Pending |
| Chord difficulty assessment | NICE | Pending |

---

## 5. Technology Stack

### Architecture
- **Chord Engine:** Rust compiled to WebAssembly (WASM)
  - Chord identification logic
  - Music theory computation (intervals, note math, tuning calculations)
  - Transposition logic
  - Built with `wasm-pack`
- **UI Layer:** Vanilla HTML / CSS / JavaScript
  - Fretboard rendering and interaction (click handling, visual feedback)
  - Hotkey handling
  - Text input mode
  - Calls into WASM module for chord identification
- **No server required** -- entirely client-side, runs in browser
- **No npm/node dependency** -- no build toolchain beyond `wasm-pack` and `cargo`

### Build & Deploy
- `wasm-pack build` to compile Rust to WASM
- Static files (HTML, CSS, JS, WASM) served from any web server or opened locally
- No bundler needed

## 6. Out of Scope (For Now)

- Chord library/database browsing
- Saving or favoriting custom fingerings
- Audio playback of chords
- MIDI input
- Learning/tutorial features

---

## 7. Non-Functional Requirements

- **Performance:** Chord identification should complete in <100ms (WASM ensures this)
- **Accuracy:** 100% match accuracy for known chord shapes
- **Scalability:** Support at least 500+ chord variations
- **Browser Compatibility:** All modern browsers with WASM support (Chrome, Firefox, Safari, Edge)
- **No external dependencies at runtime** -- fully offline-capable once loaded

---

## 8. Next Steps

1. Database/library of known chord shapes
2. UI/UX design mockups
3. Development timeline and milestones
4. Testing strategy
