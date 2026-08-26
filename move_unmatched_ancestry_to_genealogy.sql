-- Move ancestry records that don't exist in genealogy to the genealogy table
-- Based on matching: name, census_age, census_folio, census_piece

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
    a.name, a.first_name, a.middle_name, a.surname, a.census_age, a.census_relation, a.census_gender,
    a.census_ed, a.census_household_schedule, a.census_piece, a.census_folio, a.census_page,
    a.civil_parish, a.ecclesiastical_parish, a.registration_district, a.sub_registration_district,
    a.street_address, a.house_number, a.sub_area, a.street_name,
    a.birth_year, a.birth_town, a.birth_county, a.birth_country, a.where_born,
    a.profession, a.occupation_expanded, a.genealogy_source, a.genealogy_id,
    a.business_name, a.business_type,
    a.postcode, a.postcode_area, a.postcode_district, a.postcode_sector, a.postcode_unit,
    a.latitude, a.longitude, a.created_at,
    a.id, a.matched_to_business_id, a.match_confidence, a.match_status
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

SELECT 'Inserted ' || changes() || ' ancestry records into unmatched_genealogy';

-- Delete the moved records from unmatched_ancestry
DELETE FROM unmatched_ancestry
WHERE id IN (
    SELECT a.id
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
);

SELECT 'Deleted ' || changes() || ' records from unmatched_ancestry';

-- Show final counts
SELECT 'unmatched_genealogy total records:', COUNT(*) FROM unmatched_genealogy;
SELECT 'unmatched_ancestry total records:', COUNT(*) FROM unmatched_ancestry;
