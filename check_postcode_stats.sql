-- Check postcode statistics in unmatched_genealogy

SELECT 'Records with postcodes:' as label, COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NOT NULL AND postcode != '';

SELECT 'Records without postcodes:' as label, COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NULL OR postcode = '';

SELECT 'Total records:' as label, COUNT(*) as count
FROM unmatched_genealogy;

-- Check how many unique folio/piece combinations have at least one postcode
SELECT 'Folio/piece combos with postcodes:' as label, COUNT(DISTINCT census_folio || '|' || census_piece) as count
FROM unmatched_genealogy
WHERE census_folio IS NOT NULL
  AND census_piece IS NOT NULL
  AND postcode IS NOT NULL
  AND postcode != '';

-- Check how many records need postcodes AND have folio/piece data
SELECT 'Records needing postcodes with folio/piece:' as label, COUNT(*) as count
FROM unmatched_genealogy
WHERE (postcode IS NULL OR postcode = '')
  AND census_folio IS NOT NULL
  AND census_piece IS NOT NULL;
