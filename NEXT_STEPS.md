# Saturday at Three - Next Steps & Roadmap

## Current Status

✅ **Complete Foundation Built**
- Rust backend with AssetManager and REST API
- React + TypeScript frontend with generic components
- Pluggable asset pack system
- Clean architecture with separation of concerns
- Two asset packs (historical_1888, modern) - empty, ready for images
- Documentation and guides

## Immediate Next Steps (Start Here)

### 1. Test the System (Day 1)

```bash
# Terminal 1: Start backend
cd backend
cargo build
cargo run

# Terminal 2: Start frontend
cd frontend
npm install
npm run dev

# Browser: http://127.0.0.1:5173
```

Verify:
- Backend starts and serves `/health`
- Frontend loads
- Theme dropdown shows "Historical 1888" and "Modern"
- Navigation buttons work

### 2. Add Placeholder Assets (Day 1-2)

Create simple images and place them in asset directories:

```
assets/packs/historical_1888/
├── backgrounds/
│   └── match_scene.jpg        # Any match/stadium image
├── player_portraits/
│   ├── player_1.jpg
│   ├── player_2.jpg
│   └── player_3.jpg
└── ui_elements/
    └── (decorations later)
```

Restart backend - images will render automatically in:
- Dashboard → Squad Highlights
- Matchday → Background

### 3. Integrate Match Simulator (Day 2-3)

Connect the match simulator from "Football Man" project:

**What to do:**
1. Copy core simulation modules from Football Man to backend
2. Create REST endpoint: `POST /api/simulate/match`
3. Connect to MatchdayScreen

**Backend changes needed:**

```rust
// src/simulation/mod.rs
pub mod match_engine;
pub mod team_form;
// ... other modules

// src/api/handlers.rs
pub async fn simulate_match(
    req: web::Json<MatchRequest>
) -> Result<HttpResponse, MatchError> {
    let result = run_match_simulation(req.home_team, req.away_team)?;
    Ok(HttpResponse::Ok().json(result))
}
```

**Frontend changes needed:**

```tsx
// MatchdayScreen.tsx
useEffect(() => {
  const fetchMatch = async () => {
    const response = await fetch('/api/simulate/match', {
      method: 'POST',
      body: JSON.stringify({ homeTeam: 'Liverpool', awayTeam: 'Watford' })
    });
    const match = await response.json();
    setMatch(match);
  };
  fetchMatch();
}, []);
```

## Phase 1: Core Gameplay (Week 1-2)

### Goal: Playable Season Loop

Build these screens in order:

#### 1. Squad Management Screen
**What:** Display team roster, player details, contracts
**Components needed:**
- PlayerCard (already have generic version)
- Squad list grid
- Player detail view
- Formation selector

**File:** `frontend/src/screens/SquadScreen.tsx`

**Asset needs:**
- Player portraits for each player
- Position badges/icons

#### 2. League Standings Screen
**What:** Show current season standings, next fixtures, recent results
**Components needed:**
- Standings table (team, P, W, D, L, GD, PTS)
- Next fixtures section
- Recent results section

**File:** `frontend/src/screens/StandingsScreen.tsx`

**Asset needs:**
- Team colors/crests (optional)
- Background textures

#### 3. Pre-Match Setup Screen
**What:** Choose formation, select team, set tactics
**Components needed:**
- Formation selector (grid based, drag/drop later)
- Player selection UI
- Tactic options (attacking/defensive/balanced)

**File:** `frontend/src/screens/PreMatchScreen.tsx`

**Asset needs:**
- Pitch/field background
- Formation diagrams

#### 4. Game Loop Integration
**What:** Connect screens into playable season flow
**Changes:**
- Add Gameweek progression button
- Update gameweek number when advancing
- Show matches for current gameweek
- Save/load game state

**Files:**
- `backend/src/game_loop.rs` (new)
- `frontend/src/screens/GameweekScreen.tsx` (new)

## Phase 2: Game Features (Week 3-4)

### Financial Management
- Budget display
- Wage calculations
- Transfer budget
- Sponsorship deals

### Player Development
- Training system
- Morale/form tracking
- Injury/recovery
- Contract management

### Transfer Market
- Buy/sell players
- Scouting system
- Offer negotiations
- Transfer list

### Staff Management
- Hire/fire coaches
- Assistant managers
- Tactical advisors

## Architecture Reference

### Adding a New Screen

1. **Create component:**
```tsx
// src/screens/MyScreen.tsx
interface MyScreenProps {
  gameState: GameState
  theme: string
}

export const MyScreen: React.FC<MyScreenProps> = ({ gameState, theme }) => {
  return <div className="my-screen">{/* Content */}</div>
}
```

2. **Add to App.tsx:**
```tsx
import { MyScreen } from './screens/MyScreen'

// In navigation:
<button onClick={() => handleScreenChange('myscreen')}>My Screen</button>

// In render:
{currentScreen === 'myscreen' && <MyScreen {...props} />}
```

3. **Style in index.css:**
```css
.my-screen {
  padding: 20px;
  /* Your styles */
}
```

### Adding Backend Features

1. **Create module:** `src/features/my_feature.rs`
2. **Add to lib.rs:** `pub mod features;`
3. **Create API handler:** `api/handlers.rs` (add function)
4. **Add route:** `api/routes.rs` (add endpoint)
5. **Test with curl or frontend**

### Adding Assets

1. Create directory: `assets/packs/{pack}/category/`
2. Add image files
3. Restart backend
4. Use in components:
```tsx
<AssetImage
  pack={theme}
  category="backgrounds"
  assetId="my_image.jpg"
  alt="Description"
/>
```

## Database Integration (Later)

When ready to persist game state:

1. **Add SQLx to backend:**
   - Already in Cargo.toml
   - Create migrations in `migrations/`
   - Design schema for season, clubs, players, matches

2. **Create DB queries:**
   - `src/database/season_queries.rs`
   - `src/database/match_queries.rs`
   - etc.

3. **Connect to API:**
   - Load game state from DB
   - Save changes back to DB

Example structure from Football Man can be adapted:
- Season tracking
- League standings
- Match history
- Player statistics

## Testing Strategy

### Backend
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# Run server
cargo run
```

### Frontend
```bash
# (Future) Component tests
npm test

# Dev server
npm run dev

# Build
npm run build
```

## Deployment Checklist

Before release:

- [ ] Backend compiles without warnings
- [ ] Frontend builds without errors
- [ ] Assets load correctly
- [ ] All screens are functional
- [ ] API endpoints tested
- [ ] Documentation complete
- [ ] .gitignore configured
- [ ] No hardcoded paths/secrets

## Performance Optimization (Later)

- Asset caching optimization
- Database query optimization
- Frontend code-splitting
- Image compression
- Lazy loading screens

## Visual Polish (Weeks 5+)

- Historical 1888 asset pack creation
  - Authentic period photographs
  - Victorian-era UI elements
  - Sepia/brown color grading
  - Period-appropriate fonts

- Modern asset pack completion
  - Contemporary player photos
  - Modern UI design
  - Smooth animations
  - Professional graphics

## Integration with "Football Man"

Key systems to copy/adapt:

1. **Match Simulation**
   - Copy: `src/simulation/match_engine.rs`
   - Copy: `src/simulation/intensity_simulator.rs`
   - Copy: `src/simulation/captain_ai.rs`

2. **Player Attributes**
   - Copy: `src/models/player.rs`
   - Adapt: Use in REST API

3. **Team Formation**
   - Copy: `src/simulation/formation.rs`
   - Expose via UI

4. **Season Tracking**
   - Copy: `src/simulation/season_context.rs`
   - Integrate with game loop

5. **Database Schema**
   - Reference: `src/database/`
   - Create simplified version for S3

## Resource Checklist

What you'll need:

### Images
- [ ] Player photos/portraits (1000+)
- [ ] Stadium backgrounds
- [ ] Team crests/badges
- [ ] UI decorations
- [ ] Match scene backgrounds

### Code
- [ ] Match simulator integration
- [ ] Season progression logic
- [ ] Database schema
- [ ] Save/load system

### Documentation
- [ ] API documentation
- [ ] Component library
- [ ] Asset pack guide
- [ ] Deployment guide

## Estimated Timeline

- **Week 1:** Asset integration + Core screens
- **Week 2:** Game loop + Match simulator
- **Week 3:** Financial/Development systems
- **Week 4:** Polish + Testing
- **Week 5+:** Visual polish + Asset creation

## Success Criteria

✅ Can simulate a full 38-gameweek season
✅ Manage squad (buy/sell/develop players)
✅ Watch matches play out realistically
✅ Track financial position
✅ Beautiful period-accurate visuals
✅ Responsive, fast UI
✅ Save/load game state

## Questions to Answer

1. **Asset priority:** Which screenshots/mockups first?
2. **Platform:** Tauri desktop, web-only, or both?
3. **Database:** SQLite or other?
4. **Save location:** Local files or cloud?
5. **Multiplayer:** Single-player only?

## Final Notes

The foundation is solid. Everything that follows will be building features on top of:
- ✅ Proven asset system
- ✅ Clean architecture
- ✅ Type-safe code
- ✅ Scalable structure

Focus on:
1. Getting to "playable" first (Season loop + matches)
2. Adding visual assets (make it look good)
3. Polishing gameplay (balance, fun, bugs)
4. Extending features (transfers, staff, etc.)

Start with the immediate next steps and ship early. 🚀
