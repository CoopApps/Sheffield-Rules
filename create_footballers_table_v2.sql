-- Create sheffield_footballers table
-- Populated with males aged 14-40 from sheffield_people

CREATE TABLE IF NOT EXISTS sheffield_footballers (
    footballer_id INTEGER PRIMARY KEY AUTOINCREMENT,
    sheffield_person_id INTEGER NOT NULL,
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    census_gender TEXT,
    census_age INTEGER,
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
    FOREIGN KEY (sheffield_person_id) REFERENCES sheffield_people(rowid)
);

-- Populate with males aged 14-40
INSERT INTO sheffield_footballers (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (census_gender = 'M' OR census_gender = 'Male' OR census_gender = 'male')
  AND census_age >= 14
  AND census_age <= 40;

CREATE INDEX IF NOT EXISTS idx_footballer_person_id ON sheffield_footballers(sheffield_person_id);

SELECT 'Total footballers (males aged 14-40):' as label, COUNT(*) as count FROM sheffield_footballers;
