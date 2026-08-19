// ================================================
// THE RUST LABORATORY - Frontend Application Logic
// ================================================

let gameState = null;
let inputHistory = [];
let historyIndex = -1;
let acIndex = -1;
let acItems = [];

// ============ Tauri API Bridge ============
const tauri = window.__TAURI__;

async function invoke(cmd, args) {
    if (tauri) {
        return await tauri.core.invoke(cmd, args);
    }
    return null;
}

// ============ INITIALIZATION ============
async function startNewGame() {
    switchScreen('game-screen');
    gameState = await invoke('initialize_game', {});
    if (gameState) renderGameState(gameState);
    document.getElementById('command-input').focus();
}

async function showLoadScreen() {
    const slots = await invoke('get_save_slots', {});
    let html = '';
    for (const slot of (slots || [])) {
        if (slot.exists) {
            html += `<div class="save-slot" onclick="loadGame(${slot.slot})">
                <div class="save-slot-info">
                    <h4>Slot ${slot.slot + 1} - ${slot.room_name}</h4>
                    <p>${slot.play_time} | ${slot.timestamp}</p>
                </div>
                <div style="display:flex;gap:6px">
                    <button class="save-slot-action" onclick="event.stopPropagation();loadGame(${slot.slot})">Load</button>
                    <button class="save-slot-action danger" onclick="event.stopPropagation();deleteSave(${slot.slot})">Delete</button>
                </div>
            </div>`;
        } else {
            html += `<div class="save-slot" style="opacity:0.5">
                <div class="save-slot-info"><h4>Slot ${slot.slot + 1} - Empty</h4><p>No save data</p></div>
                <span style="color:var(--text-muted)">---</span>
            </div>`;
        }
    }
    showModal('Load Game', html);
}

async function loadGame(slot) {
    const result = await invoke('load_game', { slot });
    if (result && result.success) {
        closeModal();
        switchScreen('game-screen');
        gameState = await invoke('initialize_game', {});
        if (gameState) renderGameState(gameState);
        showNotification(result.message);
    } else {
        showNotification(result?.message || 'Failed to load', true);
    }
}

async function deleteSave(slot) {
    const result = await invoke('delete_save', { slot });
    showNotification(result?.message || 'Deleted');
    showLoadScreen();
}

async function saveGame(slot) {
    const result = await invoke('save_game', { slot });
    if (result) showNotification(result.message, !result.success);
}

function showSaveScreen() {
    if (!gameState) return;
    let html = '';
    for (let i = 0; i < 3; i++) {
        const existing = gameState.saveSlots?.[i];
        const label = existing?.exists
            ? `Slot ${i + 1} - ${existing.room_name} (${existing.play_time})`
            : `Slot ${i + 1} - Empty`;
        html += `<div class="save-slot" onclick="saveGame(${i})">
            <div class="save-slot-info"><h4>${label}</h4></div>
            <button class="save-slot-action">Save</button>
        </div>`;
    }
    showModal('Save Game', html);
}

// ============ COMMAND PROCESSING ============
async function sendCommand(cmd) {
    if (!cmd.trim()) return;
    inputHistory.unshift(cmd);
    historyIndex = -1;
    document.getElementById('command-input').value = '';
    gameState = await invoke('send_command', { command: cmd });
    if (gameState) renderGameState(gameState);
}

// ============ RENDERING ============
function renderGameState(state) {
    // Messages
    const log = document.getElementById('message-log');
    for (const msg of state.messages) {
        const div = document.createElement('div');
        div.className = `game-message ${msg.msg_type}`;
        div.innerHTML = `<span class="msg-time">${msg.timestamp || ''}</span>${escapeHtml(msg.text)}`;
        log.appendChild(div);
    }
    log.scrollTop = log.scrollHeight;

    // Room
    document.getElementById('room-name').textContent = state.current_room.name;

    // Stats
    document.getElementById('stat-score').textContent = state.score;
    document.getElementById('stat-moves').textContent = state.player.moves;
    document.getElementById('stat-rooms').textContent = `${state.player.rooms_explored}`;
    document.getElementById('stat-items').textContent = state.player.items_collected;
    document.getElementById('stat-puzzles').textContent = state.player.puzzles_solved;

    // Exits
    const exitsBar = document.getElementById('exits-bar');
    exitsBar.innerHTML = '';
    for (const exit of state.current_room.exits) {
        const btn = document.createElement('button');
        btn.className = `exit-btn ${exit.locked ? 'locked' : ''}`;
        btn.innerHTML = `<span class="exit-arrow">${exit.arrow}</span> ${exit.direction}`;
        if (!exit.locked) {
            btn.onclick = () => sendCommand('go ' + exit.direction.toLowerCase());
        }
        exitsBar.appendChild(btn);
    }

    // Ground items
    const groundEl = document.getElementById('ground-items');
    groundEl.innerHTML = '';
    for (const item of state.current_room.items_on_ground) {
        const span = document.createElement('span');
        span.className = 'ground-item';
        span.innerHTML = `<span class="item-icon">${item.icon}</span> ${escapeHtml(item.name)}`;
        span.onclick = () => sendCommand('take ' + item.name.toLowerCase());
        groundEl.appendChild(span);
    }

    // Inventory
    const invEl = document.getElementById('inventory-list');
    if (state.inventory.length === 0) {
        invEl.innerHTML = '<p class="empty-msg">No items yet</p>';
    } else {
        invEl.innerHTML = '';
        for (const item of state.inventory) {
            const div = document.createElement('div');
            div.className = 'inv-item';
            div.innerHTML = `<span class="item-icon">${item.icon}</span>
                <span class="item-name">${escapeHtml(item.name)}</span>
                <span class="item-cat">${item.category}</span>`;
            div.onclick = () => sendCommand('examine ' + item.name.toLowerCase());
            invEl.appendChild(div);
        }
    }

    // Achievements
    const achEl = document.getElementById('achievements-list');
    const unlocked = state.achievements.filter(a => a.unlocked);
    if (unlocked.length === 0) {
        achEl.innerHTML = '<p class="empty-msg">None unlocked</p>';
    } else {
        achEl.innerHTML = '';
        for (const ach of unlocked) {
            const div = document.createElement('div');
            div.className = 'ach-item unlocked';
            div.innerHTML = `<span class="ach-icon">${ach.icon}</span><span class="ach-name">${ach.name}</span>`;
            div.title = ach.description;
            achEl.appendChild(div);
        }
    }

    // Minimap
    renderMinimap(state.map_data);
}

// ============ MINIMAP ============
function renderMinimap(mapData) {
    const canvas = document.getElementById('minimap');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    const w = canvas.width;
    const h = canvas.height;
    ctx.clearRect(0, 0, w, h);

    const rooms = mapData.rooms;
    const conns = mapData.connections;
    if (!rooms || rooms.length === 0) return;

    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    for (const r of rooms) {
        minX = Math.min(minX, r.x); maxX = Math.max(maxX, r.x);
        minY = Math.min(minY, r.y); maxY = Math.max(maxY, r.y);
    }
    const pad = 20;
    const scaleX = (w - pad * 2) / Math.max(maxX - minX, 1);
    const scaleY = (h - pad * 2) / Math.max(maxY - minY, 1);
    const scale = Math.min(scaleX, scaleY);
    const offX = (w - (maxX - minX) * scale) / 2;
    const offY = (h - (maxY - minY) * scale) / 2;

    const tx = (x) => (x - minX) * scale + offX;
    const ty = (y) => (y - minY) * scale + offY;

    const roomMap = {};
    for (const r of rooms) { roomMap[r.id] = { ...r, sx: tx(r.x), sy: ty(r.y) }; }

    // Connections
    for (const c of conns) {
        const from = roomMap[c.from];
        const to = roomMap[c.to];
        if (from && to) {
            ctx.beginPath();
            ctx.moveTo(from.sx, from.sy);
            ctx.lineTo(to.sx, to.sy);
            ctx.strokeStyle = 'rgba(30, 58, 95, 0.6)';
            ctx.lineWidth = 1;
            ctx.stroke();
        }
    }

    // Rooms
    for (const r of Object.values(roomMap)) {
        ctx.beginPath();
        ctx.arc(r.sx, r.sy, r.is_current ? 6 : 4, 0, Math.PI * 2);
        if (r.is_current) {
            ctx.fillStyle = '#06b6d4';
            ctx.shadowColor = '#06b6d4';
            ctx.shadowBlur = 10;
        } else if (r.visited) {
            ctx.fillStyle = '#1e3a5f';
            ctx.shadowBlur = 0;
        } else {
            ctx.fillStyle = '#0a0e17';
            ctx.shadowBlur = 0;
        }
        ctx.fill();
        ctx.shadowBlur = 0;

        if (r.visited && !r.is_current) {
            ctx.strokeStyle = '#1e3a5f';
            ctx.lineWidth = 1;
            ctx.stroke();
        }
    }
}

// ============ AUTOCOMPLETE ============
async function handleAutocomplete(value) {
    const dropdown = document.getElementById('autocomplete-dropdown');
    if (!value || value.length < 1) {
        dropdown.classList.remove('visible');
        return;
    }
    if (tauri) {
        const suggestions = await invoke('get_autocomplete', { partial: value });
        acItems = suggestions || [];
    } else {
        acItems = [];
    }
    acIndex = -1;
    if (acItems.length === 0) {
        dropdown.classList.remove('visible');
        return;
    }
    dropdown.innerHTML = acItems.map((s, i) =>
        `<div class="ac-item${i === 0 ? ' active' : ''}" onclick="selectAc(${i})" data-idx="${i}">${escapeHtml(s)}</div>`
    ).join('');
    dropdown.classList.add('visible');
}

function selectAc(idx) {
    const input = document.getElementById('command-input');
    input.value = acItems[idx];
    document.getElementById('autocomplete-dropdown').classList.remove('visible');
    input.focus();
}

// ============ MODAL ============
function showModal(title, bodyHtml) {
    document.getElementById('modal-title').textContent = title;
    document.getElementById('modal-body').innerHTML = bodyHtml;
    document.getElementById('modal-overlay').classList.add('active');
}

function closeModal() {
    document.getElementById('modal-overlay').classList.remove('active');
}

function showHelp() {
    showModal('Help - Commands', `
        <div style="font-family:var(--font-mono);font-size:0.85rem;line-height:2">
        <p><b style="color:var(--accent-cyan)">Navigation</b></p>
        <p>go [direction] / north, south, east, west, up, down</p>
        <p>look / look [target]</p>
        <p><b style="color:var(--accent-cyan)">Interaction</b></p>
        <p>take [item] / drop [item] / use [item]</p>
        <p>combine [item1] [item2] / read [document]</p>
        <p>examine [item]</p>
        <p><b style="color:var(--accent-cyan)">Puzzles</b></p>
        <p>solve [puzzle] / solve 7319 (for codes)</p>
        <p>solve stabilize / solve repair / solve decrypt</p>
        <p><b style="color:var(--accent-cyan)">Info</b></p>
        <p>inventory / map / status / hint</p>
        </div>
    `);
}

function showSettings() {
    showModal('Settings', `
        <div class="setting-row">
            <div><div class="setting-label">Text Speed</div><div class="setting-sublabel">Typing animation delay</div></div>
            <div class="setting-control"><input type="range" min="0" max="80" value="30"><span id="speed-val" style="font-family:var(--font-mono);font-size:0.8rem;min-width:35px">30ms</span></div>
        </div>
        <div class="setting-row">
            <div><div class="setting-label">Theme</div></div>
            <select><option>Cyberpunk</option><option>Midnight</option><option>Terminal</option></select>
        </div>
        <div class="setting-row">
            <div><div class="setting-label">Show Minimap</div></div>
            <div class="toggle on" onclick="this.classList.toggle('on')"></div>
        </div>
        <div class="setting-row">
            <div><div class="setting-label">Auto-Save</div></div>
            <div class="toggle on" onclick="this.classList.toggle('on')"></div>
        </div>
    `);
}

function showNotification(msg, isError = false) {
    const el = document.getElementById('notification');
    el.textContent = msg;
    el.className = `notification ${isError ? 'error' : ''} visible`;
    setTimeout(() => el.classList.remove('visible'), 3000);
}

function switchScreen(id) {
    document.querySelectorAll('.screen').forEach(s => s.classList.remove('active'));
    document.getElementById(id).classList.add('active');
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// ============ INPUT HANDLING ============
document.addEventListener('DOMContentLoaded', () => {
    const input = document.getElementById('command-input');
    const dropdown = document.getElementById('autocomplete-dropdown');

    input.addEventListener('keydown', async (e) => {
        if (e.key === 'Enter') {
            dropdown.classList.remove('visible');
            sendCommand(input.value);
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            if (acItems.length > 0) {
                acIndex = Math.max(0, acIndex - 1);
                updateAcHighlight();
            } else {
                if (historyIndex < inputHistory.length - 1) {
                    historyIndex++;
                    input.value = inputHistory[historyIndex];
                }
            }
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            if (acItems.length > 0) {
                acIndex = Math.min(acItems.length - 1, acIndex + 1);
                updateAcHighlight();
            } else {
                if (historyIndex > 0) { historyIndex--; input.value = inputHistory[historyIndex]; }
                else { historyIndex = -1; input.value = ''; }
            }
        } else if (e.key === 'Tab') {
            e.preventDefault();
            if (acItems.length > 0) {
                selectAc(acIndex >= 0 ? acIndex : 0);
            }
        } else if (e.key === 'Escape') {
            dropdown.classList.remove('visible');
            closeModal();
        }
    });

    input.addEventListener('input', () => handleAutocomplete(input.value));

    // Close modal on overlay click
    document.getElementById('modal-overlay').addEventListener('click', (e) => {
        if (e.target.id === 'modal-overlay') closeModal();
    });

    // Keyboard shortcut: Escape closes modal
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') closeModal();
    });

    // Splash particles
    initSplashParticles();
});

function updateAcHighlight() {
    const items = document.querySelectorAll('.ac-item');
    items.forEach((el, i) => el.classList.toggle('active', i === acIndex));
    if (acIndex >= 0 && items[acIndex]) {
        items[acIndex].scrollIntoView({ block: 'nearest' });
    }
}

// ============ SPLASH PARTICLES ============
function initSplashParticles() {
    const canvas = document.getElementById('splash-particles');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    let w, h;
    const particles = [];

    function resize() {
        w = canvas.width = window.innerWidth;
        h = canvas.height = window.innerHeight;
    }
    resize();
    window.addEventListener('resize', resize);

    for (let i = 0; i < 60; i++) {
        particles.push({
            x: Math.random() * w, y: Math.random() * h,
            vx: (Math.random() - 0.5) * 0.5, vy: (Math.random() - 0.5) * 0.5,
            r: Math.random() * 2 + 0.5,
            a: Math.random() * 0.5 + 0.1,
        });
    }

    function draw() {
        ctx.clearRect(0, 0, w, h);
        for (const p of particles) {
            p.x += p.vx; p.y += p.vy;
            if (p.x < 0) p.x = w; if (p.x > w) p.x = 0;
            if (p.y < 0) p.y = h; if (p.y > h) p.y = 0;
            ctx.beginPath();
            ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
            ctx.fillStyle = `rgba(6, 182, 212, ${p.a})`;
            ctx.fill();
        }
        // Draw connections
        for (let i = 0; i < particles.length; i++) {
            for (let j = i + 1; j < particles.length; j++) {
                const dx = particles[i].x - particles[j].x;
                const dy = particles[i].y - particles[j].y;
                const dist = Math.sqrt(dx * dx + dy * dy);
                if (dist < 150) {
                    ctx.beginPath();
                    ctx.moveTo(particles[i].x, particles[i].y);
                    ctx.lineTo(particles[j].x, particles[j].y);
                    ctx.strokeStyle = `rgba(6, 182, 212, ${0.1 * (1 - dist / 150)})`;
                    ctx.lineWidth = 0.5;
                    ctx.stroke();
                }
            }
        }
        requestAnimationFrame(draw);
    }
    draw();
}
