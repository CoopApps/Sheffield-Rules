-- Find exact name matches between census imports (genealogy_source IS NULL)
-- and unmatched businesses

-- Count unique names
SELECT 'Unique names matching between census imports and unmatched businesses:' as label,
       COUNT(DISTINCT g.name) as count
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE (g.genealogy_source IS NULL OR g.genealogy_source = '')
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '');

-- Show examples
SELECT 'Examples of matching names:' as label;
SELECT g.name, g.profession, g.census_age, b.business_type, b.street_address as business_address, g.street_address as census_address
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE (g.genealogy_source IS NULL OR g.genealogy_source = '')
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '')
LIMIT 20;

-- Count total match pairs
SELECT 'Total match pairs (including duplicates):' as label,
       COUNT(*) as count
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE (g.genealogy_source IS NULL OR g.genealogy_source = '')
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '');
