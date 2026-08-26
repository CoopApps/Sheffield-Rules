-- Create sheffield_employers table
-- Populated with all people who employ others from sheffield_people

CREATE TABLE IF NOT EXISTS sheffield_employers (
    employer_id INTEGER PRIMARY KEY AUTOINCREMENT,
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

-- Populate with all employers (anyone with "Employing" in profession)
INSERT INTO sheffield_employers (
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
WHERE profession LIKE '%Employing%'
   OR profession LIKE '%employing%'
   OR profession LIKE '%Employer%'
   OR profession LIKE '%employer%';

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_employer_person_id ON sheffield_employers(sheffield_person_id);

-- Report results
SELECT 'Total employers:' as label, COUNT(*) as count FROM sheffield_employers;

SELECT 'Large employers (10+ employees):' as label, COUNT(*) as count 
FROM sheffield_employers 
WHERE profession LIKE '%Employing 1_ %'
   OR profession LIKE '%Employing 2_ %'
   OR profession LIKE '%Employing 3_ %'
   OR profession LIKE '%Employing 4_ %'
   OR profession LIKE '%Employing 5_ %'
   OR profession LIKE '%Employing 6_ %'
   OR profession LIKE '%Employing 7_ %'
   OR profession LIKE '%Employing 8_ %'
   OR profession LIKE '%Employing 9_ %'
   OR profession LIKE '%Employing 1__ %'
   OR profession LIKE '%Employing 2__ %'
   OR profession LIKE '%Employing 3__ %'
   OR profession LIKE '%Employing 4__ %'
   OR profession LIKE '%Employing 5__ %'
   OR profession LIKE '%Employing 6__ %'
   OR profession LIKE '%Employing 7__ %'
   OR profession LIKE '%Employing 8__ %'
   OR profession LIKE '%Employing 9__ %';
