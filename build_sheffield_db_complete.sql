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


-- Sheffield Clubs Data with Reserves
-- Auto-generated from src-tauri/src/sheffield_rules/clubs.rs
-- 186 main clubs + 186 reserve teams = 372 total

INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-fc', 'Sheffield FC', 1857, 'East Bank', 'played at East Bank', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-fc-reserves', 'Sheffield FC Reserves', 1857, 'East Bank', 'played at East Bank (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hallam-fc', 'Hallam FC', 1860, 'Sandygate', 'play at Sandygate', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hallam-fc-reserves', 'Hallam FC Second XI', 1860, 'Sandygate', 'play at Sandygate (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norfolk-fc', 'Norfolk FC', 1861, 'Norfolk Park', 'played at Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norfolk-fc-reserves', 'Norfolk FC B Team', 1861, 'Norfolk Park', 'played at Norfolk Park (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('cemetery-road-church-fc', 'Cemetery Road Church FC', 1861, 'Hunters Bar', 'Oldest church club, played at Hunters Bar', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('cemetery-road-church-fc-reserves', 'Cemetery Road Church FC Juniors', 1861, 'Hunters Bar', 'Oldest church club, played at Hunters Bar (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('york-fc', 'York FC', 1861, 'Endcliffe Cricket Ground', 'From York Hotel, Broomhill, played at Endcliﬀe Cricket Ground, N Creswick was President', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('york-fc-reserves', 'York FC Second Team', 1861, 'Endcliffe Cricket Ground', 'From York Hotel, Broomhill, played at Endcliﬀe Cricket Ground, N Creswick was President (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norton-fc', 'Norton FC', 1861, 'Oaks Park', 'played at Oaks Park, Norton', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norton-fc-reserves', 'Norton FC Reserve XI', 1861, 'Oaks Park', 'played at Oaks Park, Norton (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pitsmoor-fc', 'Pitsmoor FC', 1861, 'Pitsmoor CC', 'played at Pitsmoor CC, now SUFC Academy', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pitsmoor-fc-reserves', 'Pitsmoor FC Junior XI', 1861, 'Pitsmoor CC', 'played at Pitsmoor CC, now SUFC Academy (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('fir-vale-fc', 'Fir Vale FC', 1862, 'Pitsmoor CC', 'played at Pitsmoor CC, now SUFC Academy', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('fir-vale-fc-reserves', 'Fir Vale FC Colts', 1862, 'Pitsmoor CC', 'played at Pitsmoor CC, now SUFC Academy (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('heeley-christ-church-fc', 'Heeley Christ Church FC', 1862, 'Meersbrook Park', 'played at Meersbrook Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('heeley-christ-church-fc-reserves', 'Heeley Christ Church FC Reserves', 1862, 'Meersbrook Park', 'played at Meersbrook Park (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('mackenzie-fc', 'Mackenzie FC', 1862, 'Myrtle Road, Heeley', 'played at Myrtle Road, Heeley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('mackenzie-fc-reserves', 'Mackenzie FC Second XI', 1862, 'Myrtle Road, Heeley', 'played at Myrtle Road, Heeley (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('milton-fc', 'Milton FC', 1862, 'Cremorne Gardens, London Road', 'played at Cremorne Gardens, London Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('milton-fc-reserves', 'Milton FC B Team', 1862, 'Cremorne Gardens, London Road', 'played at Cremorne Gardens, London Road (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('howard-hill-steel-bank-fc', 'Howard Hill Steel Bank FC', 1862, 'Howard Hotel, Howard Road', 'Met in Howard Hotel, Howard Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('howard-hill-steel-bank-fc-reserves', 'Howard Hill Steel Bank FC Juniors', 1862, 'Howard Hotel, Howard Road', 'Met in Howard Hotel, Howard Road (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ranmoor-fc', 'Ranmoor FC', 1862, 'Ranmoor', 'played in Ranmoor', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ranmoor-fc-reserves', 'Ranmoor FC Second Team', 1862, 'Ranmoor', 'played in Ranmoor (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-george-fc', 'St George FC', 1862, 'Broad Lane', 'From St George''s Church, Broad Lane.', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-george-fc-reserves', 'St George FC Reserve XI', 1862, 'Broad Lane', 'From St George''s Church, Broad Lane. (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-stephen-fc', 'St Stephen FC', 1862, 'Crookes', 'From St Stephen''s Church Fawcett Road Netherthorpe. Played at Crookes.', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-stephen-fc-reserves', 'St Stephen FC Junior XI', 1862, 'Crookes', 'From St Stephen''s Church Fawcett Road Netherthorpe. Played at Crookes. (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('united-norfolk-fc', 'United Norfolk FC', 1862, 'Unknown', 'Origins uncertain', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('united-norfolk-fc-reserves', 'United Norfolk FC Colts', 1862, 'Unknown', 'Origins uncertain (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crabtree-fc', 'Crabtree FC', 1863, 'Fir Vale area', 'Likely from Fir Vale area', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crabtree-fc-reserves', 'Crabtree FC Reserves', 1863, 'Fir Vale area', 'Likely from Fir Vale area (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('broomhall-fc', 'Broomhall FC', 1863, 'Ecclesall Road', 'played at Ecclesall Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('broomhall-fc-reserves', 'Broomhall FC Second XI', 1863, 'Ecclesall Road', 'played at Ecclesall Road (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tudor-fc', 'Tudor FC', 1863, 'Unknown', 'Other', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tudor-fc-reserves', 'Tudor FC B Team', 1863, 'Unknown', 'Other (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('w-h-hutchinson-fc', 'W & H Hutchinson''s FC', 1863, 'Unknown', 'Works', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('w-h-hutchinson-fc-reserves', 'W & H Hutchinson''''s FC Juniors', 1863, 'Unknown', 'Works (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hemsworth-fc', 'Hemsworth FC', 1863, 'Unknown', 'Other', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hemsworth-fc-reserves', 'Hemsworth FC Second Team', 1863, 'Unknown', 'Other (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('united-mechanics', 'United Mechanics', 1865, 'Norfolk Park', 'played at Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('united-mechanics-reserves', 'United Mechanics Reserve XI', 1865, 'Norfolk Park', 'played at Norfolk Park (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('garrick-fc', 'Garrick FC', 1866, 'East Bank', 'played at East Bank', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('garrick-fc-reserves', 'Garrick FC Junior XI', 1866, 'East Bank', 'played at East Bank (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wellington-fc', 'Wellington FC', 1866, 'Hounsfield Park', 'played at Hounsﬁeld Park near Bramall Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wellington-fc-reserves', 'Wellington FC Colts', 1866, 'Hounsfield Park', 'played at Hounsﬁeld Park near Bramall Lane (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('loxley-fc', 'Loxley FC', 1866, 'The Rodney Inn, Loxley', 'Met at The Rodney Inn, Loxley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('loxley-fc-reserves', 'Loxley FC Reserves', 1866, 'The Rodney Inn, Loxley', 'Met at The Rodney Inn, Loxley (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wednesday-fc', 'Wednesday FC', 1867, 'Highfields, now Hillsborough', 'played at Highﬁelds, now Hillsborough', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wednesday-fc-reserves', 'Wednesday FC Second XI', 1867, 'Highfields, now Hillsborough', 'played at Highﬁelds, now Hillsborough (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('exchange-fc', 'Exchange FC', 1867, 'Hallam''s Farm, now Hyde Park Flats', 'played at Hallam''s Farm, now Hyde Park Flats', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('exchange-fc-reserves', 'Exchange FC B Team', 1867, 'Hallam''s Farm, now Hyde Park Flats', 'played at Hallam''s Farm, now Hyde Park Flats (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dore-fc', 'Dore FC', 1867, 'The Devonshire Arms, Dore', 'Met at The Devonshire Arms, Dore', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dore-fc-reserves', 'Dore FC Juniors', 1867, 'The Devonshire Arms, Dore', 'Met at The Devonshire Arms, Dore (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tapton-fc', 'Tapton FC', 1867, 'Tapton Hall', 'Based at Tapton Hall', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tapton-fc-reserves', 'Tapton FC Second Team', 1867, 'Tapton Hall', 'Based at Tapton Hall (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-fc', 'Dronfield FC', 1868, 'Bagley''s Field, Dronfield', 'played at Bagley''s Field, Dronfield', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-fc-reserves', 'Dronfield FC Reserve XI', 1868, 'Bagley''s Field, Dronfield', 'played at Bagley''s Field, Dronfield (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brincliffe-fc', 'Brincliffe FC', 1868, 'Cherry Tree Farm', 'played at Cherry Tree Farm, near Union pub', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brincliffe-fc-reserves', 'Brincliffe FC Junior XI', 1868, 'Cherry Tree Farm', 'played at Cherry Tree Farm, near Union pub (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hanover-united-fc', 'Hanover United FC', 1868, 'Crookes', 'played at Crookes', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hanover-united-fc-reserves', 'Hanover United FC Colts', 1868, 'Crookes', 'played at Crookes (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('stannington-fc', 'Stannington FC', 1868, 'Unknown', 'unknown ground loca_on', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('stannington-fc-reserves', 'Stannington FC Reserves', 1868, 'Unknown', 'unknown ground loca_on (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('redhill-fc', 'Redhill FC', 1868, 'Winter Street, near Weston Park', 'played at Winter Street, near Weston Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('redhill-fc-reserves', 'Redhill FC Second XI', 1868, 'Winter Street, near Weston Park', 'played at Winter Street, near Weston Park (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('parkwood-springs-fc', 'Parkwood Springs FC', 1869, 'Parkwood Springs Recreation Ground', 'played at Parkwood Springs Recrea_on Ground', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('parkwood-springs-fc-reserves', 'Parkwood Springs FC B Team', 1869, 'Parkwood Springs Recreation Ground', 'played at Parkwood Springs Recrea_on Ground (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oxford-fc', 'Oxford FC', 1869, 'Ecclesall Road', 'played at Ecclesall Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oxford-fc-reserves', 'Oxford FC Juniors', 1869, 'Ecclesall Road', 'played at Ecclesall Road (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('totley-fc', 'Totley FC', 1869, 'Cross Scythes Inn, Totley', 'played at ﬁeld next to Cross Scythes Inn, Totley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('totley-fc-reserves', 'Totley FC Second Team', 1869, 'Cross Scythes Inn, Totley', 'played at ﬁeld next to Cross Scythes Inn, Totley (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-norfolk-fc', 'Sheffield Norfolk FC', 1869, 'Unknown', 'Other', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-norfolk-fc-reserves', 'Sheffield Norfolk FC Reserve XI', 1869, 'Unknown', 'Other (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-vincent-fc', 'St Vincent''s', 1869, 'Queens Ground', 'from Solly Street – played at Queens Ground', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-vincent-fc-reserves', 'St Vincent''''s Junior XI', 1869, 'Queens Ground', 'from Solly Street – played at Queens Ground (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-james-church-fc', 'St James Church FC', 1869, 'Norton Lees Lane', 'played at Norton Lees Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-james-church-fc-reserves', 'St James Church FC Colts', 1869, 'Norton Lees Lane', 'played at Norton Lees Lane (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('lockwood-brothers-fc', 'Lockwood Brothers FC', 1870, 'Hunters Bar', 'played at Hunters Bar, oldest works club', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('lockwood-brothers-fc-reserves', 'Lockwood Brothers FC Reserves', 1870, 'Hunters Bar', 'played at Hunters Bar, oldest works club (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('talbot-fc', 'Talbot FC', 1870, 'Norfolk Road', 'played at Norfolk Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('talbot-fc-reserves', 'Talbot FC Second XI', 1870, 'Norfolk Road', 'played at Norfolk Road (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-united-gymnastic-club', 'Sheffield United Gymnastic Club', 1870, 'Unknown', 'Other', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-united-gymnastic-club-reserves', 'Sheffield United Gymnastic Club B Team', 1870, 'Unknown', 'Other (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-college-fc', 'Ecclesall College FC', 1870, 'Unknown', 'School', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-college-fc-reserves', 'Ecclesall College FC Juniors', 1870, 'Unknown', 'School (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-grammar-school-fc', 'Sheffield Grammar School FC', 1870, 'Unknown', 'School', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheffield-grammar-school-fc-reserves', 'Sheffield Grammar School FC Second Team', 1870, 'Unknown', 'School (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('surrey-catholic-club', 'Surrey Catholic Club', 1870, 'The Farm, now Sheffield College', 'played at The Farm, now Sheﬃeld College', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('surrey-catholic-club-reserves', 'Surrey Catholic Club Reserve XI', 1870, 'The Farm, now Sheffield College', 'played at The Farm, now Sheﬃeld College (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('attercliffe-christ-church', 'Attercliffe (Christ Church)', 1870, 'The Old Forge Ground, Shirland Lane', 'played at The Old Forge Ground, Shirland Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('attercliffe-christ-church-reserves', 'Attercliffe (Christ Church) Junior XI', 1870, 'The Old Forge Ground, Shirland Lane', 'played at The Old Forge Ground, Shirland Lane (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('walkey-new-connexion-fc', 'Walkey New Connexion FC', 1870, 'Queens Ground, Hillsborough', 'played at Queens Ground, Hillsborough', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('walkey-new-connexion-fc-reserves', 'Walkey New Connexion FC Colts', 1870, 'Queens Ground, Hillsborough', 'played at Queens Ground, Hillsborough (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crookes-fc', 'Crookes FC', 1870, 'Lydgate Lane', 'played at Lydgate Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crookes-fc-reserves', 'Crookes FC Reserves', 1870, 'Lydgate Lane', 'played at Lydgate Lane (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bankers-thursday', 'Bankers / Thursday', 1870, 'Hunters Bar', 'played at Hunters Bar', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bankers-thursday-reserves', 'Bankers / Thursday Second XI', 1870, 'Hunters Bar', 'played at Hunters Bar (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('alliance-fc', 'Alliance FC', 1870, 'Norfolk Park', 'played at Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('alliance-fc-reserves', 'Alliance FC B Team', 1870, 'Norfolk Park', 'played at Norfolk Park (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('perseverance-fc', 'Perseverance FC', 1870, 'Norfolk Park', 'played at Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('perseverance-fc-reserves', 'Perseverance FC Juniors', 1870, 'Norfolk Park', 'played at Norfolk Park (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('gleadless-fc', 'Gleadless FC', 1870, 'Charnock Hall, Gleadless', 'played at Charnock Hall, Gleadless', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('gleadless-fc-reserves', 'Gleadless FC Second Team', 1870, 'Charnock Hall, Gleadless', 'played at Charnock Hall, Gleadless (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('engineers-fc', 'Engineers FC', 1870, 'Endcliffe Crescent', 'Played at Endcliﬀe Crescent', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('engineers-fc-reserves', 'Engineers FC Reserve XI', 1870, 'Endcliffe Crescent', 'Played at Endcliﬀe Crescent (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('attercliffe-zion-fc', 'Attercliffe Zion FC', 1871, 'Unknown', 'from Zion Church, Attercliffe', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('attercliffe-zion-fc-reserves', 'Attercliffe Zion FC Junior XI', 1871, 'Unknown', 'from Zion Church, Attercliffe (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('grimesthorpe-fc', 'Grimesthorpe FC', 1871, 'Victoria Hotel, Grimesthorpe', 'Met at Victoria Hotel, Grimesthorpe', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('grimesthorpe-fc-reserves', 'Grimesthorpe FC Colts', 1871, 'Victoria Hotel, Grimesthorpe', 'Met at Victoria Hotel, Grimesthorpe (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('exchange-brewery-fc', 'Exchange Brewery FC', 1871, 'Fox Street, Pye Bank', 'played at Fox Street, Pye Bank', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('exchange-brewery-fc-reserves', 'Exchange Brewery FC Reserves', 1871, 'Fox Street, Pye Bank', 'played at Fox Street, Pye Bank (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('all-saints-night-school-fc', 'All Saints Night School FC', 1871, 'Hall Carr Lane, now Carwood Road', 'Hall Carr Lane, now Carwood Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('all-saints-night-school-fc-reserves', 'All Saints Night School FC Second XI', 1871, 'Hall Carr Lane, now Carwood Road', 'Hall Carr Lane, now Carwood Road (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('millhouses-fc', 'Millhouses FC', 1871, 'The Old Corn Mill', 'possibly played at The Old Corn Mill', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('millhouses-fc-reserves', 'Millhouses FC B Team', 1871, 'The Old Corn Mill', 'possibly played at The Old Corn Mill (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('albion-fc', 'Albion FC', 1872, 'Ecclesall Road', 'played at Ecclesall Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('albion-fc-reserves', 'Albion FC Juniors', 1872, 'Ecclesall Road', 'played at Ecclesall Road (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pye-bank-fc', 'Pye Bank FC', 1872, 'Fox Street, Pye Bank', 'played at Fox Street, Pye Bank', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pye-bank-fc-reserves', 'Pye Bank FC Second Team', 1872, 'Fox Street, Pye Bank', 'played at Fox Street, Pye Bank (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brightside-fc', 'Brightside FC', 1872, 'Blackburn Meadows', 'played at Blackburn Meadows, Blackburn', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brightside-fc-reserves', 'Brightside FC Reserve XI', 1872, 'Blackburn Meadows', 'played at Blackburn Meadows, Blackburn (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norfolk-works-fc', 'Norfolk Works FC', 1872, 'Newhall Athletic Ground', 'Played at Newhall Athle_c Ground', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norfolk-works-fc-reserves', 'Norfolk Works FC Junior XI', 1872, 'Newhall Athletic Ground', 'Played at Newhall Athle_c Ground (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('eldon-st-jude-fc', 'Eldon St Jude''s FC', 1872, 'Brocco Bank', 'played at Brocco Bank', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('eldon-st-jude-fc-reserves', 'Eldon St Jude''''s FC Colts', 1872, 'Brocco Bank', 'played at Brocco Bank (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('garden-street-fc', 'Garden Street FC', 1872, 'Hollins Crog', 'From Garden Street, Hollins Crog', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('garden-street-fc-reserves', 'Garden Street FC Reserves', 1872, 'Hollins Crog', 'From Garden Street, Hollins Crog (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sharrow-rangers-fc', 'Sharrow Rangers FC', 1873, 'Crescent Road, Sharrow', 'played at Crescent Road, Sharrow', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sharrow-rangers-fc-reserves', 'Sharrow Rangers FC Second XI', 1873, 'Crescent Road, Sharrow', 'played at Crescent Road, Sharrow (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('endcliffe-fc', 'Endcliffe FC', 1873, 'Ecclesall Road', 'played at Ecclesall Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('endcliffe-fc-reserves', 'Endcliffe FC B Team', 1873, 'Ecclesall Road', 'played at Ecclesall Road (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('owlerton-fc', 'Owlerton FC', 1873, 'Rawson''s Meadow Ground, Owlerton', 'played at Rawson''s Meadow Ground, Owlerton', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('owlerton-fc-reserves', 'Owlerton FC Juniors', 1873, 'Rawson''s Meadow Ground, Owlerton', 'played at Rawson''s Meadow Ground, Owlerton (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesfield-fc', 'Ecclesfield FC', 1873, 'Fairham''s Crog', 'played at Fairham''s Crog', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesfield-fc-reserves', 'Ecclesfield FC Second Team', 1873, 'Fairham''s Crog', 'played at Fairham''s Crog (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('philadelphia-fc', 'Philadelphia FC', 1873, 'Queens Ground, Hillsborough', 'played at Queens Ground Hillsborough', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('philadelphia-fc-reserves', 'Philadelphia FC Reserve XI', 1873, 'Queens Ground, Hillsborough', 'played at Queens Ground Hillsborough (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('artillery-hallamshire-fc', 'Artillery & Hallamshire FC', 1873, 'Endcliffe Hall', 'Based at Endcliﬀe Hall (later just Artillery)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('artillery-hallamshire-fc-reserves', 'Artillery & Hallamshire FC Junior XI', 1873, 'Endcliffe Hall', 'Based at Endcliﬀe Hall (later just Artillery) (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('intake-fc', 'Intake FC', 1873, 'Intake', 'From Intake', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('intake-fc-reserves', 'Intake FC Colts', 1873, 'Intake', 'From Intake (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('cherrytree-fc', 'Cherrytree FC', 1873, 'Cherrytree Orphanage', 'Possibly from Cherrytree Orphanage', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('cherrytree-fc-reserves', 'Cherrytree FC Reserves', 1873, 'Cherrytree Orphanage', 'Possibly from Cherrytree Orphanage (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('victoria-burngreave-fc', 'Victoria (Burngreave) FC', 1873, 'Hall Carr Lane', 'played at Hall Carr Lane, east end', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('victoria-burngreave-fc-reserves', 'Victoria (Burngreave) FC Second XI', 1873, 'Hall Carr Lane', 'played at Hall Carr Lane, east end (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('west-end-fc', 'West End FC', 1873, 'Hunters Bar', 'played at Hunters Bar, from West End Hotel', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('west-end-fc-reserves', 'West End FC B Team', 1873, 'Hunters Bar', 'played at Hunters Bar, from West End Hotel (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bury-co-fc', 'Bury''s & Co FC', 1873, 'Regents Works', 'From Regents Works, now Wicks', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bury-co-fc-reserves', 'Bury''''s & Co FC Juniors', 1873, 'Regents Works', 'From Regents Works, now Wicks (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('beadshaw-baltic-fc', 'Beadshaw''s (Baltic) FC', 1873, 'Baltic Works, Attercliffe', 'From Baltic Works, Attercliffe', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('beadshaw-baltic-fc-reserves', 'Beadshaw''''s (Baltic) FC Second Team', 1873, 'Baltic Works, Attercliffe', 'From Baltic Works, Attercliffe (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oughtibridge-fc', 'Oughtibridge FC', 1873, 'Oughtibridge', 'From Oughtibridge', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oughtibridge-fc-reserves', 'Oughtibridge FC Reserve XI', 1873, 'Oughtibridge', 'From Oughtibridge (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('weston-fc', 'Weston FC', 1873, 'Weston Hall', 'Possibly played at Weston Hall', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('weston-fc-reserves', 'Weston FC Junior XI', 1873, 'Weston Hall', 'Possibly played at Weston Hall (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wardsend-steel-works-fc', 'Wardsend Steel Works FC', 1873, 'Herries Road', 'From Wardsend Steel Works, Herries Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wardsend-steel-works-fc-reserves', 'Wardsend Steel Works FC Colts', 1873, 'Herries Road', 'From Wardsend Steel Works, Herries Road (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('owlerton-reform-fc', 'Owlerton Reform FC', 1873, 'Borough Road, Owlerton', 'From Wesleyan Reform Church on Borough Road, Owlerton', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('owlerton-reform-fc-reserves', 'Owlerton Reform FC Reserves', 1873, 'Borough Road, Owlerton', 'From Wesleyan Reform Church on Borough Road, Owlerton (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('roebuck-fc', 'Roebuck FC', 1873, 'East Bank', 'Roebuck pub, played at East Bank', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('roebuck-fc-reserves', 'Roebuck FC Second XI', 1873, 'East Bank', 'Roebuck pub, played at East Bank (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tennant-brothers-fc', 'Tennant Brothers & Co FC', 1873, 'Exchange Brewery', 'club from Exchange Brewery', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tennant-brothers-fc-reserves', 'Tennant Brothers & Co FC B Team', 1873, 'Exchange Brewery', 'club from Exchange Brewery (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crosspool-rangers-fc', 'Crosspool Rangers FC', 1873, 'Crosspool', 'From Crosspool', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crosspool-rangers-fc-reserves', 'Crosspool Rangers FC Juniors', 1873, 'Crosspool', 'From Crosspool (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('clifford-fc', 'Clifford FC', 1873, 'Psalter Lane', 'From Clifford House, Psalter Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('clifford-fc-reserves', 'Clifford FC Second Team', 1873, 'Psalter Lane', 'From Clifford House, Psalter Lane (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('grange-fc', 'Grange FC', 1873, 'Intake Road', 'played at Intake Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('grange-fc-reserves', 'Grange FC Reserve XI', 1873, 'Intake Road', 'played at Intake Road (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pitsmoor-coal-company-fc', 'Pitsmoor Coal Company FC', 1873, 'Brightside', 'From Brightside', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pitsmoor-coal-company-fc-reserves', 'Pitsmoor Coal Company FC Junior XI', 1873, 'Brightside', 'From Brightside (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('boston-street-fc', 'Boston Street FC', 1873, 'Heeley', 'From Boston Street, played in Heeley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('boston-street-fc-reserves', 'Boston Street FC Colts', 1873, 'Heeley', 'From Boston Street, played in Heeley (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('birley-fc', 'Birley FC', 1873, 'Hollinsend', 'played at Hollinsend, Birley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('birley-fc-reserves', 'Birley FC Reserves', 1873, 'Hollinsend', 'played at Hollinsend, Birley (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sherrington-fc', 'Sherrington FC', 1873, 'Norfolk Park', 'From Sherrington Road, Sharrow, played Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sherrington-fc-reserves', 'Sherrington FC Second XI', 1873, 'Norfolk Park', 'From Sherrington Road, Sharrow, played Norfolk Park (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('105th-regiment', '105th Regiment', 1874, 'Unknown', 'Joined FA in 1874. Played in FA Cup 1875 to 1879.Nickname was ''Light Bobs''', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('105th-regiment-reserves', '105th Regiment B Team', 1874, 'Unknown', 'Joined FA in 1874. Played in FA Cup 1875 to 1879.Nickname was ''Light Bobs'' (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('carnforth-fc', 'Carnforth FC', 1874, 'Sharrow Vale Road', 'played at Sharrow Vale Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('carnforth-fc-reserves', 'Carnforth FC Juniors', 1874, 'Sharrow Vale Road', 'played at Sharrow Vale Road (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hollinsend-fc', 'Hollinsend FC', 1874, 'Unknown', 'Other', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hollinsend-fc-reserves', 'Hollinsend FC Second Team', 1874, 'Unknown', 'Other (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('providence-fc', 'Providence FC', 1874, 'Park Hill Lane', 'played at Park Hill Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('providence-fc-reserves', 'Providence FC Reserve XI', 1874, 'Park Hill Lane', 'played at Park Hill Lane (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('handsworth-fc', 'Handsworth FC', 1874, 'Unknown', 'Ground unknown', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('handsworth-fc-reserves', 'Handsworth FC Junior XI', 1874, 'Unknown', 'Ground unknown (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('woodseats-fc', 'Woodseats FC', 1874, 'Woodseats Hotel', 'Met at the Woodseats Hotel, now Viraaj', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('woodseats-fc-reserves', 'Woodseats FC Colts', 1874, 'Woodseats Hotel', 'Met at the Woodseats Hotel, now Viraaj (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oak-street-fc', 'Oak Street FC', 1874, 'Heeley', 'From street of same name, Heeley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oak-street-fc-reserves', 'Oak Street FC Reserves', 1874, 'Heeley', 'From street of same name, Heeley (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('atlas-fc', 'Atlas FC', 1874, 'East End', 'formed from Atlas Works, East End', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('atlas-fc-reserves', 'Atlas FC Second XI', 1874, 'East End', 'formed from Atlas Works, East End (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-fc', 'Ecclesall FC', 1874, 'Hunters Bar', 'played at Hunters Bar', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-fc-reserves', 'Ecclesall FC B Team', 1874, 'Hunters Bar', 'played at Hunters Bar (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('nether-fc', 'Nether FC', 1874, 'Eastborne', 'played at Eastborne', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('nether-fc-reserves', 'Nether FC Juniors', 1874, 'Eastborne', 'played at Eastborne (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('broomfield-fc', 'Broomfield FC', 1874, 'Broomfield', 'From Broomfield area of Sheffield', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('broomfield-fc-reserves', 'Broomfield FC Second Team', 1874, 'Broomfield', 'From Broomfield area of Sheffield (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-mark-fc', 'St Mark''s FC', 1874, 'Broomfield Road', 'From St Mark''s Broomfield Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-mark-fc-reserves', 'St Mark''''s FC Reserve XI', 1874, 'Broomfield Road', 'From St Mark''s Broomfield Road (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tabernacle-fc', 'Tabernacle FC', 1874, 'Albert Terrace Road', 'From Tabernacle Church, Albert Terrace Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('tabernacle-fc-reserves', 'Tabernacle FC Junior XI', 1874, 'Albert Terrace Road', 'From Tabernacle Church, Albert Terrace Road (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-michael-angels-fc', 'St Michaels Angels FC', 1874, 'Parkwood Springs', 'From St Michael''s Church, Parkwood Springs', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-michael-angels-fc-reserves', 'St Michaels Angels FC Colts', 1874, 'Parkwood Springs', 'From St Michael''s Church, Parkwood Springs (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('kenwood-fc', 'Kenwood FC', 1874, 'Abbeydale Road', 'played at a ground oﬀ Abbeydale Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('kenwood-fc-reserves', 'Kenwood FC Reserves', 1874, 'Abbeydale Road', 'played at a ground oﬀ Abbeydale Road (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('port-mahon-fc', 'Port Mahon FC', 1874, 'Port Mahon', 'From Port Mahon, now Ponderosa', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('port-mahon-fc-reserves', 'Port Mahon FC Second XI', 1874, 'Port Mahon', 'From Port Mahon, now Ponderosa (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-luke-fc', 'St Luke''s FC', 1874, 'Park', 'From St Luke''s, Park (behind Midland Station),', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-luke-fc-reserves', 'St Luke''''s FC B Team', 1874, 'Park', 'From St Luke''s, Park (behind Midland Station), (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('heeley-victoria-fc', 'Heeley Victoria FC', 1874, 'Heeley', 'From Victoria Hotel in Heeley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('heeley-victoria-fc-reserves', 'Heeley Victoria FC Juniors', 1874, 'Heeley', 'From Victoria Hotel in Heeley (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('shrewsbury-road-fc', 'Shrewsbury Road FC', 1874, 'Behind Midland Station', 'Road of same name behind Midland station', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('shrewsbury-road-fc-reserves', 'Shrewsbury Road FC Second Team', 1874, 'Behind Midland Station', 'Road of same name behind Midland station (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('park-united-fc', 'Park United FC', 1874, 'Park', 'From Park area', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('park-united-fc-reserves', 'Park United FC Reserve XI', 1874, 'Park', 'From Park area (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('white-star-fc', 'White Star FC', 1874, 'Unknown', 'Location origins unknown', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('white-star-fc-reserves', 'White Star FC Junior XI', 1874, 'Unknown', 'Location origins unknown (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('good-intent-fc', 'Good Intent FC', 1874, 'Truro Ground, Matilda Street', 'Likely played at Truro Ground on Matilda Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('good-intent-fc-reserves', 'Good Intent FC Colts', 1874, 'Truro Ground, Matilda Street', 'Likely played at Truro Ground on Matilda Street (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('mount-tabor-fc', 'Mount Tabor FC', 1874, 'City Centre', 'Church in city centre, now demolish', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('mount-tabor-fc-reserves', 'Mount Tabor FC Reserves', 1874, 'City Centre', 'Church in city centre, now demolish (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-jude-fc', 'St Jude''s FC', 1874, 'Cupola Street', 'From St Jude''s church on Cupola Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-jude-fc-reserves', 'St Jude''''s FC Second XI', 1874, 'Cupola Street', 'From St Jude''s church on Cupola Street (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wingfield-rowbotham-fc', 'Wingfield & Rowbotham FC', 1874, 'Tenter Street', 'From their works on Tenter Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wingfield-rowbotham-fc-reserves', 'Wingfield & Rowbotham FC B Team', 1874, 'Tenter Street', 'From their works on Tenter Street (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('young-broomhall-fc', 'Young Broomhall FC', 1874, 'Unknown', 'Ground location unknown', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('young-broomhall-fc-reserves', 'Young Broomhall FC Juniors', 1874, 'Unknown', 'Ground location unknown (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('regents-works-fc', 'Regents Works FC', 1874, 'Penistone Road', 'Regents Works, now Wicks on Penistone Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('regents-works-fc-reserves', 'Regents Works FC Second Team', 1874, 'Penistone Road', 'Regents Works, now Wicks on Penistone Road (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('lo-good-templars-fc', 'L. O. Good Templars FC', 1874, 'Unknown', 'Origins unknown', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('lo-good-templars-fc-reserves', 'L. O. Good Templars FC Reserve XI', 1874, 'Unknown', 'Origins unknown (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brookes-crookes-fc', 'Brookes & Crookes FC', 1874, 'Brook Lane', 'From Brook Steel Works, Brook Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brookes-crookes-fc-reserves', 'Brookes & Crookes FC Junior XI', 1874, 'Brook Lane', 'From Brook Steel Works, Brook Lane (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('polar-star-fc', 'Polar Star FC', 1874, 'Norfolk Park', 'played at Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('polar-star-fc-reserves', 'Polar Star FC Colts', 1874, 'Norfolk Park', 'played at Norfolk Park (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norton-mount-view-fc', 'Norton Mount View FC', 1874, 'Norton Lees', 'From Mount View Methodists Church, Norton Lees', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('norton-mount-view-fc-reserves', 'Norton Mount View FC Reserves', 1874, 'Norton Lees', 'From Mount View Methodists Church, Norton Lees (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('mill-sands-fc', 'Mill Sands FC', 1874, 'Mill Sands Works', 'From Mill Sands Works, now Vulcan House', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('mill-sands-fc-reserves', 'Mill Sands FC Second XI', 1874, 'Mill Sands Works', 'From Mill Sands Works, now Vulcan House (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('fenton-brothers-fc', 'Fenton Brothers FC', 1874, 'East Street, Park', 'From their works, East Street, Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('fenton-brothers-fc-reserves', 'Fenton Brothers FC B Team', 1874, 'East Street, Park', 'From their works, East Street, Park (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sir-john-brown-fc', 'Sir John Brown''s FC', 1874, 'Osgathorpe', 'played at Osgathorpe, near Earl Marshall', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sir-john-brown-fc-reserves', 'Sir John Brown''''s FC Juniors', 1874, 'Osgathorpe', 'played at Osgathorpe, near Earl Marshall (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-silas-fc', 'St Silas FC', 1874, 'Broomhall', 'From St Silas Church, Broomhall', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-silas-fc-reserves', 'St Silas FC Second Team', 1874, 'Broomhall', 'From St Silas Church, Broomhall (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-church-fc', 'Ecclesall Church FC', 1874, 'Ecclesall', 'From Ecclesall All Saints', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-church-fc-reserves', 'Ecclesall Church FC Reserve XI', 1874, 'Ecclesall', 'From Ecclesall All Saints (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('firth-fc', 'Firth''s FC', 1874, 'East End', 'From Thomas Firth & Son''s, east end', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('firth-fc-reserves', 'Firth''''s FC Junior XI', 1874, 'East End', 'From Thomas Firth & Son''s, east end (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('owlerton-united-fc', 'Owlerton United FC', 1874, 'Wadsley Bridge', 'played at Wadsley Bridge', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('owlerton-united-fc-reserves', 'Owlerton United FC Colts', 1874, 'Wadsley Bridge', 'played at Wadsley Bridge (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('atkin-brothers-fc', 'Atkin Brothers FC', 1874, 'Matilda Street', 'From Truro Works, Matilda Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('atkin-brothers-fc-reserves', 'Atkin Brothers FC Reserves', 1874, 'Matilda Street', 'From Truro Works, Matilda Street (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('unitarian-fc', 'Unitarian FC', 1874, 'Norfolk Street', 'From Unitarian Church, Norfolk Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('unitarian-fc-reserves', 'Unitarian FC Second XI', 1874, 'Norfolk Street', 'From Unitarian Church, Norfolk Street (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('deep-pits-fc', 'Deep Pits FC', 1874, 'City Road', 'Near Manor Top, on City Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('deep-pits-fc-reserves', 'Deep Pits FC B Team', 1874, 'City Road', 'Near Manor Top, on City Road (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheaf-fc', 'Sheaf FC', 1874, 'Victoria Station', 'From Sheaf Works next to Victoria station', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('sheaf-fc-reserves', 'Sheaf FC Juniors', 1874, 'Victoria Station', 'From Sheaf Works next to Victoria station (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('montague-fc', 'Montague FC', 1874, 'Sharrow', 'From Montague Street Sharrow', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('montague-fc-reserves', 'Montague FC Second Team', 1874, 'Sharrow', 'From Montague Street Sharrow (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('collegiate-fc', 'Collegiate FC', 1875, 'Unknown', '(exact date unknown)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('collegiate-fc-reserves', 'Collegiate FC Reserve XI', 1875, 'Unknown', '(exact date unknown) (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('stag-home-fc', 'Stag Home FC', 1875, 'Unknown', 'Origins uncertain, possibly played at Kenwood', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('stag-home-fc-reserves', 'Stag Home FC Junior XI', 1875, 'Unknown', 'Origins uncertain, possibly played at Kenwood (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('stanley-street-fc', 'Stanley Street FC', 1875, 'Norfolk Park', 'From Stanley Street, Wicker, played mainly at Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('stanley-street-fc-reserves', 'Stanley Street FC Colts', 1875, 'Norfolk Park', 'From Stanley Street, Wicker, played mainly at Norfolk Park (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('langsett-road-fc', 'Langsett Road FC', 1875, 'Queens Ground', 'Likely played at Queens Ground', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('langsett-road-fc-reserves', 'Langsett Road FC Reserves', 1875, 'Queens Ground', 'Likely played at Queens Ground (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('manor-fc', 'Manor FC', 1875, 'Manor Lane', 'played at Manor Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('manor-fc-reserves', 'Manor FC Second XI', 1875, 'Manor Lane', 'played at Manor Lane (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wheatman-smith-fc', 'Wheatman & Smith''s FC', 1875, 'Russell Works', 'From Russell Works, near Kelham Island', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wheatman-smith-fc-reserves', 'Wheatman & Smith''''s FC B Team', 1875, 'Russell Works', 'From Russell Works, near Kelham Island (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('netherthorpe-fc', 'Netherthorpe FC', 1875, 'Unknown', 'Ground location uncertain', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('netherthorpe-fc-reserves', 'Netherthorpe FC Juniors', 1875, 'Unknown', 'Ground location uncertain (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-peter-fc', 'St Peter''s FC', 1875, 'Myrtle Road', 'From the now Cathedral, played at Myrtle Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-peter-fc-reserves', 'St Peter''''s FC Second Team', 1875, 'Myrtle Road', 'From the now Cathedral, played at Myrtle Road (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-philip-fc', 'St Philip''s FC', 1875, 'Netherthorpe', 'From St Philip''s church, Netherthorpe', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-philip-fc-reserves', 'St Philip''''s FC Reserve XI', 1875, 'Netherthorpe', 'From St Philip''s church, Netherthorpe (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-united-fc', 'Dronfield United FC', 1875, 'Dronfield', 'From Dronfield', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-united-fc-reserves', 'Dronfield United FC Junior XI', 1875, 'Dronfield', 'From Dronfield (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hope-club', 'Hope Club', 1875, 'Weston Field', 'From Hope Works on Sussex Road, Played at Weston Field', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hope-club-reserves', 'Hope Club Colts', 1875, 'Weston Field', 'From Hope Works on Sussex Road, Played at Weston Field (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brown-bailey-dixon-fc', 'Brown Bailey & Dixon FC', 1875, 'Attercliffe', 'From their AUercliﬀe works, Leeds Road', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('brown-bailey-dixon-fc-reserves', 'Brown Bailey & Dixon FC Reserves', 1875, 'Attercliffe', 'From their AUercliﬀe works, Leeds Road (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ward-payne-fc', 'Ward & Payne''s FC', 1875, 'Limbrick Works, Hillsborough', 'From their Limbrick Works, Hillsborough', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ward-payne-fc-reserves', 'Ward & Payne''''s FC Second XI', 1875, 'Limbrick Works, Hillsborough', 'From their Limbrick Works, Hillsborough (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crookes-united-fc', 'Crookes United FC', 1875, 'Lydgate Lane', 'Likely played at Lydgate Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('crookes-united-fc-reserves', 'Crookes United FC B Team', 1875, 'Lydgate Lane', 'Likely played at Lydgate Lane (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('harold-fc', 'Harold FC', 1875, 'Walkley', 'Likely from Harold Street, Walkley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('harold-fc-reserves', 'Harold FC Juniors', 1875, 'Walkley', 'Likely from Harold Street, Walkley (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('huffton-son-fc', 'Huffton & Son FC', 1875, 'West Street', 'From Huffton''s Works, West Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('huffton-son-fc-reserves', 'Huffton & Son FC Second Team', 1875, 'West Street', 'From Huffton''s Works, West Street (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('malinda-works-fc', 'Malinda Works FC', 1875, 'Malinda Street', 'From Malinda Works, Malinda Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('malinda-works-fc-reserves', 'Malinda Works FC Reserve XI', 1875, 'Malinda Street', 'From Malinda Works, Malinda Street (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('parkwood-juniors-fc', 'Parkwood Juniors FC', 1875, 'Parkwood Springs', 'From Parkwood Springs', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('parkwood-juniors-fc-reserves', 'Parkwood Juniors FC Junior XI', 1875, 'Parkwood Springs', 'From Parkwood Springs (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('aston-fc', 'Aston FC', 1875, 'Aston', 'From Aston', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('aston-fc-reserves', 'Aston FC Colts', 1875, 'Aston', 'From Aston (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('washington-fc', 'Washington FC', 1875, 'Washington Street', 'From Wostenholm''s Washington Works, Washington Street, poss played at Cobden View', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('washington-fc-reserves', 'Washington FC Reserves', 1875, 'Washington Street', 'From Wostenholm''s Washington Works, Washington Street, poss played at Cobden View (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-paul-fc', 'St Paul''s FC', 1875, 'Peace Gardens', 'From St Paul''s church , now Peace Gardens', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('st-paul-fc-reserves', 'St Paul''''s FC Second XI', 1875, 'Peace Gardens', 'From St Paul''s church , now Peace Gardens (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('south-view-fc', 'South View FC', 1875, 'Machon Bank', 'From South View Road in Sharrow, played at Machon Bank', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('south-view-fc-reserves', 'South View FC B Team', 1875, 'Machon Bank', 'From South View Road in Sharrow, played at Machon Bank (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-united-fc', 'Ecclesall United FC', 1875, 'Ecclesall', 'played at Ecclesall', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ecclesall-united-fc-reserves', 'Ecclesall United FC Juniors', 1875, 'Ecclesall', 'played at Ecclesall (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('msl-loco-fc', 'MS&L Loco FC', 1875, 'Unknown', 'Ground location unknown', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('msl-loco-fc-reserves', 'MS&L Loco FC Second Team', 1875, 'Unknown', 'Ground location unknown (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oxford-wanderers-fc', 'Oxford Wanderers FC', 1875, 'Norfolk Park', 'Poss from Oxford Street, played Norfolk Park', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('oxford-wanderers-fc-reserves', 'Oxford Wanderers FC Reserve XI', 1875, 'Norfolk Park', 'Poss from Oxford Street, played Norfolk Park (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-free-church-fc', 'Dronfield Free Church FC', 1875, 'Dronfield', 'From Dronfield United Methodist Free Church, now Peel Centre, High Street, Dronfield', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-free-church-fc-reserves', 'Dronfield Free Church FC Junior XI', 1875, 'Dronfield', 'From Dronfield United Methodist Free Church, now Peel Centre, High Street, Dronfield (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('atlantic-juniors-fc', 'Atlantic (Juniors) FC', 1875, 'Cobden View', 'played at Cobden View', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('atlantic-juniors-fc-reserves', 'Atlantic (Juniors) FC Colts', 1875, 'Cobden View', 'played at Cobden View (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wincobank-fc', 'Wincobank FC', 1875, 'Wincobank', 'From Wincobank area', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wincobank-fc-reserves', 'Wincobank FC Reserves', 1875, 'Wincobank', 'From Wincobank area (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('alexandra-fc', 'Alexandra FC', 1875, 'Alexandra Road, Heeley', 'From Alexandra Road, Heeley', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('alexandra-fc-reserves', 'Alexandra FC Second XI', 1875, 'Alexandra Road, Heeley', 'From Alexandra Road, Heeley (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('carbrook-united-fc', 'Carbrook United FC', 1875, 'Carbrook', 'played behind St Batholowmew''s Church Carbrook', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('carbrook-united-fc-reserves', 'Carbrook United FC B Team', 1875, 'Carbrook', 'played behind St Batholowmew''s Church Carbrook (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('r-sorby-son-fc', 'R Sorby & Son FC', 1875, 'Trafalgar Street', 'From Kangaroo Works, Trafalgar Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('r-sorby-son-fc-reserves', 'R Sorby & Son FC Juniors', 1875, 'Trafalgar Street', 'From Kangaroo Works, Trafalgar Street (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('upperthorpe-fc', 'Upperthorpe FC', 1875, 'Cobden View', 'played at Cobden View', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('upperthorpe-fc-reserves', 'Upperthorpe FC Second Team', 1875, 'Cobden View', 'played at Cobden View (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('howard-street-fc', 'Howard Street FC', 1875, 'Heeley', 'Likely played in Heeley, from Howard Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('howard-street-fc-reserves', 'Howard Street FC Reserve XI', 1875, 'Heeley', 'Likely played in Heeley, from Howard Street (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('otley-son-fc', 'Otley & Son''s FC', 1875, 'Shalesmoor', 'From Meadow Works Shalesmoor', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('otley-son-fc-reserves', 'Otley & Son''''s FC Junior XI', 1875, 'Shalesmoor', 'From Meadow Works Shalesmoor (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hillsborough-fc', 'Hillsborough / Hillsborough School FC', 1875, 'Hillsborough', 'played at Hillsborough', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('hillsborough-fc-reserves', 'Hillsborough / Hillsborough School FC Colts', 1875, 'Hillsborough', 'played at Hillsborough (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bellefield-fc', 'Bellefield FC', 1875, 'Bellefield Lane, Netherthorpe', 'From Bellefield Works, Bellefield Lane, Netherthorpe', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bellefield-fc-reserves', 'Bellefield FC Reserves', 1875, 'Bellefield Lane, Netherthorpe', 'From Bellefield Works, Bellefield Lane, Netherthorpe (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('red-rose-fc', 'Red Rose FC', 1875, 'Unknown', 'Could be from Pitsmoor or Bramall Lane or was possibly 2 clubs', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('red-rose-fc-reserves', 'Red Rose FC Second XI', 1875, 'Unknown', 'Could be from Pitsmoor or Bramall Lane or was possibly 2 clubs (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('weston-rovers-fc', 'Weston Rovers FC', 1875, 'Unknown', 'Uncertain origins', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('weston-rovers-fc-reserves', 'Weston Rovers FC B Team', 1875, 'Unknown', 'Uncertain origins (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('j-round-son-fc', 'J Round & Son FC', 1875, 'Tudor Street', 'From Tudor Works, Tudor Street, now Tudor Sq', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('j-round-son-fc-reserves', 'J Round & Son FC Juniors', 1875, 'Tudor Street', 'From Tudor Works, Tudor Street, now Tudor Sq (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-baptists-fc', 'Dronfield Baptist FC', 1875, 'Dronfield', 'Church', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-baptists-fc-reserves', 'Dronfield Baptist FC Second Team', 1875, 'Dronfield', 'Church (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pye-bank-free-church-fc', 'Pye Bank Free Church FC', 1875, 'Pye Bank', 'Pye Bank, Sheﬃeld', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('pye-bank-free-church-fc-reserves', 'Pye Bank Free Church FC Reserve XI', 1875, 'Pye Bank', 'Pye Bank, Sheﬃeld (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-independent-fc', 'Dronfield Independent FC', 1875, 'Dronfield', 'From Independent Chapel Lea Road, Dronfield', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('dronfield-independent-fc-reserves', 'Dronfield Independent FC Junior XI', 1875, 'Dronfield', 'From Independent Chapel Lea Road, Dronfield (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ebenezer-reform-fc', 'Ebenezer Reform FC', 1875, 'Unknown', 'Ebenezer Chaple, Neepsend', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('ebenezer-reform-fc-reserves', 'Ebenezer Reform FC Colts', 1875, 'Unknown', 'Ebenezer Chaple, Neepsend (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('clough-house-fc', 'Clough House FC', 1875, 'Clough area near Bramall Lane', 'Likley from The Clough area near Bramall Lane', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('clough-house-fc-reserves', 'Clough House FC Reserves', 1875, 'Clough area near Bramall Lane', 'Likley from The Clough area near Bramall Lane (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('trinity-fc', 'Trinity FC', 1875, 'Trinity Works', 'From George Butler & Co, Trinity Works', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('trinity-fc-reserves', 'Trinity FC Second XI', 1875, 'Trinity Works', 'From George Butler & Co, Trinity Works (Second XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wostenholm-fc', 'Wostenholm FC', 1875, 'Washington Works', 'club from Washington Works', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('wostenholm-fc-reserves', 'Wostenholm FC B Team', 1875, 'Washington Works', 'club from Washington Works (B Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bee-hive-works-fc', 'Bee Hive Works FC', 1875, 'Bee Hive Works, Neepsend', 'From Bee Hive Works Neepsend (not Milton St)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('bee-hive-works-fc-reserves', 'Bee Hive Works FC Juniors', 1875, 'Bee Hive Works, Neepsend', 'From Bee Hive Works Neepsend (not Milton St) (Juniors)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('chester-brothers-fc', 'Chester Brothers FC', 1875, 'West End Cutlery Works, West Street', 'From West End Cuterly Works West Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('chester-brothers-fc-reserves', 'Chester Brothers FC Second Team', 1875, 'West End Cutlery Works, West Street', 'From West End Cuterly Works West Street (Second Team)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('heeley-united-fc', 'Heeley United FC', 1875, 'Unknown', 'Unknown origins', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('heeley-united-fc-reserves', 'Heeley United FC Reserve XI', 1875, 'Unknown', 'Unknown origins (Reserve XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('cornish-place-fc', 'Cornish Place FC', 1875, 'Cornish Works, Neepsend', 'From Cornish Works, Neepsend', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('cornish-place-fc-reserves', 'Cornish Place FC Junior XI', 1875, 'Cornish Works, Neepsend', 'From Cornish Works, Neepsend (Junior XI)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('minerva-fc', 'Minerva FC', 1875, 'John Street', 'From their works on John Street', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('minerva-fc-reserves', 'Minerva FC Colts', 1875, 'John Street', 'From their works on John Street (Colts)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('woodhouse-fc', 'Woodhouse FC', 1875, 'Woodhouse', 'From Woodhouse', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('woodhouse-fc-reserves', 'Woodhouse FC Reserves', 1875, 'Woodhouse', 'From Woodhouse (Reserves)', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('kenyons-works-fc', 'Kenyon''s Works FC', 1875, 'Hollins Crog', 'From Kenyon Works, Hollins Crog', NULL, NULL);
INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('kenyons-works-fc-reserves', 'Kenyon''''s Works FC Second XI', 1875, 'Hollins Crog', 'From Kenyon Works, Hollins Crog (Second XI)', NULL, NULL);

-- Verify
SELECT COUNT(*) as total_clubs FROM sheffield_clubs;
