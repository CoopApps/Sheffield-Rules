-- Find unique name matches that ALSO have matching addresses

-- Get names that appear exactly once in census imports
CREATE TEMP TABLE unique_census_names AS
SELECT name, COUNT(*) as count
FROM unmatched_genealogy
WHERE (genealogy_source IS NULL OR genealogy_source = '')
  AND name IS NOT NULL
GROUP BY name
HAVING COUNT(*) = 1;

-- Get names that appear exactly once in unmatched businesses
CREATE TEMP TABLE unique_business_names AS
SELECT name, COUNT(*) as count
FROM sheffield_businesses
WHERE (matched_to_genealogy_id IS NULL OR matched_to_genealogy_id = '')
  AND name IS NOT NULL
GROUP BY name
HAVING COUNT(*) = 1;

-- Find matches with same address (exact match)
SELECT 'Exact address matches:' as label, COUNT(*) as count
FROM unmatched_genealogy g
INNER JOIN unique_census_names uc ON g.name = uc.name
INNER JOIN unique_business_names ub ON g.name = ub.name
INNER JOIN sheffield_businesses b ON g.name = b.name
WHERE (g.genealogy_source IS NULL OR g.genealogy_source = '')
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '')
  AND g.street_address = b.street_address;

-- Show examples
SELECT 'Examples with exact address match:' as label;
SELECT
    g.name,
    g.census_age,
    g.profession,
    g.street_address as address,
    b.business_type
FROM unmatched_genealogy g
INNER JOIN unique_census_names uc ON g.name = uc.name
INNER JOIN unique_business_names ub ON g.name = ub.name
INNER JOIN sheffield_businesses b ON g.name = b.name
WHERE (g.genealogy_source IS NULL OR g.genealogy_source = '')
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '')
  AND g.street_address = b.street_address
LIMIT 20;

-- Clean up
DROP TABLE unique_census_names;
DROP TABLE unique_business_names;
