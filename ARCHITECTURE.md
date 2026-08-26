# Saturday at Three - Architecture Guide

## Overview

Saturday at Three is built with a **clean separation between game logic and presentation**. This document outlines the architecture and how to extend it.

## Core Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                     React Frontend (Vite)                       │
│  - Generic UI Components (AssetImage, PlayerCard, MatchPanel)  │
│  - Screen Views (Dashboard, Matchday, Squad, Standings)        │
│  - Asset Context Provider                                       │
└────────────────────────┬─────────────────────────────────────────┘
                         │ REST API
                         ↓
┌────────────────────────────────────────────────────────────────┐
│            Rust Backend (Actix-web)                             │
│  - AssetManager: Load and serve assets                          │
│  - REST API Routes: /api/assets/*                               │
│  - Asset Loaders: Image, JSON, etc.                             │
│  - Error Handling: AssetError types                             │
└────────────────────────┬─────────────────────────────────────────┘
                         ↓
┌────────────────────────────────────────────────────────────────┐
│            Asset Packs (Pluggable)                              │
│  - historical_1888/: Period-accurate visuals                    │
│  - modern/: Contemporary design                                 │
│  - custom/*: User-created packs                                 │
└────────────────────────────────────────────────────────────────┘
```

## Asset System

### Asset Pack Structure

Each asset pack is self-contained and discoverable:

```
assets/packs/{pack_name}/
├── metadata.json              # Pack definition
├── backgrounds/               # Background images (matchday, stadiums)
├── player_portraits/          # Player photos/illustrations
└── ui_elements/               # UI decorations, icons, borders
```

### Asset Management Flow

1. **Discovery** (Startup)
   - AssetManager scans `assets/packs/` directory
   - Loads `metadata.json` from each pack
   - Registers packs for serving

2. **Serving** (Runtime)
   - REST API provides asset URLs
   - Frontend requests: `/api/assets/{pack}/{category}/{id}`
   - Async file loading with caching

3. **Rendering** (UI)
   - React components accept `pack`, `category`, `assetId` props
   - AssetImage component handles loading/errors
   - CSS styles theme and layout

## Backend Architecture

### AssetManager (src/assets/manager.rs)

Core service handling asset operations:

```rust
pub struct AssetManager {
    assets_dir: PathBuf,           // Root assets directory
    loaded_packs: HashMap<...>,    // Discovered packs
    asset_cache: HashMap<...>,     // In-memory cache
}

impl AssetManager {
    pub async fn load_asset(&mut self, pack, category, id) -> Result<Vec<u8>>
    pub fn get_asset_path(&self, pack, category, id) -> Result<AssetReference>
    pub fn get_theme(&self, pack_name) -> Option<ThemeMetadata>
    pub fn list_packs(&self) -> Vec<AssetPack>
}
```

**Key Features:**
- Async/await for non-blocking I/O
- Caching to reduce disk reads
- Auto-discovery of packs
- Theme metadata generation

### REST API Routes (src/api/routes.rs)

```
GET  /api/assets/packs                      # List all packs
GET  /api/assets/pack/{pack_name}           # Get pack info
GET  /api/assets/theme/{pack_name}          # Get theme metadata
GET  /api/assets/path/{pack}/{cat}/{id}    # Get asset path (no download)
GET  /api/assets/{pack}/{category}/{id}    # Get asset (serve/redirect)
GET  /health                                 # Health check
```

### Error Handling (src/errors.rs)

Custom error types for asset operations:

```rust
pub enum AssetError {
    NotFound(String),           // Asset doesn't exist
    InvalidAssetPack(String),   // Pack not found/corrupted
    IoError(std::io::Error),    // File system error
    InvalidPath(String),        // Invalid path component
}
```

## Frontend Architecture

### React Component Hierarchy

```
App
├── Sidebar (Navigation)
│   ├── Main Nav (Dashboard, Matchday, Squad, Standings)
│   └── Theme Selector
├── Main Content
│   ├── DashboardScreen
│   │   ├── News Section
│   │   └── Squad Preview
│   │       └── PlayerCard[] (uses AssetImage)
│   ├── MatchdayScreen
│   │   ├── MatchPanel (uses AssetImage)
│   │   └── Match Events
│   ├── SquadScreen (TODO)
│   └── StandingsScreen (TODO)
└── AssetContext
    └── useAssets() hook
```

### Generic Components

#### AssetImage

Loads images from asset packs:

```tsx
<AssetImage
  pack="historical_1888"
  category="player_portraits"
  assetId="john_smith.png"
  alt="John Smith"
  className="player-photo"
/>
```

**Features:**
- Loading states
- Error handling
- Fallback displays

#### PlayerCard

Displays player with portrait:

```tsx
<PlayerCard
  playerId="123"
  playerName="John Smith"
  position="Striker"
  rating={8.5}
  theme="historical_1888"
  portraitAsset="john_smith.png"
/>
```

#### MatchPanel

Shows match details with background:

```tsx
<MatchPanel
  match={match}
  theme="historical_1888"
  backgroundAsset="stadiums/anfield.jpg"
/>
```

### Context & Hooks

#### AssetContext

Provides asset pack information globally:

```tsx
const { assets, loading } = useAssets();
// assets = { packs: [...], theme?: ThemeMetadata }
```

## Styling Strategy

### CSS Variable System

All colors/fonts defined as CSS variables:

```css
:root {
  --primary: #1a1a2e;
  --accent: #0f3460;
  --text: #eaeaea;
}
```

### Theming Approach

1. **Asset-driven theming**: Visual elements come from asset packs
2. **CSS theming**: Color/font system via CSS variables
3. **Theme metadata**: Pack provides color/font info via API

### Responsive Design

- Sidebar fixed width (200px)
- Main content fluid
- Grid layouts for components
- Mobile-friendly (future)

## Extending the System

### Adding a New Feature Screen

1. Create component in `frontend/src/screens/`
2. Add route in `frontend/src/App.tsx`
3. Use generic components (AssetImage, PlayerCard, etc.)
4. Props accept asset pack/paths

Example:

```tsx
// src/screens/SquadScreen.tsx
export const SquadScreen: React.FC<ScreenProps> = ({ gameState, theme }) => {
  return (
    <div className="squad-screen">
      <div className="player-list">
        {players.map(p => (
          <PlayerCard
            key={p.id}
            playerName={p.name}
            position={p.position}
            rating={p.rating}
            theme={theme}
            portraitAsset={p.portrait}
          />
        ))}
      </div>
    </div>
  )
}
```

### Adding a New Asset Pack

1. Create `assets/packs/my_pack/`
2. Add subdirectories: `backgrounds/`, `player_portraits/`, `ui_elements/`
3. Create `metadata.json`:

```json
{
  "name": "My Pack",
  "version": "1.0.0",
  "description": "Custom asset pack",
  "author": "Your Name",
  "era": "custom",
  "metadata": {
    "color_scheme": "custom",
    "photography_style": "custom"
  }
}
```

4. Add assets to directories
5. Restart backend (auto-discovers)
6. Select in theme dropdown

### Adding a New Component

1. Create `frontend/src/components/MyComponent.tsx`
2. Accept `theme` prop and `assetId` props
3. Use `AssetImage` for graphics
4. Style with CSS classes

```tsx
interface MyComponentProps {
  theme: string
  backgroundAsset?: string
}

export const MyComponent: React.FC<MyComponentProps> = ({ theme, backgroundAsset }) => {
  return (
    <div className="my-component">
      {backgroundAsset && (
        <AssetImage
          pack={theme}
          category="backgrounds"
          assetId={backgroundAsset}
          alt="Background"
        />
      )}
      {/* Component content */}
    </div>
  )
}
```

## Data Flow

### Loading Assets

1. Frontend requests: `GET /api/assets/packs`
2. Backend responds: `{ packs: [...], count: N }`
3. AssetContext caches response
4. Components read from context

### Rendering with Assets

1. Component requests: `GET /api/assets/{pack}/{category}/{id}`
2. Backend loads from cache or disk
3. Frontend displays with loading/error states
4. CSS styles the presentation

## Performance Considerations

### Backend
- Asset caching reduces disk I/O
- Async I/O prevents blocking
- Pack discovery happens once at startup

### Frontend
- AssetContext caches pack list
- Components memoize asset paths
- CSS handles responsive sizing

### Network
- Asset URLs are RESTful and cacheable
- Browser caching helps with repeated assets
- Consider CDN for static assets in production

## Future Enhancements

1. **Asset Pack Versioning**: Support multiple versions per pack
2. **Hot Reloading**: Reload assets without restart
3. **Asset Validation**: Schema validation for packs
4. **Compression**: gzip assets for faster serving
5. **Database Integration**: Store asset metadata in DB
6. **Asset Editor**: In-game tool for creating/editing packs

## Testing Strategy

### Backend
- Unit tests for AssetManager
- Integration tests for API endpoints
- Mock asset packs for testing

### Frontend
- Component tests with Mock AssetContext
- Integration tests with real API
- Visual regression tests for themed components

## Deployment

### Containerization (Docker)

```dockerfile
FROM rust:latest
WORKDIR /app
COPY backend ./backend
RUN cd backend && cargo build --release

FROM node:18
WORKDIR /app
COPY frontend ./frontend
RUN cd frontend && npm install && npm run build

# Combine in final image
FROM debian:bookworm
COPY --from=builder /app/backend/target/release/saturday_at_three /usr/local/bin/
COPY --from=builder /app/frontend/dist /var/www/html
COPY assets /var/assets
```

### Configuration
- Asset directory: Environment variable
- Port: Configuration file
- Log level: Environment variable

## Summary

The architecture provides:

✅ **Flexibility**: Swap asset packs without code changes
✅ **Maintainability**: Clear separation of concerns
✅ **Extensibility**: Easy to add features and components
✅ **Performance**: Async, cached, optimized
✅ **Type Safety**: Rust + TypeScript throughout
