-- Create sheffield_publicans table
-- Populated with publicans, innkeepers, and beer sellers

CREATE TABLE IF NOT EXISTS sheffield_publicans (
    publican_id INTEGER PRIMARY KEY AUTOINCREMENT,
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

-- Populate with publicans, innkeepers, beer sellers
INSERT INTO sheffield_publicans (
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
WHERE profession LIKE '%publican%' OR profession LIKE '%Publican%'
   OR profession LIKE '%innkeeper%' OR profession LIKE '%Innkeeper%'
   OR profession LIKE '%beerseller%' OR profession LIKE '%Beer Seller%'
   OR profession LIKE '%beer seller%' OR profession LIKE '%Beerseller%';

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_publican_person_id ON sheffield_publicans(sheffield_person_id);

-- Report results
SELECT 'Total publicans/innkeepers:' as label, COUNT(*) as count FROM sheffield_publicans;

SELECT 'Publicans by gender:' as label;
SELECT census_gender, COUNT(*) as count
FROM sheffield_publicans
WHERE census_gender IS NOT NULL
GROUP BY census_gender;

SELECT 'Top 10 pub addresses:' as label;
SELECT name, street_address, profession
FROM sheffield_publicans
WHERE street_address IS NOT NULL
ORDER BY street_address
LIMIT 10;
