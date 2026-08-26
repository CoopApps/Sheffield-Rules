# Sheffield Rules Implementation Plan - Full 19-Year Historical Accuracy

## Overview

Implement Sheffield Rules as a complete separate game from Saturday at Three (1888+).
- **Time Period**: 1858-1877 (19 years)
- **Game Modes**: Historical Timeline, Ahistorical (3 versions), Historical From Any Year
- **Rule Versions**: All 19 annual rulesets with exact historical mechanics
- **Ending**: April 1877 (Sheffield FA adopts FA rules - game ends)

---

## Phase 1: Architecture & Core Systems

### 1.1 Create RuleSet Abstraction Layer

**File**: `src-tauri/src/sheffield_rules/ruleset.rs`

```rust
// Core ruleset trait - defines all rule mechanics
pub trait RuleSet: Clone {
    fn name(&self) -> &'static str;
    fn year(&self) -> u32;
    fn goal_width(&self) -> Option<u16>;
    fn goal_height(&self) -> Option<u16>;
    fn tiebreaker(&self) -> TieBreakerRule;
    fn offside_rule(&self) -> OffsideRule;
    fn handling_rules(&self) -> HandlingRule;
    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger>;
    fn set_piece_rules(&self) -> SetPieceRules;
    fn scoring_system(&self) -> ScoringSystem;
}

pub enum TieBreakerRule {
    None,
    RougeWithTouchdown,
    RougeWithoutTouchdown,
}

pub enum OffsideRule {
    None,
    OneOpponentLevel,
    StrictThreeOpponents,
    AnyPlayerAhead,
}

pub enum HandlingRule {
    FairCatchOnly,
    FairCatchAndPushing,
    NoHandling,
    FairCatchAndAttemptedCatch,
    ThreeYardZoneAndNoExtension,
    GoalkeeperOnly,
}

pub enum ScoringSystem {
    GoalsOnly,
    GoalsAndRouges {
        goal_points: u16,
        rouge_points: u16
    },
}

pub struct SetPieceRules {
    pub throw_in: ThrowInRule,
    pub kick_in: Option<KickInRule>,
    pub goal_kick: GoalKickRule,
    pub corner_kicks: bool,
}

pub enum ThrowInRule {
    FirstTeamTouch,
    AwayTeamIfInTouch,
    NoThrowIns,
}

pub enum KickInRule {
    InAnyDirection,
    RestrictedDirection,
}
```

### 1.2 Create Sheffield Rules Game State

**File**: `src-tauri/src/sheffield_rules/game_state.rs`

```rust
use serde::{Deserialize, Serialize};
use crate::game::GameState as BaseGameState; // Reuse from Saturday at Three

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheffieldRulesGameState {
    // Inherit from base game state
    #[serde(flatten)]
    pub base: BaseGameState,

    // Sheffield-specific fields
    pub game_mode: GameMode,
    pub current_ruleset_year: u32,
    pub end_year: u32, // For historical modes
    pub ruleset_version: SheffieldRulesetVersion,

    // Historical tracking
    pub rule_change_log: Vec<RuleChangeEvent>,
    pub historical_events: Vec<HistoricalEvent>,

    // Rouge-specific (if applicable to current ruleset)
    pub rouge_tally: Option<RougeScoring>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMode {
    HistoricalTimeline, // 1858-1877 with all rule changes
    AhistoricalRougEra, // 1862 rules indefinitely
    AhistoricalPostRouge, // 1868 rules indefinitely
    AhistoricalStableEra, // 1875 rules indefinitely
    HistoricalFromYear(u32), // Start at chosen year, follow history
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SheffieldRulesetVersion {
    Y1858,
    Y1860,
    Y1862,
    Y1863,
    Y1865,
    Y1866,
    Y1867March,
    Y1867October,
    Y1868,
    Y1869,
    Y1871,
    Y1875,
    Y1876,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleChangeEvent {
    pub year: u32,
    pub changes: String, // Description of what changed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEvent {
    pub date: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeScoring {
    pub home_rouges: u16,
    pub away_rouges: u16,
}
```

---

## Phase 2: Implement 19 Annual Rulesets

**File Structure**:
```
src-tauri/src/sheffield_rules/rulesets/
├── y1858.rs
├── y1860.rs
├── y1862.rs (ROUGE INTRODUCED)
├── y1863.rs
├── y1865.rs
├── y1866.rs
├── y1867_march.rs
├── y1867_october.rs
├── y1868.rs (MASSIVE CHANGES - rouge abolished, goals wider)
├── y1869.rs
├── y1871.rs
├── y1875.rs (STABLE FINAL VERSION)
├── y1876.rs
└── mod.rs (central lookup)
```

### Example: 1862 Ruleset (Rouge Era)

**File**: `src-tauri/src/sheffield_rules/rulesets/y1862.rs`

```rust
use super::*;

pub struct Ruleset1862;

impl RuleSet for Ruleset1862 {
    fn name(&self) -> &'static str { "1862 Sheffield Rules (Rouge Era)" }
    fn year(&self) -> u32 { 1862 }
    fn goal_width(&self) -> Option<u16> { Some(12) }
    fn goal_height(&self) -> Option<u16> { Some(9) }

    fn tiebreaker(&self) -> TieBreakerRule {
        TieBreakerRule::RougeWithTouchdown
    }

    fn offside_rule(&self) -> OffsideRule {
        OffsideRule::None
    }

    fn handling_rules(&self) -> HandlingRule {
        HandlingRule::FairCatchOnly
    }

    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger> {
        vec![FreeKickTrigger::FairCatch]
    }

    fn set_piece_rules(&self) -> SetPieceRules {
        SetPieceRules {
            throw_in: ThrowInRule::FirstTeamTouch,
            kick_in: None,
            goal_kick: GoalKickRule::WheneverNoGoalOrRouge,
            corner_kicks: false,
        }
    }

    fn scoring_system(&self) -> ScoringSystem {
        ScoringSystem::GoalsAndRouges {
            goal_points: 1,
            rouge_points: 1, // Rouge counts as 1 point (or separate category)
        }
    }
}
```

### Example: 1868 Ruleset (Post-Rouge - Biggest Transformation)

**File**: `src-tauri/src/sheffield_rules/rulesets/y1868.rs`

```rust
pub struct Ruleset1868;

impl RuleSet for Ruleset1868 {
    fn name(&self) -> &'static str { "1868 Sheffield Rules (Post-Rouge)" }
    fn year(&self) -> u32 { 1868 }
    fn goal_width(&self) -> Option<u16> { Some(24) } // DOUBLED!
    fn goal_height(&self) -> Option<u16> { Some(9) }

    fn tiebreaker(&self) -> TieBreakerRule {
        TieBreakerRule::None // ROUGE ABOLISHED
    }

    fn offside_rule(&self) -> OffsideRule {
        OffsideRule::OneOpponentLevel
    }

    fn handling_rules(&self) -> HandlingRule {
        HandlingRule::FairCatchOnly
    }

    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger> {
        vec![
            FreeKickTrigger::FairCatch,
            FreeKickTrigger::Handball,
            FreeKickTrigger::FoulPlay,
        ]
    }

    fn set_piece_rules(&self) -> SetPieceRules {
        SetPieceRules {
            throw_in: ThrowInRule::NoThrowIns, // KICK-INS INTRODUCED
            kick_in: Some(KickInRule::InAnyDirection), // NEW!
            goal_kick: GoalKickRule::VariableByLocation,
            corner_kicks: true, // CORNER KICKS INTRODUCED (first ever!)
        }
    }

    fn scoring_system(&self) -> ScoringSystem {
        ScoringSystem::GoalsOnly // Just goals, no rouges
    }
}
```

### Ruleset Lookup Function

**File**: `src-tauri/src/sheffield_rules/rulesets/mod.rs`

```rust
pub fn get_ruleset_for_year(year: u32, month: Option<u32>) -> Box<dyn RuleSet> {
    match (year, month) {
        (1858..=1859, _) => Box::new(y1858::Ruleset1858),
        (1860..=1861, _) => Box::new(y1860::Ruleset1860),
        (1862, _) => Box::new(y1862::Ruleset1862),
        (1863..=1864, _) => Box::new(y1863::Ruleset1863),
        (1865, _) => Box::new(y1865::Ruleset1865),
        (1866, _) => Box::new(y1866::Ruleset1866),
        (1867, Some(m)) if m < 10 => Box::new(y1867_march::Ruleset1867March),
        (1867, Some(m)) if m >= 10 => Box::new(y1867_october::Ruleset1867October),
        (1867, None) => Box::new(y1867_march::Ruleset1867March), // Default to March
        (1868, _) => Box::new(y1868::Ruleset1868),
        (1869..=1870, _) => Box::new(y1869::Ruleset1869),
        (1871..=1874, _) => Box::new(y1871::Ruleset1871),
        (1875, _) => Box::new(y1875::Ruleset1875),
        (1876, _) => Box::new(y1876::Ruleset1876),
        (1877, _) => Box::new(y1877_fa::RulesetFa1877), // FA rules
        _ => panic!("Year {} out of Sheffield Rules range", year),
    }
}
```

---

## Phase 3: Game Mode System

**File**: `src-tauri/src/sheffield_rules/modes.rs`

```rust
pub struct GameModeHandler;

impl GameModeHandler {
    pub fn get_next_rule_change_year(
        mode: &GameMode,
        current_year: u32,
    ) -> Option<u32> {
        match mode {
            GameMode::HistoricalTimeline => {
                // Return next rule change year
                match current_year {
                    1858..=1859 => Some(1860),
                    1860..=1861 => Some(1862),
                    1862 => Some(1863),
                    1863..=1864 => Some(1865),
                    1865 => Some(1866),
                    1866 => Some(1867),
                    1867 => Some(1868), // Major change Oct 1868
                    1868 => Some(1869),
                    1869..=1870 => Some(1871),
                    1871..=1874 => Some(1875),
                    1875 => Some(1876),
                    1876 => Some(1877), // End
                    _ => None,
                }
            },
            GameMode::AhistoricalRougEra => None, // No changes
            GameMode::AhistoricalPostRouge => None, // No changes
            GameMode::AhistoricalStableEra => None, // No changes
            GameMode::HistoricalFromYear(start_year) => {
                // Same as HistoricalTimeline but starting from given year
                Self::get_next_rule_change_year(&GameMode::HistoricalTimeline, current_year)
                    .and_then(|next| if next > 1877 { None } else { Some(next) })
            },
        }
    }

    pub fn should_end_game(mode: &GameMode, current_year: u32) -> bool {
        match mode {
            GameMode::HistoricalTimeline => current_year > 1877,
            GameMode::HistoricalFromYear(start) => current_year > 1877,
            _ => false, // Indefinite
        }
    }
}
```

---

## Phase 4: Sheffield Clubs & Team Data

**File**: `src-tauri/src/sheffield_rules/clubs.rs`

```rust
pub fn default_sheffield_clubs() -> Vec<Club> {
    vec![
        Club {
            id: "sheffield-fc".to_string(),
            name: "Sheffield F.C.".to_string(),
            founded: 1857,
            ..Default::default()
        },
        Club {
            id: "hallam-fc".to_string(),
            name: "Hallam F.C.".to_string(),
            founded: 1860,
            ..Default::default()
        },
        Club {
            id: "sheffield-wednesday".to_string(),
            name: "The Wednesday".to_string(), // Founded 1867
            founded: 1867,
            ..Default::default()
        },
        // ... other Sheffield clubs
    ]
}
```

---

## Phase 5: Fixture Generation

**File**: `src-tauri/src/sheffield_rules/fixtures.rs`

```rust
pub fn generate_sheffield_fixtures(year: u32) -> Vec<Match> {
    let num_teams = match year {
        1858..=1866 => 6,  // Small, informal
        1867..=1875 => 12, // Sheffield FA formed
        1876..=1877 => 14,
        _ => 12,
    };

    // Generate round-robin fixtures
    let mut matches = Vec::new();
    // ... procedural generation logic
    matches
}
```

---

## Phase 6: Frontend Updates

### 6.1 Update GameSelectionScreen

**File**: `frontend/src/screens/GameSelectionScreen.tsx`

Add Sheffield Rules option to game selection menu.

### 6.2 Create Sheffield Game Mode Selector

**File**: `frontend/src/components/SheffieldModeSelector.tsx`

Modal to choose between:
- Historical Timeline
- Ahistorical: Rouge Era (1862)
- Ahistorical: Post-Rouge (1868)
- Ahistorical: Stable Era (1875)
- Historical From Any Year (dropdown: 1858, 1862, 1868, 1875)

### 6.3 Update Dashboard for Historical Context

Display:
- Current ruleset year
- Next rule change (if applicable)
- Historical events for the era
- Rouge scoring (if applicable)

---

## Phase 7: Historical Events & Game Ending

**File**: `src-tauri/src/sheffield_rules/events.rs`

```rust
pub fn get_historical_events() -> Vec<HistoricalEvent> {
    vec![
        HistoricalEvent {
            date: "1858-10-28".to_string(),
            title: "Sheffield F.C. Founded".to_string(),
            description: "Nathaniel Creswick and William Prest establish Sheffield F.C.".to_string(),
        },
        HistoricalEvent {
            date: "1860-12-26".to_string(),
            title: "First Match vs Hallam".to_string(),
            description: "Sheffield F.C. plays first match against Hallam F.C.".to_string(),
        },
        HistoricalEvent {
            date: "1862-02-22".to_string(),
            title: "Rouge Introduced".to_string(),
            description: "New tiebreaker mechanic debuts in Sheffield Rules.".to_string(),
        },
        HistoricalEvent {
            date: "1867-03-01".to_string(),
            title: "Sheffield FA Formed".to_string(),
            description: "Sheffield Football Association created, formalizing the rules.".to_string(),
        },
        HistoricalEvent {
            date: "1867-02-15".to_string(),
            title: "Youdan Cup - First Football Tournament".to_string(),
            description: "The world's first football tournament played under Sheffield Rules.".to_string(),
        },
        HistoricalEvent {
            date: "1868-10-01".to_string(),
            title: "Major Rule Changes".to_string(),
            description: "Rouge abolished, corner kicks introduced, goals widened, kick-ins replace throws.".to_string(),
        },
        HistoricalEvent {
            date: "1877-04-01".to_string(),
            title: "Unification".to_string(),
            description: "Sheffield FA votes to adopt FA rules. Era ends.".to_string(),
        },
    ]
}
```

### Game Ending Screen

When year > 1877 in historical modes:

```typescript
<div className="game-ending">
  <h1>Sheffield Rules Era Concluded</h1>
  <p>April 1877: Sheffield FA votes to adopt FA rules</p>
  <p>Your career in Sheffield Rules has ended.</p>
  <p>Historical Outcome: [Show what happened to your club in reality]</p>
  <button onClick={returnToMenu}>Return to Main Menu</button>
  <button onClick={startNewGame}>Start New Sheffield Rules Career</button>
</div>
```

---

## Implementation Order

1. ✅ Design architecture & RuleSet trait
2. ✅ Create Sheffield GameState
3. ⬜ Implement 19 rulesets (highest priority - big task)
4. ⬜ Game mode system
5. ⬜ Clubs and fixture generation
6. ⬜ Frontend updates (mode selector, dashboard)
7. ⬜ Historical events
8. ⬜ Test and build

---

## Key Design Decisions

- **Reuse Saturday at Three code**: ~80% of logic is transferable
- **Separate game entirely**: No rule mixing between games
- **Full historical accuracy**: All 19 annual rulesets
- **Natural ending**: Game ends 1877 for historical modes
- **Three ahistorical options**: Explore rouges, post-rouge, and stable eras indefinitely
- **Rouge mechanic**: Central to 1862-1868, must implement fully

---

## Testing Checklist

- [ ] Each of 19 rulesets loads correctly
- [ ] Historical Timeline progresses rules at correct years
- [ ] Ahistorical modes never change rules
- [ ] "From Any Year" starts at correct point and progresses
- [ ] Rouge scoring works (1862-1868)
- [ ] Goal dimensions affect gameplay
- [ ] Offside rules enforce correctly
- [ ] Set pieces work with correct rules
- [ ] Game ends at 1877 for historical modes
- [ ] Historical events display at correct times
