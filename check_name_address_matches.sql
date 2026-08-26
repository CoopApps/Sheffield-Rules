-- Find matches where first_name and surname match, and street_address matches
-- Even if middle_name differs or is missing

SELECT 'Matches with same first/last name and address:' as label,
       COUNT(*) as count
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
  AND (g.matched_to_business_id IS NULL OR g.matched_to_business_id = 0);

-- Show examples
SELECT 'Examples of first/last name + address matches:' as label;
SELECT
    g.name as genealogy_name,
    g.first_name,
    g.middle_name as gen_middle,
    g.surname,
    g.street_address,
    b.name as business_name,
    b.middle_name as bus_middle,
    b.business_type,
    g.profession
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
LIMIT 20;

-- Check how many have different middle names
SELECT 'Matches where middle names differ:' as label,
       COUNT(*) as count
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
  AND (
    (g.middle_name IS NOT NULL AND b.middle_name IS NOT NULL AND g.middle_name != b.middle_name)
    OR (g.middle_name IS NULL AND b.middle_name IS NOT NULL)
    OR (g.middle_name IS NOT NULL AND b.middle_name IS NULL)
  );
