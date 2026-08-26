-- Match the 90 safe first/last name + address pairs
-- Excludes the 2 ambiguous cases (Thomas Ward and William James Greenwood at Woodhouse)

UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT b.rowid
    FROM sheffield_businesses b
    WHERE b.first_name = unmatched_genealogy.first_name
      AND b.surname = unmatched_genealogy.surname
      AND b.street_address = unmatched_genealogy.street_address
    LIMIT 1
)
WHERE first_name IS NOT NULL
  AND first_name != ''
  AND surname IS NOT NULL
  AND surname != ''
  AND street_address IS NOT NULL
  AND street_address != ''
  AND (matched_to_business_id IS NULL OR matched_to_business_id = 0)
  -- Exclude the 2 ambiguous cases (must match EXACT street_address from ambiguous query)
  AND NOT (first_name = 'Thomas' AND surname = 'Ward' AND street_address = 'Woodhouse')
  AND NOT (first_name = 'William' AND surname = 'Greenwood' AND street_address = 'Woodhouse')
  -- Only match if there's exactly one business record (unambiguous)
  AND (
      SELECT COUNT(*)
      FROM sheffield_businesses b2
      WHERE b2.first_name = unmatched_genealogy.first_name
        AND b2.surname = unmatched_genealogy.surname
        AND b2.street_address = unmatched_genealogy.street_address
  ) = 1;

-- Report results
SELECT 'Total matches after name+address matching:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL
  AND matched_to_business_id != 0;

SELECT 'Ambiguous cases requiring manual review:' as label;
SELECT
    g.name,
    g.street_address,
    g.profession,
    COUNT(DISTINCT b.rowid) as business_count,
    GROUP_CONCAT(b.name || ' (' || b.business_type || ')', ' | ') as business_options
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
  ON g.first_name = b.first_name
  AND g.surname = b.surname
  AND g.street_address = b.street_address
WHERE (g.name = 'Thomas Ward' AND g.street_address = 'Woodhouse')
   OR (g.name = 'William James Greenwood' AND g.street_address = 'Woodhouse')
GROUP BY g.rowid, g.name, g.street_address, g.profession;
