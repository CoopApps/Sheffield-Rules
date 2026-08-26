-- Create and populate Sheffield ecclesiastical parishes table
-- Based on 1861 Census ecclesiastical districts

DROP TABLE IF EXISTS sheffield_parishes;

CREATE TABLE sheffield_parishes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    postcode TEXT,
    type TEXT DEFAULT 'ecclesiastical',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Major Sheffield ecclesiastical parishes from 1860s-1870s
-- Mapped to modern postcodes

-- S1 - City Centre
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-peter', 'St Peter', 'S1', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-paul', 'St Paul', 'S1', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('cathedral', 'Cathedral', 'S1', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-marie', 'St Marie', 'S1', 'ecclesiastical');

-- S2 - Heeley, Manor, Norfolk Park
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-bartholomew', 'St Bartholomew', 'S2', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('heeley-christ-church', 'Heeley Christ Church', 'S2', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-luke', 'St Luke', 'S2', 'ecclesiastical');

-- S3 - Broomhall, Netherthorpe, Pitsmoor
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-silas', 'St Silas', 'S3', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-philip', 'St Philip', 'S3', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-stephen', 'St Stephen', 'S3', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('broomhall', 'Broomhall', 'S3', 'ecclesiastical');

-- S4 - Brightside, Pitsmoor, Grimesthorpe
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-matthew', 'St Matthew', 'S4', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('pitsmoor', 'Pitsmoor', 'S4', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('brightside', 'Brightside', 'S4', 'ecclesiastical');

-- S5 - Ecclesfield, Fir Vale
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('ecclesfield', 'Ecclesfield', 'S5', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-mary-ecclesfield', 'St Mary Ecclesfield', 'S5', 'ecclesiastical');

-- S6 - Hillsborough, Walkley, Wadsley
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('hillsborough', 'Hillsborough', 'S6', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('walkley', 'Walkley', 'S6', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('wadsley', 'Wadsley', 'S6', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('stannington', 'Stannington', 'S6', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('owlerton', 'Owlerton', 'S6', 'ecclesiastical');

-- S7 - Nether Edge, Millhouses
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('nether-edge', 'Nether Edge', 'S7', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('millhouses', 'Millhouses', 'S7', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-augustine', 'St Augustine', 'S7', 'ecclesiastical');

-- S8 - Norton, Woodseats
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('norton', 'Norton', 'S8', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('woodseats', 'Woodseats', 'S8', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-james-norton', 'St James Norton', 'S8', 'ecclesiastical');

-- S9 - Attercliffe, Darnall
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('attercliffe', 'Attercliffe', 'S9', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-lawrence', 'St Lawrence', 'S9', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('darnall', 'Darnall', 'S9', 'ecclesiastical');

-- S10 - Crookes, Ranmoor, Broomhill
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('crookes', 'Crookes', 'S10', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('ranmoor', 'Ranmoor', 'S10', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('broomhill', 'Broomhill', 'S10', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-mark-broomhill', 'St Mark Broomhill', 'S10', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('fulwood', 'Fulwood', 'S10', 'ecclesiastical');

-- S11 - Ecclesall, Sharrow
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('ecclesall', 'Ecclesall', 'S11', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('sharrow', 'Sharrow', 'S11', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-george', 'St George', 'S11', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('bents-green', 'Bents Green', 'S11', 'ecclesiastical');

-- S12 - Gleadless, Intake
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('gleadless', 'Gleadless', 'S12', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('intake', 'Intake', 'S12', 'ecclesiastical');

-- S13 - Handsworth, Woodhouse
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('handsworth', 'Handsworth', 'S13', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('woodhouse', 'Woodhouse', 'S13', 'ecclesiastical');

-- S17 - Dore, Totley
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('dore', 'Dore', 'S17', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('totley', 'Totley', 'S17', 'ecclesiastical');

-- S18 - Dronfield
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('dronfield', 'Dronfield', 'S18', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('st-john-dronfield', 'St John Dronfield', 'S18', 'ecclesiastical');

-- S35 - Chapeltown, Grenoside, Oughtibridge
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('chapeltown', 'Chapeltown', 'S35', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('grenoside', 'Grenoside', 'S35', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('oughtibridge', 'Oughtibridge', 'S35', 'ecclesiastical');

-- S36 - Stocksbridge, Deepcar
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('stocksbridge', 'Stocksbridge', 'S36', 'ecclesiastical');
INSERT INTO sheffield_parishes (id, name, postcode, type) VALUES ('deepcar', 'Deepcar', 'S36', 'ecclesiastical');

SELECT COUNT(*) as total_parishes FROM sheffield_parishes;
