-- Create spouse matching based on household members list
-- Only matches when the spouse name is UNIQUE in the database (exactly 1 person with that name)

-- First, add spouse_person_id column to sheffield_people
ALTER TABLE sheffield_people ADD COLUMN spouse_person_id INTEGER;

-- Create a temporary table of unique names (only 1 person with this exact name)
CREATE TEMP TABLE unique_names AS
SELECT name, rowid as person_id
FROM sheffield_people
WHERE name IS NOT NULL AND name != ''
GROUP BY name
HAVING COUNT(*) = 1;

-- Create index for faster lookups
CREATE INDEX idx_temp_unique_names ON unique_names(name);

-- Update spouse_person_id where:
-- 1. Person A's census_household_schedule matches Person B's census_household_schedule
-- 2. Person B's name is unique in the database
-- 3. Person A's census_relation is "Wife" or "Husband" or contains "Wife" or "Husband"
-- Note: We're using household schedule as the grouping, even though it's imperfect

UPDATE sheffield_people
SET spouse_person_id = (
    SELECT un.person_id
    FROM sheffield_people sp2
    JOIN unique_names un ON sp2.name = un.name
    WHERE sp2.rowid != sheffield_people.rowid
      AND sp2.census_household_schedule = sheffield_people.census_household_schedule
      AND sp2.census_household_schedule IS NOT NULL
      AND sp2.census_household_schedule != ''
      AND (
          (sheffield_people.census_relation LIKE '%Wife%' AND sp2.census_relation LIKE '%Husband%')
          OR (sheffield_people.census_relation LIKE '%Husband%' AND sp2.census_relation LIKE '%Wife%')
          OR (sheffield_people.census_relation LIKE '%wife%' AND sp2.census_relation LIKE '%husband%')
          OR (sheffield_people.census_relation LIKE '%husband%' AND sp2.census_relation LIKE '%wife%')
          OR (sheffield_people.census_relation LIKE '%Head%' AND sp2.census_relation LIKE '%Wife%')
          OR (sheffield_people.census_relation LIKE '%Wife%' AND sp2.census_relation LIKE '%Head%')
      )
    LIMIT 1
)
WHERE spouse_person_id IS NULL
  AND census_household_schedule IS NOT NULL
  AND census_household_schedule != ''
  AND (census_relation LIKE '%Wife%' OR census_relation LIKE '%wife%'
       OR census_relation LIKE '%Husband%' OR census_relation LIKE '%husband%'
       OR census_relation LIKE '%Head%');

-- Create index on spouse_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_people_spouse ON sheffield_people(spouse_person_id);

-- Report results
SELECT 'Total unique names in database:' as label, COUNT(*) as count FROM unique_names;

SELECT 'People with spouse matched:' as label, COUNT(*) as count
FROM sheffield_people
WHERE spouse_person_id IS NOT NULL;

SELECT 'Married couples identified (spouse_person_id / 2):' as label, COUNT(*) / 2 as count
FROM sheffield_people
WHERE spouse_person_id IS NOT NULL;

-- Show examples of matched couples
SELECT 'Example matched couples (first 10):' as label;
SELECT
    p1.name as person_name,
    p1.census_age as person_age,
    p1.census_gender as person_gender,
    p1.census_relation as person_relation,
    p2.name as spouse_name,
    p2.census_age as spouse_age,
    p2.census_gender as spouse_gender,
    p2.census_relation as spouse_relation,
    p1.street_address
FROM sheffield_people p1
JOIN sheffield_people p2 ON p1.spouse_person_id = p2.rowid
WHERE p1.spouse_person_id IS NOT NULL
LIMIT 10;

-- Show footballers with spouses
SELECT 'Footballers with spouses identified:' as label, COUNT(*) as count
FROM sheffield_footballers f
JOIN sheffield_people p ON f.sheffield_person_id = p.rowid
WHERE p.spouse_person_id IS NOT NULL;

-- Clean up temp table
DROP TABLE unique_names;
