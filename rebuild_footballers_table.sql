-- Rebuild sheffield_footballers table with realistic football-playing population
-- Include: Males aged 14-40 who are likely to play football
-- Exclude: Clergy, large employers, very old/young

-- Drop existing table
DROP TABLE IF EXISTS sheffield_footballers;

-- Create new table with all columns from sheffield_people
CREATE TABLE sheffield_footballers (
    footballer_id INTEGER PRIMARY KEY AUTOINCREMENT,
    sheffield_person_id INTEGER NOT NULL,
    id INT,
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    census_age INT,
    census_relation TEXT,
    census_gender TEXT,
    census_ed TEXT,
    census_household_schedule TEXT,
    census_piece TEXT,
    census_folio TEXT,
    census_page TEXT,
    civil_parish TEXT,
    ecclesiastical_parish TEXT,
    registration_district TEXT,
    sub_registration_district TEXT,
    street_address TEXT,
    house_number TEXT,
    sub_area TEXT,
    street_name TEXT,
    birth_year INT,
    birth_town TEXT,
    birth_county TEXT,
    birth_country TEXT,
    where_born TEXT,
    profession TEXT,
    occupation_expanded TEXT,
    genealogy_source TEXT,
    genealogy_id TEXT,
    business_name TEXT,
    business_type TEXT,
    postcode TEXT,
    postcode_area TEXT,
    postcode_district TEXT,
    postcode_sector TEXT,
    postcode_unit TEXT,
    latitude REAL,
    longitude REAL,
    created_at NUM,
    matched_to_ancestry_id INT,
    matched_to_business_id INT,
    match_confidence REAL,
    match_status TEXT,
    FOREIGN KEY (sheffield_person_id) REFERENCES sheffield_people(rowid)
);

-- Populate with males aged 14-40, excluding unlikely players
INSERT INTO sheffield_footballers (
    sheffield_person_id,
    id, name, first_name, middle_name, surname,
    census_age, census_relation, census_gender,
    census_ed, census_household_schedule, census_piece, census_folio, census_page,
    civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
    street_address, house_number, sub_area, street_name,
    birth_year, birth_town, birth_county, birth_country, where_born,
    profession, occupation_expanded,
    genealogy_source, genealogy_id,
    business_name, business_type,
    postcode, postcode_area, postcode_district, postcode_sector, postcode_unit,
    latitude, longitude,
    created_at,
    matched_to_ancestry_id, matched_to_business_id,
    match_confidence, match_status
)
SELECT
    rowid,
    id, name, first_name, middle_name, surname,
    census_age, census_relation, census_gender,
    census_ed, census_household_schedule, census_piece, census_folio, census_page,
    civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
    street_address, house_number, sub_area, street_name,
    birth_year, birth_town, birth_county, birth_country, where_born,
    profession, occupation_expanded,
    genealogy_source, genealogy_id,
    business_name, business_type,
    postcode, postcode_area, postcode_district, postcode_sector, postcode_unit,
    latitude, longitude,
    created_at,
    matched_to_ancestry_id, matched_to_business_id,
    match_confidence, match_status
FROM sheffield_people
WHERE (census_gender = 'M' OR census_gender = 'Male' OR census_gender = 'male')
  AND census_age >= 14
  AND census_age <= 40
  -- Exclude clergy
  AND (profession IS NULL OR (
      profession NOT LIKE '%clergy%'
      AND profession NOT LIKE '%Clergy%'
      AND profession NOT LIKE '%vicar%'
      AND profession NOT LIKE '%Vicar%'
      AND profession NOT LIKE '%priest%'
      AND profession NOT LIKE '%Priest%'
      AND profession NOT LIKE '%minister%'
      AND profession NOT LIKE '%Minister%'
      AND profession NOT LIKE '%reverend%'
      AND profession NOT LIKE '%Reverend%'
  ))
  -- Exclude large employers (10+ employees)
  AND (profession IS NULL OR (
      profession NOT LIKE '%Employing 1_ %'
      AND profession NOT LIKE '%Employing 2_ %'
      AND profession NOT LIKE '%Employing 3_ %'
      AND profession NOT LIKE '%Employing 4_ %'
      AND profession NOT LIKE '%Employing 5_ %'
      AND profession NOT LIKE '%Employing 6_ %'
      AND profession NOT LIKE '%Employing 7_ %'
      AND profession NOT LIKE '%Employing 8_ %'
      AND profession NOT LIKE '%Employing 9_ %'
      AND profession NOT LIKE '%Employing 1__ %'
      AND profession NOT LIKE '%Employing 2__ %'
      AND profession NOT LIKE '%Employing 3__ %'
      AND profession NOT LIKE '%Employing 4__ %'
      AND profession NOT LIKE '%Employing 5__ %'
      AND profession NOT LIKE '%Employing 6__ %'
      AND profession NOT LIKE '%Employing 7__ %'
      AND profession NOT LIKE '%Employing 8__ %'
      AND profession NOT LIKE '%Employing 9__ %'
  ))
  -- Exclude police (shift work, duty restrictions)
  AND (profession IS NULL OR (
      profession NOT LIKE '%police%'
      AND profession NOT LIKE '%Police%'
      AND profession NOT LIKE '%constable%'
      AND profession NOT LIKE '%Constable%'
  ));

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_footballer_person_id ON sheffield_footballers(sheffield_person_id);

-- Create index on profession for filtering
CREATE INDEX IF NOT EXISTS idx_footballer_profession ON sheffield_footballers(profession);

-- Create index on postcode for geographic queries
CREATE INDEX IF NOT EXISTS idx_footballer_postcode ON sheffield_footballers(postcode);

-- Create index on age for age-based queries
CREATE INDEX IF NOT EXISTS idx_footballer_age ON sheffield_footballers(census_age);

-- Report results
SELECT 'Total potential footballers (males 14-40, excluding clergy/large employers/police):' as label,
       COUNT(*) as count
FROM sheffield_footballers;

-- Age breakdown
SELECT 'Age breakdown:' as label;
SELECT
    CASE
        WHEN census_age BETWEEN 14 AND 17 THEN '14-17 (Youth)'
        WHEN census_age BETWEEN 18 AND 25 THEN '18-25 (Prime)'
        WHEN census_age BETWEEN 26 AND 30 THEN '26-30 (Peak)'
        WHEN census_age BETWEEN 31 AND 35 THEN '31-35 (Experienced)'
        WHEN census_age BETWEEN 36 AND 40 THEN '36-40 (Veteran)'
        ELSE 'Unknown'
    END as age_group,
    COUNT(*) as count
FROM sheffield_footballers
GROUP BY age_group
ORDER BY census_age;

-- Check how many have profession data
SELECT 'Footballers with profession data:' as label,
       COUNT(*) as count,
       ROUND(COUNT(*) * 100.0 / (SELECT COUNT(*) FROM sheffield_footballers), 1) as percentage
FROM sheffield_footballers
WHERE profession IS NOT NULL AND profession != '';

-- Sample professions
SELECT 'Top 10 professions among footballers:' as label;
SELECT profession, COUNT(*) as count
FROM sheffield_footballers
WHERE profession IS NOT NULL AND profession != ''
GROUP BY profession
ORDER BY count DESC
LIMIT 10;
