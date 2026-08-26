-- Create sheffield_irish table
-- Populated with Irish immigrants

CREATE TABLE IF NOT EXISTS sheffield_irish (
    irish_id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    birth_town TEXT,
    birth_county TEXT,
    where_born TEXT,
    FOREIGN KEY (sheffield_person_id) REFERENCES sheffield_people(rowid)
);

-- Populate with Irish immigrants
INSERT INTO sheffield_irish (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece,
    birth_town, birth_county, where_born
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece,
    birth_town, birth_county, where_born
FROM sheffield_people
WHERE birth_country LIKE '%Ireland%' OR birth_country LIKE '%ireland%'
   OR where_born LIKE '%Ireland%' OR where_born LIKE '%ireland%';

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_irish_person_id ON sheffield_irish(sheffield_person_id);

-- Report results
SELECT 'Total Irish immigrants:' as label, COUNT(*) as count FROM sheffield_irish;

SELECT 'Irish immigrants by age group:' as label;
SELECT
    CASE
        WHEN census_age < 14 THEN 'Children (under 14)'
        WHEN census_age >= 14 AND census_age < 40 THEN 'Working age (14-40)'
        WHEN census_age >= 40 THEN 'Older (40+)'
        ELSE 'Unknown age'
    END as age_group,
    COUNT(*) as count
FROM sheffield_irish
GROUP BY age_group
ORDER BY count DESC;

SELECT 'Top 10 Irish surnames:' as label;
SELECT surname, COUNT(*) as count
FROM sheffield_irish
WHERE surname IS NOT NULL
GROUP BY surname
ORDER BY count DESC
LIMIT 10;
