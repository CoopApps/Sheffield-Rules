-- Create indexes to speed up the comparison
CREATE INDEX IF NOT EXISTS idx_genealogy_match ON unmatched_genealogy(name, census_age, census_folio, census_piece);
CREATE INDEX IF NOT EXISTS idx_ancestry_match ON unmatched_ancestry(name, census_age, census_folio, census_piece);

SELECT 'Indexes created';

-- Count how many ancestry records don't have a match in genealogy
SELECT COUNT(*) as ancestry_not_in_genealogy
FROM unmatched_ancestry a
WHERE a.name IS NOT NULL
  AND a.census_age IS NOT NULL
  AND a.census_folio IS NOT NULL
  AND a.census_piece IS NOT NULL
  AND NOT EXISTS (
    SELECT 1
    FROM unmatched_genealogy g
    WHERE g.name = a.name
      AND g.census_age = a.census_age
      AND g.census_folio = a.census_folio
      AND g.census_piece = a.census_piece
  );

-- Show some examples
SELECT a.name, a.census_age, a.census_folio, a.census_piece, a.street_address, a.profession
FROM unmatched_ancestry a
WHERE a.name IS NOT NULL
  AND a.census_age IS NOT NULL
  AND a.census_folio IS NOT NULL
  AND a.census_piece IS NOT NULL
  AND NOT EXISTS (
    SELECT 1
    FROM unmatched_genealogy g
    WHERE g.name = a.name
      AND g.census_age = a.census_age
      AND g.census_folio = a.census_folio
      AND g.census_piece = a.census_piece
  )
LIMIT 20;
