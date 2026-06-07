use leptos::prelude::*;

use crate::media::{
    encode_url_path, AudioEntry, FolderEntry, SearchEntry, SearchEntryKind, VideoEntry,
};

// ── Shared minimal style (used by error/search pages) ────────────────────────

const BASE_STYLE: &str = r#"
:root {
    color-scheme: dark;
    --bg:         #0f0f0f;
    --panel:      #1a1a1a;
    --panel2:     #242424;
    --ink:        #e8e8e8;
    --muted:      #888;
    --line:       #333;
    --accent:     #6366f1;
    --accent-dim: #3730a3;
    --focus:      #818cf8;
}

* { box-sizing: border-box; }

html, body {
    margin: 0;
    min-height: 100%;
    background: var(--bg);
    color: var(--ink);
    font-family: "Inter", system-ui, sans-serif;
}

a { color: inherit; text-decoration: none; }

.sr-only {
    position: absolute; width: 1px; height: 1px;
    padding: 0; margin: -1px; overflow: hidden;
    clip: rect(0,0,0,0); white-space: nowrap; border: 0;
}
"#;

// ── TV UI style ───────────────────────────────────────────────────────────────

const TV_STYLE: &str = r#"
/* ── Reset / base ─────────────────────────────────────── */
:root {
    color-scheme: dark;
    --bg:         #0f0f0f;
    --panel:      #1a1a1a;
    --panel2:     #242424;
    --ink:        #e8e8e8;
    --muted:      #888;
    --line:       #2a2a2a;
    --accent:     #6366f1;
    --accent-dim: #3730a3;
    --focus:      #818cf8;
    --playing:    #22c55e;
}

* { box-sizing: border-box; -webkit-tap-highlight-color: transparent; }

html, body {
    margin: 0;
    min-height: 100%;
    background: var(--bg);
    color: var(--ink);
    font-family: "Inter", system-ui, sans-serif;
    font-size: 16px;
    overscroll-behavior: none;
}

a { color: inherit; text-decoration: none; }

/* ── Zone wrapper ──────────────────────────────────────── */
.tv-app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
}

/* ── Zone focus ring ───────────────────────────────────── */
.zone-focused {
    outline: 2px solid var(--focus);
    outline-offset: -2px;
}

/* ── Zone 1: Player ────────────────────────────────────── */
.tv-player {
    width: 100%;
    background: #000;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

#tv-media {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.tv-player video,
.tv-player audio {
    width: 100%;
    height: auto;
    max-height: 100vh;
    object-fit: contain;
    display: block;
    background: #000;
}

.tv-player audio {
    max-height: unset;
    min-height: 10vh;
    width: 100%;
}

.player-placeholder {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 0;
    color: var(--muted);
    font-size: 1.4rem;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    user-select: none;
}

.player-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 1rem;
    background: #000;
    border-top: 1px solid var(--line);
    font-size: 0.82rem;
    color: var(--muted);
    min-height: 2.4rem;
}

.player-bar-title {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--ink);
    font-weight: 600;
}

.player-time {
    font-size: 0.82rem;
    font-family: monospace;
    color: var(--muted);
    white-space: nowrap;
    flex-shrink: 0;
}

.player-hint {
    font-size: 0.72rem;
    color: var(--muted);
    white-space: nowrap;
}

/* ── Zone 2: Queue ─────────────────────────────────────── */
.tv-queue {
    background: var(--panel);
    border-top: 2px solid var(--line);
}

.tv-queue-header {
    display: flex;
    align-items: center;
    padding: 0.6rem 1rem;
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--muted);
    border-bottom: 1px solid var(--line);
    cursor: pointer;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
}
.tv-queue-header::after {
    content: "+";
    margin-left: auto;
    color: var(--accent);
    font-weight: 400;
    font-size: 1rem;
}
.tv-queue-header:hover { color: var(--ink); }
.tv-queue-header:hover::after { color: var(--focus); }
.tv-queue-header:active { color: var(--ink); }
.tv-queue-header:active::after { color: var(--ink); }

.queue-row {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    padding: 0.65rem 1rem;
    border-bottom: 1px solid var(--line);
    font-size: 0.95rem;
    cursor: default;
    user-select: none;
    touch-action: pan-y;
}

.queue-row:last-child {
    border-bottom: none;
}

.queue-row.is-focused {
    background: var(--panel2);
    outline: 2px solid var(--focus);
    outline-offset: -2px;
}

.queue-row.is-playing .queue-row-label {
    color: var(--playing);
    font-weight: 700;
}

.queue-row-num {
    font-size: 0.75rem;
    color: var(--muted);
    min-width: 1.6rem;
    text-align: right;
    flex-shrink: 0;
}

.queue-row-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.queue-row-badge {
    font-size: 0.68rem;
    text-transform: uppercase;
    letter-spacing: 0.07em;
    color: var(--muted);
    background: var(--panel2);
    border: 1px solid var(--line);
    padding: 0.1rem 0.35rem;
    flex-shrink: 0;
}

.queue-row-badge.badge-playing {
    color: var(--playing);
    border-color: var(--playing);
}

.queue-row.is-undefined {
    color: var(--muted);
    font-style: italic;
}

.queue-row-play {
    cursor: pointer;
    background: none;
    border: none;
    color: var(--muted);
    font-size: 1.1rem;
    padding: 0.5rem;
    flex-shrink: 0;
    touch-action: manipulation;
    -webkit-tap-highlight-color: transparent;
    user-select: none;
}
.queue-row-play:hover { color: var(--accent); }
.queue-row-play:active { color: var(--focus); }
.queue-row.is-playing .queue-row-play {
    color: var(--playing);
}

.queue-empty-hint {
    padding: 1rem;
    color: var(--muted);
    font-size: 0.88rem;
    text-align: center;
}

.queue-hint {
    padding: 0.4rem 1rem;
    font-size: 0.72rem;
    color: var(--muted);
    border-top: 1px solid var(--line);
}

/* ── Zone 3: Command picker ────────────────────────────── */
.tv-cmdpicker {
    background: var(--panel2);
    border-top: 2px solid var(--accent-dim);
    display: none;
}

.tv-cmdpicker.is-visible {
    display: block;
}

.tv-cmdpicker-header {
    padding: 0.6rem 1rem;
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--accent);
}

.cmd-options {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.5rem 1rem 1rem;
}

.cmd-option {
    padding: 0.7rem 1.2rem;
    border: 2px solid var(--line);
    background: var(--panel);
    color: var(--ink);
    font: inherit;
    font-size: 0.9rem;
    cursor: pointer;
    user-select: none;
    border-radius: 2px;
    -webkit-tap-highlight-color: transparent;
    touch-action: manipulation;
}
.cmd-option:hover { border-color: var(--accent-dim); }

.cmd-option.is-focused {
    border-color: var(--focus);
    background: var(--panel2);
    color: var(--focus);
    outline: none;
}

/* ── Zone 4: Command config ────────────────────────────── */
.tv-cmdconfig {
    background: var(--bg);
    border-top: 2px solid var(--accent-dim);
    display: none;
}

.tv-cmdconfig.is-visible {
    display: block;
}

.tv-cmdconfig-header {
    padding: 0.6rem 1rem;
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--accent);
    border-bottom: 1px solid var(--line);
    display: flex;
    align-items: center;
    gap: 1rem;
}

.tv-cmdconfig-title {
    flex: 1;
}

/* ── Browser (inside Zone 4) ───────────────────────────── */
.browser-search-row {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.6rem 1rem;
    border-bottom: 1px solid var(--line);
    background: var(--panel);
}

.browser-search-input {
    flex: 1;
    background: var(--panel2);
    border: 2px solid var(--line);
    color: var(--ink);
    font: inherit;
    font-size: 0.9rem;
    padding: 0.4rem 0.7rem;
    height: 2.2rem;
}

.browser-search-input:focus {
    outline: none;
    border-color: var(--focus);
}

.browser-search-input.is-focused {
    border-color: var(--focus);
}

.browser-breadcrumb {
    padding: 0.45rem 1rem;
    font-size: 0.78rem;
    color: var(--muted);
    border-bottom: 1px solid var(--line);
    background: var(--panel);
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
}

.browser-breadcrumb a,
.browser-breadcrumb span {
    color: var(--muted);
}
.breadcrumb-segment {
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
}
.breadcrumb-segment:hover { color: var(--focus); }

.browser-select-row {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    padding: 0.75rem 1rem;
    border-bottom: 2px solid var(--accent-dim);
    background: var(--panel2);
    font-size: 0.92rem;
    font-weight: 600;
    color: var(--accent);
    cursor: pointer;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
}
.browser-select-row:hover { background: var(--panel); }

.browser-select-row.is-focused {
    outline: 2px solid var(--focus);
    outline-offset: -2px;
    color: var(--focus);
}

.browser-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1px;
    background: var(--line);
}

.browser-card {
    display: flex;
    flex-direction: column;
    background: var(--panel);
    overflow: hidden;
    cursor: pointer;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
}
.browser-card:hover { background: var(--panel2); }

.browser-card.is-focused {
    outline: 3px solid var(--focus);
    outline-offset: -3px;
    background: var(--panel2);
}

.browser-card .thumb {
    width: 100%;
    aspect-ratio: 16/9;
    object-fit: cover;
    display: block;
    background: #111;
}

.browser-card .thumb.placeholder {
    display: grid;
    place-items: center;
    color: var(--muted);
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
}

.browser-card .card-meta {
    padding: 0.5rem 0.7rem 0.6rem;
    border-top: 1px solid var(--line);
}

.browser-card .card-meta h3 {
    margin: 0 0 0.2rem;
    font-size: 0.82rem;
    font-weight: 700;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.browser-card .card-meta p {
    margin: 0;
    font-size: 0.7rem;
    color: var(--muted);
}

/* ── Loop picker (inside Zone 4) ───────────────────────── */
.loop-picker {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1.2rem;
    padding: 2rem 1rem;
}

.loop-picker-label {
    font-size: 1rem;
    color: var(--muted);
}

.loop-picker-value {
    font-size: 3rem;
    font-weight: 700;
    color: var(--ink);
    min-width: 6rem;
    text-align: center;
    background: transparent;
    border: 2px solid var(--line);
    border-radius: 4px;
    padding: 0.2rem 0.5rem;
    width: 8rem;
    font-family: inherit;
}
.loop-picker-value:focus { outline: none; border-color: var(--focus); }

.loop-picker-hint {
    font-size: 0.78rem;
    color: var(--muted);
}

.loop-picker-actions {
    display: flex;
    gap: 1rem;
}

.loop-btn {
    padding: 0.6rem 1.6rem;
    border: 2px solid var(--line);
    background: var(--panel);
    color: var(--ink);
    font: inherit;
    font-size: 0.9rem;
    cursor: pointer;
    user-select: none;
    -webkit-tap-highlight-color: transparent;
    touch-action: manipulation;
}
.loop-btn:hover { border-color: var(--accent-dim); }

.loop-btn.is-focused {
    border-color: var(--focus);
    color: var(--focus);
}

/* ── Misc ──────────────────────────────────────────────── */
.helper {
    padding: 0.8rem 1rem;
    color: var(--muted);
    font-size: 0.88rem;
}

@media (max-width: 600px) {
    .browser-grid {
        grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    }
    .cmd-options {
        flex-direction: column;
    }
    .queue-row {
        padding: 0.8rem 1rem;
    }
    .cmd-option {
        padding: 0.9rem 1.4rem;
    }
    .queue-hint {
        font-size: 0.78rem;
        padding: 0.5rem 1rem;
    }
}
"#;

// ── Shared base page shell (for error / search pages) ────────────────────────

const SIMPLE_STYLE: &str = r#"
.simple-app {
    max-width: 860px;
    margin: 0 auto;
    padding: 2rem 1rem;
}
.simple-header {
    margin-bottom: 1.5rem;
    display: flex;
    gap: 1rem;
    align-items: center;
}
.simple-header a {
    color: var(--focus);
    font-size: 0.88rem;
}
.results { border: 1px solid var(--line); background: var(--panel); }
.results-header {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--line);
    color: var(--muted);
    font-size: 0.86rem;
}
.results-list { list-style: none; margin: 0; padding: 0; }
.results-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.6rem;
    padding: 0.72rem 1rem;
    border-top: 1px solid #1e1e1e;
}
.results-row:first-child { border-top: none; }
.results-kind {
    color: var(--muted);
    font-size: 0.74rem;
    text-transform: uppercase;
    letter-spacing: 0.07em;
}
.results-path {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--focus);
}
.results-empty { padding: 1rem; color: var(--muted); }
"#;

// ── TV script ─────────────────────────────────────────────────────────────────

const TV_SCRIPT: &str = r#"
(() => {
'use strict';

const STORAGE_KEY   = 'sapling_queue_v2';
const VIDEO_EXTS    = ['m4v','mp4','webm','mkv','mov','avi'];
const AUDIO_EXTS    = ['mp3','m4a','aac','ogg','opus','wav','flac'];

// ── State ──────────────────────────────────────────────────────────────────
const S = {
    zone:             'queue',   // 'player' | 'queue' | 'cmdpicker' | 'cmdconfig'
    queue:            loadQueue(),
    currentPlaying:   -1,        // index of item currently playing
    queueFocus:       0,         // focused row index in queue zone
    pendingIdx:       -1,        // queue index of the [undefined] item being configured
    pendingCmd:       null,      // 'play_file'|'play_folder'|'random_folder'|'loop'
    cmdFocus:         0,         // focused index in command picker
    // browser sub-state
    browser: {
        path:         '',
        items:        [],
        focus:        0,         // 0 = search bar, -N = select row N (1-indexed), 1+ = grid item
        searchQuery:  '',
        selectRows:   [],        // [{ label, filter }] shown above the grid for play_folder
    },
    // loop picker sub-state
    loopCount:        3,         // 0 = infinite
    loopFocus:        'value',   // 'value' | 'confirm' | 'cancel'
};

// ── Persistence ────────────────────────────────────────────────────────────
function loadQueue() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '[]'); }
    catch { return []; }
}
function saveQueue() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(S.queue));
}

// ── DOM refs ───────────────────────────────────────────────────────────────
const elPlayer    = document.getElementById('tv-player');
const elQueue     = document.getElementById('tv-queue');
const elPicker    = document.getElementById('tv-cmdpicker');
const elConfig    = document.getElementById('tv-cmdconfig');
const elMedia     = document.getElementById('tv-media');
const elPlayerBar = document.getElementById('tv-player-bar');

// ── Click handlers ─────────────────────────────────────────────────────────
elQueue.querySelector('.tv-queue-header').addEventListener('click', () => {
    insertUndefinedAt(S.queue.length);
});

elQueue.querySelector('.tv-queue-rows').addEventListener('click', e => {
    const btn = e.target.closest('.queue-row-play');
    if (!btn) return;
    const idx = parseInt(btn.dataset.idx);
    const item = S.queue[idx];
    if (!item) return;
    if (item.type === 'undefined') {
        S.pendingIdx = idx;
        S.zone = 'cmdpicker';
        S.cmdFocus = 0;
        renderAll();
        elPicker.scrollIntoView({ block: 'nearest' });
    } else {
        S.currentPlaying = idx;
        executeNext();
    }
});

elPicker.querySelector('.cmd-options').addEventListener('click', e => {
    const opt = e.target.closest('.cmd-option');
    if (!opt) return;
    const idx = parseInt(opt.dataset.idx);
    S.cmdFocus = idx;
    const cmd = COMMANDS[idx];
    S.pendingCmd = cmd.id;
    if (cmd.id === 'loop') {
        S.zone = 'cmdconfig';
        S.loopFocus = 'value';
        renderAll();
        elConfig.scrollIntoView({ block: 'nearest' });
    } else {
        S.browser.path = '';
        S.browser.items = [];
        S.browser.focus = 1;
        S.browser.searchQuery = '';
        S.zone = 'cmdconfig';
        renderAll();
        elConfig.scrollIntoView({ block: 'nearest' });
        fetchBrowser('', '');
    }
});

elConfig.addEventListener('click', e => {
    const loopBtn = e.target.closest('.loop-btn');
    if (loopBtn) {
        if (loopBtn.id === 'loop-cancel') {
            cancelConfig();
        } else {
            const n = S.loopCount;
            const lbl = n === 0 ? '↺ Loop ∞' : `↺ Loop ×${n}`;
            replaceUndefinedItem({ type: 'loop', label: lbl, loopCount: n });
        }
        return;
    }
    const card = e.target.closest('.browser-card');
    if (card) {
        const idx = parseInt(card.dataset.idx);
        const b = S.browser;
        const item = b.items[idx];
        if (!item) return;
        if (item.item_type === 'folder') {
            b.path = item.path;
            b.focus = b.selectRows.length > 0 ? -1 : 1;
            b.searchQuery = '';
            fetchBrowser(item.path, '');
        } else if (S.pendingCmd === 'play_file') {
            confirmBrowserSelection(item);
        }
        return;
    }
    const seg = e.target.closest('.breadcrumb-segment');
    if (seg) {
        const path = seg.dataset.path;
        S.browser.path = path;
        S.browser.focus = S.browser.selectRows.length > 0 ? -1 : 1;
        S.browser.searchQuery = '';
        fetchBrowser(path, '');
        return;
    }
    const row = e.target.closest('.browser-select-row');
    if (row) {
        const rowIdx = parseInt(row.dataset.selectIdx);
        confirmSelectRow(rowIdx);
        return;
    }
});

// ── Helpers ────────────────────────────────────────────────────────────────
function mediaTypeFromUrl(url) {
    const ext = url.split('.').pop().split('?')[0].toLowerCase();
    if (VIDEO_EXTS.includes(ext)) return 'video';
    if (AUDIO_EXTS.includes(ext)) return 'audio';
    return 'video';
}

function esc(s) {
    return String(s)
        .replace(/&/g,'&amp;').replace(/</g,'&lt;')
        .replace(/>/g,'&gt;').replace(/"/g,'&quot;');
}

function itemTypeLabel(type) {
    switch (type) {
        case 'play_file':      return 'FILE';
        case 'random_folder':  return '🎲 RANDOM';
        case 'loop':           return '↺ LOOP';
        default:               return type.toUpperCase();
    }
}

function queueItemLabel(item) {
    return item.label || '—';
}

function clamp(v, lo, hi) { return Math.max(lo, Math.min(hi, v)); }

// ── Render: Player ─────────────────────────────────────────────────────────

function formatTime(sec) {
    if (!isFinite(sec) || isNaN(sec)) return '--:--';
    sec = Math.floor(sec);
    const h = Math.floor(sec / 3600);
    const m = Math.floor((sec % 3600) / 60);
    const s = sec % 60;
    const mm = String(m).padStart(h > 0 ? 2 : 1, '0');
    const ss = String(s).padStart(2, '0');
    return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}

function renderPlayer() {
    // media element managed by playMedia / closeMedia — only update bar
    const item = S.currentPlaying >= 0 ? S.queue[S.currentPlaying] : null;
    if (item) {
        const el = currentMediaEl();
        const cur = el ? formatTime(el.currentTime) : '--:--';
        const dur = el ? formatTime(el.duration) : '--:--';
        elPlayerBar.innerHTML = `
            <span class="player-time" id="player-time">${cur} of ${dur}</span>
            <span class="player-bar-title">${esc(item.label)}</span>
            <span class="player-hint">Enter=play/pause  ·  ◀▶=seek 10s  ·  ↓=queue</span>
        `;
    } else {
        elPlayerBar.innerHTML = `<span class="player-hint">Nothing playing  ·  ↓=queue</span>`;
    }
}

// ── Render: Queue ──────────────────────────────────────────────────────────
function renderQueue() {
    const rows = S.queue.length === 0
        ? `<div class="queue-empty-hint">Queue is empty — tap Queue header above to add an item</div>`
        : S.queue.map((item, idx) => {
            const focused  = idx === S.queueFocus && S.zone === 'queue' ? ' is-focused' : '';
            const playing  = idx === S.currentPlaying ? ' is-playing' : '';
            const undef    = item.type === 'undefined' ? ' is-undefined' : '';
            const badge    = item.type === 'undefined'
                ? `<span class="queue-row-badge">— select —</span>`
                : `<span class="queue-row-badge${idx === S.currentPlaying ? ' badge-playing' : ''}">${itemTypeLabel(item.type)}</span>`;
            const playBtn  = item.type === 'undefined'
                ? `<button class="queue-row-play" data-idx="${idx}">✎</button>`
                : `<button class="queue-row-play" data-idx="${idx}">▶</button>`;
            return `
            <div class="queue-row${focused}${playing}${undef}" data-idx="${idx}">
                <span class="queue-row-num">${idx + 1}</span>
                <span class="queue-row-label">${esc(queueItemLabel(item))}</span>
                ${badge}
                ${playBtn}
            </div>`;
        }).join('');

    elQueue.querySelector('.tv-queue-rows').innerHTML = rows;

    // scroll focused row into view
    const focusedEl = elQueue.querySelector('.queue-row.is-focused');
    if (focusedEl) focusedEl.scrollIntoView({ block: 'nearest' });
}

// ── Render: Command picker ─────────────────────────────────────────────────
const COMMANDS = [
    { id: 'play_file',     label: '▶ Play file' },
    { id: 'play_folder',   label: '📂 Play from folder' },
    { id: 'random_folder', label: '🎲 Random from folder' },
    { id: 'loop',          label: '↺ Loop N times' },
];

function renderCmdPicker() {
    const opts = COMMANDS.map((cmd, idx) => {
        const focused = idx === S.cmdFocus ? ' is-focused' : '';
        return `<div class="cmd-option${focused}" data-idx="${idx}">${esc(cmd.label)}</div>`;
    }).join('');
    elPicker.querySelector('.cmd-options').innerHTML = opts;
}

// ── Render: Command config ─────────────────────────────────────────────────
function renderCmdConfig() {
    if (S.pendingCmd === 'loop') {
        renderLoopPicker();
    } else if (S.pendingCmd === 'play_file' || S.pendingCmd === 'play_folder' || S.pendingCmd === 'random_folder') {
        renderBrowser();
    }
}

function renderLoopPicker() {
    const val  = S.loopCount === 0 ? 0 : S.loopCount;
    const hint = val === 0 ? 'Loops forever' : `Loops ${val} time${val === 1 ? '' : 's'}`;
    const confirmFocused = S.loopFocus === 'confirm' ? ' is-focused' : '';
    const cancelFocused  = S.loopFocus === 'cancel'  ? ' is-focused' : '';

    elConfig.querySelector('.tv-cmdconfig-body').innerHTML = `
        <div class="loop-picker">
            <div class="loop-picker-label">Loop how many times? (0 = forever)</div>
            <input type="number" min="0" step="1" value="${val}" class="loop-picker-value" id="loop-count-input">
            <div class="loop-picker-hint">${hint}</div>
            <div class="loop-picker-actions">
                <div class="loop-btn${confirmFocused}" id="loop-confirm">Confirm</div>
                <div class="loop-btn${cancelFocused}"  id="loop-cancel">Cancel</div>
            </div>
        </div>
    `;

    const inp = document.getElementById('loop-count-input');
    if (inp) {
        if (S.loopFocus === 'value') inp.focus();
        inp.addEventListener('input', () => {
            S.loopCount = Math.max(0, parseInt(inp.value) || 0);
            renderCmdConfig();
        });
        inp.addEventListener('keydown', e => {
            if (e.key === 'ArrowDown') {
                e.stopPropagation();
                return;
            }
            if (e.key === 'Enter') {
                e.preventDefault();
                e.stopPropagation();
                const n = S.loopCount;
                const lbl = n === 0 ? '↺ Loop ∞' : `↺ Loop ×${n}`;
                replaceUndefinedItem({ type: 'loop', label: lbl, loopCount: n });
            }
        });
    }
}

function renderBrowser() {
    const b = S.browser;
    const crumbHtml = buildBreadcrumbHtml(b.path);

    const searchFocused = b.focus === 0 ? ' is-focused' : '';

    const selectRowsHtml = b.selectRows.map((row, i) => {
        const focusIdx = -(i + 1);
        const sf = b.focus === focusIdx ? ' is-focused' : '';
        return `<div class="browser-select-row${sf}" data-select-idx="${i}">${esc(row.label)}</div>`;
    }).join('');

    const gridHtml = b.items.length === 0
        ? '<div class="helper">No items here.</div>'
        : b.items.map((item, idx) => {
            // focus index: 0 = search, -N = select row N, 1..N = grid items
            const fi = idx + 1;
            const focused = b.focus === fi ? ' is-focused' : '';
            const thumbHtml = item.thumbnail_url
                ? `<img class="thumb" src="${esc(item.thumbnail_url)}" alt="${esc(item.label)}">`
                : `<div class="thumb placeholder">${item.item_type === 'folder' ? 'Folder' : item.item_type === 'video' ? 'Video' : 'Audio'}</div>`;
            const subtitle = item.item_type === 'folder' ? 'Folder' : 'Play in browser';
            return `
            <div class="browser-card${focused}" data-idx="${idx}">
                ${thumbHtml}
                <div class="card-meta">
                    <h3>${esc(item.label)}</h3>
                    <p>${subtitle}</p>
                </div>
            </div>`;
        }).join('');

    elConfig.querySelector('.tv-cmdconfig-body').innerHTML = `
        <div class="browser-search-row">
            <input class="browser-search-input${searchFocused}" id="browser-search"
                   type="search" placeholder="Search…" value="${esc(b.searchQuery)}">
        </div>
        <div class="browser-breadcrumb">${crumbHtml}</div>
        ${selectRowsHtml}
        <div class="browser-grid" id="browser-grid">${gridHtml}</div>
    `;

    // wire search input
    const inp = document.getElementById('browser-search');
    if (inp) {
        if (b.focus === 0) inp.focus();
        inp.addEventListener('input', e => {
            S.browser.searchQuery = e.target.value;
            fetchBrowser(b.path, e.target.value);
        });
        inp.addEventListener('keydown', e => {
            if (e.key === 'ArrowDown') {
                e.preventDefault();
                S.browser.focus = b.selectRows.length > 0 ? -1 : 1;
                renderCmdConfig();
            }
            if (e.key === 'Escape') {
                e.preventDefault();
                inp.value = '';
                S.browser.searchQuery = '';
                fetchBrowser(b.path, '');
            }
        });
    }

    // scroll focused card into view
    const focusedCard = elConfig.querySelector('.browser-card.is-focused');
    if (focusedCard) focusedCard.scrollIntoView({ block: 'nearest' });
    const focusedSelect = elConfig.querySelector('.browser-select-row.is-focused');
    if (focusedSelect) focusedSelect.scrollIntoView({ block: 'nearest' });
}

function buildBreadcrumbHtml(path) {
    const parts = path ? path.split('/').filter(Boolean) : [];
    let html = `<span class="breadcrumb-segment" data-path="">Root</span>`;
    let running = '';
    for (const part of parts) {
        running += (running ? '/' : '') + part;
        html += ` <span>/</span> <span class="breadcrumb-segment" data-path="${esc(running)}">${esc(part)}</span>`;
    }
    return html;
}

// ── Full re-render ─────────────────────────────────────────────────────────
function renderAll() {
    renderPlayer();
    renderQueue();
    if (S.zone === 'cmdpicker' || S.zone === 'cmdconfig') {
        elPicker.classList.add('is-visible');
        renderCmdPicker();
    } else {
        elPicker.classList.remove('is-visible');
    }
    if (S.zone === 'cmdconfig') {
        elConfig.classList.add('is-visible');
        renderCmdConfig();
    } else {
        elConfig.classList.remove('is-visible');
    }

    // scroll active zone into view
    if (S.zone === 'player')    elPlayer.scrollIntoView({ block: 'nearest' });
    if (S.zone === 'queue')     elQueue.scrollIntoView({ block: 'nearest' });
    if (S.zone === 'cmdpicker') elPicker.scrollIntoView({ block: 'nearest' });
    if (S.zone === 'cmdconfig') elConfig.scrollIntoView({ block: 'nearest' });
}

// ── Queue execution engine ─────────────────────────────────────────────────
function executeNext() {
    if (S.currentPlaying < 0 || S.currentPlaying >= S.queue.length) {
        S.currentPlaying = -1;
        renderAll();
        return;
    }

    renderQueue();
    const item = S.queue[S.currentPlaying];

    if (item.type === 'play_file') {
        playMedia(item.url, item.label);
    } else if (item.type === 'random_folder') {
        fetchRandom(item.path).then(resolved => {
            if (resolved) playMedia(resolved.url, resolved.label);
            else { S.currentPlaying++; executeNext(); }
        });
    } else if (item.type === 'loop') {
        const n = item.loopCount;
        if (n === 0) {
            // infinite — jump to start
            S.currentPlaying = 0;
            executeNext();
        } else {
            item._loopsDone = (item._loopsDone || 0) + 1;
            if (item._loopsDone < n) {
                S.currentPlaying = 0;
                executeNext();
            } else {
                item._loopsDone = 0;
                S.currentPlaying++;
                executeNext();
            }
        }
    } else {
        S.currentPlaying++;
        executeNext();
    }
}

async function fetchRandom(path) {
    try {
        const r = await fetch('/api/list?path=' + encodeURIComponent(path));
        if (!r.ok) return null;
        const items = await r.json();
        const playable = items.filter(i => i.item_type === 'video' || i.item_type === 'audio');
        if (!playable.length) return null;
        return playable[Math.floor(Math.random() * playable.length)];
    } catch { return null; }
}

// ── Media playback ─────────────────────────────────────────────────────────
function playMedia(url, label) {
    const type = mediaTypeFromUrl(url);
    const el = document.createElement(type);
    el.controls = true;
    el.autoplay = true;
    el.preload  = 'auto';
    el.src = url;
    el.id = 'tv-media-el';

    el.addEventListener('ended', () => {
        S.currentPlaying++;
        executeNext();
    });

    elMedia.innerHTML = '';
    elMedia.appendChild(el);

    document.getElementById('player-placeholder')?.remove();
    renderPlayer();
}

function closeMedia() {
    const el = elMedia.querySelector('video, audio');
    if (el) { el.pause(); el.src = ''; }
    elMedia.innerHTML = '<div class="player-placeholder" id="player-placeholder">Sapling</div>';
    renderPlayer();
}

function currentMediaEl() {
    return elMedia.querySelector('video, audio');
}

// ── Browser fetch ──────────────────────────────────────────────────────────
async function fetchBrowser(path, query) {
    const url = '/api/list?path=' + encodeURIComponent(path || '')
        + (query ? '&q=' + encodeURIComponent(query) : '');
    try {
        const r = await fetch(url);
        if (!r.ok) return;
        const items = await r.json();
        S.browser.items = items;
        S.browser.path  = path || '';
        updateSelectRow();
        // default focus to first select row if available, else first grid item
        if (S.browser.selectRows.length > 0) {
            S.browser.focus = -1;
        } else if (S.browser.focus < 0) {
            S.browser.focus = 1;
        }
        renderCmdConfig();
    } catch {}
}

function updateSelectRow() {
    const b = S.browser;
    const cmd = S.pendingCmd;
    if (cmd === 'play_folder') {
        const all    = b.items.filter(i => i.item_type === 'video' || i.item_type === 'audio');
        const videos = b.items.filter(i => i.item_type === 'video');
        const audios = b.items.filter(i => i.item_type === 'audio');
        const rows = [];
        if (all.length > 0)    rows.push({ label: `📂 All files from this folder (${all.length})`,         filter: 'all'   });
        if (videos.length > 0) rows.push({ label: `🎬 All video files from this folder (${videos.length})`, filter: 'video' });
        if (audios.length > 0) rows.push({ label: `🎵 All audio files from this folder (${audios.length})`, filter: 'audio' });
        b.selectRows = rows;
    } else if (cmd === 'random_folder') {
        const all = b.items.filter(i => i.item_type === 'video' || i.item_type === 'audio');
        b.selectRows = all.length > 0
            ? [{ label: `🎲 Random from this folder`, filter: 'all' }]
            : [];
    } else {
        b.selectRows = [];
    }
}

// ── Confirm selection from browser ────────────────────────────────────────
function confirmBrowserSelection(item) {
    let queueItem = null;

    if (S.pendingCmd === 'play_file') {
        queueItem = { type: 'play_file', url: item.url, label: item.label };
    } else if (S.pendingCmd === 'random_folder') {
        queueItem = { type: 'random_folder', path: item.path, label: '🎲 ' + item.label };
    }

    if (!queueItem) return;
    replaceUndefinedItem(queueItem);
}

function confirmSelectRow(rowIdx) {
    const b = S.browser;
    const row = b.selectRows[rowIdx];
    if (!row) return;

    if (S.pendingCmd === 'play_folder') {
        let files = b.items.filter(i => i.item_type === 'video' || i.item_type === 'audio');
        if (row.filter === 'video') files = files.filter(i => i.item_type === 'video');
        if (row.filter === 'audio') files = files.filter(i => i.item_type === 'audio');
        if (files.length === 0) return;

        const newItems = files.map(f => ({ type: 'play_file', url: f.url, label: f.label }));
        // replace the [undefined] slot with first item, insert rest after
        if (S.pendingIdx >= 0 && S.pendingIdx < S.queue.length) {
            S.queue.splice(S.pendingIdx, 1, ...newItems);
        } else {
            S.queue.push(...newItems);
            S.pendingIdx = S.queue.length - newItems.length;
        }
        saveQueue();
        S.pendingCmd = null;
        S.zone = 'queue';
        S.queueFocus = S.pendingIdx;
        elConfig.classList.remove('is-visible');
        elPicker.classList.remove('is-visible');
        renderAll();
    } else if (S.pendingCmd === 'random_folder') {
        const queueItem = { type: 'random_folder', path: b.path, label: '🎲 ' + (b.path.split('/').pop() || 'Root') };
        replaceUndefinedItem(queueItem);
    }
}

function replaceUndefinedItem(queueItem) {
    if (S.pendingIdx >= 0 && S.pendingIdx < S.queue.length) {
        S.queue[S.pendingIdx] = queueItem;
    } else {
        S.queue.push(queueItem);
        S.pendingIdx = S.queue.length - 1;
    }
    saveQueue();
    S.queueFocus  = S.pendingIdx;
    S.pendingIdx  = -1;
    S.pendingCmd  = null;
    S.zone        = 'queue';
    elPicker.classList.remove('is-visible');
    elConfig.classList.remove('is-visible');
    renderAll();
}

// ── Cancel configuration ───────────────────────────────────────────────────
function cancelConfig() {
    // remove the [undefined] item that triggered this
    if (S.pendingIdx >= 0 && S.pendingIdx < S.queue.length
        && S.queue[S.pendingIdx].type === 'undefined') {
        S.queue.splice(S.pendingIdx, 1);
        if (S.currentPlaying > S.pendingIdx) S.currentPlaying--;
        S.queueFocus = clamp(S.pendingIdx, 0, Math.max(0, S.queue.length - 1));
    }
    S.pendingIdx = -1;
    S.pendingCmd = null;
    S.zone       = 'queue';
    elPicker.classList.remove('is-visible');
    elConfig.classList.remove('is-visible');
    saveQueue();
    renderAll();
}

// ── Vertical grid navigation ───────────────────────────────────────────────
function gridNeighbor(direction, currentFocusIdx, items) {
    // focus 1..N maps to items 0..N-1
    const idx = currentFocusIdx - 1;
    if (idx < 0 || idx >= items.length) return currentFocusIdx;

    const grid = document.getElementById('browser-grid');
    if (!grid) return currentFocusIdx;
    const cards = Array.from(grid.querySelectorAll('.browser-card'));
    if (!cards[idx]) return currentFocusIdx;

    const cur = cards[idx].getBoundingClientRect();
    const cx  = cur.left + cur.width  / 2;
    const cy  = cur.top  + cur.height / 2;
    let best = -1, bestScore = Infinity;

    cards.forEach((card, i) => {
        if (i === idx) return;
        const r  = card.getBoundingClientRect();
        const rx = r.left + r.width  / 2;
        const ry = r.top  + r.height / 2;
        const dy = ry - cy;
        if (direction === 'up'   && dy >= -2) return;
        if (direction === 'down' && dy <=  2) return;
        const score = Math.abs(dy) * 4 + Math.abs(rx - cx);
        if (score < bestScore) { bestScore = score; best = i; }
    });

    return best >= 0 ? best + 1 : currentFocusIdx;
}

// ── Key dispatch ───────────────────────────────────────────────────────────
document.addEventListener('keydown', e => {
    const tag = document.activeElement?.tagName;
    const isInput = tag === 'INPUT' || tag === 'TEXTAREA';

    // let the search input handle its own keys (except Escape/ArrowDown handled in renderBrowser)
    if (isInput && e.key !== 'Escape' && e.key !== 'ArrowDown') return;

    switch (S.zone) {
        case 'player':    handlePlayerKey(e); break;
        case 'queue':     handleQueueKey(e);  break;
        case 'cmdpicker': handlePickerKey(e); break;
        case 'cmdconfig': handleConfigKey(e); break;
    }
});

function handlePlayerKey(e) {
    const el = currentMediaEl();
    switch (e.key) {
        case 'ArrowDown':
            e.preventDefault();
            S.zone = 'queue';
            S.queueFocus = clamp(S.queueFocus, 0, Math.max(0, S.queue.length - 1));
            renderAll();
            window.scrollTo({ top: window.innerHeight, behavior: 'smooth' });
            break;
        case 'Enter':
            e.preventDefault();
            if (el) { if (el.paused) el.play(); else el.pause(); }
            break;
        case 'ArrowLeft':
            e.preventDefault();
            if (el) el.currentTime = Math.max(0, el.currentTime - 10);
            break;
        case 'ArrowRight':
            e.preventDefault();
            if (el) el.currentTime = el.currentTime + 10;
            break;
    }
}

function handleQueueKey(e) {
    const len = S.queue.length;

    switch (e.key) {
        case 'ArrowUp':
            e.preventDefault();
            if (S.queueFocus <= 0) {
                S.zone = 'player';
                renderAll();
                window.scrollTo({ top: 0, behavior: 'smooth' });
                const mediaEl = currentMediaEl();
                if (mediaEl) mediaEl.focus();
            } else {
                S.queueFocus--;
                renderQueue();
            }
            break;

        case 'ArrowDown':
            e.preventDefault();
            if (S.queueFocus < len - 1) {
                S.queueFocus++;
                renderQueue();
            }
            // if cmdpicker is open, move focus there
            else if (S.zone === 'queue' && elPicker.classList.contains('is-visible')) {
                S.zone = 'cmdpicker';
                renderAll();
            }
            break;

        case 'ArrowRight':
            e.preventDefault();
            if (len === 0 || S.queueFocus >= len - 1) {
                // append undefined at end
                insertUndefinedAt(len);
            } else {
                // insert undefined after current focus
                insertUndefinedAt(S.queueFocus + 1);
            }
            break;

        case 'ArrowLeft':
            e.preventDefault();
            if (len > 0) {
                const idx = S.queueFocus;
                if (idx === S.currentPlaying) { closeMedia(); S.currentPlaying = -1; }
                else if (idx < S.currentPlaying) S.currentPlaying--;
                S.queue.splice(idx, 1);
                S.queueFocus = clamp(idx, 0, Math.max(0, S.queue.length - 1));
                saveQueue();
                renderAll();
            }
            break;

        case 'Enter':
            e.preventDefault();
            if (len === 0) {
                // Start configuring a new item
                insertUndefinedAt(0);
            } else {
                const item = S.queue[S.queueFocus];
                if (item && item.type !== 'undefined') {
                    S.currentPlaying = S.queueFocus;
                    executeNext();
                } else if (item && item.type === 'undefined') {
                    // open picker for this undefined item
                    S.pendingIdx = S.queueFocus;
                    S.zone       = 'cmdpicker';
                    S.cmdFocus   = 0;
                    renderAll();
                    elPicker.scrollIntoView({ block: 'nearest' });
                }
            }
            break;

        case 'Backspace':
            e.preventDefault();
            // same as left — delete focused item
            if (len > 0) {
                const idx = S.queueFocus;
                if (idx === S.currentPlaying) { closeMedia(); S.currentPlaying = -1; }
                else if (idx < S.currentPlaying) S.currentPlaying--;
                S.queue.splice(idx, 1);
                S.queueFocus = clamp(idx, 0, Math.max(0, S.queue.length - 1));
                saveQueue();
                renderAll();
            }
            break;
    }
}

function insertUndefinedAt(idx) {
    S.queue.splice(idx, 0, { type: 'undefined', label: '—' });
    if (S.currentPlaying >= idx) S.currentPlaying++;
    S.queueFocus = idx;
    S.pendingIdx = idx;
    S.zone       = 'cmdpicker';
    S.cmdFocus   = 0;
    saveQueue();
    renderAll();
    elPicker.scrollIntoView({ block: 'nearest' });
}

function handlePickerKey(e) {
    switch (e.key) {
        case 'ArrowLeft':
            e.preventDefault();
            S.cmdFocus = Math.max(0, S.cmdFocus - 1);
            renderCmdPicker();
            break;
        case 'ArrowRight':
            e.preventDefault();
            S.cmdFocus = Math.min(COMMANDS.length - 1, S.cmdFocus + 1);
            renderCmdPicker();
            break;
        case 'ArrowUp':
        case 'Backspace':
            e.preventDefault();
            cancelConfig();
            break;
        case 'Enter': {
            e.preventDefault();
            const cmd = COMMANDS[S.cmdFocus];
            S.pendingCmd = cmd.id;

            if (cmd.id === 'loop') {
                S.zone     = 'cmdconfig';
                S.loopFocus = 'value';
                renderAll();
                elConfig.scrollIntoView({ block: 'nearest' });
            } else {
                // browser-based config
                S.browser.path        = '';
                S.browser.items       = [];
                S.browser.focus       = 1;
                S.browser.searchQuery = '';
                S.zone = 'cmdconfig';
                renderAll();
                elConfig.scrollIntoView({ block: 'nearest' });
                fetchBrowser('', '');
            }
            break;
        }
    }
}

function handleConfigKey(e) {
    if (S.pendingCmd === 'loop') {
        handleLoopKey(e);
    } else {
        handleBrowserKey(e);
    }
}

function handleLoopKey(e) {
    switch (e.key) {
        case 'ArrowLeft':
            e.preventDefault();
            if (S.loopFocus === 'value') {
                S.loopCount = S.loopCount <= 0 ? 0 : S.loopCount - 1;
                renderCmdConfig();
            } else if (S.loopFocus === 'cancel') {
                S.loopFocus = 'confirm';
                renderCmdConfig();
            }
            break;
        case 'ArrowRight':
            e.preventDefault();
            if (S.loopFocus === 'value') {
                S.loopCount++;
                renderCmdConfig();
            } else if (S.loopFocus === 'confirm') {
                S.loopFocus = 'cancel';
                renderCmdConfig();
            }
            break;
        case 'ArrowDown':
            e.preventDefault();
            S.loopFocus = 'confirm';
            renderCmdConfig();
            break;
        case 'ArrowUp':
            e.preventDefault();
            if (S.loopFocus === 'value') {
                S.zone = 'cmdpicker';
                renderAll();
            } else {
                S.loopFocus = 'value';
                renderCmdConfig();
            }
            break;
        case 'Enter':
            e.preventDefault();
            if (S.loopFocus === 'cancel') {
                cancelConfig();
            } else {
                // confirm or value row — treat as confirm
                const n = S.loopCount;
                const lbl = n === 0 ? '↺ Loop ∞' : `↺ Loop ×${n}`;
                replaceUndefinedItem({ type: 'loop', label: lbl, loopCount: n });
            }
            break;
        case 'Backspace':
            e.preventDefault();
            cancelConfig();
            break;
    }
}

function handleBrowserKey(e) {
    const b   = S.browser;
    const len = b.items.length;
    const nsr = b.selectRows.length; // number of select rows

    // helper: is focus on a select row?
    const isSelectRow = f => f < 0 && f >= -nsr;
    // select row focus: -1 = first, -2 = second, -3 = third

    // if search input is focused, let it handle typing; intercept only nav keys
    if (b.focus === 0) return; // handled by inline listener in renderBrowser

    switch (e.key) {
        case 'ArrowUp': {
            e.preventDefault();
            if (isSelectRow(b.focus)) {
                // move up through select rows, then to search
                if (b.focus < -1) {
                    b.focus++;
                } else {
                    b.focus = 0;
                }
            } else if (b.focus === 1) {
                // top of grid → last select row or search
                b.focus = nsr > 0 ? -nsr : 0;
            } else {
                const nf = gridNeighbor('up', b.focus, b.items);
                b.focus = nf === b.focus ? (nsr > 0 ? -nsr : 0) : nf;
            }
            if (b.focus === 0) {
                renderCmdConfig();
                document.getElementById('browser-search')?.focus();
                return;
            }
            renderCmdConfig();
            break;
        }
        case 'ArrowDown': {
            e.preventDefault();
            if (isSelectRow(b.focus)) {
                // move down through select rows, then to first grid item
                if (b.focus < -1) {
                    b.focus++;
                } else {
                    b.focus = 1;
                }
            } else {
                const nf = gridNeighbor('down', b.focus, b.items);
                b.focus = nf;
            }
            renderCmdConfig();
            break;
        }
        case 'ArrowLeft': {
            e.preventDefault();
            if (b.focus > 1) {
                b.focus--;
            } else if (b.focus === 1 && nsr > 0) {
                b.focus = -1;
            }
            renderCmdConfig();
            break;
        }
        case 'ArrowRight': {
            e.preventDefault();
            if (b.focus < len) {
                b.focus++;
            } else if (isSelectRow(b.focus)) {
                b.focus = 1;
            }
            renderCmdConfig();
            break;
        }
        case 'Enter': {
            e.preventDefault();
            if (isSelectRow(b.focus)) {
                const rowIdx = (-b.focus) - 1;
                confirmSelectRow(rowIdx);
                return;
            }
            const item = b.items[b.focus - 1];
            if (!item) return;
            if (item.item_type === 'folder') {
                // navigate into folder
                b.path        = item.path;
                b.focus       = nsr > 0 ? -1 : 1;
                b.searchQuery = '';
                fetchBrowser(item.path, '');
            } else {
                // file selected
                if (S.pendingCmd === 'play_file') {
                    confirmBrowserSelection(item);
                }
                // play_folder / random_folder should use select row, not individual files
            }
            break;
        }
        case 'Backspace': {
            e.preventDefault();
            // navigate up
            const parts = (b.path || '').split('/').filter(Boolean);
            if (parts.length > 0) {
                parts.pop();
                const newPath = parts.join('/');
                b.path        = newPath;
                b.focus       = 1;
                b.searchQuery = '';
                fetchBrowser(newPath, '');
            } else {
                // at root — cancel
                cancelConfig();
            }
            break;
        }
    }
}

// ── Boot ───────────────────────────────────────────────────────────────────
S.zone       = 'queue';
S.queueFocus = 0;
renderAll();

// tick the player time display every 500ms without a full re-render
setInterval(() => {
    const timeEl = document.getElementById('player-time');
    if (!timeEl) return;
    const el = currentMediaEl();
    if (!el) return;
    timeEl.textContent = `${formatTime(el.currentTime)} of ${formatTime(el.duration)}`;
}, 500);

// restore playing state — if queue has a currentPlaying stored we can't
// auto-resume (media src is gone), so just reset
S.currentPlaying = -1;

})();
"#;

// ── Shared simple shell (error / search results) ──────────────────────────

fn simple_shell(title: &'static str, content: impl IntoView + 'static) -> String {
    let html = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{title}</title>
                <style>{BASE_STYLE}{SIMPLE_STYLE}</style>
            </head>
            <body>
                <div class="simple-app">
                    <div class="simple-header">
                        <a href="/browse/">"⌂ Home"</a>
                    </div>
                    {content}
                </div>
            </body>
        </html>
    };
    format!("<!DOCTYPE html>{}", html.to_html())
}

// ── TV shell ──────────────────────────────────────────────────────────────

fn tv_shell(content: impl IntoView + 'static) -> String {
    let html = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"Sapling"</title>
                <style>{TV_STYLE}</style>
            </head>
            <body>
                <div class="tv-app">
                    {content}
                    // Zone 3: command picker (hidden until needed)
                    <section id="tv-cmdpicker" class="tv-cmdpicker">
                        <div class="tv-cmdpicker-header">"Select queue command"</div>
                        <div class="cmd-options"></div>
                    </section>
                    // Zone 4: command config (hidden until needed)
                    <section id="tv-cmdconfig" class="tv-cmdconfig">
                        <div class="tv-cmdconfig-header">
                            <span class="tv-cmdconfig-title">"Configure"</span>
                        </div>
                        <div class="tv-cmdconfig-body"></div>
                    </section>
                </div>
                <script>{TV_SCRIPT}</script>
            </body>
        </html>
    };
    format!("<!DOCTYPE html>{}", html.to_html())
}

// ── Public render functions ───────────────────────────────────────────────

pub fn render_browse_page(
    _breadcrumbs: &[(String, String)],
    _folders: &[FolderEntry],
    _videos: &[VideoEntry],
    _audio_files: &[AudioEntry],
) -> String {
    tv_shell(view! {
        // Zone 1: Player
        <section id="tv-player" class="tv-player">
            <div id="tv-media">
                <div class="player-placeholder" id="player-placeholder">"Sapling"</div>
            </div>
            <div id="tv-player-bar" class="player-bar">
                <span class="player-hint">"Nothing playing  ·  ↓ = queue"</span>
            </div>
        </section>

        // Zone 2: Queue
        <section id="tv-queue" class="tv-queue">
            <div class="tv-queue-header">"Queue"</div>
            <div class="tv-queue-rows"></div>
            <div class="queue-hint">"Tap ▶ to play  ·  header + to add  ·  ✎ to configure"</div>
        </section>
    })
}

pub fn render_search_results_page(query: &str, entries: &[SearchEntry]) -> String {
    let listing = if entries.is_empty() {
        view! { <div class="results-empty">"No matches found."</div> }.into_any()
    } else {
        entries
            .iter()
            .map(|entry| {
                let href = match entry.kind {
                    SearchEntryKind::Folder => {
                        format!("/browse/{}", encode_url_path(&entry.relative_path))
                    }
                    SearchEntryKind::Video | SearchEntryKind::Audio => {
                        format!("/browse/{}", encode_url_path(
                            entry.relative_path.parent().unwrap_or(&entry.relative_path)
                        ))
                    }
                };
                let kind = match entry.kind {
                    SearchEntryKind::Folder => "Folder",
                    SearchEntryKind::Video  => "Video",
                    SearchEntryKind::Audio  => "Audio",
                };
                view! {
                    <li class="results-row">
                        <a class="results-path" href=href>{entry.relative_path.to_string_lossy().to_string()}</a>
                        <span class="results-kind">{kind}</span>
                    </li>
                }
            })
            .collect_view()
            .into_any()
    };

    simple_shell(
        "Search Results",
        view! {
            <h2 style="margin: 0 0 1rem; font-size: 1.1rem;">{format!("Search: {}", query)}</h2>
            <div class="results">
                <div class="results-header">{format!("{} match(es)", entries.len())}</div>
                {if entries.is_empty() {
                    listing
                } else {
                    view! { <ol class="results-list">{listing}</ol> }.into_any()
                }}
            </div>
        },
    )
}

pub fn render_not_found(message: String) -> String {
    simple_shell(
        "Not Found",
        view! {
            <h2 style="margin: 0 0 0.5rem;">"Not found"</h2>
            <p style="color: var(--muted);">{message}</p>
        },
    )
}
