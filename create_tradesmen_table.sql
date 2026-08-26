-- Create sheffield_tradesmen table
-- Populated with skilled metalworkers (cutlers, grinders, file makers, steel workers, silverplate workers)

CREATE TABLE IF NOT EXISTS sheffield_tradesmen (
    tradesman_id INTEGER PRIMARY KEY AUTOINCREMENT,
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
    trade_type TEXT,  -- cutler, grinder, file_maker, steel_worker, silverplate_worker
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

-- Populate with cutlers
INSERT INTO sheffield_tradesmen (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'cutler' as trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE profession LIKE '%cutler%' OR profession LIKE '%Cutler%';

-- Populate with grinders
INSERT INTO sheffield_tradesmen (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'grinder' as trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (profession LIKE '%grinder%' OR profession LIKE '%Grinder%')
  AND rowid NOT IN (SELECT sheffield_person_id FROM sheffield_tradesmen);

-- Populate with file makers
INSERT INTO sheffield_tradesmen (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'file_maker' as trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (profession LIKE '%file%' OR profession LIKE '%File%')
  AND rowid NOT IN (SELECT sheffield_person_id FROM sheffield_tradesmen);

-- Populate with steel workers
INSERT INTO sheffield_tradesmen (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'steel_worker' as trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (profession LIKE '%steel%' OR profession LIKE '%Steel%')
  AND rowid NOT IN (SELECT sheffield_person_id FROM sheffield_tradesmen);

-- Populate with silverplate workers
INSERT INTO sheffield_tradesmen (
    sheffield_person_id,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
)
SELECT
    rowid,
    name, first_name, middle_name, surname,
    census_gender, census_age, birth_year,
    profession, occupation_expanded,
    'silverplate_worker' as trade_type,
    street_address, house_number, sub_area, street_name, postcode,
    civil_parish, ecclesiastical_parish,
    census_folio, census_piece
FROM sheffield_people
WHERE (profession LIKE '%silver%' OR profession LIKE '%Silver%' OR profession LIKE '%plate%')
  AND rowid NOT IN (SELECT sheffield_person_id FROM sheffield_tradesmen);

-- Create index on sheffield_person_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_tradesman_person_id ON sheffield_tradesmen(sheffield_person_id);

-- Create index on trade_type for filtering by trade
CREATE INDEX IF NOT EXISTS idx_tradesman_trade_type ON sheffield_tradesmen(trade_type);

-- Report results
SELECT 'Total tradesmen:' as label, COUNT(*) as count FROM sheffield_tradesmen;

SELECT 'Breakdown by trade:' as label;
SELECT trade_type, COUNT(*) as count
FROM sheffield_tradesmen
GROUP BY trade_type
ORDER BY count DESC;
