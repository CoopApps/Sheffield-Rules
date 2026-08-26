-- Match surname + first initial + address pairs
-- Matches when one side has just initial (W, J.) and other has full name (William, James)
-- OR when both have full names but share same first letter
-- Only match when it's unambiguous (1 genealogy record to 1 business record)

UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT b.rowid
    FROM sheffield_businesses b
    WHERE SUBSTR(b.first_name, 1, 1) = SUBSTR(unmatched_genealogy.first_name, 1, 1)
      AND b.surname = unmatched_genealogy.surname
      AND b.street_address = unmatched_genealogy.street_address
      -- At least ONE side must be just an initial (length <= 2, accounting for period)
      AND (LENGTH(b.first_name) <= 2 OR LENGTH(unmatched_genealogy.first_name) <= 2)
    LIMIT 1
)
WHERE first_name IS NOT NULL
  AND first_name != ''
  AND surname IS NOT NULL
  AND surname != ''
  AND street_address IS NOT NULL
  AND street_address != ''
  AND (matched_to_business_id IS NULL OR matched_to_business_id = 0)
  -- Only match if there's exactly ONE business with this initial+surname+address
  AND (
      SELECT COUNT(*)
      FROM sheffield_businesses b2
      WHERE SUBSTR(b2.first_name, 1, 1) = SUBSTR(unmatched_genealogy.first_name, 1, 1)
        AND b2.surname = unmatched_genealogy.surname
        AND b2.street_address = unmatched_genealogy.street_address
        -- Same constraint: at least one side is an initial
        AND (LENGTH(b2.first_name) <= 2 OR LENGTH(unmatched_genealogy.first_name) <= 2)
  ) = 1;

-- Report results
SELECT 'Total matches after surname+initial+address matching:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL
  AND matched_to_business_id != 0;

-- Show how many were added
SELECT 'Matches added in this round:' as label;
