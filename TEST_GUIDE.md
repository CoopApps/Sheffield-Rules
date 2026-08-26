# Saturday at Three - Testing Guide

## What Changed

The app has been updated to:
1. **Default to Saturday at Three game** - Opens directly in the game
2. **Load Accrington FC squad by default** - Shows 250+ real 1888-89 players
3. **Display all club rosters** - Switch between 12 founding clubs
4. **Show all player stats** - 34 attributes per player

## How to Test

### Step 1: Clean Start

Open Command Prompt and run:

```cmd
cd D:\projects\Saturday at Three
npm run tauri dev
```

The app should:
- Open at `http://localhost:5173/`
- Show "Saturday at Three" in the title bar
- Start with the **Squad** screen visible
- Load Accrington FC roster by default

### Step 2: Verify Players Load

When the app opens, you should see:
- **Club Info**: "Accrington FC" with ground, city, region details
- **Squad Breakdown**: Shows how many players in each position
- **Player List**: Organized by position (GK, CB, FB, MID, FWD, WG)
  - Each player row shows: Name, Age, Nationality, Pace, Strength, Passing, Dribbling, Finishing
- **View Stats button**: On each player row

### Step 3: Switch Clubs

Use the **"Select Club"** dropdown in the left sidebar to:
1. Click the dropdown
2. Select a different club (e.g., "Aston Villa")
3. Watch the squad list update with that club's players

Clubs available:
- Accrington FC ✓ (default)
- Aston Villa
- Blackburn Rovers
- Bolton Wanderers
- Burnley FC
- Derby County
- Everton
- Notts County
- Preston North End
- Stoke City
- Sunderland AFC
- Wolverhampton Wanderers

### Step 4: View Player Details

For any player in the list:
1. Click the **"View Stats"** button
2. A panel opens showing all 34 attributes:
   - Physical (6 stats): Pace, Strength, Stamina, Balance, Jumping, Agility
   - Technical (5 stats): Passing, Dribbling, Heading, Crossing, Tackling
   - Mental (5 stats): Courage, Concentration, Leadership, Aggression, Determination
   - Positional (4 stats): Awareness, Marking, Positioning, Work Rate
   - Specializations (3 stats): Finishing, Penalties, Set Pieces
3. Click the back arrow to return to squad list

### Step 5: Test Menu Button

Click the red **"Main Menu"** button at the bottom of the sidebar to:
- Return to game selection screen
- See both Sheffield Rules and Saturday at Three options
- Click Saturday at Three again to reload the game

## Expected Results

✅ **Squad Screen Should Show:**
- 250+ real historical players from 1888-89 season
- 12 founding Football League clubs selectable
- Each player with 34 detailed attributes
- All player stats loading from database

✅ **No Errors:**
- Database queries should complete without errors
- Player data should load smoothly
- Club switching should be instant

## If Something Doesn't Work

### Players not showing:
1. Check that `saturday_at_three.db` exists in the project root
2. Verify the database was initialized:
   ```cmd
   cd D:\projects\Saturday at Three\src-tauri
   cargo run --bin init_database --release
   ```
3. Restart the dev server

### Dropdown not changing club:
- Make sure you're clicking the Club selector dropdown (not the sidebar)
- Try selecting a different club and watch the player list update

### Database connection error:
- Ensure the database path is correct
- The app looks for: `D:/projects/Saturday at Three/saturday_at_three.db`
- Check file permissions on the database

## Performance Notes

- First load may take a few seconds as the database is queried
- Switching clubs is instant (cached in React state)
- Player detail panel opens smoothly

## Success Criteria

✅ App opens with Saturday at Three selected
✅ Squad screen shows Accrington FC with players listed
✅ Can switch between 12 clubs in dropdown
✅ Players display correct stats and attributes
✅ Clicking "View Stats" shows all 34 attributes
✅ Main Menu button returns to game selection

Once all of these work, the club roster viewer is fully functional!
