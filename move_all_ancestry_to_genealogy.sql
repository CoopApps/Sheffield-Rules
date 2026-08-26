-- Move ALL ancestry records to genealogy table (simpler approach)

INSERT INTO unmatched_genealogy (
    name, first_name, middle_name, surname, census_age, census_relation, census_gender,
    census_ed, census_household_schedule, census_piece, census_folio, census_page,
    civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
    street_address, house_number, sub_area, street_name,
    birth_year, birth_town, birth_county, birth_country, where_born,
    profession, occupation_expanded, genealogy_source, genealogy_id,
    business_name, business_type,
    postcode, postcode_area, postcode_district, postcode_sector, postcode_unit,
    latitude, longitude, created_at,
    matched_to_ancestry_id, matched_to_business_id, match_confidence, match_status
)
SELECT
    name, first_name, middle_name, surname, census_age, census_relation, census_gender,
    census_ed, census_household_schedule, census_piece, census_folio, census_page,
    civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
    street_address, house_number, sub_area, street_name,
    birth_year, birth_town, birth_county, birth_country, where_born,
    profession, occupation_expanded, genealogy_source, genealogy_id,
    business_name, business_type,
    postcode, postcode_area, postcode_district, postcode_sector, postcode_unit,
    latitude, longitude, created_at,
    id, matched_to_business_id, match_confidence, match_status
FROM unmatched_ancestry;

SELECT 'Inserted ' || changes() || ' ancestry records into unmatched_genealogy';

-- Delete all records from unmatched_ancestry
DELETE FROM unmatched_ancestry;

SELECT 'Deleted ' || changes() || ' records from unmatched_ancestry';

-- Show final counts
SELECT 'unmatched_genealogy total records:', COUNT(*) FROM unmatched_genealogy;
SELECT 'unmatched_ancestry total records:', COUNT(*) FROM unmatched_ancestry;
