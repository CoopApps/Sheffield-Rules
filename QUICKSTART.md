# Saturday at Three - Quick Start Guide

## Overview

**Saturday at Three** is a football management simulator focusing on the inaugural 1888-89 Football League season in Britain. It features 12 founding clubs with 250+ real historical players with detailed attributes.

## Running the Application

### Windows (Easiest)

1. Open Command Prompt and navigate to the project:
   ```cmd
   cd D:\projects\Saturday at Three
   ```

2. Run the application:
   ```cmd
   npm run tauri dev
   ```

3. The app will open automatically at `http://localhost:5173/`

### Alternative: Using Batch File

Double-click `test-app.bat` in the project root directory.

## Application Features

### Game Selection Screen
When you first start the app, you'll see two game options:
- **Sheffield Rules** - Full management game (future expansion)
- **Saturday at Three** - Demo featuring 1888-89 season

Click on "Saturday at Three" to begin.

### Main Navigation
Once in the game, the left sidebar provides access to:
- **Squad** - Browse all players with detailed stats
- **Dashboard** - Season overview
- **Matchday** - View matches
- **Standings** - League table
- **Main Menu** - Return to game selection (red button)

### Club Selection
A dropdown menu in the sidebar lets you switch between 12 founding clubs:
- Accrington FC, Aston Villa, Blackburn Rovers, Bolton Wanderers
- Burnley FC, Derby County, Everton, Notts County
- Preston North End, Stoke City, Sunderland AFC, Wolverhampton Wanderers

## Viewing Squads and Player Stats

### Squad Screen
1. Click the **Squad** button in the sidebar
2. Select a club from the dropdown
3. View:
   - Club info (ground, capacity, location)
   - Squad breakdown by position
   - All players organized by position

### Player Statistics
Each player shows:
- Name, Age, Nationality
- Key stats: Pace, Strength, Passing, Dribbling, Finishing

### Detailed Player View
Click "View Stats" on any player to see all 34 attributes:
- **Physical** (6): Pace, Strength, Stamina, Balance, Jumping, Agility
- **Technical** (5): Passing, Dribbling, Heading, Crossing, Tackling
- **Mental** (5): Courage, Concentration, Leadership, Aggression, Determination
- **Positional** (4): Awareness, Marking, Positioning, Work Rate
- **Specializations** (3): Finishing, Penalties, Set Pieces

All attributes are on a 1-20 scale.

## Database

The app uses SQLite with:
- 12 founding clubs with real information
- 250 real players from 1888-89 with 34 attributes each
- Match fixture structure for 22-round season

## Next Steps

- Explore all 12 clubs and their rosters
- View player attributes for tactical analysis
- Watch for upcoming match simulation features
