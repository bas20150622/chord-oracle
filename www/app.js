import init, {
    identify_chord,
    transpose_frets,
    transpose_strings_wasm,
    get_chord_positions,
    get_tuning_presets,
    validate_text_input,
} from './pkg/chord_oracle.js';

const NUM_STRINGS = 6;
const MAX_FRET = 24;

const STRING_NAMES_LOW_TO_HIGH = ['E', 'A', 'D', 'G', 'B', 'e'];
const STRING_THICKNESS_LOW_TO_HIGH = [3, 2.6, 2.2, 1.8, 1.4, 1];

const DOT_FRETS = new Set([3, 5, 7, 9, 15, 17, 19, 21]);
const DOUBLE_DOT_FRETS = new Set([12, 24]);

const VIEW_FRETS = 24;

const state = {
    frets: [-1, -1, -1, -1, -1, -1],
    tuning: [40, 45, 50, 55, 59, 64],
    expandedView: false,
    altIndex: 0,
    textMode: false,
    chordResult: null,
    tuningPresets: [],
};

function rowToStateIndex(row) { return NUM_STRINGS - 1 - row; }

// --- SVG layout ---
const VB_WIDTH = 1000;
const VB_HEIGHT = 260;
const MARGIN_LEFT = 50;
const MARGIN_RIGHT = 80;
const MARGIN_TOP = 30;
const MARGIN_BOTTOM = 20;
const NUT_ZONE_WIDTH = 34;
const SVG_NS = 'http://www.w3.org/2000/svg';

function el(tag, attrs = {}) {
    const node = document.createElementNS(SVG_NS, tag);
    for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
    return node;
}

function computeFretWindow() {
    return { start: 0, end: VIEW_FRETS };
}

function fretX(fretNum, startFret, span) {
    const usableWidth = VB_WIDTH - MARGIN_LEFT - MARGIN_RIGHT - NUT_ZONE_WIDTH;
    return MARGIN_LEFT + NUT_ZONE_WIDTH + ((fretNum - startFret) / span) * usableWidth;
}

function stringY(row) {
    const usableHeight = VB_HEIGHT - MARGIN_TOP - MARGIN_BOTTOM;
    return MARGIN_TOP + (row / (NUM_STRINGS - 1)) * usableHeight;
}

function getSelectedMatch() {
    if (!state.chordResult) return null;
    const all = [state.chordResult.primary, ...state.chordResult.alternatives];
    return all[state.altIndex] || state.chordResult.primary;
}

function getOrderedNotesAndIntervals(referencePc, sourceIntervals) {
    const items = sourceIntervals.map(iv => {
        const semitones = ((iv.note_pc - referencePc) % 12 + 12) % 12;
        return { note_name: iv.note_name, semitones, short_name: INTERVAL_SHORT_NAMES[semitones] };
    });
    items.sort((a, b) => {
        if (a.semitones === 0 && b.semitones !== 0) return -1;
        if (b.semitones === 0 && a.semitones !== 0) return 1;
        return a.semitones - b.semitones;
    });
    return { notes: items.map(i => i.note_name), intervals: items.map(i => i.short_name) };
}

function hexToRgba(hex, alpha) {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

function getOrderedChordPcs() {
    const selected = getSelectedMatch();
    if (!selected || !state.chordResult) return null;
    const cr = state.chordResult;
    const items = cr.intervals_from_root.map(iv => {
        const semitones = ((iv.note_pc - selected.root_pc) % 12 + 12) % 12;
        return { note_pc: iv.note_pc, semitones };
    });
    items.sort((a, b) => {
        if (a.semitones === 0 && b.semitones !== 0) return -1;
        if (b.semitones === 0 && a.semitones !== 0) return 1;
        return a.semitones - b.semitones;
    });
    return items.map(i => i.note_pc);
}

function getNoteColor(pc, orderedPcs) {
    const idx = orderedPcs ? orderedPcs.indexOf(pc) : -1;
    return idx >= 0 && idx < NOTE_COLORS.length ? NOTE_COLORS[idx] : '#00d4aa';
}

function getIntervalLabel(stateIdx, fret) {
    const selected = getSelectedMatch();
    if (!selected || fret < 0) return null;
    const midi = state.tuning[stateIdx] + fret;
    const pc = midi % 12;
    const semitones = ((pc - selected.root_pc) % 12 + 12) % 12;
    return INTERVAL_SHORT_NAMES[semitones];
}

function renderFretboard() {
    const svg = document.getElementById('fretboard-svg');
    svg.innerHTML = '';

    const { start, end } = computeFretWindow();
    const span = end - start;
    const showNut = start === 0;

    const g = el('g', { class: 'fretboard-group' });

    // Fret lines
    for (let f = start; f <= end; f++) {
        const x = fretX(f, start, span);
        if (f === 0 && showNut) {
            g.appendChild(el('line', {
                class: 'nut-line', x1: x, y1: stringY(0) - 10,
                x2: x, y2: stringY(5) + 10, 'stroke-width': 6, 'stroke-linecap': 'round',
            }));
        } else if (f > start || !showNut) {
            g.appendChild(el('line', {
                class: 'fret-line', x1: x, y1: stringY(0) - 6,
                x2: x, y2: stringY(5) + 6, 'stroke-width': 2,
            }));
        }
    }

    // Fret numbers
    for (let f = start; f <= end; f++) {
        if (f === 0) continue;
        const xMid = (fretX(f - 1, start, span) + fretX(f, start, span)) / 2;
        const t = el('text', { class: 'fret-number', x: xMid, y: MARGIN_TOP - 12 });
        t.textContent = String(f);
        g.appendChild(t);
    }

    // Fret dots
    for (let f = start + 1; f <= end; f++) {
        if (!DOT_FRETS.has(f) && !DOUBLE_DOT_FRETS.has(f)) continue;
        const xMid = (fretX(f - 1, start, span) + fretX(f, start, span)) / 2;
        const midY = (stringY(0) + stringY(5)) / 2;
        if (DOUBLE_DOT_FRETS.has(f)) {
            g.appendChild(el('circle', { class: 'fret-dot', cx: xMid, cy: midY - 18, r: 6 }));
            g.appendChild(el('circle', { class: 'fret-dot', cx: xMid, cy: midY + 18, r: 6 }));
        } else {
            g.appendChild(el('circle', { class: 'fret-dot', cx: xMid, cy: midY, r: 6 }));
        }
    }

    // Strings + labels
    const sStartX = showNut ? fretX(0, start, span) : MARGIN_LEFT;
    const sEndX = fretX(end, start, span);
    for (let row = 0; row < NUM_STRINGS; row++) {
        const y = stringY(row);
        const si = rowToStateIndex(row);
        g.appendChild(el('line', {
            class: 'string-line', x1: sStartX, y1: y, x2: sEndX, y2: y,
            'stroke-width': STRING_THICKNESS_LOW_TO_HIGH[si], 'stroke-linecap': 'round',
        }));
        const lbl = el('text', { class: 'string-label', x: sEndX + 16, y });
        lbl.textContent = STRING_NAMES_LOW_TO_HIGH[si];
        g.appendChild(lbl);
    }

    // Expanded view: secondary chord tone markers
    if (state.expandedView && state.chordResult) {
        renderExpandedPositions(g, start, end, span);
    }

    // Hit areas
    const hitG = el('g', { class: 'hit-group' });
    const rowH = (VB_HEIGHT - MARGIN_TOP - MARGIN_BOTTOM) / (NUM_STRINGS - 1);

    if (showNut) {
        for (let row = 0; row < NUM_STRINGS; row++) {
            hitG.appendChild(el('rect', {
                class: 'nut-hit', x: MARGIN_LEFT, y: stringY(row) - rowH / 2,
                width: NUT_ZONE_WIDTH, height: rowH, fill: 'transparent',
                'data-string': rowToStateIndex(row), 'data-nut': '1',
            }));
        }
    }
    for (let f = Math.max(start, 1); f <= end; f++) {
        const xPrev = fretX(f - 1, start, span);
        const xCur = fretX(f, start, span);
        for (let row = 0; row < NUM_STRINGS; row++) {
            hitG.appendChild(el('rect', {
                class: 'fret-hit', x: xPrev, y: stringY(row) - rowH / 2,
                width: xCur - xPrev, height: rowH, fill: 'transparent',
                'data-string': rowToStateIndex(row), 'data-fret': f,
            }));
        }
    }
    g.appendChild(hitG);

    // Note markers
    const noteG = el('g', { class: 'note-group' });
    const orderedPcs = getOrderedChordPcs();
    for (let row = 0; row < NUM_STRINGS; row++) {
        const si = rowToStateIndex(row);
        const fret = state.frets[si];
        const y = stringY(row);

        if (fret === -1) {
            if (showNut) {
                const cx = fretX(0, start, span);
                const s = 6;
                noteG.appendChild(el('line', { class: 'mute-marker', x1: cx - s, y1: y - s, x2: cx + s, y2: y + s, 'stroke-linecap': 'round' }));
                noteG.appendChild(el('line', { class: 'mute-marker', x1: cx - s, y1: y + s, x2: cx + s, y2: y - s, 'stroke-linecap': 'round' }));
            }
            continue;
        }

        if (fret === 0) {
            if (showNut) {
                const cx = fretX(0, start, span);
                const openColor = getNoteColor(state.tuning[si] % 12, orderedPcs);
                noteG.appendChild(el('circle', { class: 'open-marker', cx, cy: y, r: 10, stroke: openColor }));
                const label = getIntervalLabel(si, 0);
                if (label) {
                    const t = el('text', { class: 'open-label', x: cx, y: y + 0.5, fill: openColor });
                    t.textContent = label;
                    noteG.appendChild(t);
                }
            }
            continue;
        }

        if (fret < start || fret > end) continue;
        const xPrev = fretX(fret - 1, start, span);
        const xCur = fretX(fret, start, span);
        const cx = (xPrev + xCur) / 2;
        const fretColor = getNoteColor((state.tuning[si] + fret) % 12, orderedPcs);
        noteG.appendChild(el('circle', { class: 'note-marker', cx, cy: y, r: 13, fill: fretColor }));
        const label = getIntervalLabel(si, fret);
        const t = el('text', { class: 'note-label', x: cx, y });
        t.textContent = label || '';
        noteG.appendChild(t);
    }
    g.appendChild(noteG);

    svg.appendChild(g);
    hitG.addEventListener('click', onFretboardClick);
}

function renderExpandedPositions(g, start, end, span) {
    const cr = state.chordResult;
    if (!cr) return;

    const selected = getSelectedMatch();
    const expRootPc = selected ? selected.root_pc : cr.primary.root_pc;
    const intervals = cr.intervals_from_root.map(iv => iv.semitones);
    const positions = get_chord_positions(
        expRootPc, new Uint8Array(intervals),
        new Uint8Array(state.tuning), start, end
    );

    if (!positions || positions.error) return;

    const orderedPcs = getOrderedChordPcs();

    for (const pos of positions) {
        const row = NUM_STRINGS - 1 - pos.string_index;
        const y = stringY(row);
        const fret = pos.fret;

        if (state.frets[pos.string_index] === fret) continue;

        let cx;
        if (fret === 0) {
            cx = fretX(0, start, span);
        } else if (fret >= start && fret <= end) {
            const xPrev = fretX(fret - 1, start, span);
            const xCur = fretX(fret, start, span);
            cx = (xPrev + xCur) / 2;
        } else {
            continue;
        }

        const expPc = (state.tuning[pos.string_index] + pos.fret) % 12;
        const expColor = getNoteColor(expPc, orderedPcs);
        g.appendChild(el('circle', {
            class: 'expanded-marker', cx, cy: y, r: 10,
            fill: hexToRgba(expColor, 0.15), stroke: hexToRgba(expColor, 0.4),
        }));
        const t = el('text', { class: 'expanded-label', x: cx, y, fill: hexToRgba(expColor, 0.6) });
        t.textContent = pos.interval_short;
        g.appendChild(t);
    }
}

// --- Click handling ---

function onFretboardClick(evt) {
    const target = evt.target;
    if (!(target instanceof Element)) return;
    const stringAttr = target.getAttribute('data-string');
    if (stringAttr === null) return;
    const s = parseInt(stringAttr, 10);

    if (target.hasAttribute('data-nut')) {
        state.frets[s] = state.frets[s] === 0 ? -1 : 0;
    } else {
        const fretAttr = target.getAttribute('data-fret');
        if (fretAttr === null) return;
        const fret = parseInt(fretAttr, 10);
        state.frets[s] = state.frets[s] === fret ? -1 : fret;
    }
    state.altIndex = 0;
    onStateChanged();
}

// --- State change handler ---

function onStateChanged() {
    updateChordResult();
    renderFretboard();
    renderChordInfo();
    syncTextInput();
}

function updateChordResult() {
    const anyPlayed = state.frets.some(f => f >= 0);
    if (!anyPlayed) {
        state.chordResult = null;
        return;
    }
    const result = identify_chord(
        new Int8Array(state.frets),
        new Uint8Array(state.tuning)
    );
    state.chordResult = result && !result.error ? result : null;
}

// --- Chord info panel ---

function renderChordInfo() {
    const panel = document.getElementById('chord-info');
    panel.innerHTML = '';

    const anySelected = state.frets.some(f => f !== -1);
    if (!anySelected) {
        const p = document.createElement('p');
        p.className = 'chord-info-placeholder';
        p.textContent = 'Select notes on the fretboard';
        panel.appendChild(p);
        return;
    }

    const cr = state.chordResult;
    if (!cr) {
        const p = document.createElement('p');
        p.className = 'chord-info-placeholder';
        p.textContent = 'No chord found';
        panel.appendChild(p);

        const fingering = document.createElement('p');
        fingering.className = 'chord-info-fingering';
        fingering.innerHTML = `<span class="label">Fingering:</span>${formatFingering(state.frets)}`;
        panel.appendChild(fingering);
        return;
    }

    // Chord names row
    const namesRow = document.createElement('div');
    namesRow.className = 'chord-names';

    const allNames = [cr.primary, ...cr.alternatives];
    allNames.forEach((match, i) => {
        const btn = document.createElement('button');
        btn.className = 'chord-name-btn' + (i === state.altIndex ? ' active' : '');
        btn.textContent = match.display_name;
        btn.addEventListener('click', () => { state.altIndex = i; renderFretboard(); renderChordInfo(); });
        namesRow.appendChild(btn);
    });
    panel.appendChild(namesRow);

    const selected = allNames[state.altIndex] || cr.primary;

    // Info grid: root-relative + bass-relative
    const grid = document.createElement('div');
    grid.className = 'chord-info-grid';

    // Root-relative box
    const rootInfo = getOrderedNotesAndIntervals(selected.root_pc, cr.intervals_from_root);
    const rootBox = document.createElement('div');
    rootBox.className = 'interval-box';
    rootBox.innerHTML = `
        <h3>Root-relative <span class="root-note">(${selected.root_name})</span></h3>
        <div class="info-row"><span class="label">Notes:</span> ${rootInfo.notes.join(', ')}</div>
        <div class="info-row"><span class="label">Intervals:</span> ${rootInfo.intervals.join(' - ')}</div>
    `;
    grid.appendChild(rootBox);

    // Bass-relative box
    const bassInfo = getOrderedNotesAndIntervals(selected.bass_pc, cr.intervals_from_root);
    const bassBox = document.createElement('div');
    bassBox.className = 'interval-box';
    bassBox.innerHTML = `
        <h3>Bass-relative <span class="root-note">(${selected.bass_name})</span></h3>
        <div class="info-row"><span class="label">Notes:</span> ${bassInfo.notes.join(', ')}</div>
        <div class="info-row"><span class="label">Intervals:</span> ${bassInfo.intervals.join(' - ')}</div>
    `;
    grid.appendChild(bassBox);
    panel.appendChild(grid);

    // Fingering row
    const meta = document.createElement('div');
    meta.className = 'chord-meta';
    meta.innerHTML = `
        <span><span class="label">Fingering:</span> ${formatFingering(state.frets)}</span>
        <span><span class="label">Position:</span> ${formatPosition(state.frets)}</span>
        <span><span class="label">Voicing:</span> ${getInversionLabel(selected)}</span>
    `;
    panel.appendChild(meta);
}

function getInversionLabel(selected) {
    if (!selected.is_inversion) return 'Root position';
    const interval = ((selected.bass_pc - selected.root_pc) % 12 + 12) % 12;
    if (interval === 3 || interval === 4) return '1st inversion';
    if (interval === 6 || interval === 7 || interval === 8) return '2nd inversion';
    return '3rd inversion';
}

function formatFingering(frets) {
    return frets.map(f => f === -1 ? 'X' : String(f)).join(' ');
}

function formatPosition(frets) {
    const played = frets.filter(f => f >= 0);
    if (played.length === 0) return '-';
    const min = Math.min(...played);
    const max = Math.max(...played.filter(f => f > 0));
    if (max === 0 || isNaN(max) || !isFinite(max)) return 'Open';
    return min === 0 ? `Open/Fret 1-${max}` : `Fret ${min}-${max}`;
}

// --- Text input ---

function syncTextInput() {
    const input = document.getElementById('fret-text');
    if (!input || document.activeElement === input) return;
    const any = state.frets.some(f => f !== -1);
    input.value = any ? state.frets.map(f => f === -1 ? 'X' : String(f)).join(',') : '';
}

function handleTextInputConfirm() {
    const input = document.getElementById('fret-text');
    if (!input) return;

    const result = validate_text_input(input.value);
    if (result.valid && result.frets) {
        for (let i = 0; i < 6; i++) state.frets[i] = result.frets[i];
        state.altIndex = 0;
        state.textMode = false;
        input.blur();
        onStateChanged();
    } else {
        input.classList.add('input-error');
        setTimeout(() => input.classList.remove('input-error'), 800);
    }
}

// --- Keyboard controls ---

function onKeyDown(e) {
    const input = document.getElementById('fret-text');
    const inputFocused = document.activeElement === input;

    if (e.key === 'Enter') {
        e.preventDefault();
        if (inputFocused) {
            handleTextInputConfirm();
        } else {
            state.textMode = true;
            input?.focus();
        }
        return;
    }

    if (inputFocused) return;

    switch (e.key.toLowerCase()) {
        case 'e':
            e.preventDefault();
            state.expandedView = !state.expandedView;
            renderFretboard();
            break;
        case 'c':
            e.preventDefault();
            state.frets.fill(-1);
            state.altIndex = 0;
            state.expandedView = false;
            onStateChanged();
            break;
        case 'tab':
            e.preventDefault();
            if (state.chordResult) {
                const total = 1 + state.chordResult.alternatives.length;
                state.altIndex = (state.altIndex + 1) % total;
                renderFretboard();
                renderChordInfo();
            }
            break;
        case 'arrowright':
            e.preventDefault();
            handleTransposeFrets(1);
            break;
        case 'arrowleft':
            e.preventDefault();
            handleTransposeFrets(-1);
            break;
        case 'arrowup':
            e.preventDefault();
            handleTransposeStrings(1);
            break;
        case 'arrowdown':
            e.preventDefault();
            handleTransposeStrings(-1);
            break;
    }
}

function handleTransposeFrets(direction) {
    const result = transpose_frets(new Int8Array(state.frets), direction);
    if (result && result.success) {
        for (let i = 0; i < 6; i++) state.frets[i] = result.frets[i];
        state.altIndex = 0;
        onStateChanged();
    }
}

function handleTransposeStrings(direction) {
    const result = transpose_strings_wasm(
        new Int8Array(state.frets),
        new Uint8Array(state.tuning),
        direction
    );
    if (result && result.success) {
        for (let i = 0; i < 6; i++) state.frets[i] = result.frets[i];
        state.altIndex = 0;
        onStateChanged();
    }
}

// --- Tuning selector ---

function initTuningSelector() {
    const select = document.getElementById('tuning');
    if (!select) return;

    const presets = get_tuning_presets();
    state.tuningPresets = presets;
    select.innerHTML = '';
    select.disabled = false;

    for (const preset of presets) {
        const opt = document.createElement('option');
        opt.value = preset.name;
        opt.textContent = `${preset.name}  ${formatTuningNotes(preset.strings)}`;
        select.appendChild(opt);
    }

    select.addEventListener('change', () => {
        const preset = presets.find(p => p.name === select.value);
        if (preset) {
            for (let i = 0; i < 6; i++) state.tuning[i] = preset.strings[i];
            state.frets.fill(-1);
            state.altIndex = 0;
            onStateChanged();
        }
    });
}

const NOTE_NAMES = ['C', 'C#', 'D', 'Eb', 'E', 'F', 'F#', 'G', 'Ab', 'A', 'Bb', 'B'];
const INTERVAL_SHORT_NAMES = ['R', 'm2', 'M2', 'm3', 'M3', 'P4', 'b5', 'P5', '#5', 'M6', 'm7', 'M7'];
const NOTE_COLORS = ['#e84040', '#00d4aa', '#ffd700', '#4a9eff', '#ff69b4', '#ff8c00'];
function formatTuningNotes(strings) {
    return strings.map(midi => NOTE_NAMES[midi % 12]).join(' ');
}

// --- Init ---

async function main() {
    await init();
    initTuningSelector();
    document.addEventListener('keydown', onKeyDown);
    renderFretboard();
    renderChordInfo();
    syncTextInput();
}

main();
