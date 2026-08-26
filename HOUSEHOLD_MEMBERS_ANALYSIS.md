# Household Members Column Analysis

## Database: Sheffield1867.db
**Location:** `D:/projects/Saturday at Three/Sheffield1867.db`

---

## Summary

**FINDING:** The census CSV files contain a **"HOUSEHOLD MEMBERS"** column with actual family member names and ages, but this data **IS NOT** imported into the database.

---

## What Exists in the Database

### Table: `sheffield_players`

The database schema includes these **census-related columns**:

| Column Name | Type | Description |
|-------------|------|-------------|
| `census_household_schedule` | TEXT | **Household schedule number** (links family members together) |
| `ecclesiastical_parish` | TEXT | Church parish (used with schedule number to identify household) |
| `census_age` | INTEGER | Age at time of census |
| `census_relation` | TEXT | Relation to head of household (Head, Wife, Son, Daughter, etc.) |
| `census_gender` | TEXT | Male/Female |
| `census_ed` | TEXT | Enumeration District number |
| `census_piece` | TEXT | Piece reference number |
| `census_folio` | TEXT | Folio number |
| `census_page` | TEXT | Page number |

### Key Point: No Household Members Data Column

There is **NO column** in the database that stores the actual household member names/data as text. The columns that might contain this do NOT exist:
- ❌ `household_members`
- ❌ `household_data`
- ❌ `family_members`
- ❌ `census_household_members`

---

## What Exists in the Census CSV Files

### CSV Column 12: "HOUSEHOLD MEMBERS"

The census CSV files (e.g., `1804.csv`) contain a column with **actual household member names and ages**.

**Example from 1804.csv:**
```
HOUSEHOLD MEMBERS: "Name	Age | Mary Martin	68 | Thomas Martin	67 | George Martin	32 | Thomas Martin	19 | William Martin	15"
```

**Format:**
```
Name[TAB]Age | PersonName[TAB]Age | PersonName[TAB]Age | ...
```

**Real Examples:**

1. **Thomas Martin's Household (Schedule 41):**
   ```
   Name	Age | Mary Martin	68 | Thomas Martin	67 | George Martin	32 | Thomas Martin	19 | William Martin	15
   ```

2. **Henry Ashton's Household (Schedule 179):**
   ```
   Name	Age | Henry Ashton	67 | Sarah Ashton	37 | Larena Ashton	12 | Lilly Ashton	4 | John H Ashton	2
   ```

3. **Samuel Beckett's Household (Schedule 6):**
   ```
   Name	Age | Harriet Beckett	68 | Samuel Beckett	67 | Emma Beckett	31
   ```

---

## How the Import Currently Works

### Rust Census Importer

**File:** `src-tauri/src/database/census_importer.rs`

The importer reads these columns from the CSV:
```rust
let i_age = idx("AGE");
let i_birth_place = idx("Birth Place");
let i_civil_parish = idx("CIVIL PARISH");
// ... etc ...
let i_household = idx("HOUSEHOLD SCHEDULE NUMBER");
// ... etc ...
```

**Notice:** There is **NO** line like:
```rust
let i_household_members = idx("HOUSEHOLD MEMBERS"); // ❌ NOT IMPLEMENTED
```

The importer processes each **individual row** as a separate player record. It does NOT parse or store the "HOUSEHOLD MEMBERS" column.

---

## How to Get Household Members (Current Method)

Since individual census records ARE imported, you can reconstruct households by querying:

### SQL Query to Find Household Members:

```sql
SELECT
  first_name,
  surname,
  birth_year,
  census_age,
  census_relation,
  census_gender
FROM sheffield_players
WHERE census_household_schedule = '41'  -- The household number
  AND ecclesiastical_parish = 'St Philip'  -- The parish
ORDER BY
  CASE census_relation
    WHEN 'Head' THEN 1
    WHEN 'Wife' THEN 2
    WHEN 'Son' THEN 3
    WHEN 'Daughter' THEN 4
    ELSE 5
  END,
  census_age DESC;
```

### Example Result for Thomas Martin (Schedule 41, St Philip):

| Name | Age | Relation | Gender |
|------|-----|----------|--------|
| Thomas Martin | 67 | Head | Male |
| Mary Martin | 68 | Wife | Female |
| George Martin | 32 | Son | Male |
| Thomas Martin | 19 | Son | Male |
| William Martin | 15 | Son | Male |

---

## Key Differences

| Feature | CSV "HOUSEHOLD MEMBERS" Column | Database Reconstruction |
|---------|-------------------------------|-------------------------|
| **Storage** | Raw text with all names | Individual player records |
| **Format** | "Name[TAB]Age \| Name[TAB]Age" | Normalized relational data |
| **Access** | Parse text string | SQL JOIN query |
| **Data** | Complete household summary | Same data, different structure |

---

## Why CSV Data Isn't Imported

The "HOUSEHOLD MEMBERS" column in the CSV is **redundant** because:

1. Each household member has their **own row** in the CSV
2. All share the same `census_household_schedule` number
3. All share the same `ecclesiastical_parish`
4. Each has their own `census_relation` (Head, Wife, Son, etc.)

**Example:** Thomas Martin's household appears as:
- Row 1: Thomas Martin (Head, Schedule 41)
- Row 2: Mary Martin (Wife, Schedule 41)
- Row 3: George Martin (Son, Schedule 41)
- Row 4: Thomas Martin Jr (Son, Schedule 41)
- Row 5: William Martin (Son, Schedule 41)

The importer treats each row as a separate player, storing them individually.

---

## How to Query Household Data

### Method 1: Find All Household Members for a Player

```javascript
// Get household members for a specific player
const player = db.prepare(`
  SELECT census_household_schedule, ecclesiastical_parish
  FROM sheffield_players
  WHERE id = ?
`).get(playerId);

const household = db.prepare(`
  SELECT
    id,
    first_name,
    surname,
    birth_year,
    census_age,
    census_relation,
    census_gender
  FROM sheffield_players
  WHERE census_household_schedule = ?
    AND ecclesiastical_parish = ?
  ORDER BY
    CASE census_relation
      WHEN 'Head' THEN 1
      WHEN 'Wife' THEN 2
      WHEN 'Son' THEN 3
      WHEN 'Daughter' THEN 4
      ELSE 5
    END,
    census_age DESC
`).all(player.census_household_schedule, player.ecclesiastical_parish);
```

### Method 2: Find Households with Multiple Members

```sql
SELECT
  census_household_schedule,
  ecclesiastical_parish,
  COUNT(*) as member_count
FROM sheffield_players
WHERE census_household_schedule IS NOT NULL
GROUP BY census_household_schedule, ecclesiastical_parish
HAVING COUNT(*) > 1
ORDER BY member_count DESC;
```

### Method 3: Find Family Relationships

```sql
-- Find all fathers with their sons
SELECT
  h.first_name as father_first,
  h.surname as father_surname,
  h.birth_year as father_birth_year,
  s.first_name as son_first,
  s.surname as son_surname,
  s.birth_year as son_birth_year
FROM sheffield_players h
JOIN sheffield_players s ON
  h.census_household_schedule = s.census_household_schedule
  AND h.ecclesiastical_parish = s.ecclesiastical_parish
WHERE h.census_relation = 'Head'
  AND s.census_relation = 'Son'
  AND h.census_gender = 'Male';
```

---

## What the "HOUSEHOLD MEMBERS" CSV Column Contains

The CSV column appears to be a **human-readable summary** of all household members, possibly for:
- Quick visual reference
- Data validation
- Original census transcription format

**Format Analysis:**
```
Column Header: "HOUSEHOLD MEMBERS"
Format: "Name	Age | PersonName	Age | PersonName	Age | ..."
Separator: " | " (space-pipe-space)
Name-Age Link: TAB character
```

**Note:** Some rows contain just "1" which appears to be:
- Single-person households, OR
- Data entry placeholder, OR
- Original census notation

---

## Recommendation

### If You Need Household Member Names as Text:

You could add a new column and populate it from existing data:

```sql
-- Add column
ALTER TABLE sheffield_players ADD COLUMN household_members_text TEXT;

-- Populate it with a query that builds the text
-- (Would need to be done via script, not pure SQL)
```

**Script example (Node.js):**
```javascript
const households = db.prepare(`
  SELECT DISTINCT census_household_schedule, ecclesiastical_parish
  FROM sheffield_players
  WHERE census_household_schedule IS NOT NULL
`).all();

for (const household of households) {
  const members = db.prepare(`
    SELECT first_name, surname, census_age
    FROM sheffield_players
    WHERE census_household_schedule = ?
      AND ecclesiastical_parish = ?
  `).all(household.census_household_schedule, household.ecclesiastical_parish);

  const text = members
    .map(m => `${m.first_name} ${m.surname}\t${m.census_age}`)
    .join(' | ');

  db.prepare(`
    UPDATE sheffield_players
    SET household_members_text = ?
    WHERE census_household_schedule = ?
      AND ecclesiastical_parish = ?
  `).run(text, household.census_household_schedule, household.ecclesiastical_parish);
}
```

---

## Files Reference

### Census CSV Files
- **Location:** `D:/projects/Saturday at Three/Sheffield Census/`
- **Count:** 73 files (birth years 1772-1844)
- **Format:** 22 columns, column 12 is "HOUSEHOLD MEMBERS"

### Database Schema
- **File:** `src-tauri/src/database/sheffield_schema.sql`
- **Table:** `sheffield_players` (lines 17-158)

### Census Importer
- **File:** `src-tauri/src/database/census_importer.rs`
- **Columns Imported:** 21 of 22 (excludes "HOUSEHOLD MEMBERS")

### Query Scripts
- `query_household_members.cjs` - Queries household data
- `analyze_henry_abel.cjs` - Example household analysis
- `check_schema.cjs` - Shows table schema

---

## Conclusion

**The "HOUSEHOLD MEMBERS" column from the census CSV files is NOT stored in the database.**

However, you can **reconstruct the exact same information** by querying all players who share the same:
- `census_household_schedule` number
- `ecclesiastical_parish` value

The database stores the data in **normalized relational form** (one row per person) rather than as **denormalized text summaries** (all names in one field).

This is actually **better database design** as it:
- Avoids data duplication
- Allows proper querying and filtering
- Maintains referential integrity
- Enables relationship analysis

If you need the text format for display purposes, you can generate it on-the-fly with a query, or add a computed column.
