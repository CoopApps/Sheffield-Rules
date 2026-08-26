-- Create a temporary table with matched data
CREATE TEMP TABLE geni_matches AS
SELECT
    g.rowid as genealogy_rowid,
    ge.census_piece,
    ge.census_folio,
    ge.registration_district,
    ge.sub_registration_district
FROM unmatched_genealogy g
INNER JOIN unmatched_geni ge
    ON g.name = ge.name
    AND g.census_age = ge.census_age;

-- Update genealogy table from matches
UPDATE unmatched_genealogy
SET
    census_piece = (SELECT census_piece FROM geni_matches WHERE genealogy_rowid = unmatched_genealogy.rowid),
    census_folio = (SELECT census_folio FROM geni_matches WHERE genealogy_rowid = unmatched_genealogy.rowid),
    registration_district = (SELECT registration_district FROM geni_matches WHERE genealogy_rowid = unmatched_genealogy.rowid),
    sub_registration_district = (SELECT sub_registration_district FROM geni_matches WHERE genealogy_rowid = unmatched_genealogy.rowid)
WHERE rowid IN (SELECT genealogy_rowid FROM geni_matches);

-- Show results
SELECT COUNT(*) as updated_records FROM unmatched_genealogy
WHERE census_piece IS NOT NULL OR census_folio IS NOT NULL
   OR registration_district IS NOT NULL OR sub_registration_district IS NOT NULL;
