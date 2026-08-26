-- Create sheffield_professionals table
-- Populated with medical professionals, teachers, clergy, and police

CREATE TABLE IF NOT EXISTS sheffield_professionals (
    professional_id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    professional_type TEXT,  -- medical, teacher, clergy, police
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

-- Populate with medical professionals
INSERT INTO sheffield_professionals (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'medical' as professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE profession LIKE '%doctor%' OR profession LIKE '%surgeon%' OR profession LIKE '%physician%'
   OR profession LIKE '%Doctor%' OR profession LIKE '%Surgeon%' OR profession LIKE '%Physician%'
   OR profession LIKE '%nurse%' OR profession LIKE '%Nurse%';

-- Populate with teachers
INSERT INTO sheffield_professionals (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'teacher' as professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (profession LIKE '%teacher%' OR profession LIKE '%Teacher%' OR profession LIKE '%schoolmaster%'
   OR profession LIKE '%Schoolmaster%' OR profession LIKE '%governess%' OR profession LIKE '%Governess%')
  AND rowid NOT IN (SELECT sheffield_person_id FROM sheffield_professionals);

-- Populate with clergy
INSERT INTO sheffield_professionals (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'clergy' as professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (profession LIKE '%clergy%' OR profession LIKE '%vicar%' OR profession LIKE '%priest%'
   OR profession LIKE '%minister%' OR profession LIKE '%Clergy%' OR profession LIKE '%Vicar%'
   OR profession LIKE '%Priest%' OR profession LIKE '%Minister%')
  AND rowid NOT IN (SELECT sheffield_person_id FROM sheffield_professionals);

-- Populate with police
INSERT INTO sheffield_professionals (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'police' as professional_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (profession LIKE '%police%' OR profession LIKE '%Police%'
   OR profession LIKE '%constable%' OR profession LIKE '%Constable%')
  AND rowid NOT IN (SELECT sheffield_person_id FROM sheffield_professionals);

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_professional_person_id ON sheffield_professionals(sheffield_person_id);

-- Create index on professional_type for filtering by profession
CREATE INDEX IF NOT EXISTS idx_professional_type ON sheffield_professionals(professional_type);

-- Report results
SELECT 'Total professionals:' as label, COUNT(*) as count FROM sheffield_professionals;

SELECT 'Breakdown by profession:' as label;
SELECT professional_type, COUNT(*) as count
FROM sheffield_professionals
GROUP BY professional_type
ORDER BY count DESC;
