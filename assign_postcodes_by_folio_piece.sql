-- Assign postcodes based on folio/piece numbers
-- Records with same folio/piece are geographically close, so use the most common postcode

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_folio_piece_postcode ON unmatched_genealogy(census_folio, census_piece, postcode);

-- For each folio/piece combination, find the most common postcode and assign it to records without postcodes
UPDATE unmatched_genealogy
SET postcode = (
    SELECT postcode
    FROM (
        SELECT
            postcode,
            COUNT(*) as postcode_count
        FROM unmatched_genealogy inner_table
        WHERE inner_table.census_folio = unmatched_genealogy.census_folio
          AND inner_table.census_piece = unmatched_genealogy.census_piece
          AND inner_table.postcode IS NOT NULL
          AND inner_table.postcode != ''
        GROUP BY postcode
        ORDER BY postcode_count DESC
        LIMIT 1
    )
)
WHERE (postcode IS NULL OR postcode = '')
  AND census_folio IS NOT NULL
  AND census_piece IS NOT NULL
  AND EXISTS (
      SELECT 1
      FROM unmatched_genealogy check_table
      WHERE check_table.census_folio = unmatched_genealogy.census_folio
        AND check_table.census_piece = unmatched_genealogy.census_piece
        AND check_table.postcode IS NOT NULL
        AND check_table.postcode != ''
  );

-- Report results
SELECT 'Records with postcodes after folio/piece assignment:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NOT NULL AND postcode != '';

SELECT 'Records still without postcodes:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NULL OR postcode = '';

-- Show breakdown by ecclesiastical district for records still without postcodes
SELECT 'Top ecclesiastical districts still without postcodes:' as label;
SELECT
    ecclesiastical_parish,
    COUNT(*) as count
FROM unmatched_genealogy
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish IS NOT NULL
  AND ecclesiastical_parish != ''
GROUP BY ecclesiastical_parish
ORDER BY count DESC
LIMIT 10;
