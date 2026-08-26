-- Match the 1,437 unique name pairs (census records to businesses)
-- Uses matched_to_business_id in unmatched_genealogy

-- Create temp tables for unique names
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
WHERE name IS NOT NULL
GROUP BY name
HAVING COUNT(*) = 1;

-- Update genealogy records to link to business records via rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT b.rowid
    FROM sheffield_businesses b
    INNER JOIN unique_business_names ubn ON b.name = ubn.name
    WHERE b.name = unmatched_genealogy.name
    LIMIT 1
)
WHERE name IN (
    SELECT ucn.name
    FROM unique_census_names ucn
    INNER JOIN unique_business_names ubn ON ucn.name = ubn.name
)
AND (genealogy_source IS NULL OR genealogy_source = '')
AND (matched_to_business_id IS NULL OR matched_to_business_id = 0);

-- Report results
SELECT 'Total genealogy records now matched to business:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL AND matched_to_business_id != 0;

-- Show new matches
SELECT 'Examples of newly matched unique names:' as label;
SELECT
    g.name,
    g.census_age,
    g.profession as census_profession,
    g.street_address as census_address,
    b.business_type,
    b.street_address as business_address
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b ON g.matched_to_business_id = b.rowid
WHERE g.name IN (
    SELECT ucn.name
    FROM unique_census_names ucn
    INNER JOIN unique_business_names ubn ON ucn.name = ubn.name
)
LIMIT 10;

-- Clean up
DROP TABLE unique_census_names;
DROP TABLE unique_business_names;
