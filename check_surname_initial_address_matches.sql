-- Find matches where surname, first initial, and street address match
-- Even if full first name differs (e.g., "Geo." vs "George")

SELECT 'Matches with same surname, first initial, and address:' as label,
       COUNT(*) as count
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
  AND (g.matched_to_business_id IS NULL OR g.matched_to_business_id = 0);

-- Show examples
SELECT 'Examples of surname + first initial + address matches:' as label;
SELECT
    g.name as genealogy_name,
    g.first_name as gen_first,
    g.surname,
    g.street_address,
    b.name as business_name,
    b.first_name as bus_first,
    b.business_type,
    g.profession
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
LIMIT 20;
