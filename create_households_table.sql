-- Create households table based on census_household_schedule
-- This is actual census household data, more accurate than surname matching

CREATE TABLE IF NOT EXISTS sheffield_households (
    household_id INTEGER PRIMARY KEY AUTOINCREMENT,
    census_household_schedule TEXT UNIQUE NOT NULL,
    street_address TEXT,
    postcode TEXT,
    household_size INTEGER,
    males_total INTEGER,
    females_total INTEGER,
    footballers_count INTEGER,
    has_head INTEGER DEFAULT 0,
    head_person_id INTEGER,  -- sheffield_person_id of household head
    head_name TEXT,
    head_profession TEXT,
    is_employer_household INTEGER DEFAULT 0,
    is_publican_household INTEGER DEFAULT 0,
    is_lodging_house INTEGER DEFAULT 0,  -- 10+ lodgers/boarders
    avg_age REAL,
    household_type TEXT  -- 'family', 'lodging_house', 'single', 'institution'
);

-- Populate households
INSERT INTO sheffield_households (
    census_household_schedule,
    street_address,
    postcode,
    household_size,
    males_total,
    females_total,
    footballers_count,
    has_head,
    head_person_id,
    head_name,
    head_profession,
    avg_age
)
SELECT
    p.census_household_schedule,
    MAX(p.street_address) as street_address,
    MAX(p.postcode) as postcode,
    COUNT(*) as household_size,
    SUM(CASE WHEN p.census_gender IN ('M', 'Male', 'male') THEN 1 ELSE 0 END) as males_total,
    SUM(CASE WHEN p.census_gender IN ('F', 'Female', 'female') THEN 1 ELSE 0 END) as females_total,
    SUM(CASE WHEN f.footballer_id IS NOT NULL THEN 1 ELSE 0 END) as footballers_count,
    MAX(CASE WHEN p.census_relation LIKE '%Head%' THEN 1 ELSE 0 END) as has_head,
    MAX(CASE WHEN p.census_relation LIKE '%Head%' THEN p.rowid ELSE NULL END) as head_person_id,
    MAX(CASE WHEN p.census_relation LIKE '%Head%' THEN p.name ELSE NULL END) as head_name,
    MAX(CASE WHEN p.census_relation LIKE '%Head%' THEN p.profession ELSE NULL END) as head_profession,
    AVG(p.census_age) as avg_age
FROM sheffield_people p
LEFT JOIN sheffield_footballers f ON p.rowid = f.sheffield_person_id
WHERE p.census_household_schedule IS NOT NULL
  AND p.census_household_schedule != ''
GROUP BY p.census_household_schedule;

-- Mark employer households
UPDATE sheffield_households
SET is_employer_household = 1
WHERE head_person_id IN (SELECT sheffield_person_id FROM sheffield_employers);

-- Mark publican households
UPDATE sheffield_households
SET is_publican_household = 1
WHERE head_person_id IN (SELECT sheffield_person_id FROM sheffield_publicans);

-- Mark lodging houses (10+ people, high lodger count)
UPDATE sheffield_households
SET is_lodging_house = 1
WHERE household_size >= 10
  AND census_household_schedule IN (
      SELECT census_household_schedule
      FROM sheffield_people
      WHERE census_relation LIKE '%Lodger%' OR census_relation LIKE '%Boarder%'
      GROUP BY census_household_schedule
      HAVING COUNT(*) >= 5
  );

-- Classify household types
UPDATE sheffield_households
SET household_type = CASE
    WHEN is_lodging_house = 1 THEN 'lodging_house'
    WHEN household_size = 1 THEN 'single'
    WHEN household_size >= 20 THEN 'institution'
    WHEN has_head = 1 THEN 'family'
    ELSE 'other'
END;

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_household_schedule ON sheffield_households(census_household_schedule);
CREATE INDEX IF NOT EXISTS idx_household_footballers ON sheffield_households(footballers_count);
CREATE INDEX IF NOT EXISTS idx_household_type ON sheffield_households(household_type);

-- Create household members table linking people to households
CREATE TABLE IF NOT EXISTS sheffield_household_members (
    member_id INTEGER PRIMARY KEY AUTOINCREMENT,
    household_id INTEGER NOT NULL,
    sheffield_person_id INTEGER NOT NULL,
    census_household_schedule TEXT,
    name TEXT,
    census_age INTEGER,
    census_gender TEXT,
    census_relation TEXT,
    profession TEXT,
    is_footballer INTEGER DEFAULT 0,
    is_head INTEGER DEFAULT 0,
    FOREIGN KEY (household_id) REFERENCES sheffield_households(household_id),
    FOREIGN KEY (sheffield_person_id) REFERENCES sheffield_people(rowid)
);

-- Populate household members
INSERT INTO sheffield_household_members (
    household_id,
    sheffield_person_id,
    census_household_schedule,
    name,
    census_age,
    census_gender,
    census_relation,
    profession,
    is_footballer,
    is_head
)
SELECT
    h.household_id,
    p.rowid,
    p.census_household_schedule,
    p.name,
    p.census_age,
    p.census_gender,
    p.census_relation,
    p.profession,
    CASE WHEN f.footballer_id IS NOT NULL THEN 1 ELSE 0 END as is_footballer,
    CASE WHEN p.census_relation LIKE '%Head%' THEN 1 ELSE 0 END as is_head
FROM sheffield_people p
JOIN sheffield_households h ON p.census_household_schedule = h.census_household_schedule
LEFT JOIN sheffield_footballers f ON p.rowid = f.sheffield_person_id
WHERE p.census_household_schedule IS NOT NULL
  AND p.census_household_schedule != '';

CREATE INDEX IF NOT EXISTS idx_member_household ON sheffield_household_members(household_id);
CREATE INDEX IF NOT EXISTS idx_member_person ON sheffield_household_members(sheffield_person_id);
CREATE INDEX IF NOT EXISTS idx_member_footballer ON sheffield_household_members(is_footballer);

-- Report results
SELECT 'Total households:' as label, COUNT(*) as count FROM sheffield_households;

SELECT 'Household type breakdown:' as label;
SELECT household_type, COUNT(*) as count
FROM sheffield_households
GROUP BY household_type
ORDER BY count DESC;

SELECT 'Households with footballers:' as label,
       COUNT(*) as count
FROM sheffield_households
WHERE footballers_count > 0;

SELECT 'Households with 2+ footballers (brothers/sons):' as label,
       COUNT(*) as count
FROM sheffield_households
WHERE footballers_count >= 2;

SELECT 'Households with 3+ footballers:' as label,
       COUNT(*) as count
FROM sheffield_households
WHERE footballers_count >= 3;

SELECT 'Employer households:' as label, COUNT(*) as count
FROM sheffield_households
WHERE is_employer_household = 1;

SELECT 'Publican households:' as label, COUNT(*) as count
FROM sheffield_households
WHERE is_publican_household = 1;

SELECT 'Lodging houses:' as label, COUNT(*) as count
FROM sheffield_households
WHERE is_lodging_house = 1;

-- Show top 10 households with most footballers
SELECT 'Top 10 households with most footballers:' as label;
SELECT
    head_name,
    head_profession,
    street_address,
    household_size,
    footballers_count,
    household_type
FROM sheffield_households
WHERE footballers_count > 0
ORDER BY footballers_count DESC
LIMIT 10;
