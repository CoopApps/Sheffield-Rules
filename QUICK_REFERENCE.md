# Saturday at Three - Quick Reference Card

## Start the Game (30 seconds)

```bash
# Terminal 1
cd "D:\projects\Saturday at Three\backend"
cargo run

# Terminal 2
cd "D:\projects\Saturday at Three\frontend"
npm install
npm run dev

# Browser
http://127.0.0.1:5173
```

## File Locations

| What | Where |
|------|-------|
| Rust Code | `backend/src/` |
| React Code | `frontend/src/` |
| Asset Packs | `assets/packs/` |
| Game Assets (Images) | `assets/packs/{theme}/{category}/` |
| Styles | `frontend/src/index.css` |
| Docs | `*.md` files in root |

## Add an Image

1. Save image to: `assets/packs/historical_1888/{category}/{name}.jpg`
2. Restart backend
3. Use: `<AssetImage pack="historical_1888" category="{cat}" assetId="{name}.jpg" />`

## Add a New Screen

1. Create: `frontend/src/screens/MyScreen.tsx`
2. Add navigation button in `App.tsx`
3. Add rendering logic in `App.tsx`
4. Style in `index.css`

## API Endpoints

| Endpoint | Purpose |
|----------|---------|
| `GET /api/assets/packs` | List all packs |
| `GET /api/assets/pack/{name}` | Get pack info |
| `GET /api/assets/theme/{name}` | Get colors/fonts |
| `GET /api/assets/{pack}/{cat}/{id}` | Get asset path |
| `GET /health` | Server health |

## Key Components

| Component | File | Purpose |
|-----------|------|---------|
| AssetImage | `components/AssetImage.tsx` | Load images from packs |
| PlayerCard | `components/PlayerCard.tsx` | Show player details |
| MatchPanel | `components/MatchPanel.tsx` | Display match |
| App | `App.tsx` | Main app shell |

## Directories to Know

```
backend/src/
├── assets/          → AssetManager and loaders
├── api/             → REST endpoints
├── models.rs        → Data types
└── errors.rs        → Error handling

frontend/src/
├── components/      → Reusable UI parts
├── screens/         → Full page views
├── contexts/        → Global state (assets)
└── hooks/           → Custom React hooks

assets/packs/
├── historical_1888/ → Period theme
│   ├── backgrounds/
│   ├── player_portraits/
│   └── ui_elements/
└── modern/         → Modern theme
```

## Common Tasks

### Change a Color
Edit `frontend/src/index.css` - Change `--primary`, `--accent`, etc.

### Add a Theme
1. Create `assets/packs/my_theme/`
2. Add `metadata.json`
3. Add subdirectories and images
4. Restart backend

### Build for Release
```bash
# Backend
cd backend && cargo build --release
# Output: backend/target/release/saturday_at_three.exe

# Frontend
cd frontend && npm run build
# Output: frontend/dist/
```

### Debug in Browser
- Open DevTools (F12)
- Console tab for errors
- Network tab to see API calls
- Elements tab to inspect

## Performance Tips

- Assets cache automatically
- CSS is lightweight
- React components memoize
- Async operations don't block

## Common Errors & Fixes

| Error | Fix |
|-------|-----|
| Port 8080 in use | `taskkill /PID {pid} /F` |
| Compilation fails | `cd backend && cargo clean && cargo build` |
| Assets not showing | Restart backend after adding images |
| Frontend won't connect | Check backend is running, check proxy in `vite.config.ts` |
| Theme dropdown empty | Check `/api/assets/packs` in browser |

## Git Commands

```bash
# First time
cd "D:\projects\Saturday at Three"
git init
git add .
git commit -m "Initial setup"

# Regular commits
git add .
git commit -m "Add squad management screen"
git push
```

## Code Style

- **Rust:** Follow `rustfmt` (cargo fmt)
- **React:** Use functional components and hooks
- **CSS:** Use CSS variables for colors/fonts
- **Naming:** descriptive, lowercase, snake_case (Rust), camelCase (JS)

## Documentation Files

| File | Read When |
|------|-----------|
| `QUICKSTART.md` | Getting started |
| `ARCHITECTURE.md` | Need to understand design |
| `NEXT_STEPS.md` | Planning next features |
| `README.md` | Overview of project |
| `QUICK_REFERENCE.md` | This file - quick lookup |

## Testing Commands

```bash
# Backend checks
cd backend
cargo check          # Quick syntax check
cargo build          # Full build
cargo test           # Run tests

# Frontend checks
cd frontend
npm run build        # Build for production
```

## Useful URLs

| URL | Purpose |
|-----|---------|
| http://127.0.0.1:8080/health | Check backend |
| http://127.0.0.1:8080/api/assets/packs | Get asset packs |
| http://127.0.0.1:5173 | Game UI |

## Key Concepts

- **Asset Pack:** Folder with images + metadata.json
- **Category:** Type of asset (backgrounds, player_portraits, ui_elements)
- **AssetImage:** React component that loads images
- **Theme:** Selected asset pack for visual style
- **AssetManager:** Rust service that loads and serves assets

## Pro Tips

1. Use React DevTools browser extension for debugging
2. Use VS Code Rest Client extension to test API
3. Keep asset file sizes small (compress images)
4. Use SVG for UI elements
5. Organize assets logically in packs

## Emergency Commands

```bash
# Kill all Node processes
taskkill /F /IM node.exe

# Kill all Rust processes
taskkill /F /IM cargo.exe

# Clear npm cache
npm cache clean --force

# Clear Rust build
cargo clean
```

## Version Info

- Rust: 1.70+
- Node: 18+
- React: 18
- TypeScript: 5
- Vite: 5

## Remember

- Backend on port **8080**
- Frontend on port **5173**
- Assets auto-discover at startup
- Images go in `assets/packs/`
- Components are generic and reusable
- Everything is type-safe
- Restart backend after adding assets

---

**This is your quick lookup!** 📝⚽
