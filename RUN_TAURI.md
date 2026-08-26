# Quick Start - Run Saturday at Three as Desktop App

## One Command to Start

```bash
cd "D:\projects\Saturday at Three\frontend"
npm install
npm run tauri-dev
```

That's it! The game will:
1. ✅ Compile Rust backend
2. ✅ Start React frontend
3. ✅ Open desktop window
4. ✅ Show the game

---

## What You'll See

- **Sidebar** with navigation (Dashboard, Matchday, Squad, Standings)
- **Main content area** showing current screen
- **Save status** indicator (✓ Saved / • Unsaved)
- **Toolbar** with New Game, Save, Undo, Redo buttons

---

## Try These Shortcuts

While the app is open:

| Shortcut | What Happens |
|----------|--------------|
| Ctrl+N | New Game |
| Ctrl+S | Save |
| Ctrl+Z | Undo last change |
| Ctrl+Y | Redo |

---

## Click Around

1. **Dashboard** - Shows club status and top players
2. **Matchday** - Click "Play" to simulate a match
3. **Squad** - Lists your team's players
4. **Standings** - Shows league table

---

## Build Single Executable

When ready to share:

```bash
npm run tauri-build
```

Creates: `src-tauri/target/release/saturday-at-three.exe`

No npm, no servers needed - just the executable!

---

## Architecture

```
One App
├─ React Frontend (React + TypeScript)
└─ Rust Backend (via Tauri IPC)

Single GameState object
├─ Auto-saves every 1 second
├─ Full undo/redo support
└─ Lazy-loaded screens
```

---

## Next Development Tasks

After running `npm run tauri-dev`, the app is ready for:

1. **Add game data:**
   - 12 clubs (Preston, Bolton, Wolves, etc.)
   - 38-game fixtures
   - Players for each club

2. **Complete screens:**
   - Squad management (formations, tactics)
   - Pre-match setup
   - Financial management
   - Transfer market

3. **Integrate simulator:**
   - Copy from Football Man project
   - Hook into match simulation command
   - Update standings

---

## Stop the App

Press **Ctrl+C** in the terminal, or close the window.

---

## Troubleshooting

**Port 5173 in use:**
```bash
netstat -ano | findstr :5173
taskkill /PID {number} /F
```

**Rust won't compile:**
```bash
cargo clean
npm run tauri-dev
```

**Dependencies missing:**
```bash
npm install
```

---

## That's It!

You now have a professional desktop app with:
- ✅ Single executable
- ✅ Full undo/redo
- ✅ Auto-save
- ✅ Keyboard shortcuts
- ✅ Type-safe code

Run it and start building! 🚀⚽
