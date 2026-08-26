-- Create sheffield_footballers table
-- Populated with males aged 14-40 from sheffield_people
-- Links back to sheffield_people via sheffield_person_id

CREATE TABLE IF NOT EXISTS sheffield_footballers (
    footballer_id INTEGER PRIMARY KEY AUTOINCREMENT,
    sheffield_person_id INTEGER NOT NULL,

    -- Copy all person data
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    gender TEXT,
    age INTEGER,
    birth_year INTEGER,
    profession TEXT,
    occupation_expanded TEXT,
    street_address TEXT,
    house_number TEXT,
    sub_area TEXT,
    street_name TEXT,
    postcode TEXT,
    civil_parish TEXT,
    ecclesiastical_parish TEXT,
    census_folio TEXT,
    census_piece TEXT,
    census_age INTEGER,
    census_birth_year INTEGER,
    genealogy_source TEXT,
    matched_to_business_id INTEGER,

    -- Foreign key constraint
    FOREIGN KEY (sheffield_person_id) REFERENCES sheffield_people(rowid)
);

-- Populate with males aged 14-40
INSERT INTO sheffield_footballers (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    gender, age, birth_year,
    profession, occupation_expanded,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece, census_age, census_birth_year,
    genealogy_source, matched_to_business_id
)
SELECT
    rowid as sheffield_person_id,
    name, first_name, middle_name, surname,
    gender, age, birth_year,
    profession, occupation_expanded,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece, census_age, census_birth_year,
    genealogy_source, matched_to_business_id
FROM sheffield_people
WHERE (gender = 'M' OR gender = 'Male' OR gender = 'male')
  AND age >= 14
  AND age <= 40;

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_footballer_person_id ON sheffield_footballers(sheffield_person_id);

-- Report results
SELECT 'Total footballers (males aged 14-40):' as label, COUNT(*) as count FROM sheffield_footballers;
SELECT 'Age distribution:' as label;
SELECT age, COUNT(*) as count
FROM sheffield_footballers
GROUP BY age
ORDER BY age;
