-- Check folio/piece values from records WITH postcodes
SELECT 'Sample folio/piece WITH postcodes:' as label;
SELECT DISTINCT census_folio, census_piece
FROM unmatched_genealogy
WHERE postcode IS NOT NULL AND postcode != ''
LIMIT 10;

-- Check folio/piece values from records WITHOUT postcodes
SELECT 'Sample folio/piece WITHOUT postcodes:' as label;
SELECT DISTINCT census_folio, census_piece
FROM unmatched_genealogy
WHERE (postcode IS NULL OR postcode = '')
  AND census_folio IS NOT NULL
  AND census_piece IS NOT NULL
LIMIT 10;

-- Check if there's ANY overlap
SELECT 'Overlapping folio/piece combinations:' as label, COUNT(*) as count
FROM (
    SELECT DISTINCT census_folio, census_piece
    FROM unmatched_genealogy
    WHERE postcode IS NOT NULL AND postcode != ''
) AS with_postcode
INNER JOIN (
    SELECT DISTINCT census_folio, census_piece
    FROM unmatched_genealogy
    WHERE (postcode IS NULL OR postcode = '')
      AND census_folio IS NOT NULL
      AND census_piece IS NOT NULL
) AS without_postcode
ON with_postcode.census_folio = without_postcode.census_folio
   AND with_postcode.census_piece = without_postcode.census_piece;
