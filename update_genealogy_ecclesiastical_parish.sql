-- Create lookup table for census_piece + census_folio -> ecclesiastical_parish
CREATE TEMP TABLE parish_lookup AS
SELECT DISTINCT
    census_piece,
    census_folio,
    ecclesiastical_parish
FROM unmatched_ancestry
WHERE census_piece IS NOT NULL
    AND census_folio IS NOT NULL
    AND ecclesiastical_parish IS NOT NULL;

-- Create index for fast lookup
CREATE INDEX idx_parish_lookup ON parish_lookup(census_piece, census_folio);

-- Update genealogy with ecclesiastical_parish from ancestry
UPDATE unmatched_genealogy
SET ecclesiastical_parish = (
    SELECT ecclesiastical_parish
    FROM parish_lookup
    WHERE parish_lookup.census_piece = unmatched_genealogy.census_piece
    AND parish_lookup.census_folio = unmatched_genealogy.census_folio
    LIMIT 1
)
WHERE census_piece IS NOT NULL
    AND census_folio IS NOT NULL;

-- Show results
SELECT COUNT(*) as records_with_ecclesiastical_parish
FROM unmatched_genealogy
WHERE ecclesiastical_parish IS NOT NULL;
