-- Create family relationships table
-- Identifies potential brothers/family members for game engine to use
-- Based on: same surname + same address + appropriate age gap

CREATE TABLE IF NOT EXISTS sheffield_family_groups (
    family_group_id INTEGER PRIMARY KEY AUTOINCREMENT,
    surname TEXT NOT NULL,
    street_address TEXT NOT NULL,
    member_count INTEGER,
    males_14_40 INTEGER,  -- Potential footballers
    average_age REAL,
    is_large_family INTEGER DEFAULT 0  -- 1 if 5+ members
);

-- Populate family groups (same surname + address)
INSERT INTO sheffield_family_groups (
    surname,
    street_address,
    member_count,
    males_14_40,
    average_age,
    is_large_family
)
SELECT
    surname,
    street_address,
    COUNT(*) as member_count,
    SUM(CASE
        WHEN (census_gender = 'M' OR census_gender = 'Male' OR census_gender = 'male')
        AND census_age BETWEEN 14 AND 40
        THEN 1 ELSE 0
    END) as males_14_40,
    AVG(census_age) as average_age,
    CASE WHEN COUNT(*) >= 5 THEN 1 ELSE 0 END as is_large_family
FROM sheffield_people
WHERE surname IS NOT NULL
  AND street_address IS NOT NULL
  AND surname != ''
  AND street_address != ''
  AND street_address != 'Sheffield Yorkshire England'
  AND street_address != 'Sheffield'
GROUP BY surname, street_address
HAVING member_count >= 2;  -- At least 2 people with same surname at same address

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_family_surname ON sheffield_family_groups(surname);
CREATE INDEX IF NOT EXISTS idx_family_address ON sheffield_family_groups(street_address);
CREATE INDEX IF NOT EXISTS idx_family_footballers ON sheffield_family_groups(males_14_40);

-- Create detailed family members table linking back to people
CREATE TABLE IF NOT EXISTS sheffield_family_members (
    member_id INTEGER PRIMARY KEY AUTOINCREMENT,
    family_group_id INTEGER NOT NULL,
    sheffield_person_id INTEGER NOT NULL,
    name TEXT,
    first_name TEXT,
    census_age INTEGER,
    census_gender TEXT,
    census_relation TEXT,  -- Head, Son, Brother, etc.
    profession TEXT,
    is_footballer INTEGER DEFAULT 0,
    FOREIGN KEY (family_group_id) REFERENCES sheffield_family_groups(family_group_id),
    FOREIGN KEY (sheffield_person_id) REFERENCES sheffield_people(rowid)
);

-- Populate family members
INSERT INTO sheffield_family_members (
    family_group_id,
    sheffield_person_id,
    name,
    first_name,
    census_age,
    census_gender,
    census_relation,
    profession,
    is_footballer
)
SELECT
    fg.family_group_id,
    p.rowid,
    p.name,
    p.first_name,
    p.census_age,
    p.census_gender,
    p.census_relation,
    p.profession,
    CASE WHEN f.footballer_id IS NOT NULL THEN 1 ELSE 0 END as is_footballer
FROM sheffield_family_groups fg
JOIN sheffield_people p
    ON p.surname = fg.surname
    AND p.street_address = fg.street_address
LEFT JOIN sheffield_footballers f
    ON f.sheffield_person_id = p.rowid;

-- Create indexes on family members
CREATE INDEX IF NOT EXISTS idx_member_family_group ON sheffield_family_members(family_group_id);
CREATE INDEX IF NOT EXISTS idx_member_person ON sheffield_family_members(sheffield_person_id);
CREATE INDEX IF NOT EXISTS idx_member_footballer ON sheffield_family_members(is_footballer);

-- Report results
SELECT 'Total family groups (2+ same surname at address):' as label,
       COUNT(*) as count
FROM sheffield_family_groups;

SELECT 'Family groups with 2+ male footballers (brothers):' as label,
       COUNT(*) as count
FROM sheffield_family_groups
WHERE males_14_40 >= 2;

SELECT 'Large families (5+ members):' as label,
       COUNT(*) as count
FROM sheffield_family_groups
WHERE is_large_family = 1;

SELECT 'Top 10 largest families:' as label;
SELECT
    surname,
    street_address,
    member_count,
    males_14_40 as potential_footballers
FROM sheffield_family_groups
ORDER BY member_count DESC
LIMIT 10;

SELECT 'Families with most potential footballers:' as label;
SELECT
    surname,
    street_address,
    member_count as total_family,
    males_14_40 as footballers
FROM sheffield_family_groups
WHERE males_14_40 > 0
ORDER BY males_14_40 DESC
LIMIT 10;

-- Show example of a family with multiple footballers
SELECT 'Example: Smith family with footballers' as label;
SELECT
    fm.name,
    fm.census_age,
    fm.census_relation,
    fm.profession,
    CASE WHEN fm.is_footballer = 1 THEN 'YES' ELSE 'no' END as is_footballer
FROM sheffield_family_members fm
JOIN sheffield_family_groups fg ON fm.family_group_id = fg.family_group_id
WHERE fg.surname = 'Smith'
  AND fg.males_14_40 >= 2
LIMIT 1 OFFSET 0;
