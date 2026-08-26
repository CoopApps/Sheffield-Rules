-- Link unique name + numbered address matches from sheffield_businesses to sheffield_people
-- Only updates where there is exactly 1 person match

-- First, create a temp table with the unique matches
CREATE TEMP TABLE unique_matches AS
SELECT
    b.id as business_id,
    p.id as person_id
FROM sheffield_businesses b
JOIN sheffield_people p ON
    b.name = p.name
    AND LOWER(REPLACE(b.street_address, ' ', '')) = LOWER(REPLACE(p.street_address, ' ', ''))
WHERE b.street_address GLOB '*[0-9]*'
  AND p.street_address GLOB '*[0-9]*'
GROUP BY b.id
HAVING COUNT(p.id) = 1;

-- Show how many unique matches found
SELECT 'Unique matches found:' as info, COUNT(*) as count FROM unique_matches;

-- Update the businesses table
UPDATE sheffield_businesses
SET matched_to_genealogy_id = (
    SELECT person_id FROM unique_matches WHERE business_id = sheffield_businesses.id
)
WHERE id IN (SELECT business_id FROM unique_matches);

-- Show results
SELECT 'Businesses now linked:' as info, COUNT(*) as count
FROM sheffield_businesses
WHERE matched_to_genealogy_id IS NOT NULL;

-- Show some examples
SELECT
    b.id as business_id,
    b.name,
    b.street_address,
    b.profession,
    b.matched_to_genealogy_id,
    p.profession as person_profession
FROM sheffield_businesses b
JOIN sheffield_people p ON b.matched_to_genealogy_id = p.id
LIMIT 10;

-- Clean up
DROP TABLE unique_matches;
