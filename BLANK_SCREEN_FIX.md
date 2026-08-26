# Blank Screen Fix - Next Day Button Issue

## Problem
When clicking "Next Day", the screen goes blank because the GameState has empty arrays for:
- `clubs: []`
- `players: []`
- `matches: []`
- `standings: []`

## Root Cause
The `GameState::new()` function in `src-tauri/src/game.rs` initializes empty arrays. When `get_latest_game()` doesn't find a saved game, it returns this empty state.

## Solution
Initialize GameState with default clubs and standings data.

### Step 1: Update `src-tauri/src/game.rs`

Replace the `GameState::new()` implementation with one that includes default clubs and standings:

```rust
impl GameState {
    pub fn new(user_club_id: String) -> Self {
        GameState {
            id: uuid::Uuid::new_v4().to_string(),
            season: 1888,
            current_gameweek: 1,
            user_club_id,
            clubs: Self::default_clubs(),      // NEW: Populate with clubs
            matches: vec![],
            players: vec![],
            standings: Self::default_standings(), // NEW: Populate with standings
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn default_clubs() -> Vec<Club> {
        vec![
            Club { id: "accrington".to_string(), name: "Accrington FC".to_string(), budget: 5000, ... },
            Club { id: "aston-villa".to_string(), name: "Aston Villa".to_string(), budget: 6000, ... },
            Club { id: "blackburn".to_string(), name: "Blackburn Rovers".to_string(), budget: 6000, ... },
            // ... etc for all 11 clubs
        ]
    }

    fn default_standings() -> Vec<Standing> {
        vec![
            Standing { club_id: "accrington".to_string(), club_name: "Accrington FC".to_string(), position: 1, ... },
            Standing { club_id: "aston-villa".to_string(), club_name: "Aston Villa".to_string(), position: 2, ... },
            // ... etc for all 11 clubs
        ]
    }
}
```

## Why This Works
1. When game starts, `get_latest_game()` returns a GameState with populated clubs and standings
2. DashboardScreen can now render club information even if there are no matches or players yet
3. Clicking "Next Day" updates the date without losing the club/standings data

## Alternative Approach (If DashboardScreen Should Handle Empty Data)
Add conditional rendering in `DashboardScreen.tsx` to handle empty data gracefully:

```tsx
{userClub && userStanding ? (
  <div className="status-card">
    {/* render club status */}
  </div>
) : (
  <div className="status-card">
    <p>No club data loaded</p>
  </div>
)}
```

## What to Test After Fix
1. Launch the app
2. Check that Dashboard shows club information (not blank)
3. Click "Next Day" button
4. Verify date updates and screen still shows content
5. Continue clicking several times to ensure consistency

## Files to Modify
- `src-tauri/src/game.rs` - Add default_clubs() and default_standings() methods

## Full Implementation
See `BLANK_SCREEN_FIX_FULL_CODE.rs` for complete implementation with all 11 clubs pre-populated.
