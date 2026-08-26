# Promotion/Relegation Implementation Plan

## Overview
Implement automatic promotion/relegation system for Sheffield & Hallamshire League with regional distribution for Divisions 6 and 7.

## Promotion Rules
- **Div 2**: Top 4 → Div 1
- **Div 3**: Top 3 → Div 2
- **Div 4**: Top 3 → Div 3
- **Div 5A+5B**: Top 2 from each → Div 4 (4 total)
- **Div 6A-D**: Top 1 from each → Div 5 (4 total, distributed: 2→5A, 2→5B)
- **Div 7A-D**: Top 1 from each → Div 6 (same region)

## Relegation Rules
- **Div 1**: Bottom 4 → Div 2
- **Div 2**: Bottom 3 → Div 3
- **Div 3**: Bottom 3 → Div 4
- **Div 4**: Bottom 4 → Div 5 (split: 2→5A, 2→5B)
- **Div 5A+5B**: Bottom → Div 6 (distributed to A/B/C/D)
- **Div 6A-D**: Bottom 1 from each → Div 7 (same region)
- **Div 7**: No relegation (lowest tier)

## Implementation Steps

### Step 1: Create Database Schema Updates
- Add `sheffield_league_movement_history` table for tracking promotions/relegations
- Add indexes on `division_id` and `club_id`

### Step 2: Create Core Promotion Module
- File: `src-tauri/src/sheffield_rules/promotion.rs`
- Implement calculation functions for each division
- Implement regional distribution logic
- Implement position reassignment

### Step 3: Create Database Operations Module
- File: `src-tauri/src/database/promotion_db.rs`
- Implement standing retrieval queries
- Implement club movement updates
- Implement movement history recording

### Step 4: Integrate with Commands
- File: `src-tauri/src/commands.rs`
- Add `advance_season` command
- Modify `advance_gameweek` to detect season end
- Add Tauri command registration in `main.rs`

### Step 5: Update Game State
- File: `src-tauri/src/game.rs`
- Add season completion detection
- Add season end processing

### Step 6: Frontend Integration (Optional)
- Display promotion/relegation notifications
- Show season end summary
- Update division displays

## Key Functions to Implement

### Core Logic
```
calculate_promotions_from_division(division, season) -> Vec<(clubId, position)>
calculate_relegations_from_division(division, season) -> Vec<(clubId, position)>
get_division_final_standings(division, season) -> Vec<Standing>
allocate_promoted_teams(fromDiv, toDiv, teams) -> Result
allocate_relegated_teams(fromDiv, toDivs, teams) -> Result
distribute_teams_regionally(teams, targetDivs) -> Vec<RegionalDistribution>
process_season_end_promotions_relegations(season) -> SeasonEndReport
```

### Database Queries
```
get_division_standings(division_id) -> Vec<Standing>
update_club_divisions(club_id, oldDiv, newDiv, newPos) -> Result
batch_update_club_divisions(updates) -> Result
recalculate_division_positions(division_id) -> Result
record_movement_history(event, season) -> Result
```

## Regional Distribution Logic

### Div 7 → Div 6 (Same Region)
- 7A top 1 → 6A
- 7B top 1 → 6B
- 7C top 1 → 6C
- 7D top 1 → 6D

### Div 6 → Div 7 (Same Region)
- 6A bottom 1 → 7A
- 6B bottom 1 → 7B
- 6C bottom 1 → 7C
- 6D bottom 1 → 7D

### Div 6 → Div 5 (Regional Pairs)
- 6A, 6B top 1 each → 5A (or similar pairing)
- 6C, 6D top 1 each → 5B

### Div 5 → Div 6 (Scatter Distribution)
- Distribute bottom teams from 5A and 5B to 6A, 6B, 6C, 6D
- Use geographic proximity if available

### Div 4 → Div 5 (Simple Split)
- Bottom 4: 2 to 5A, 2 to 5B
- Use standing order for alternation

## Data Structures

```rust
struct PromotionEvent {
    club_id: String,
    club_name: String,
    from_division_id: String,
    to_division_id: String,
    final_position: i32,
}

struct RelegationEvent {
    club_id: String,
    club_name: String,
    from_division_id: String,
    to_division_id: String,
    final_position: i32,
}

struct SeasonEndReport {
    season: u16,
    promotions: Vec<PromotionEvent>,
    relegations: Vec<RelegationEvent>,
    divisions_affected: Vec<String>,
    processed_at: String,
}
```

## Process Flow

```
Season End Detection (in advance_gameweek/day)
  ↓
Check: All matches played OR date >= season end
  ↓
Call process_season_end_promotions_relegations()
  ↓
Get standings for each division
  ↓
Calculate promotions/relegations (top-down)
  ↓
Allocate teams to destination divisions (bottom-up)
  ↓
Update positions in new divisions
  ↓
Record movement history
  ↓
Return SeasonEndReport to frontend
  ↓
Reset for new season
```

## Testing Checklist

- [ ] Division 1 bottom 4 relegated to Div 2
- [ ] Div 2 top 4 promoted to Div 1
- [ ] Div 3-4 promotion/relegation works
- [ ] Div 5A/B split promotions correctly
- [ ] Div 6 regional same-region promotion/relegation
- [ ] Div 7 top 1 from each region promotes to Div 6
- [ ] Regional distribution logic (6→5, 5→6)
- [ ] No data loss during transitions
- [ ] Positions correctly reassigned
- [ ] Reserve teams follow main teams
- [ ] Movement history recorded correctly
- [ ] Full season end processing works end-to-end

## Priority Order for Implementation

1. **High Priority**
   - Core promotion/relegation calculation functions
   - Basic division updates (Div 1-5 handling)
   - Database schema for movement history

2. **Medium Priority**
   - Regional distribution logic (Div 6-7)
   - Movement history tracking
   - Season end orchestration command

3. **Low Priority**
   - UI notifications
   - Movement history display
   - Detailed reports

## Edge Cases to Handle

- Tied positions (use tiebreakers: GD, GF)
- Incomplete seasons
- Invalid division references
- Orphaned clubs
- Reserve teams
- Database transaction rollback
- Concurrent requests
