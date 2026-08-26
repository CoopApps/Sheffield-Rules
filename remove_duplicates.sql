-- Find and remove exact duplicates from sheffield_people
-- Keep only the record with the lowest rowid (first occurrence)

-- First, count total duplicates
SELECT 'Total records before deduplication:' as label, COUNT(*) as count FROM sheffield_people;

SELECT 'Duplicate records to remove:' as label, COUNT(*) as count
FROM sheffield_people p1
WHERE EXISTS (
    SELECT 1 FROM sheffield_people p2
    WHERE p2.name = p1.name
      AND COALESCE(p2.census_age, -1) = COALESCE(p1.census_age, -1)
      AND COALESCE(p2.census_folio, '') = COALESCE(p1.census_folio, '')
      AND COALESCE(p2.census_piece, '') = COALESCE(p1.census_piece, '')
      AND COALESCE(p2.street_address, '') = COALESCE(p1.street_address, '')
      AND p2.rowid < p1.rowid
);

-- Delete duplicates (keep the one with lowest rowid)
DELETE FROM sheffield_people
WHERE rowid IN (
    SELECT p1.rowid
    FROM sheffield_people p1
    WHERE EXISTS (
        SELECT 1 FROM sheffield_people p2
        WHERE p2.name = p1.name
          AND COALESCE(p2.census_age, -1) = COALESCE(p1.census_age, -1)
          AND COALESCE(p2.census_folio, '') = COALESCE(p1.census_folio, '')
          AND COALESCE(p2.census_piece, '') = COALESCE(p1.census_piece, '')
          AND COALESCE(p2.street_address, '') = COALESCE(p1.street_address, '')
          AND p2.rowid < p1.rowid
    )
);

SELECT 'Total records after deduplication:' as label, COUNT(*) as count FROM sheffield_people;
