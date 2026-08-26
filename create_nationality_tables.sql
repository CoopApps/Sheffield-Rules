-- Create nationality tables for Scottish, Welsh, and German immigrants
-- These provide data for the game engine to use however it wants

-- ============================================
-- SCOTTISH TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS sheffield_scottish (
    scottish_id INTEGER PRIMARY KEY AUTOINCREMENT,
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

INSERT INTO sheffield_scottish (
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
WHERE birth_country LIKE '%Scotland%' OR birth_country LIKE '%scotland%'
   OR where_born LIKE '%Scotland%' OR where_born LIKE '%scotland%';

CREATE INDEX IF NOT EXISTS idx_scottish_person_id ON sheffield_scottish(sheffield_person_id);

-- ============================================
-- WELSH TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS sheffield_welsh (
    welsh_id INTEGER PRIMARY KEY AUTOINCREMENT,
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

INSERT INTO sheffield_welsh (
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
WHERE birth_country LIKE '%Wales%' OR birth_country LIKE '%wales%'
   OR where_born LIKE '%Wales%' OR where_born LIKE '%wales%';

CREATE INDEX IF NOT EXISTS idx_welsh_person_id ON sheffield_welsh(sheffield_person_id);

-- ============================================
-- GERMAN TABLE
-- ============================================
CREATE TABLE IF NOT EXISTS sheffield_german (
    german_id INTEGER PRIMARY KEY AUTOINCREMENT,
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

INSERT INTO sheffield_german (
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
WHERE birth_country LIKE '%German%' OR birth_country LIKE '%german%'
   OR where_born LIKE '%German%' OR where_born LIKE '%german%';

CREATE INDEX IF NOT EXISTS idx_german_person_id ON sheffield_german(sheffield_person_id);

-- ============================================
-- REPORT RESULTS
-- ============================================
SELECT 'Total Scottish:' as label, COUNT(*) as count FROM sheffield_scottish;
SELECT 'Scottish by age group:' as label;
SELECT
    CASE
        WHEN census_age < 14 THEN 'Children (under 14)'
        WHEN census_age BETWEEN 14 AND 40 THEN 'Football age (14-40)'
        WHEN census_age > 40 THEN 'Older (40+)'
        ELSE 'Unknown age'
    END as age_group,
    COUNT(*) as count
FROM sheffield_scottish
GROUP BY age_group;

SELECT 'Total Welsh:' as label, COUNT(*) as count FROM sheffield_welsh;
SELECT 'Welsh by age group:' as label;
SELECT
    CASE
        WHEN census_age < 14 THEN 'Children (under 14)'
        WHEN census_age BETWEEN 14 AND 40 THEN 'Football age (14-40)'
        WHEN census_age > 40 THEN 'Older (40+)'
        ELSE 'Unknown age'
    END as age_group,
    COUNT(*) as count
FROM sheffield_welsh
GROUP BY age_group;

SELECT 'Total German:' as label, COUNT(*) as count FROM sheffield_german;
SELECT 'German by age group:' as label;
SELECT
    CASE
        WHEN census_age < 14 THEN 'Children (under 14)'
        WHEN census_age BETWEEN 14 AND 40 THEN 'Football age (14-40)'
        WHEN census_age > 40 THEN 'Older (40+)'
        ELSE 'Unknown age'
    END as age_group,
    COUNT(*) as count
FROM sheffield_german
GROUP BY age_group;

-- Top Scottish surnames
SELECT 'Top 10 Scottish surnames:' as label;
SELECT surname, COUNT(*) as count
FROM sheffield_scottish
WHERE surname IS NOT NULL
GROUP BY surname
ORDER BY count DESC
LIMIT 10;

-- Top Welsh surnames
SELECT 'Top 10 Welsh surnames:' as label;
SELECT surname, COUNT(*) as count
FROM sheffield_welsh
WHERE surname IS NOT NULL
GROUP BY surname
ORDER BY count DESC
LIMIT 10;
