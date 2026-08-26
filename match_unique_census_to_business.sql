-- Match unique census names to unique business names
-- This will update sheffield_businesses.matched_to_genealogy_id
-- for the 1,437 unique name matches

-- First, create temp tables for unique names
CREATE TEMP TABLE unique_census_names AS
SELECT name, COUNT(*) as count
FROM unmatched_genealogy
WHERE (genealogy_source IS NULL OR genealogy_source = '')
  AND name IS NOT NULL
GROUP BY name
HAVING COUNT(*) = 1;

CREATE TEMP TABLE unique_business_names AS
SELECT name, COUNT(*) as count
FROM sheffield_businesses
WHERE (matched_to_genealogy_id IS NULL OR matched_to_genealogy_id = 0)
  AND name IS NOT NULL
GROUP BY name
HAVING COUNT(*) = 1;

-- Update businesses to link to genealogy records
UPDATE sheffield_businesses
SET matched_to_genealogy_id = (
    SELECT g.id
    FROM unmatched_genealogy g
    INNER JOIN unique_census_names uc ON g.name = uc.name
    WHERE g.name = sheffield_businesses.name
      AND (g.genealogy_source IS NULL OR g.genealogy_source = '')
    LIMIT 1
)
WHERE name IN (
    SELECT uc.name
    FROM unique_census_names uc
    INNER JOIN unique_business_names ub ON uc.name = ub.name
)
AND (matched_to_genealogy_id IS NULL OR matched_to_genealogy_id = 0);

-- Report results
SELECT 'Businesses matched to census records:' as label,
       COUNT(*) as count
FROM sheffield_businesses
WHERE matched_to_genealogy_id IS NOT NULL
  AND matched_to_genealogy_id != '';

-- Show examples of matched records
SELECT 'Examples of matched records:' as label;
SELECT
    b.name,
    b.business_type,
    b.street_address as business_address,
    g.profession as census_profession,
    g.census_age,
    g.street_address as census_address
FROM sheffield_businesses b
INNER JOIN unmatched_genealogy g ON b.matched_to_genealogy_id = g.id
WHERE b.matched_to_genealogy_id IS NOT NULL
LIMIT 10;

-- Clean up
DROP TABLE unique_census_names;
DROP TABLE unique_business_names;
