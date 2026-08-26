-- Find exact name matches between newly imported ancestry records (marked by matched_to_ancestry_id)
-- and unmatched businesses in sheffield_businesses

-- First, count unique name matches
SELECT 'Unique names matching between ancestry imports and unmatched businesses:' as label,
       COUNT(DISTINCT g.name) as count
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE g.matched_to_ancestry_id IS NOT NULL  -- These are the records from ancestry
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '');  -- Unmatched businesses

-- Show some examples
SELECT 'Examples of matching names:' as label;
SELECT DISTINCT g.name, g.profession, b.business_type, b.street_address
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE g.matched_to_ancestry_id IS NOT NULL
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '')
LIMIT 20;

-- Count total matches (not unique, including duplicates)
SELECT 'Total match pairs (including duplicates):' as label,
       COUNT(*) as count
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE g.matched_to_ancestry_id IS NOT NULL
  AND (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '');
