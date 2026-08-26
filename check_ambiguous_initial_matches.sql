-- Find cases where one genealogy record matches multiple businesses
-- Based on surname, first initial, and address

SELECT
    g.name as gen_name,
    g.first_name as gen_first,
    g.surname,
    g.street_address,
    COUNT(DISTINCT b.rowid) as business_count,
    GROUP_CONCAT(b.name, ' | ') as business_names
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON SUBSTR(g.first_name, 1, 1) = SUBSTR(b.first_name, 1, 1)
  AND g.surname = b.surname
  AND g.street_address = b.street_address
WHERE g.first_name IS NOT NULL
  AND g.first_name != ''
  AND g.surname IS NOT NULL
  AND g.surname != ''
  AND g.street_address IS NOT NULL
  AND g.street_address != ''
  AND (g.matched_to_business_id IS NULL OR g.matched_to_business_id = 0)
GROUP BY g.rowid, g.name, g.first_name, g.surname, g.street_address
HAVING COUNT(DISTINCT b.rowid) > 1;
