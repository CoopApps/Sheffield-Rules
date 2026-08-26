# Saturday at Three - 1888-89 Football League Simulator

A historical football management simulation game set in 1888-89, the inaugural season of the Football League.

## Project Structure

```
Saturday at Three/
├── backend/              # Rust backend with match simulator
│   ├── src/
│   │   ├── main.rs      # Server entry point
│   │   ├── lib.rs
│   │   ├── assets/      # Asset management system
│   │   │   ├── manager.rs    # Core AssetManager
│   │   │   └── loader.rs     # Generic asset loaders
│   │   ├── api/         # REST API endpoints
│   │   │   ├── routes.rs     # API routes
│   │   │   └── handlers.rs   # Request handlers
│   │   ├── models.rs    # Data models
│   │   └── errors.rs    # Error types
│   └── Cargo.toml
│
├── frontend/            # React + TypeScript UI
│   ├── src/
│   │   ├── main.tsx
│   │   ├── App.tsx
│   │   ├── index.css
│   │   ├── components/  # Reusable UI components
│   │   │   ├── AssetImage.tsx
│   │   │   ├── PlayerCard.tsx
│   │   │   └── MatchPanel.tsx
│   │   ├── screens/     # Full screen views
│   │   │   ├── DashboardScreen.tsx
│   │   │   └── MatchdayScreen.tsx
│   │   ├── contexts/    # React contexts
│   │   │   └── AssetContext.tsx
│   │   ├── hooks/       # Custom hooks
│   │   │   └── useAssets.ts
│   │   └── types/       # TypeScript types
│   │       └── GameState.ts
│   ├── index.html
│   ├── vite.config.ts
│   ├── tsconfig.json
│   └── package.json
│
└── assets/              # Game assets (pluggable)
    └── packs/
        ├── historical_1888/   # Period-accurate assets
        │   ├── metadata.json
        │   ├── backgrounds/
        │   ├── player_portraits/
        │   └── ui_elements/
        └── modern/            # Modern asset pack
            ├── metadata.json
            ├── backgrounds/
            ├── player_portraits/
            └── ui_elements/
```

## Architecture

### Generic Asset System

The project uses a **pluggable asset architecture** that allows different visual themes:

1. **AssetManager** (Rust) - Central asset management service
   - Discovers asset packs from `/assets/packs/`
   - Provides REST API for asset retrieval
   - Supports custom asset loaders

2. **Asset Packs** - Modular, self-contained visual themes
   - Each pack has metadata defining its era/style
   - Can be swapped without changing code
   - Organized by category (backgrounds, portraits, UI)

3. **React Components** - Generic UI components
   - Accept asset pack/path as props
   - Fetch assets via REST API
   - Style-agnostic rendering

### API Endpoints

```
GET  /api/assets/packs                          # List all packs
GET  /api/assets/pack/{pack_name}               # Get pack details
GET  /api/assets/theme/{pack_name}              # Get theme metadata
GET  /api/assets/path/{pack}/{category}/{id}   # Get asset path
GET  /api/assets/{pack}/{category}/{id}         # Get asset data
GET  /health                                     # Health check
```

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+
- npm or yarn

### Backend Setup

```bash
cd backend
cargo build
cargo run
# Server runs on http://localhost:8080
```

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
# Frontend runs on http://localhost:5173
```

## Development

### Adding a New Asset Pack

1. Create directory: `assets/packs/my_pack/`
2. Add subdirectories: `backgrounds/`, `player_portraits/`, `ui_elements/`
3. Create `metadata.json`:
   ```json
   {
     "name": "My Pack",
     "version": "1.0.0",
     "description": "My custom asset pack",
     "author": "Your Name",
     "era": "custom",
     "metadata": {}
   }
   ```
4. Add asset files to respective directories
5. Restart backend - packs auto-discovered

### Using Assets in Components

```tsx
<AssetImage
  pack="historical_1888"
  category="backgrounds"
  assetId="match_scene.jpg"
  alt="Match scene"
/>
```

## Features

### Core Systems (Complete)
- ✅ Asset management and serving
- ✅ Generic React components
- ✅ Multiple asset pack support
- ✅ REST API for asset retrieval
- ✅ Dashboard and matchday screens

### To Build
- 🚧 Game loop and season progression
- 🚧 Squad management interface
- 🚧 Pre-match tactics/formations
- 🚧 League standings display
- 🚧 Financial management
- 🚧 Transfer market
- 🚧 Player development system
- 🚧 Match simulation integration

## Key Design Decisions

1. **Separation of Concerns** - Backend (game logic) and Frontend (UI) are decoupled
2. **Asset-Driven UI** - All graphics come from asset packs, UI is generic
3. **Type Safety** - Rust backend + TypeScript frontend for compile-time safety
4. **REST API** - Simple HTTP interface for asset/game data
5. **Modularity** - Each feature is self-contained and testable

## Performance

- Asset discovery: <100ms on startup
- Asset serving: <10ms per request
- Frontend load time: <2s on typical connection

## License

TBD
