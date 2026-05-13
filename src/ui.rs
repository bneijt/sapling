use leptos::prelude::*;

use crate::media::{
    encode_url_path, AudioEntry, FolderEntry, SearchEntry, SearchEntryKind, VideoEntry,
};

const STYLE: &str = r#"
:root {
    color-scheme: light;
    --bg: #f3f4f6;
    --panel: #ffffff;
    --panel-hover: #f8fafc;
    --header: #ffffff;
    --ink: #111827;
    --muted: #6b7280;
    --line: #d1d5db;
}

* { box-sizing: border-box; }

html,
body {
    min-height: 100%;
}

body {
    margin: 0;
    font-family: "Inter", sans-serif;
    color: var(--ink);
    background: var(--bg);
}

a { color: inherit; text-decoration: none; }

.app {
    min-height: 100vh;
}

.header {
    padding: 0.85rem 1rem;
    border-bottom: 1px solid var(--line);
    background: var(--header);
    display: flex;
    gap: 0.8rem;
    align-items: center;
    justify-content: flex-end;
}

.path {
    color: var(--muted);
    font-size: 0.88rem;
    display: flex;
    flex-wrap: wrap;
    gap: 0.45rem;
}

.header-right {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    min-width: min(72vw, 760px);
    width: 100%;
}

.search-tools {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 0.4rem;
}

.home-button {
    width: auto;
    height: 2.1rem;
    border: 1px solid var(--line);
    background: #ffffff;
    color: var(--ink);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0 0.55rem;
    font-size: 0.95rem;
    line-height: 1;
}

.home-button:hover {
    background: var(--panel-hover);
}

.search-form {
    margin: 0;
    width: min(420px, 100%);
}

.search-input {
    width: 100%;
    height: 2.1rem;
    border: 1px solid var(--line);
    background: #ffffff;
    color: var(--ink);
    padding: 0 0.7rem;
    font: inherit;
}

.search-input:focus {
    outline: 2px solid #c7d2fe;
    outline-offset: 0;
}

.grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 1px;
    background: var(--line);
    border-top: 1px solid var(--line);
    border-bottom: 1px solid var(--line);
}

.grid:empty {
    border-bottom: none;
}

.card {
    display: block;
    background: var(--panel);
    overflow: hidden;
    min-width: 0;
    transition: background 0.18s ease;
}

.card:hover {
    background: var(--panel-hover);
}

.is-selected {
    outline: 3px solid #c7d2fe;
    outline-offset: -3px;
    background: #eef2ff;
}

.thumb {
    width: 100%;
    aspect-ratio: 16 / 9;
    object-fit: cover;
    display: block;
    background: #e5e7eb;
}

.thumb.placeholder {
    display: grid;
    place-items: center;
    color: var(--muted);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
}

.meta {
    padding: 0.65rem 0.8rem 0.8rem;
    border-top: 1px solid var(--line);
}

.meta h3 {
    margin: 0 0 0.3rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 0.9rem;
    font-weight: 700;
}

.meta p {
    margin: 0;
    color: var(--muted);
    font-size: 0.76rem;
}

.player-wrap {
    padding: 0.8rem 1rem;
}

audio,
video {
    width: 100%;
}

video {
    max-height: calc(100vh - 8rem);
    background: #000;
}

.helper {
    padding: 0.8rem 1rem;
    color: var(--muted);
    background: #f9fafb;
}

.cta {
    display: inline-block;
    padding: 0.5rem 0.85rem;
    border: 1px solid var(--line);
    background: #ffffff;
    font-weight: 700;
    color: var(--ink);
}

.results {
    border-top: 1px solid var(--line);
    border-bottom: 1px solid var(--line);
    background: var(--panel);
}

.results-header {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--line);
    color: var(--muted);
    font-size: 0.86rem;
}

.results-list {
    list-style: none;
    margin: 0;
    padding: 0;
}

.results-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.6rem;
    padding: 0.72rem 1rem;
    border-top: 1px solid #eceff3;
}

.results-row:first-child {
    border-top: none;
}

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
}

.results-empty {
    padding: 1rem;
    color: var(--muted);
}

@media (max-width: 640px) {
    .header { flex-direction: column; align-items: stretch; }
    .header-right { width: 100%; min-width: 0; flex-direction: column; align-items: stretch; }
    .search-tools { margin-left: 0; width: 100%; }
    .search-form { width: 100%; }
    .grid { grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); }
}

/* ── Queue panel ─────────────────────────────────────────────── */

body.has-queue {
    padding-bottom: 48px;
}

.queue-panel {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 100;
    background: var(--header);
    border-top: 1px solid var(--line);
    height: 48px;
    overflow: hidden;
    transition: height 0.2s ease;
    display: flex;
    flex-direction: column;
}

.queue-panel.is-open {
    height: 240px;
}

.queue-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 0.8rem;
    height: 48px;
    flex-shrink: 0;
    border-bottom: 1px solid var(--line);
}

.queue-header-title {
    font-size: 0.82rem;
    font-weight: 700;
    color: var(--ink);
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.queue-now-playing {
    font-size: 0.78rem;
    color: var(--muted);
    flex: 2;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.queue-btn {
    border: 1px solid var(--line);
    background: #fff;
    color: var(--ink);
    font: inherit;
    font-size: 0.78rem;
    padding: 0.2rem 0.5rem;
    cursor: pointer;
    height: 1.8rem;
    line-height: 1;
    white-space: nowrap;
}

.queue-btn:hover { background: var(--panel-hover); }

.queue-list {
    overflow-y: auto;
    flex: 1;
}

.queue-empty {
    padding: 0.6rem 1rem;
    color: var(--muted);
    font-size: 0.82rem;
}

.queue-item {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0.3rem 0.8rem;
    border-bottom: 1px solid #f0f0f0;
    font-size: 0.82rem;
}

.queue-item.is-active {
    background: #eef2ff;
}

.queue-item-index {
    color: var(--muted);
    font-size: 0.72rem;
    min-width: 1.4rem;
    text-align: right;
}

.queue-item-label {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.queue-item-type {
    font-size: 0.68rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--muted);
    background: var(--bg);
    padding: 0.1rem 0.3rem;
    border: 1px solid var(--line);
}

.queue-item-btn {
    border: none;
    background: none;
    cursor: pointer;
    color: var(--muted);
    font-size: 0.8rem;
    padding: 0.1rem 0.25rem;
    line-height: 1;
}

.queue-item-btn:hover { color: var(--ink); }
.queue-item-btn:disabled { opacity: 0.3; cursor: default; }

/* ── Player overlay ──────────────────────────────────────────── */

.player-overlay {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 201;
    width: min(600px, calc(100vw - 2rem));
    background: var(--panel);
    border: 1px solid var(--line);
    box-shadow: 0 8px 32px rgba(0,0,0,0.18);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: height 0.18s ease;
}

.player-overlay[hidden] { display: none; }

.player-overlay.is-minimized {
    height: 52px !important;
}

.overlay-header {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0 0.7rem;
    height: 52px;
    flex-shrink: 0;
    border-bottom: 1px solid var(--line);
    background: var(--header);
    cursor: move;
}

.overlay-title {
    flex: 1;
    font-size: 0.82rem;
    font-weight: 700;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.overlay-btn {
    border: 1px solid var(--line);
    background: #fff;
    color: var(--ink);
    font: inherit;
    font-size: 0.82rem;
    padding: 0.15rem 0.45rem;
    cursor: pointer;
    height: 1.7rem;
    line-height: 1;
}

.overlay-btn:hover { background: var(--panel-hover); }

.overlay-media-wrap {
    background: #000;
}

.overlay-media-wrap video,
.overlay-media-wrap audio {
    width: 100%;
    max-height: 340px;
    display: block;
}

.overlay-media-wrap audio {
    max-height: unset;
    background: #1a1a2e;
    padding: 1rem;
}

/* ── Context menu ────────────────────────────────────────────── */

.ctx-menu {
    position: fixed;
    z-index: 300;
    background: var(--panel);
    border: 1px solid var(--line);
    box-shadow: 0 4px 16px rgba(0,0,0,0.14);
    min-width: 200px;
    padding: 0.25rem 0;
}

.ctx-menu[hidden] { display: none; }

.ctx-item {
    display: block;
    width: 100%;
    text-align: left;
    border: none;
    background: none;
    font: inherit;
    font-size: 0.85rem;
    color: var(--ink);
    padding: 0.48rem 1rem;
    cursor: pointer;
    white-space: nowrap;
}

.ctx-item:hover { background: var(--panel-hover); }

.ctx-separator {
    height: 1px;
    background: var(--line);
    margin: 0.25rem 0;
}
"#;

const KEYBOARD_NAV_SCRIPT: &str = r#"
(() => {
    const items = Array.from(document.querySelectorAll('[data-nav-item]'));
    if (items.length === 0) {
        return;
    }

    const parentHref = document.body.getAttribute('data-parent-href') || '';
    let selectedIndex = 0;

    const isTypingTarget = () => {
        const active = document.activeElement;
        if (!active) {
            return false;
        }

        if (active.matches('input, textarea, select')) {
            return true;
        }

        return active.getAttribute('contenteditable') === 'true';
    };

    const setSelected = (index) => {
        if (!Number.isInteger(index) || index < 0 || index >= items.length) {
            return;
        }

        items[selectedIndex].classList.remove('is-selected');
        selectedIndex = index;
        items[selectedIndex].classList.add('is-selected');
        items[selectedIndex].scrollIntoView({ block: 'nearest', inline: 'nearest' });
    };

    const openSelected = () => {
        const href = items[selectedIndex].getAttribute('href');
        if (href) {
            window.location.assign(href);
        }
    };

    const verticalNeighbor = (direction) => {
        const currentRect = items[selectedIndex].getBoundingClientRect();
        const currentCenterX = currentRect.left + currentRect.width / 2;
        const currentCenterY = currentRect.top + currentRect.height / 2;
        let bestIndex = selectedIndex;
        let bestScore = Number.POSITIVE_INFINITY;

        items.forEach((item, idx) => {
            if (idx === selectedIndex) {
                return;
            }

            const rect = item.getBoundingClientRect();
            const centerX = rect.left + rect.width / 2;
            const centerY = rect.top + rect.height / 2;
            const verticalDelta = centerY - currentCenterY;

            if (direction === 'up' && verticalDelta >= -2) {
                return;
            }

            if (direction === 'down' && verticalDelta <= 2) {
                return;
            }

            const verticalDistance = Math.abs(verticalDelta);
            const horizontalDistance = Math.abs(centerX - currentCenterX);
            const score = verticalDistance * 4 + horizontalDistance;

            if (score < bestScore) {
                bestScore = score;
                bestIndex = idx;
            }
        });

        return bestIndex;
    };

    items.forEach((item, idx) => {
        item.addEventListener('mouseenter', () => setSelected(idx));
        item.addEventListener('focus', () => setSelected(idx));
    });

    setSelected(0);

    document.addEventListener('keydown', (event) => {
        if (isTypingTarget()) {
            return;
        }

        switch (event.key) {
            case 'ArrowLeft':
                event.preventDefault();
                setSelected(Math.max(0, selectedIndex - 1));
                break;
            case 'ArrowRight':
                event.preventDefault();
                setSelected(Math.min(items.length - 1, selectedIndex + 1));
                break;
            case 'ArrowUp':
                event.preventDefault();
                setSelected(verticalNeighbor('up'));
                break;
            case 'ArrowDown':
                event.preventDefault();
                setSelected(verticalNeighbor('down'));
                break;
            case 'Enter':
                event.preventDefault();
                openSelected();
                break;
            case 'Backspace':
                if (parentHref) {
                    event.preventDefault();
                    window.location.assign(parentHref);
                }
                break;
            default:
                break;
        }
    });
})();
"#;

const QUEUE_SCRIPT: &str = r#"
(() => {
    // ── Constants ────────────────────────────────────────────────
    const STORAGE_KEY = 'sapling_queue';
    const VIDEO_EXTS = ['m4v','mp4','webm','mkv','mov','avi'];
    const AUDIO_EXTS = ['mp3','m4a','aac','ogg','opus','wav','flac'];

    // ── State ────────────────────────────────────────────────────
    let queue = loadQueue();
    let currentIndex = -1;
    let overlayMinimized = false;

    // ── Persistence ──────────────────────────────────────────────
    function loadQueue() {
        try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '[]'); }
        catch { return []; }
    }

    function saveQueue() {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(queue));
    }

    // ── DOM refs (injected by page_shell) ────────────────────────
    const panel     = document.getElementById('sapling-queue-panel');
    const overlay   = document.getElementById('sapling-player-overlay');
    const ctxMenu   = document.getElementById('sapling-ctx-menu');

    if (!panel || !overlay || !ctxMenu) return;

    // ── Media type helpers ───────────────────────────────────────
    function mediaTypeFromUrl(url) {
        const ext = url.split('.').pop().split('?')[0].toLowerCase();
        if (VIDEO_EXTS.includes(ext)) return 'video';
        if (AUDIO_EXTS.includes(ext)) return 'audio';
        return 'video';
    }

    // ── Queue execution ──────────────────────────────────────────
    function startQueue(fromIndex) {
        currentIndex = fromIndex != null ? fromIndex : 0;
        executeNext();
    }

    function executeNext() {
        if (currentIndex < 0 || currentIndex >= queue.length) {
            currentIndex = -1;
            renderPanel();
            return;
        }

        renderPanel();
        const item = queue[currentIndex];

        if (item.type === 'media') {
            playMedia(item.url, item.label);
        } else if (item.type === 'random_from_folder') {
            resolveRandomFromFolder(item.path).then(resolved => {
                if (resolved) {
                    playMedia(resolved.url, resolved.label);
                } else {
                    currentIndex++;
                    executeNext();
                }
            });
        } else if (item.type === 'loop') {
            currentIndex = 0;
            executeNext();
        } else if (item.type === 'clear') {
            queue = [];
            currentIndex = -1;
            saveQueue();
            closeOverlay();
            renderPanel();
        } else {
            currentIndex++;
            executeNext();
        }
    }

    async function resolveRandomFromFolder(path) {
        try {
            const resp = await fetch('/api/list?path=' + encodeURIComponent(path));
            if (!resp.ok) return null;
            const items = await resp.json();
            const playable = items.filter(i => i.item_type === 'video' || i.item_type === 'audio');
            if (playable.length === 0) return null;
            return playable[Math.floor(Math.random() * playable.length)];
        } catch {
            return null;
        }
    }

    // ── Player overlay ───────────────────────────────────────────
    function buildOverlay() {
        overlay.innerHTML = `
            <div class="overlay-header" id="sapling-overlay-header">
                <span class="overlay-title" id="sapling-overlay-title">—</span>
                <button class="overlay-btn" id="sapling-overlay-minimize" title="Minimize">—</button>
                <button class="overlay-btn" id="sapling-overlay-close" title="Close">✕</button>
            </div>
            <div class="overlay-media-wrap" id="sapling-overlay-media"></div>
        `;

        document.getElementById('sapling-overlay-minimize').addEventListener('click', () => {
            overlayMinimized = !overlayMinimized;
            overlay.classList.toggle('is-minimized', overlayMinimized);
            document.getElementById('sapling-overlay-minimize').textContent = overlayMinimized ? '□' : '—';
        });

        document.getElementById('sapling-overlay-close').addEventListener('click', () => {
            closeOverlay();
            currentIndex = -1;
            renderPanel();
        });

        makeDraggable(overlay, document.getElementById('sapling-overlay-header'));
    }

    function playMedia(url, label) {
        const type = mediaTypeFromUrl(url);
        const mediaWrap = document.getElementById('sapling-overlay-media');
        const titleEl   = document.getElementById('sapling-overlay-title');

        titleEl.textContent = label;

        const el = document.createElement(type);
        el.controls = true;
        el.autoplay = true;
        el.preload  = 'metadata';
        el.src = url;

        el.addEventListener('ended', () => {
            currentIndex++;
            executeNext();
        });

        mediaWrap.innerHTML = '';
        mediaWrap.appendChild(el);

        overlay.hidden = false;
        overlayMinimized = false;
        overlay.classList.remove('is-minimized');
        document.getElementById('sapling-overlay-minimize').textContent = '—';
    }

    function closeOverlay() {
        const mediaWrap = document.getElementById('sapling-overlay-media');
        if (mediaWrap) {
            const el = mediaWrap.querySelector('video, audio');
            if (el) { el.pause(); el.src = ''; }
            mediaWrap.innerHTML = '';
        }
        overlay.hidden = true;
    }

    // ── Draggable overlay ────────────────────────────────────────
    function makeDraggable(el, handle) {
        let startX, startY, startLeft, startTop;

        handle.addEventListener('mousedown', e => {
            if (e.target.tagName === 'BUTTON') return;
            e.preventDefault();
            const rect = el.getBoundingClientRect();
            startX = e.clientX;
            startY = e.clientY;
            startLeft = rect.left;
            startTop  = rect.top;

            el.style.right = 'auto';

            const onMove = e => {
                el.style.left = Math.max(0, startLeft + e.clientX - startX) + 'px';
                el.style.top  = Math.max(0, startTop  + e.clientY - startY) + 'px';
            };
            const onUp = () => {
                document.removeEventListener('mousemove', onMove);
                document.removeEventListener('mouseup', onUp);
            };
            document.addEventListener('mousemove', onMove);
            document.addEventListener('mouseup', onUp);
        });
    }

    // ── Queue panel rendering ────────────────────────────────────
    function renderPanel() {
        document.body.classList.add('has-queue');

        const nowPlaying = currentIndex >= 0 && currentIndex < queue.length
            ? queue[currentIndex].label
            : '';

        const isOpen = panel.classList.contains('is-open');

        let itemsHtml = '';
        if (queue.length === 0) {
            itemsHtml = '<div class="queue-empty">Queue is empty. Right-click media to add items.</div>';
        } else {
            queue.forEach((item, idx) => {
                const active  = idx === currentIndex ? ' is-active' : '';
                const typeTag = typeLabel(item.type);
                const upDis   = idx === 0 ? ' disabled' : '';
                const downDis = idx === queue.length - 1 ? ' disabled' : '';
                itemsHtml += `
                <div class="queue-item${active}" data-idx="${idx}">
                    <span class="queue-item-index">${idx + 1}</span>
                    <span class="queue-item-label" title="${escHtml(item.label)}">${escHtml(item.label)}</span>
                    <span class="queue-item-type">${typeTag}</span>
                    <button class="queue-item-btn q-up"${upDis} data-idx="${idx}" title="Move up">↑</button>
                    <button class="queue-item-btn q-down"${downDis} data-idx="${idx}" title="Move down">↓</button>
                    <button class="queue-item-btn q-remove" data-idx="${idx}" title="Remove">✕</button>
                </div>`;
            });
        }

        panel.innerHTML = `
            <div class="queue-header">
                <span class="queue-header-title">Queue (${queue.length})</span>
                ${nowPlaying ? `<span class="queue-now-playing">▶ ${escHtml(nowPlaying)}</span>` : ''}
                <button class="queue-btn" id="sapling-q-play" title="Play queue from start">▶ Play</button>
                <button class="queue-btn" id="sapling-q-clear" title="Clear queue">Clear</button>
                <button class="queue-btn" id="sapling-q-toggle">${isOpen ? '▼' : '▲'}</button>
            </div>
            <div class="queue-list" id="sapling-queue-list">${itemsHtml}</div>
        `;

        document.getElementById('sapling-q-toggle').addEventListener('click', () => {
            panel.classList.toggle('is-open');
            renderPanel();
        });

        document.getElementById('sapling-q-play').addEventListener('click', () => {
            if (queue.length > 0) startQueue(0);
        });

        document.getElementById('sapling-q-clear').addEventListener('click', () => {
            queue = [];
            currentIndex = -1;
            saveQueue();
            closeOverlay();
            renderPanel();
        });

        document.getElementById('sapling-queue-list').addEventListener('click', e => {
            const btn = e.target.closest('button');
            if (!btn) return;
            const idx = parseInt(btn.dataset.idx, 10);

            if (btn.classList.contains('q-remove')) {
                if (idx < currentIndex) currentIndex--;
                else if (idx === currentIndex) { closeOverlay(); currentIndex = -1; }
                queue.splice(idx, 1);
                saveQueue();
                renderPanel();
            } else if (btn.classList.contains('q-up') && idx > 0) {
                [queue[idx - 1], queue[idx]] = [queue[idx], queue[idx - 1]];
                if (currentIndex === idx) currentIndex--;
                else if (currentIndex === idx - 1) currentIndex++;
                saveQueue();
                renderPanel();
            } else if (btn.classList.contains('q-down') && idx < queue.length - 1) {
                [queue[idx + 1], queue[idx]] = [queue[idx], queue[idx + 1]];
                if (currentIndex === idx) currentIndex++;
                else if (currentIndex === idx + 1) currentIndex--;
                saveQueue();
                renderPanel();
            }
        });
    }

    function typeLabel(type) {
        switch (type) {
            case 'video':              return 'VIDEO';
            case 'audio':              return 'AUDIO';
            case 'random_from_folder': return '🎲 RANDOM';
            case 'loop':               return '↺ LOOP';
            case 'clear':              return '⊘ CLEAR';
            default:                   return type.toUpperCase();
        }
    }

    function escHtml(str) {
        return String(str)
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;');
    }

    // ── Context menu ─────────────────────────────────────────────
    function buildCtxMenu() {
        ctxMenu.innerHTML = '';
    }

    function showCtxMenu(x, y, items) {
        ctxMenu.innerHTML = items.map(item => {
            if (item === 'sep') return '<div class="ctx-separator"></div>';
            return `<button class="ctx-item" data-action="${escHtml(item.action)}">${escHtml(item.label)}</button>`;
        }).join('');

        // Position — flip if off-screen
        ctxMenu.hidden = false;
        const vw = window.innerWidth, vh = window.innerHeight;
        const w = ctxMenu.offsetWidth, h = ctxMenu.offsetHeight;
        ctxMenu.style.left = (x + w > vw ? vw - w - 4 : x) + 'px';
        ctxMenu.style.top  = (y + h > vh ? vh - h - 4 : y) + 'px';
    }

    function hideCtxMenu() {
        ctxMenu.hidden = true;
    }

    document.addEventListener('click', hideCtxMenu);
    document.addEventListener('scroll', hideCtxMenu, true);
    document.addEventListener('keydown', e => { if (e.key === 'Escape') hideCtxMenu(); });

    document.addEventListener('contextmenu', e => {
        const card = e.target.closest('[data-media-url],[data-folder-path]');
        if (!card) return;
        e.preventDefault();

        if (card.dataset.folderPath !== undefined) {
            // Folder card
            const path  = card.dataset.folderPath;
            const label = card.dataset.folderLabel || path;

            showCtxMenu(e.clientX, e.clientY, [
                { label: '🎲 Add one random from here', action: 'random:' + path + '|' + label },
                { label: '📂 Add all media from here',  action: 'addall:' + path + '|' + label },
                'sep',
                { label: '↺ Insert: loop to start',     action: 'loop' },
                { label: '⊘ Insert: clear queue',        action: 'insertclear' },
            ]);
        } else if (card.dataset.mediaUrl !== undefined) {
            // Media card
            const url   = card.dataset.mediaUrl;
            const lbl   = card.dataset.mediaLabel || url;
            const type  = card.dataset.mediaType  || 'video';

            showCtxMenu(e.clientX, e.clientY, [
                { label: '+ Add to queue',  action: 'add:' + url + '|' + lbl + '|' + type },
                { label: '⏭ Play next',     action: 'next:' + url + '|' + lbl + '|' + type },
                { label: '▶ Play now',      action: 'now:' + url + '|' + lbl + '|' + type },
            ]);
        }
    });

    ctxMenu.addEventListener('click', e => {
        const btn = e.target.closest('.ctx-item');
        if (!btn) return;
        const action = btn.dataset.action;
        hideCtxMenu();
        handleCtxAction(action);
    });

    function handleCtxAction(action) {
        if (action === 'loop') {
            queue.push({ type: 'loop', label: '↺ Loop to start' });
        } else if (action === 'insertclear') {
            queue.push({ type: 'clear', label: '⊘ Clear queue' });
        } else if (action.startsWith('random:')) {
            const [path, label] = action.slice(7).split('|');
            queue.push({ type: 'random_from_folder', path, label: '🎲 ' + label });
        } else if (action.startsWith('addall:')) {
            const [path, label] = action.slice(7).split('|');
            // Fetch and append all playable items from folder
            fetch('/api/list?path=' + encodeURIComponent(path)).then(r => r.json()).then(items => {
                items.filter(i => i.item_type === 'video' || i.item_type === 'audio').forEach(i => {
                    queue.push({ type: i.item_type, url: i.url, label: i.label });
                });
                saveQueue();
                renderPanel();
            });
            return;
        } else if (action.startsWith('add:')) {
            const [url, label, type] = action.slice(4).split('|');
            queue.push({ type, url, label });
        } else if (action.startsWith('next:')) {
            const [url, label, type] = action.slice(5).split('|');
            const insertAt = currentIndex >= 0 ? currentIndex + 1 : queue.length;
            queue.splice(insertAt, 0, { type, url, label });
        } else if (action.startsWith('now:')) {
            const [url, label, type] = action.slice(4).split('|');
            const insertAt = currentIndex >= 0 ? currentIndex : 0;
            queue.splice(insertAt, 0, { type, url, label });
            startQueue(insertAt);
            saveQueue();
            renderPanel();
            return;
        }

        saveQueue();
        renderPanel();
    }

    // ── Init ─────────────────────────────────────────────────────
    buildOverlay();
    buildCtxMenu();
    renderPanel();
})();
"#;

fn page_shell(
    title: &'static str,
    path: String,
    search_query: String,
        parent_href: Option<String>,
    content: impl IntoView + 'static,
) -> String {
        let parent_attr = parent_href.unwrap_or_default();
    let html = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>{title}</title>
                <style>{STYLE}</style>
            </head>
                        <body data-parent-href=parent_attr>
                <main class="app">
                    <header class="header">
                        <div class="header-right">
                            <div class="path">{path}</div>
                            <div class="search-tools">
                                <a class="home-button" href="/browse/" aria-label="Home">"⌂"</a>
                                <form class="search-form" action="/browse/" method="get">
                                    <input
                                        class="search-input"
                                        type="search"
                                        name="q"
                                        placeholder="Search all paths"
                                        value=search_query
                                    />
                                </form>
                            </div>
                        </div>
                    </header>
                    {content}
                </main>
                <div id="sapling-queue-panel" class="queue-panel"></div>
                <div id="sapling-player-overlay" class="player-overlay" hidden></div>
                <div id="sapling-ctx-menu" class="ctx-menu" hidden></div>
                <script>{KEYBOARD_NAV_SCRIPT}</script>
                <script>{QUEUE_SCRIPT}</script>
            </body>
        </html>
    };

    format!("<!DOCTYPE html>{}", html.to_html())
}

pub fn render_browse_page(
    breadcrumbs: &[(String, String)],
    folders: &[FolderEntry],
    videos: &[VideoEntry],
    audio_files: &[AudioEntry],
) -> String {
    let parent_href = if breadcrumbs.len() > 1 {
        Some(breadcrumbs[breadcrumbs.len() - 2].1.clone())
    } else {
        Some("/browse/".to_string())
    };

    let breadcrumb_view = breadcrumbs
        .iter()
        .enumerate()
        .flat_map(|(idx, (label, href))| {
            let is_last = idx + 1 == breadcrumbs.len();
            if is_last {
                vec![view! { <span>{label.clone()}</span> }.into_any()]
            } else {
                vec![
                    view! { <a href=href.clone()>{label.clone()}</a> }.into_any(),
                    view! { <span>"/"</span> }.into_any(),
                ]
            }
        })
        .collect_view();

    let folder_cards = folders
        .iter()
        .map(|folder| {
            let browse_href = format!("/browse/{}", encode_url_path(&folder.relative_path));
            let folder_path = folder.relative_path.to_string_lossy().to_string();
            let thumb_view = match &folder.thumbnail_relative_path {
                Some(thumbnail) => {
                    let src = format!("/media/{}", encode_url_path(thumbnail));
                    view! { <img class="thumb" src=src alt=folder.name.clone()/> }.into_any()
                }
                None => view! { <div class="thumb placeholder">"Folder"</div> }.into_any(),
            };

            view! {
                <a class="card" href=browse_href data-nav-item
                   data-folder-path=folder_path
                   data-folder-label=folder.name.clone()>
                    {thumb_view}
                    <div class="meta">
                        <h3>{folder.name.clone()}</h3>
                        <p>"Folder"</p>
                    </div>
                </a>
            }
        })
        .collect_view();

    let video_cards = videos
        .iter()
        .map(|video| {
            let play_href = format!("/play/{}", encode_url_path(&video.relative_path));
            let media_url = format!("/media/{}", encode_url_path(&video.relative_path));
            let thumb_view = match &video.thumbnail_url {
                Some(thumbnail_url) => {
                    view! { <img class="thumb" src=thumbnail_url.clone() alt=video.name.clone()/> }.into_any()
                }
                None => view! { <div class="thumb placeholder">"Video"</div> }.into_any(),
            };

            view! {
                <a class="card" href=play_href data-nav-item
                   data-media-url=media_url
                   data-media-label=video.name.clone()
                   data-media-type="video">
                    {thumb_view}
                    <div class="meta">
                        <h3>{video.name.clone()}</h3>
                        <p>"Play in browser"</p>
                    </div>
                </a>
            }
        })
        .collect_view();

    let audio_cards = audio_files
        .iter()
        .map(|audio| {
            let play_href = format!("/play/{}", encode_url_path(&audio.relative_path));
            let media_url = format!("/media/{}", encode_url_path(&audio.relative_path));

            view! {
                <a class="card" href=play_href data-nav-item
                   data-media-url=media_url
                   data-media-label=audio.name.clone()
                   data-media-type="audio">
                    <div class="thumb placeholder">"Audio"</div>
                    <div class="meta">
                        <h3>{audio.name.clone()}</h3>
                        <p>"Play in browser"</p>
                    </div>
                </a>
            }
        })
        .collect_view();

    page_shell(
        "Sapling Media",
        String::new(),
        String::new(),
        parent_href,
        view! {
            <section class="helper">{breadcrumb_view}</section>
            <section class="grid">{folder_cards}{video_cards}{audio_cards}</section>
        },
    )
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
                    SearchEntryKind::Video => format!("/play/{}", encode_url_path(&entry.relative_path)),
                    SearchEntryKind::Audio => format!("/play/{}", encode_url_path(&entry.relative_path)),
                };
                let kind = match entry.kind {
                    SearchEntryKind::Folder => "Folder",
                    SearchEntryKind::Video => "Video",
                    SearchEntryKind::Audio => "Audio",
                };

                view! {
                    <li class="results-row">
                        <a class="results-path" href=href data-nav-item>{entry.relative_path.to_string_lossy().to_string()}</a>
                        <span class="results-kind">{kind}</span>
                    </li>
                }
            })
            .collect_view()
            .into_any()
    };

    page_shell(
        "Search Results",
        format!("Search: {}", query),
        query.to_string(),
        Some("/browse/".to_string()),
        view! {
            <section class="results">
                <div class="results-header">{format!("{} match(es)", entries.len())}</div>
                {if entries.is_empty() {
                    listing
                } else {
                    view! { <ol class="results-list">{listing}</ol> }.into_any()
                }}
            </section>
        },
    )
}

pub fn render_video_page(display_name: String, media_src: String, parent_href: String) -> String {
    render_player_page(
        display_name,
        parent_href,
        view! {
            <video controls=true autoplay=true preload="metadata">
                <source src=media_src/>
                "Your browser cannot play this video format natively."
            </video>
        },
    )
}

pub fn render_audio_page(display_name: String, media_src: String, parent_href: String) -> String {
    render_player_page(
        display_name,
        parent_href,
        view! {
            <audio controls=true autoplay=true preload="metadata">
                <source src=media_src/>
                "Your browser cannot play this audio format natively."
            </audio>
        },
    )
}

fn render_player_page(
    display_name: String,
    parent_href: String,
    player: impl IntoView + 'static,
) -> String {
    let parent_href_clone = parent_href.clone();
    page_shell(
        "Now Playing",
        display_name.clone(),
        String::new(),
        Some(parent_href_clone),
        view! {
            <section class="player-wrap">
                <a class="cta" href=parent_href>
                    "Back to folder"
                </a>
            </section>
            <section class="player-wrap">
                {player}
            </section>
        },
    )
}

pub fn render_not_found(message: String) -> String {
    page_shell(
        "Not Found",
        "Error".to_string(),
        String::new(),
        Some("/browse/".to_string()),
        view! {
            <section class="helper">
                <h2>"Not found"</h2>
                <p>{message}</p>
            </section>
        },
    )
}
