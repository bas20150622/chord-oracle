# Chord Oracle — GUI Mockup

## Main Layout (Octave View, 12 frets shown, D major selected: X,X,0,2,3,2)

```
+-----------------------------------------------------------------------------------+
|  CHORD ORACLE                                          Tuning: [Standard v]       |
+-----------------------------------------------------------------------------------+
|                                                                                   |
|  Nut                                                                              |
|   |   1     2     3     4     5     6     7     8     9    10    11    12           |
|   |   |     |     |     |     |     |     |     |     |     |     |     |          |
|   |---|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|  e (1)     |
|   |   |     |  (o)|     |     |     |     |     |     |     |     |     |          |
|   |---|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|  B (2)     |
|   |   |     |     |  (o)|     |     |     |     |     |     |     |     |          |
|   |---|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|  G (3)     |
|   |   |     |  (o)|     |     |     |     |     |     |     |     |     |          |
|   |---|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|  D (4)     |
|   | o |     |     |     |     |     |     |     |     |     |     |     |          |
|   |---|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|  A (5)     |
|   |   |     |     |     |     |     |     |     |     |     |     |     |          |
|   |---|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|  E (6)     |
|   |   |     |     |     |     |     |     |     |     |     |     |     |     X    |
|                        *           *                 *                             |
|                    (fret dots at 3, 5, and 10)                                     |
|                                                                                   |
|  Input: [X,X,0,2,3,2___________________]   View: [Zoom] [*Octave*] [Full]        |
|                                                                                   |
+-----------------------------------------------------------------------------------+
|                                                                                   |
|  +-- CHORD INFO ---------------------------------------------------------------+  |
|  |                                                                             |  |
|  |  Name:  [*D*]  [D major]  [D maj]           <-- Tab to cycle, * = selected  |  |
|  |                                                                             |  |
|  |  +-- Root-relative ----+  +-- Bass-relative ----+                           |  |
|  |  |  Notes:  D, F#, A   |  |  Notes:  D, F#, A   |                           |  |
|  |  |  Intervals:         |  |  Intervals:          |                           |  |
|  |  |    R - M3 - P5      |  |    R - M3 - P5       |                           |  |
|  |  +---------------------+  +----------------------+                           |  |
|  |                                                                             |  |
|  |  Fingering:  X X 0 2 3 2    Position: Open/Fret 0-3                         |  |
|  |  Difficulty: [Beginner]                                                     |  |
|  |                                                                             |  |
|  +-----------------------------------------------------------------------------+  |
|                                                                                   |
+-----------------------------------------------------------------------------------+
|  Z: View  E: Expand  C: Clear  Arrows: Transpose  Enter: Text input  Tab: Alt    |
+-----------------------------------------------------------------------------------+
```

## Fretboard Detail — Note Markers

Selected notes show interval labels inside the marker:

```
    Nut
     |    1      2      3      4
     |    |      |      |      |
     |----|------|------|------|---  e (1)
     |    |      | [M3] |      |        <-- filled circle, interval label inside
     |----|------|------|------|---  B (2)
     |    |      |      | [P5] |        <-- filled circle
     |----|------|------|------|---  G (3)
     |    |      | [R]  |      |        <-- filled circle, "R" = root
     |----|------|------|------|---  D (4)
     |[R] |      |      |      |        <-- open string marker at nut, "R" = root
     |----|------|------|------|---  A (5)
     |    |      |      |      |        <-- no marker = string not played
     |----|------|------|------|---  E (6)
     | X  |      |      |      |        <-- X at nut = not played
```

## Expanded Note View (E key toggled ON)

All chord tones highlighted across visible fretboard in secondary color:

```
    Nut
     |    1      2      3      4      5      6      7
     |    |      |      |      |      |      |      |
     |----|------|------|------|------|------|------|---  e (1)
     |    |  {P5}| [M3] |      | {P5}|      |      |
     |----|------|------|------|------|------|------|---  B (2)
     |{M3}|      |      | [P5] |      |  {R}|      |
     |----|------|------|------|------|------|------|---  G (3)
     |    |      | [R]  |      |      |{M3} |      |
     |----|------|------|------|------|------|------|---  D (4)
     |[R] |      |      |      | {M3}|      |      |
     |----|------|------|------|------|------|------|---  A (5)
     |    |      |      |      |  {R} |      |      |
     |----|------|------|------|------|------|------|---  E (6)
     | X  |      |{M3}  |      |      | {P5}|      |

     [brackets] = selected fingering (primary color)
     {braces}   = other chord tone positions (secondary/dimmed color)
```

## Chord Name Alternatives — Tab Cycling

```
  Name:  [*D*]  [D major]  [D maj]

  Press Tab -->

  Name:  [D]  [*D major*]  [D maj]

  Press Tab -->

  Name:  [D]  [D major]  [*D maj*]
```

The selected alternative is visually highlighted (bold border, accent color).
The selected name is what's used as the primary display name.

## Slash Chord Example (D/F# — first inversion)

Fingering: 2,X,0,2,3,2

```
  +-- CHORD INFO ----------------------------------------------------------+
  |                                                                        |
  |  Name:  [*D/F#*]  [D maj/F#]                                          |
  |                                                                        |
  |  +-- Root-relative ----+  +-- Bass-relative --------+                  |
  |  |  Notes:  D, F#, A   |  |  Notes:  F#, A, D       |                 |
  |  |  Intervals:         |  |  Intervals:              |                 |
  |  |    R - M3 - P5      |  |    R(bass) - m3 - m6     |                 |
  |  +---------------------+  +-------------------------+                  |
  |                                                                        |
  |  Fingering:  2 X 0 2 3 2    Position: Fret 0-3                         |
  |  Difficulty: [Intermediate]                                            |
  +------------------------------------------------------------------------+
```

## "No Chord Found" State

```
  +-- CHORD INFO ----------------------------------------------------------+
  |                                                                        |
  |  No chord found                                                        |
  |                                                                        |
  |  Notes played: C, D, F#                                                |
  |                                                                        |
  +------------------------------------------------------------------------+
```

## Tuning Selector (expanded)

```
  Tuning: [ Standard          v ]
          |----------------------|
          | Standard   E A D G B E |
          | Drop D     D A D G B E |
          | Half-step  Eb Ab Db Gb Bb Eb |
          | Open G     D G D G B D |
          | Open D     D A D F# A D |
          | Open E     E B E G# B E |
          | Custom...              |
          |------------------------|
```

## Color Scheme (proposed)

- Background: dark charcoal (#1a1a2e)
- Fretboard wood: warm brown gradient (#3d2b1f to #5c3d2e)
- Strings: silver/light gray (#c0c0c0), thicker for bass strings
- Selected notes: bright teal (#00d4aa) filled circles
- Expanded view notes: dimmed teal (#00d4aa at 40% opacity)
- Interval labels: white text on note markers
- Chord info panel: slightly lighter background (#252540)
- Accent/highlight: gold (#ffd700) for selected alternative name
- "Not played" X: muted red (#ff6b6b at 60%)
- Hotkey bar: dark footer with muted text
