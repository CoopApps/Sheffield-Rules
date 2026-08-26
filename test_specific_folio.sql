-- Test with folio 115, piece 4669 which should have postcode S9 4QB

-- Check records WITH this folio/piece that have postcodes
SELECT 'Records WITH postcode for folio 115, piece 4669:' as label;
SELECT rowid, name, postcode
FROM unmatched_genealogy
WHERE census_folio = 115
  AND census_piece = 4669
  AND postcode IS NOT NULL
  AND postcode != ''
LIMIT 5;

-- Check records WITHOUT postcode for this folio/piece
SELECT 'Records WITHOUT postcode for folio 115, piece 4669:' as label;
SELECT rowid, name, census_folio, census_piece, postcode
FROM unmatched_genealogy
WHERE census_folio = 115
  AND census_piece = 4669
  AND (postcode IS NULL OR postcode = '')
LIMIT 5;
