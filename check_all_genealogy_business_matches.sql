-- Find exact name matches between ALL genealogy records and unmatched businesses

-- Count unique names
SELECT 'Unique names matching between genealogy and unmatched businesses:' as label,
       COUNT(DISTINCT g.name) as count
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '');

-- Show examples
SELECT 'Examples of matching names:' as label;
SELECT DISTINCT g.name, g.profession, b.business_type, b.street_address
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '')
LIMIT 20;

-- Count total match pairs
SELECT 'Total match pairs (including duplicates):' as label,
       COUNT(*) as count
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.name = b.name
WHERE (b.matched_to_genealogy_id IS NULL OR b.matched_to_genealogy_id = '');
