# Saturday at Three - Tauri Desktop App Setup

## What Changed

The project is now a **single Tauri desktop application** instead of separate frontend/backend servers.

### Key Benefits
✅ Single executable (no need to run two servers)
✅ Auto-save built in (debounced)
✅ Undo/redo system
✅ Lazy-loaded screens (fast startup)
✅ Single root GameState (DAAD Builder pattern)
✅ Keyboard shortcuts (Ctrl+Z, Ctrl+S, etc.)

---

## Architecture

```
Desktop App (Single Executable)
│
├─ Frontend (React + TypeScript)
│  ├─ Single root state via useHistory
│  ├─ Lazy-loaded screens
│  ├─ Keyboard shortcuts
│  └─ Auto-save on changes
│
└─ Backend (Rust)
   ├─ Game commands (new, load, save, simulate)
   ├─ Match simulation
   ├─ Data persistence (JSON files)
   └─ Tauri IPC bridge
```

---

## Prerequisites

You need to install Tauri:

```bash
npm install -g @tauri-apps/cli
```

Or with cargo:

```bash
cargo install tauri-cli
```

Also ensure you have:
- Rust 1.70+ (already have it)
- Node.js 18+ (already have it)
- npm 10+ (already have it)

---

## Development Setup

### 1. Install Dependencies

```bash
cd "D:\projects\Saturday at Three\frontend"
npm install
```

This installs React, TypeScript, Vite, and Tauri client libraries.

### 2. Run Development Build

From the root `Saturday at Three` directory:

```bash
npm run tauri-dev
```

Or manually:

```bash
# Frontend dev server
cd frontend
npm run dev

# In another terminal, run Tauri app
npm run tauri tauri dev
```

This:
1. Starts Vite dev server on http://127.0.0.1:5173
2. Compiles Rust backend
3. Opens desktop window
4. HMR (hot reload) works for React changes
5. Rust changes require restart

### 3. Test the App

When it opens, you should see:
- Sidebar with navigation buttons
- Dashboard screen
- Keyboard shortcuts work: Ctrl+Z (undo), Ctrl+S (save), Ctrl+N (new)

---

## Production Build

Create a single executable:

```bash
npm run tauri-build
```

Or:

```bash
cd frontend
npm run build

cd ../src-tauri
cargo build --release
```

Output executable locations:
- **Windows:** `src-tauri/target/release/saturday-at-three.exe`
- **macOS:** `src-tauri/target/release/Saturday at Three.app`
- **Linux:** `src-tauri/target/release/saturday-at-three`

---

## Project Structure

```
Saturday at Three/
│
├─ frontend/
│  ├─ src/
│  │  ├─ App.tsx             ← Main component with root state
│  │  ├─ hooks/
│  │  │  └─ useHistory.ts    ← Undo/redo system
│  │  ├─ screens/            ← Dashboard, Matchday, Squad, Standings
│  │  ├─ types/
│  │  │  └─ GameState.ts     ← Single root state type
│  │  └─ index.css
│  ├─ vite.config.ts
│  ├─ tsconfig.json
│  ├─ package.json
│  └─ index.html
│
├─ src-tauri/
│  ├─ src/
│  │  ├─ main.rs            ← Tauri entry point
│  │  ├─ commands.rs        ← Tauri commands (IPC)
│  │  ├─ game.rs            ← Game state structs
│  │  ├─ database.rs        ← Persistence layer
│  │  └─ simulation.rs      ← Match simulation
│  ├─ Cargo.toml
│  └─ tauri.conf.json       ← Tauri config
│
└─ assets/packs/            ← Game graphics
```

---

## Key Concepts

### Root State Pattern (from DAAD Builder)

All game state lives in one object:

```typescript
interface GameState {
  id: string
  season: number
  currentGameweek: number
  userClubId: string
  clubs: Club[]
  matches: Match[]
  players: Player[]
  standings: Standing[]
  createdAt: string
  updatedAt: string
}
```

### Undo/Redo via useHistory Hook

```typescript
const { state, setState, undo, redo, canUndo, canRedo } = useHistory<GameState>(initialState)

// Make changes
setState(newGameState)

// Undo/redo work automatically
undo()
redo()
```

### Auto-Save with Debounce

```typescript
// Debounced 1 second - only saves if user stops typing
useEffect(() => {
  const timer = setTimeout(() => {
    saveGame()
  }, 1000)
  return () => clearTimeout(timer)
}, [isDirty, gameState])
```

### Lazy Loading Screens

```typescript
const DashboardScreen = lazy(() => import('./screens/DashboardScreen'))

<Suspense fallback={<PanelLoading />}>
  <DashboardScreen {...props} />
</Suspense>
```

### Tauri IPC Commands

Frontend calls Rust backend:

```typescript
// Frontend
const game = await invoke('new_game', { clubId: 'default' })

// Backend (Rust)
#[tauri::command]
pub async fn new_game(club_id: String) -> Result<GameState, String> {
  let game = GameState::new(club_id);
  database::save_game(&game).await?;
  Ok(game)
}
```

---

## Keyboard Shortcuts

- **Ctrl+Z** - Undo
- **Ctrl+Y** - Redo
- **Ctrl+S** - Save
- **Ctrl+N** - New Game

---

## Data Persistence

Games are saved to:
- **Windows:** `C:\Users\{username}\.saturday_at_three\{game_id}.json`
- **macOS:** `~/.saturday_at_three/{game_id}.json`
- **Linux:** `~/.saturday_at_three/{game_id}.json`

Currently uses JSON files. Can upgrade to SQLite later:

```toml
# In src-tauri/Cargo.toml
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "sqlite"] }
```

---

## Troubleshooting

### Port 5173 already in use
```bash
netstat -ano | findstr :5173
taskkill /PID {number} /F
```

### Tauri build fails
```bash
# Clean and rebuild
cargo clean
npm run tauri-build
```

### Frontend won't load
1. Check Vite dev server is running
2. Check Tauri window config in `tauri.conf.json`
3. Check browser console for errors (F12 in dev mode)

### Commands not working
1. Check Tauri command signatures match between frontend and backend
2. Verify `#[tauri::command]` is on the function
3. Check `generate_handler![]` includes the command in `main.rs`

---

## Next Steps

1. **Add missing command handlers** in `src-tauri/src/commands.rs`:
   - Import match simulator from Football Man
   - Implement `advance_gameweek`
   - Implement `get_standings`
   - Implement `get_squad`

2. **Integrate match simulator** from Football Man project:
   - Copy simulation modules into `src-tauri/src/`
   - Update `simulate_match` command

3. **Add game initialization**:
   - Create initial 1888-89 season with all clubs
   - Generate fixtures
   - Create default players

4. **Database upgrade**:
   - Replace JSON with SQLite for better querying
   - Create tables for games, matches, players, etc.

5. **Add missing screens**:
   - Squad management (edit players, set formation)
   - Financial management (budget, wages, sponsorships)
   - Transfer market
   - Player development/training

---

## Running in Production

Once built, distribute the executable:

```bash
# Windows
saturday-at-three.exe

# macOS
open "Saturday at Three.app"

# Linux
./saturday-at-three
```

No separate servers, no npm install needed for users!

---

## Development Tips

- **Hot reload:** React changes reload automatically in dev mode
- **Rust changes:** Require restarting the app
- **State debugging:** Use React DevTools to inspect GameState
- **DevTools:** Press F12 in dev build to open browser devtools
- **Logs:** Check browser console and terminal for Rust logs

---

## Summary

You now have a **professional desktop application** with:

✅ Single executable
✅ Full undo/redo support
✅ Auto-save system
✅ Lazy-loaded screens
✅ Keyboard shortcuts
✅ Production-ready architecture

Run `npm run tauri-dev` to start developing!
