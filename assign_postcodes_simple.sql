-- Create index on folio/piece for fast lookups
CREATE INDEX IF NOT EXISTS idx_genealogy_folio_piece ON unmatched_genealogy(census_folio, census_piece);

SELECT 'Index created';

-- Update in a single SQL query
UPDATE unmatched_genealogy
SET postcode = (
    SELECT g2.postcode
    FROM unmatched_genealogy g2
    WHERE g2.census_folio = unmatched_genealogy.census_folio
      AND g2.census_piece = unmatched_genealogy.census_piece
      AND g2.postcode IS NOT NULL
      AND g2.postcode != ''
    LIMIT 1
)
WHERE (postcode IS NULL OR postcode = '')
  AND census_folio IS NOT NULL
  AND census_piece IS NOT NULL
  AND EXISTS (
    SELECT 1
    FROM unmatched_genealogy g2
    WHERE g2.census_folio = unmatched_genealogy.census_folio
      AND g2.census_piece = unmatched_genealogy.census_piece
      AND g2.postcode IS NOT NULL
      AND g2.postcode != ''
  );

SELECT 'Updated ' || changes() || ' records with postcodes';

-- Final statistics
SELECT 'Records with postcodes:', COUNT(*) FROM unmatched_genealogy WHERE postcode IS NOT NULL AND postcode != '';
SELECT 'Records without postcodes:', COUNT(*) FROM unmatched_genealogy WHERE postcode IS NULL OR postcode = '';
SELECT 'Total records:', COUNT(*) FROM unmatched_genealogy;
