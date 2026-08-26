SELECT rowid, census_folio, census_piece
FROM unmatched_genealogy
WHERE (postcode IS NULL OR postcode = '')
  AND census_folio IS NOT NULL
  AND census_piece IS NOT NULL;
