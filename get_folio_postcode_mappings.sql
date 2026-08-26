SELECT DISTINCT census_folio, census_piece, postcode
FROM unmatched_genealogy
WHERE census_folio IS NOT NULL
  AND census_piece IS NOT NULL
  AND postcode IS NOT NULL
  AND postcode != '';
