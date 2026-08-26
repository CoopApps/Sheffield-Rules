-- Update records on the same street IN THE SAME TOWN using efficient JOIN
-- This assumes people living on the same street in the same town are in the same census district

-- Create temporary table with street-based matches
CREATE TEMP TABLE street_matches AS
SELECT
    g1.rowid as genealogy_rowid,
    g2.census_piece,
    g2.census_folio,
    g2.registration_district,
    g2.sub_registration_district
FROM unmatched_genealogy g1
INNER JOIN unmatched_genealogy g2
    ON g1.street_name = g2.street_name
    AND (g1.civil_parish = g2.civil_parish OR (g1.civil_parish IS NULL AND g2.civil_parish IS NULL))
    AND g2.census_piece IS NOT NULL
WHERE
    g1.census_piece IS NULL
    AND g1.street_name IS NOT NULL
    AND g1.street_name <> '';

-- Update genealogy table from matches
UPDATE unmatched_genealogy
SET
    census_piece = (SELECT census_piece FROM street_matches WHERE genealogy_rowid = unmatched_genealogy.rowid),
    census_folio = (SELECT census_folio FROM street_matches WHERE genealogy_rowid = unmatched_genealogy.rowid),
    registration_district = (SELECT registration_district FROM street_matches WHERE genealogy_rowid = unmatched_genealogy.rowid),
    sub_registration_district = (SELECT sub_registration_district FROM street_matches WHERE genealogy_rowid = unmatched_genealogy.rowid)
WHERE rowid IN (SELECT genealogy_rowid FROM street_matches);

-- Show results
SELECT COUNT(*) as total_with_census_data FROM unmatched_genealogy
WHERE census_piece IS NOT NULL;
