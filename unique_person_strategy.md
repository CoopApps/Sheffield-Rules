# Person Uniqueness Strategy

## The Problem
Multiple people can have the same name (e.g., 5x "John Smith" born in 1853).
We MUST ensure each person gets their correct:
- Address
- Profession
- Household members
- Parish information

## What Makes a Person Unique?

### From Genealogy Data:
1. **Name** - full name
2. **Birth Year** - when they were born
3. **Address** - specific street address
4. **Parish** - civil parish (Nether Hallam, Sheffield, etc.)
5. **Area** - sub-district if available
6. **Relation** - Head, Wife, Son, Daughter, Lodger, etc.
7. **Spouse** - spouse name (for married people)

### From Sheffield Census Data:
1. **Name** - full name (column 15 "Name")
2. **Birth Year** - ESTIMATED BIRTH YEAR column
3. **Census Location** - PIECE + FOLIO + PAGE (exact census book location)
4. **Household Schedule Number** - household within that page
5. **Civil Parish** - CIVIL PARISH column
6. **Ecclesiastical Parish** - ECCLESIASTICAL PARISH column
7. **Household Members** - full household list
8. **Relation** - RELATION column

## Proposed Unique Identifier

### For 18-year-olds (1853 - genealogy only):
```
UNIQUE KEY = name + birth_year + address + parish + relation
```
Example:
- "John Smith" + 1853 + "15,RockinghamStreet" + "Sheffield" + "Son" = Person A
- "John Smith" + 1853 + "42,BrownLane" + "Nether Hallam" + "Son" = Person B

### For older people (with both sources):
```
UNIQUE KEY = name + birth_year + census_piece + census_folio + census_page + household_schedule
```
This is the MOST precise - exact census location.

## Matching Strategy (for years with both sources)

### Step 1: Match by exact census location
```
Sheffield Census Person A:
  Name: "William Ogden"
  Birth: 1838
  Piece: 4661, Folio: 11, Page: 15
  Schedule: 86
  Address: (not in sheffield census)
  Profession: (not in sheffield census)

Genealogy Person A:
  Name: "William Ogden"
  Birth: 1838
  Address: "28,ClarkStreet"
  Profession: "Scale Maker"
  Spouse: "Eliza Cartledge"

MATCH if:
  - Same name ✓
  - Same birth year ✓
  - Address is in same parish/area (fuzzy match)

→ Merge into ONE person with ALL data
```

### Step 2: Handle duplicates
If TWO "William Ogden"s born 1838:

**Option A: Both in census (different households)**
```
Census William #1: Piece 4661, Folio 11, Page 15, Schedule 86, Parish "Nether Hallam"
Census William #2: Piece 4661, Folio 20, Page 30, Schedule 120, Parish "Sheffield"

Genealogy William #1: Address "28,ClarkStreet", Parish "Ecclesall Bierlow"
Genealogy William #2: Address "50,BrownLane", Parish "Sheffield"

Match by parish similarity:
- Census #1 (Nether Hallam) → Genealogy #1 (Ecclesall Bierlow) - adjacent parishes
- Census #2 (Sheffield) → Genealogy #2 (Sheffield) - exact match
```

**Option B: One in census, one only in genealogy**
```
Census William: Piece 4661, Folio 11, Schedule 86
Genealogy William #1: Address "28,ClarkStreet", Parish "Ecclesall Bierlow", Profession "Scale Maker"
Genealogy William #2: Address "50,BrownLane", Parish "Sheffield", Profession "File Cutter"

→ Import Census William + matched Genealogy William as ONE person
→ Import unmatched Genealogy William as SEPARATE person with note "genealogy_only"
```

### Step 3: Household linking
For families:
```
Sheffield Census household (Schedule 86):
  Household Members: "William Ogden 33 | Lyda Ogden 29 | John W Ogden 7 | Emily Ogden 4"

Genealogy matches:
  - William Ogden (Head, age 33, 1838)
  - Lyda Ogden (Wife, age 29, 1842, Spouse="William Ogden")
  - John W Ogden (Son, age 7, 1864)
  - Emily Ogden (Daughter, age 4, 1867)

→ Create household_id
→ Link all 4 people to same household
→ William is household head
→ Lyda's spouse links to William's person_id
```

## Database Schema

```sql
CREATE TABLE sheffield_people (
  id TEXT PRIMARY KEY,  -- UUID

  -- Basic info
  name TEXT NOT NULL,
  first_name TEXT,
  middle_name TEXT,
  surname TEXT,
  birth_year INTEGER NOT NULL,
  gender TEXT,

  -- Location
  address TEXT,
  civil_parish TEXT,
  ecclesiastical_parish TEXT,
  registration_district TEXT,
  sub_registration_district TEXT,
  birth_place TEXT,

  -- Census reference (if from census)
  census_piece TEXT,
  census_folio TEXT,
  census_page TEXT,
  census_schedule_number TEXT,
  census_ed TEXT,

  -- Personal
  profession TEXT,
  relation TEXT,  -- Head, Wife, Son, etc.

  -- Household
  household_id TEXT,  -- Links to same household
  spouse_name TEXT,
  spouse_id TEXT,  -- Foreign key to another person
  household_members TEXT,  -- Full household list from census

  -- Source tracking
  source TEXT,  -- 'census', 'genealogy', 'census+genealogy'

  -- Unique constraint
  UNIQUE(name, birth_year, census_piece, census_folio, census_page, census_schedule_number)
)
```

## Import Process

1. **For 1853 (genealogy only)**:
   - Import all 22 files
   - Use name + birth_year + address + parish as unique key
   - No household matching needed (they're all kids)
   - Mark source='genealogy'

2. **For 1838 (both sources)**:
   - Load Sheffield Census 1838.csv → extract all people
   - Load all Genealogy tg_1838_*.csv → extract all people
   - Match by name + birth_year + parish similarity
   - Merge matched records
   - Create household links
   - Keep unmatched as separate records

3. **For ALL years**:
   - Process year by year
   - Build cross-references as we go
   - Final verification pass
