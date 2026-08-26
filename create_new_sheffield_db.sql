-- Create NEW Sheffield1867.db with complete schema
-- All tables share a superset of columns from all data sources

-- Sheffield Clubs table
CREATE TABLE IF NOT EXISTS sheffield_clubs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    founded_year INTEGER,
    disbanded_year INTEGER,
    ground_name TEXT,
    origin TEXT,
    city TEXT,
    region TEXT,

    -- Postcode columns
    postcode TEXT,
    postcode_area TEXT,
    postcode_district TEXT,
    postcode_sector TEXT,
    postcode_unit TEXT,
    latitude REAL,
    longitude REAL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Superset of ALL columns for census, genealogy, and business data
-- Each import populates only its relevant columns

CREATE TABLE IF NOT EXISTS unmatched_ancestry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Name fields (expanded from abbreviations)
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,

    -- Census-specific fields
    census_age INTEGER,
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

    -- Address fields (normalized format: [number] [sub area] [street])
    street_address TEXT,
    house_number TEXT,
    sub_area TEXT,
    street_name TEXT,

    -- Birth/location info
    birth_year INTEGER,
    birth_town TEXT,
    birth_county TEXT,
    birth_country TEXT,
    where_born TEXT,

    -- Occupation
    profession TEXT,
    occupation_expanded TEXT,

    -- Genealogy fields (NULL for census data)
    genealogy_source TEXT,
    genealogy_id TEXT,

    -- Business fields (NULL for census data)
    business_name TEXT,
    business_type TEXT,

    -- Postcode columns
    postcode TEXT,
    postcode_area TEXT,
    postcode_district TEXT,
    postcode_sector TEXT,
    postcode_unit TEXT,
    latitude REAL,
    longitude REAL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unmatched_genealogy (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Name fields (expanded from abbreviations)
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,

    -- Census fields (NULL for genealogy data)
    census_age INTEGER,
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

    -- Address fields (normalized)
    street_address TEXT,
    house_number TEXT,
    sub_area TEXT,
    street_name TEXT,

    -- Birth/location info
    birth_year INTEGER,
    birth_town TEXT,
    birth_county TEXT,
    birth_country TEXT,
    where_born TEXT,

    -- Occupation (expanded abbreviations)
    profession TEXT,
    occupation_expanded TEXT,

    -- Genealogy-specific fields
    genealogy_source TEXT,
    genealogy_id TEXT,

    -- Business fields (NULL for genealogy data)
    business_name TEXT,
    business_type TEXT,

    -- Postcode columns
    postcode TEXT,
    postcode_area TEXT,
    postcode_district TEXT,
    postcode_sector TEXT,
    postcode_unit TEXT,
    latitude REAL,
    longitude REAL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sheffield_businesses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- Name fields (expanded from abbreviations in Whites data)
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,

    -- Census fields (NULL for business data)
    census_age INTEGER,
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

    -- Address fields (normalized)
    street_address TEXT,
    house_number TEXT,
    sub_area TEXT,
    street_name TEXT,

    -- Birth/location info
    birth_year INTEGER,
    birth_town TEXT,
    birth_county TEXT,
    birth_country TEXT,
    where_born TEXT,

    -- Occupation
    profession TEXT,
    occupation_expanded TEXT,

    -- Genealogy fields (NULL for business data)
    genealogy_source TEXT,
    genealogy_id TEXT,

    -- Business-specific fields
    business_name TEXT,
    business_type TEXT,

    -- Postcode columns
    postcode TEXT,
    postcode_area TEXT,
    postcode_district TEXT,
    postcode_sector TEXT,
    postcode_unit TEXT,
    latitude REAL,
    longitude REAL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Institutional tables (empty, same column structure)
CREATE TABLE IF NOT EXISTS sheffield_asylum (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    census_age INTEGER,
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
    birth_year INTEGER,
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sheffield_workhouse (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    census_age INTEGER,
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
    birth_year INTEGER,
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sheffield_prison (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    census_age INTEGER,
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
    birth_year INTEGER,
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sheffield_patron (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    census_age INTEGER,
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
    birth_year INTEGER,
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
