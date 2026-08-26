-- Find unique name matches that ALSO have matching folio/piece with businesses

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

-- Find matches with same folio AND piece
SELECT 'Matches with same folio AND piece:' as label, COUNT(*) as count
FROM unmatched_genealogy g
INNER JOIN unique_census_names uc ON g.name = uc.name
INNER JOIN unique_business_names ub ON g.name = ub.name
INNER JOIN sheffield_businesses b ON g.name = b.name
WHERE (g.genealogy_source IS NULL OR g.genealogy_source = '')
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '')
  AND g.census_folio = b.census_folio
  AND g.census_piece = b.census_piece
  AND g.census_folio IS NOT NULL
  AND b.census_folio IS NOT NULL;

-- Show examples
SELECT 'Examples with folio/piece match:' as label;
SELECT
    g.name,
    g.census_age,
    g.census_folio,
    g.census_piece,
    g.profession as census_profession,
    g.street_address as census_address,
    b.business_type,
    b.street_address as business_address
FROM unmatched_genealogy g
INNER JOIN unique_census_names uc ON g.name = uc.name
INNER JOIN unique_business_names ub ON g.name = ub.name
INNER JOIN sheffield_businesses b ON g.name = b.name
WHERE (g.genealogy_source IS NULL OR g.genealogy_source = '')
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '')
  AND g.census_folio = b.census_folio
  AND g.census_piece = b.census_piece
  AND g.census_folio IS NOT NULL
  AND b.census_folio IS NOT NULL
LIMIT 20;

-- Clean up
DROP TABLE unique_census_names;
DROP TABLE unique_business_names;
