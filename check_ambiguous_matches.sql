-- Find cases where one genealogy record matches multiple business records
-- (same first_name, surname, street_address)

SELECT 'Cases where 1 genealogy record matches multiple businesses:' as label;
SELECT
    g.name as gen_name,
    g.street_address,
    COUNT(DISTINCT b.rowid) as business_count,
    GROUP_CONCAT(b.name, ' | ') as business_names
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.first_name = b.first_name
  AND g.surname = b.surname
  AND g.street_address = b.street_address
WHERE g.first_name IS NOT NULL
  AND g.first_name != ''
  AND g.surname IS NOT NULL
  AND g.surname != ''
  AND g.street_address IS NOT NULL
  AND g.street_address != ''
  AND (g.matched_to_business_id IS NULL OR g.matched_to_business_id = 0)
GROUP BY g.rowid, g.name, g.street_address
HAVING COUNT(DISTINCT b.rowid) > 1;
