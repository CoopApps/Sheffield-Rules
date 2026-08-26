# Multi-League Architecture Implementation Plan

## Overview
Transform Saturday at Three from a single-league (Sheffield 1867) system into a multi-league platform where users can create custom leagues with their own divisions, clubs, and promotion rules.

## Requirements
1. Create custom leagues from scratch (not just Sheffield 1867)
2. Define custom division structures (names, levels, regions)
3. Import/create clubs for each league
4. Configure promotion/relegation rules per league
5. Set league-specific settings (points, scheduling, etc.)
6. Sheffield 1867 must continue to work (backward compatibility)
7. Each league must be isolated (separate data)

## Architecture Summary

### Database Changes
- Add `league_id TEXT NOT NULL` foreign key to 12 league-specific tables
- Keep `sheffield_clubs` global (shared across all leagues)
- Add `league_metadata` table for multi-league tracking (already exists)
- All queries filtered by `league_id` for data isolation

### Code Architecture
- **Template System**: Extract Sheffield 1867 hardcoded data into reusable `LeagueTemplate` trait
- **Query Scoping**: Create `LeagueScopedQuery` helper to enforce league isolation
- **Dynamic Rules**: Replace hardcoded promotion logic with database-driven rules

### UI Changes
- Add division creation modal
- Add club import modal (CSV + manual entry)
- Add promotion rules editor
- Enhance league management screen with new tabs

## Implementation Phases

### Phase 1: Database Migration
**Goal**: Add `league_id` to all tables without breaking existing data

**Steps**:
1. Create migrations infrastructure (`migrations/mod.rs`)
2. Create migration 001: Add `league_id` columns
3. Populate existing data with default league ID
4. Recreate tables with NOT NULL + foreign key constraints
5. Add indexes for performance

**Critical Files**:
- `src-tauri/src/database/migrations/mod.rs` (NEW)
- `src-tauri/src/database/migrations/001_add_league_id.rs` (NEW)
- `src-tauri/src/database/sheffield_schema.sql` (UPDATE)

**Tables to Modify** (12 total):
- sheffield_league_divisions
- sheffield_league_clubs
- sheffield_league_promotion_rules
- sheffield_league_movement_history
- sheffield_competitions
- sheffield_cup_ties
- sheffield_competition_participants
- sheffield_standings
- sheffield_fixtures
- sheffield_matches
- sheffield_match_incidents
- sheffield_league_config

### Phase 2: League Template System
**Goal**: Make Sheffield 1867 initialization reusable for custom leagues

**Steps**:
1. Create `LeagueTemplate` trait with methods: `divisions()`, `clubs()`, `club_assignments()`, `promotion_rules()`, `populate()`
2. Implement `Sheffield1867Template` extracting hardcoded data from `sheffield_league.rs`
3. Implement `CustomEmptyTemplate` for user-created leagues
4. Create template factory for instantiation
5. Refactor `populate_sheffield_league()` to use template system

**Critical Files**:
- `src-tauri/src/database/league_templates/mod.rs` (NEW)
- `src-tauri/src/database/league_templates/sheffield_1867.rs` (NEW)
- `src-tauri/src/database/league_templates/custom_empty.rs` (NEW)
- `src-tauri/src/database/league_templates/factory.rs` (NEW)
- `src-tauri/src/database/sheffield_league.rs` (UPDATE)

**Data Structures**:
```rust
trait LeagueTemplate {
    fn divisions() -> Vec<DivisionDefinition>;
    fn clubs() -> Vec<ClubDefinition>;
    fn club_assignments() -> Vec<ClubAssignment>;
    fn promotion_rules() -> Vec<PromotionRule>;
    async fn populate(pool, league_id) -> Result<()>;
}
```

### Phase 3: Query Scoping
**Goal**: Ensure all queries are filtered by `league_id`

**Steps**:
1. Create `LeagueScopedQuery` helper struct
2. Implement scoped methods: `get_divisions()`, `get_clubs_in_division()`, `get_standings()`
3. Create `get_active_league_id()` helper
4. Update all existing database functions to use scoped queries
5. Refactor promotion logic to be database-driven instead of hardcoded

**Critical Files**:
- `src-tauri/src/database/query_helpers.rs` (NEW)
- `src-tauri/src/database/sheffield_league.rs` (UPDATE)
- `src-tauri/src/database/promotion_db.rs` (UPDATE - replace match statements with DB queries)
- `src-tauri/src/commands.rs` (UPDATE - add league_id to all queries)

**Pattern**:
```rust
let league_id = get_active_league_id(pool).await?;
let query = LeagueScopedQuery::new(league_id);
let divisions = query.get_divisions(pool).await?;
```

### Phase 4: UI Implementation
**Goal**: Build UI for creating and managing custom leagues

**Components to Create**:
1. **DivisionCreatorModal.tsx** - Form to create divisions (name, level, region, size)
2. **ClubImportModal.tsx** - Import clubs via CSV or manual entry
3. **PromotionRulesEditor.tsx** - Grid interface to configure promotion/relegation rules

**Components to Update**:
4. **LeagueManagementScreen.tsx** - Add buttons for division creation, club import, promotion rules
5. **CreateLeagueModal.tsx** - Add template selection dropdown

**Critical Files**:
- `frontend/src/components/DivisionCreatorModal.tsx` (NEW)
- `frontend/src/components/ClubImportModal.tsx` (NEW)
- `frontend/src/components/PromotionRulesEditor.tsx` (NEW)
- `frontend/src/screens/LeagueManagementScreen.tsx` (UPDATE)
- `frontend/src/components/CreateLeagueModal.tsx` (UPDATE)

**UI Workflow**:
```
Create League → Select Template → Auto-populate OR manual setup
├─ Sheffield 1867 Template → Instant 186 clubs, 28 divisions
└─ Custom Empty → User creates divisions → Imports clubs → Assigns clubs → Configures rules
```

### Phase 5: Backend API
**Goal**: Expose new functionality via Tauri commands

**New Commands**:
1. `db_create_division(league_id, name, level, region, target_size)` → Division
2. `db_delete_division(division_id)` → ()
3. `db_import_clubs_csv(csv_data, league_id)` → Vec<Club>
4. `db_create_clubs_bulk(clubs, league_id)` → Vec<Club>
5. `db_save_promotion_rules(league_id, rules)` → ()
6. `db_get_promotion_rules(league_id)` → Vec<PromotionRule>

**Updated Commands**:
7. `db_create_league()` - Add template parameter
8. `db_import_league_structure()` - Use template system
9. `db_get_all_divisions()` - Filter by active league_id
10. All other division/club queries - Add league scoping

**Critical Files**:
- `src-tauri/src/commands.rs` (MAJOR UPDATE - ~500 lines)
- `src-tauri/src/main.rs` (UPDATE - register new commands)

### Phase 6: Testing
**Goal**: Ensure backward compatibility and new features work

**Test Files to Create**:
1. `src-tauri/src/database/league_templates/tests.rs` - Template population tests
2. `src-tauri/src/database/integration_tests.rs` - End-to-end workflows
3. `src-tauri/src/database/backward_compat_tests.rs` - Sheffield 1867 still works

**Key Tests**:
- Sheffield 1867 template populates 28 divisions, 186 clubs
- Multiple leagues can coexist without data leakage
- Custom league workflow (create → add divisions → import clubs → assign → configure)
- Migration preserves existing Sheffield 1867 data
- Promotion calculations work with custom rules

### Phase 7: Deployment & Migration
**Goal**: Safely migrate existing users

**Deliverables**:
1. **migrate_database.rs** - Standalone migration tool
2. **MIGRATION_GUIDE.md** - User documentation
3. Automatic backup before migration
4. Rollback capability

**Migration Process**:
1. App detects old schema (no league_id columns)
2. Automatic backup: `sheffield.db.backup_v0`
3. Run migration 001
4. Create default "Fantasy 1867" league
5. Assign existing data to default league
6. Verify integrity
7. Launch app with new multi-league UI

## User Workflows

### Create Sheffield 1867 League (Template)
1. Click "Create New League"
2. Select template: "Sheffield 1867"
3. Click "Create League"
4. ✓ Done - 186 clubs, 28 divisions auto-populated

### Create Custom League from Scratch
1. Click "Create New League" → Name: "My League" → Template: "Custom Empty"
2. Click "+ Create Division" → Add divisions (e.g., "Premier Division", level 1)
3. Click "Import Clubs" → Upload CSV or enter manually
4. Drag clubs from "Unassigned" to divisions
5. Click "Configure Promotion Rules" → Add rules (e.g., Div 2 → Div 1, 3 slots)
6. Click "Save All Changes"
7. ✓ League ready to play

## Key Design Decisions

### Decision 1: Table Recreation vs ALTER COLUMN
**Chosen**: Table recreation
**Reason**: Enforces NOT NULL and foreign key constraints, better data integrity

### Decision 2: Shared vs Per-League Clubs Table
**Chosen**: Shared `sheffield_clubs` table
**Reason**: Clubs are historical entities, can be reused across leagues (e.g., Sheffield FC exists in multiple league types)

### Decision 3: Template System vs JSON Config Files
**Chosen**: Template trait with Rust implementations
**Reason**: Type safety, compile-time validation, can include business logic

### Decision 4: Hardcoded vs Database-Driven Promotion Rules
**Chosen**: Database-driven
**Reason**: Enables custom leagues with arbitrary promotion structures

## Backward Compatibility Strategy

### Existing Data Preserved
- All Sheffield 1867 data migrated to default league
- `populate_sheffield_league()` still works (uses template internally)
- Promotion/relegation calculations unchanged for default league
- All existing saves compatible

### Migration Safety
- Automatic backup before migration
- Migration can be rolled back
- Validation checks after migration
- User-friendly error messages

## Critical Success Factors

1. **Migration Must Not Lose Data** - Backup + validation
2. **Sheffield 1867 Must Work Unchanged** - Backward compat tests
3. **Performance** - Indexed league_id columns
4. **Data Isolation** - All queries scoped by league_id
5. **User Experience** - Intuitive UI for custom league creation

## Implementation Order

**Week 1-2**: Phase 1 (Database Migration)
**Week 2-3**: Phase 2 (Template System)
**Week 3-4**: Phase 3 (Query Scoping)
**Week 4-6**: Phase 4 (UI Implementation)
**Week 5-6**: Phase 5 (Backend API)
**Week 6-7**: Phase 6 (Testing)
**Week 7**: Phase 7 (Deployment)

## Risk Mitigation

**Risk**: Data loss during migration
**Mitigation**: Automatic backup, validation checks, rollback capability

**Risk**: Performance degradation
**Mitigation**: Index league_id columns, query optimization

**Risk**: Sheffield 1867 breaks
**Mitigation**: Extensive backward compatibility tests

**Risk**: Data leakage between leagues
**Mitigation**: LeagueScopedQuery enforces isolation, integration tests verify

## User Requirements (Confirmed)

1. **Club Scope**: **Global clubs (reusable across leagues)**
   - Single `sheffield_clubs` table shared across all leagues
   - Same club can appear in multiple leagues simultaneously
   - Assignment is via `sheffield_league_clubs` with `league_id`

2. **Implementation Priority**: **Full feature parity**
   - Custom leagues must support everything Sheffield 1867 has
   - Includes: reserves, cups, promotion playoffs, weather cancellations, etc.
   - Template system must be comprehensive, not MVP

3. **Reserve Teams**: **Optional per league**
   - Add `enable_reserves` boolean to `league_metadata` table
   - Sheffield 1867 has reserves enabled by default
   - Custom leagues can choose whether to enable reserves
   - UI shows/hides reserve options based on setting

4. **Template System**: **Build template marketplace/library**
   - Infrastructure for sharing and downloading community templates
   - Templates stored as JSON files that can be imported/exported
   - Template gallery UI showing available templates
   - Users can publish their own league structures as templates
   - Beyond scope for Phase 1, but architecture must support it

## Updated Architecture for User Requirements

### Global Clubs with League Assignments
```sql
-- Clubs table (GLOBAL - shared across all leagues)
sheffield_clubs (
    id TEXT PRIMARY KEY,
    name TEXT,
    founded_year INTEGER,
    ...
)

-- League assignments (LEAGUE-SPECIFIC)
sheffield_league_clubs (
    id TEXT PRIMARY KEY,
    league_id TEXT NOT NULL,  -- Links to specific league
    club_id TEXT NOT NULL,    -- Links to global club
    division_id TEXT NOT NULL,
    position INTEGER,
    is_reserve_team BOOLEAN,
    ...
    FOREIGN KEY (league_id) REFERENCES league_metadata(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(league_id, club_id)  -- Club can only be in league once
)
```

### Reserve Teams as Optional Feature
```sql
-- League metadata with reserves flag
league_metadata (
    id TEXT PRIMARY KEY,
    name TEXT,
    enable_reserves BOOLEAN DEFAULT 0,  -- NEW FIELD
    ...
)
```

Template logic:
- Sheffield 1867: `enable_reserves = true` → Auto-generates reserve divisions & teams
- Custom leagues: User decides → UI shows reserve options if enabled

### Template Marketplace Architecture
```rust
// Template metadata for marketplace
pub struct TemplateMetadata {
    pub id: String,
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub downloads: i32,
    pub rating: f32,
    pub tags: Vec<String>,  // e.g., ["historical", "england", "1800s"]
    pub preview_image_url: Option<String>,
}

// Template storage format (JSON)
pub struct TemplateExport {
    pub metadata: TemplateMetadata,
    pub divisions: Vec<DivisionDefinition>,
    pub clubs: Vec<ClubDefinition>,
    pub assignments: Vec<ClubAssignment>,
    pub promotion_rules: Vec<PromotionRule>,
    pub league_config: LeagueConfig,
    pub enable_reserves: bool,
    pub cups: Vec<CupDefinition>,  // Cup competitions
}
```

Template marketplace workflow:
1. User creates custom league
2. Clicks "Export as Template"
3. Fills in metadata (name, description, tags)
4. Template saved as JSON file
5. (Future) Upload to community marketplace
6. Other users can "Import Template" → Browse gallery → One-click install

## Additional Implementation Phases

### Phase 8: Template Marketplace (Future)
**Goal**: Enable community template sharing

**Features**:
1. **Template Export**:
   - Export current league structure to JSON
   - Include metadata (author, description, tags)
   - Validate template before export

2. **Template Import**:
   - Browse local templates folder
   - Import JSON template file
   - Preview template before creating league

3. **Template Gallery UI**:
   - Grid view of available templates
   - Search/filter by tags
   - Template details modal with preview
   - One-click "Create League from Template"

4. **Template Validation**:
   - Schema validation for JSON files
   - Check for circular promotion rules
   - Verify division level consistency
   - Validate club count matches division sizes

**Files (Phase 8)**:
- `src-tauri/src/database/template_export.rs` (NEW)
- `src-tauri/src/database/template_import.rs` (NEW)
- `src-tauri/src/database/template_validator.rs` (NEW)
- `frontend/src/components/TemplateGallery.tsx` (NEW)
- `frontend/src/components/TemplateExporter.tsx` (NEW)

### Phase 9: Full Feature Parity (Cups, Playoffs, etc.)
**Goal**: Ensure custom leagues support all Sheffield 1867 features

**Features to Make League-Aware**:
1. **Cup Competitions**:
   - Add `league_id` to `sheffield_competitions`
   - Custom cups per league
   - Cup eligibility based on division levels
   - Already partially scoped, needs full integration

2. **Promotion Playoffs**:
   - Configure playoff format per league
   - Single-leg vs two-leg vs neutral venue
   - Number of playoff teams per division
   - Already in `league_config`, needs implementation

3. **Weather Cancellations**:
   - League-specific cancellation thresholds
   - Rearrangement windows per league
   - Already in `league_config`, works automatically

4. **Reserve Team Management**:
   - Optional reserve teams (based on `enable_reserves`)
   - Auto-generate reserve divisions if enabled
   - Mirror structure of main divisions
   - Reserve naming conventions (e.g., "Reserve Division 1")

**Updated League Config**:
```rust
pub struct LeagueConfig {
    // ... existing fields ...

    // Reserve teams (NEW)
    pub enable_reserves: bool,
    pub reserve_division_prefix: String,  // e.g., "Reserve "

    // Cups (ENHANCED)
    pub enable_cups: bool,
    pub default_cup_prestige: String,

    // Custom fields per league (NEW)
    pub custom_rules_json: Option<String>,  // For future extensibility
}
```
