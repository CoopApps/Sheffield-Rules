-- Create sheffield_lodgers table
-- Populated with lodgers and boarders

CREATE TABLE IF NOT EXISTS sheffield_lodgers (
    lodger_id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    census_relation TEXT,
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

-- Populate with lodgers and boarders
INSERT INTO sheffield_lodgers (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    census_relation,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    census_relation,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE census_relation LIKE '%Lodger%' OR census_relation LIKE '%lodger%'
   OR census_relation LIKE '%Boarder%' OR census_relation LIKE '%boarder%';

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_lodger_person_id ON sheffield_lodgers(sheffield_person_id);

-- Report results
SELECT 'Total lodgers/boarders:' as label, COUNT(*) as count FROM sheffield_lodgers;

SELECT 'Lodgers by age group:' as label;
SELECT
    CASE
        WHEN census_age < 14 THEN 'Children (under 14)'
        WHEN census_age >= 14 AND census_age < 25 THEN 'Young adults (14-24)'
        WHEN census_age >= 25 AND census_age < 40 THEN 'Adults (25-40)'
        WHEN census_age >= 40 THEN 'Older (40+)'
        ELSE 'Unknown age'
    END as age_group,
    COUNT(*) as count
FROM sheffield_lodgers
GROUP BY age_group
ORDER BY count DESC;

SELECT 'Lodgers by gender:' as label;
SELECT census_gender, COUNT(*) as count
FROM sheffield_lodgers
WHERE census_gender IS NOT NULL
GROUP BY census_gender;

SELECT 'Top 10 lodging house addresses (by number of lodgers):' as label;
SELECT street_address, COUNT(*) as lodger_count
FROM sheffield_lodgers
WHERE street_address IS NOT NULL
GROUP BY street_address
ORDER BY lodger_count DESC
LIMIT 10;
