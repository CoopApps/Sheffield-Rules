-- Step 1: Create a simple lookup of street+parish → census data
CREATE TEMP TABLE street_census_lookup AS
SELECT DISTINCT
    street_name,
    civil_parish,
    census_piece,
    census_folio,
    registration_district,
    sub_registration_district
FROM unmatched_genealogy
WHERE census_piece IS NOT NULL
    AND street_name IS NOT NULL
    AND street_name <> '';

-- Step 2: Create index for fast lookup
CREATE INDEX idx_street_lookup ON street_census_lookup(street_name, civil_parish);

-- Step 3: Update records without census data by matching street+parish
UPDATE unmatched_genealogy
SET
    census_piece = (SELECT census_piece FROM street_census_lookup WHERE street_census_lookup.street_name = unmatched_genealogy.street_name AND (street_census_lookup.civil_parish = unmatched_genealogy.civil_parish OR (street_census_lookup.civil_parish IS NULL AND unmatched_genealogy.civil_parish IS NULL)) LIMIT 1),
    census_folio = (SELECT census_folio FROM street_census_lookup WHERE street_census_lookup.street_name = unmatched_genealogy.street_name AND (street_census_lookup.civil_parish = unmatched_genealogy.civil_parish OR (street_census_lookup.civil_parish IS NULL AND unmatched_genealogy.civil_parish IS NULL)) LIMIT 1),
    registration_district = (SELECT registration_district FROM street_census_lookup WHERE street_census_lookup.street_name = unmatched_genealogy.street_name AND (street_census_lookup.civil_parish = unmatched_genealogy.civil_parish OR (street_census_lookup.civil_parish IS NULL AND unmatched_genealogy.civil_parish IS NULL)) LIMIT 1),
    sub_registration_district = (SELECT sub_registration_district FROM street_census_lookup WHERE street_census_lookup.street_name = unmatched_genealogy.street_name AND (street_census_lookup.civil_parish = unmatched_genealogy.civil_parish OR (street_census_lookup.civil_parish IS NULL AND unmatched_genealogy.civil_parish IS NULL)) LIMIT 1)
WHERE census_piece IS NULL
    AND street_name IS NOT NULL
    AND street_name <> '';

-- Show results
SELECT COUNT(*) FROM unmatched_genealogy WHERE census_piece IS NOT NULL;
