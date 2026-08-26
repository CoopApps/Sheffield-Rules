-- Update postcodes for records where street_address is just an area name
-- Based on Sheffield postcode district mappings

BEGIN TRANSACTION;

-- S5 areas
UPDATE unmatched_genealogy SET postcode = 'S5' WHERE street_address IN ('Ecclesfield', 'Firth Park', 'Fir Vale', 'Longley', 'Shirecliffe', 'Shiregreen', 'Southey', 'Parson Cross', 'Wincobank') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S5' WHERE street_address IN ('Ecclesfield', 'Firth Park', 'Fir Vale', 'Longley', 'Shirecliffe', 'Shiregreen', 'Southey', 'Parson Cross', 'Wincobank') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S5' WHERE street_address IN ('Ecclesfield', 'Firth Park', 'Fir Vale', 'Longley', 'Shirecliffe', 'Shiregreen', 'Southey', 'Parson Cross', 'Wincobank') AND (postcode IS NULL OR postcode = '');

-- S6 areas
UPDATE unmatched_genealogy SET postcode = 'S6' WHERE street_address IN ('Bradfield', 'Dungworth', 'Fox Hill', 'Hillsborough', 'Holdworth', 'Hollow Meadows', 'Loxley', 'Malin Bridge', 'Middlewood', 'Stannington', 'Storrs', 'Upperthorpe', 'Wadsley', 'Wadsley Bridge', 'Walkley', 'Wisewood') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S6' WHERE street_address IN ('Bradfield', 'Dungworth', 'Fox Hill', 'Hillsborough', 'Holdworth', 'Hollow Meadows', 'Loxley', 'Malin Bridge', 'Middlewood', 'Stannington', 'Storrs', 'Upperthorpe', 'Wadsley', 'Wadsley Bridge', 'Walkley', 'Wisewood') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S6' WHERE street_address IN ('Bradfield', 'Dungworth', 'Fox Hill', 'Hillsborough', 'Holdworth', 'Hollow Meadows', 'Loxley', 'Malin Bridge', 'Middlewood', 'Stannington', 'Storrs', 'Upperthorpe', 'Wadsley', 'Wadsley Bridge', 'Walkley', 'Wisewood') AND (postcode IS NULL OR postcode = '');

-- S7 areas
UPDATE unmatched_genealogy SET postcode = 'S7' WHERE street_address IN ('Beauchief', 'Carter Knowle', 'Millhouses', 'Nether Edge') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S7' WHERE street_address IN ('Beauchief', 'Carter Knowle', 'Millhouses', 'Nether Edge') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S7' WHERE street_address IN ('Beauchief', 'Carter Knowle', 'Millhouses', 'Nether Edge') AND (postcode IS NULL OR postcode = '');

-- S8 areas
UPDATE unmatched_genealogy SET postcode = 'S8' WHERE street_address IN ('Batemoor', 'Greenhill', 'Jordanthorpe', 'Lowedges', 'Meadowhead', 'Meersbrook', 'Norton', 'Norton Lees', 'Woodseats') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S8' WHERE street_address IN ('Batemoor', 'Greenhill', 'Jordanthorpe', 'Lowedges', 'Meadowhead', 'Meersbrook', 'Norton', 'Norton Lees', 'Woodseats') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S8' WHERE street_address IN ('Batemoor', 'Greenhill', 'Jordanthorpe', 'Lowedges', 'Meadowhead', 'Meersbrook', 'Norton', 'Norton Lees', 'Woodseats') AND (postcode IS NULL OR postcode = '');

-- S9 areas
UPDATE unmatched_genealogy SET postcode = 'S9' WHERE street_address IN ('Attercliffe', 'Brightside', 'Darnall', 'Handsworth Hill', 'Meadowhall', 'Tinsley') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S9' WHERE street_address IN ('Attercliffe', 'Brightside', 'Darnall', 'Handsworth Hill', 'Meadowhall', 'Tinsley') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S9' WHERE street_address IN ('Attercliffe', 'Brightside', 'Darnall', 'Handsworth Hill', 'Meadowhall', 'Tinsley') AND (postcode IS NULL OR postcode = '');

-- S10 areas
UPDATE unmatched_genealogy SET postcode = 'S10' WHERE street_address IN ('Broomhall', 'Broomhill', 'Crookes', 'Crookesmoor', 'Crosspool', 'Endcliffe', 'Fulwood', 'Lodge Moor', 'Ranmoor') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S10' WHERE street_address IN ('Broomhall', 'Broomhill', 'Crookes', 'Crookesmoor', 'Crosspool', 'Endcliffe', 'Fulwood', 'Lodge Moor', 'Ranmoor') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S10' WHERE street_address IN ('Broomhall', 'Broomhill', 'Crookes', 'Crookesmoor', 'Crosspool', 'Endcliffe', 'Fulwood', 'Lodge Moor', 'Ranmoor') AND (postcode IS NULL OR postcode = '');

-- S11 areas
UPDATE unmatched_genealogy SET postcode = 'S11' WHERE street_address IN ('Bents Green', 'Ecclesall', 'Greystones', 'Parkhead', 'Ringinglow', 'Sharrow', 'Whirlow') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S11' WHERE street_address IN ('Bents Green', 'Ecclesall', 'Greystones', 'Parkhead', 'Ringinglow', 'Sharrow', 'Whirlow') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S11' WHERE street_address IN ('Bents Green', 'Ecclesall', 'Greystones', 'Parkhead', 'Ringinglow', 'Sharrow', 'Whirlow') AND (postcode IS NULL OR postcode = '');

-- S12 areas (including Intake)
UPDATE unmatched_genealogy SET postcode = 'S12' WHERE street_address IN ('Frecheville', 'Gleadless', 'Hackenthorpe', 'Intake', 'Ridgeway') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S12' WHERE street_address IN ('Frecheville', 'Gleadless', 'Hackenthorpe', 'Intake', 'Ridgeway') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S12' WHERE street_address IN ('Frecheville', 'Gleadless', 'Hackenthorpe', 'Intake', 'Ridgeway') AND (postcode IS NULL OR postcode = '');

-- S13 areas
UPDATE unmatched_genealogy SET postcode = 'S13' WHERE street_address IN ('Handsworth', 'Normanton Spring', 'Orgreave', 'Richmond', 'Woodhouse', 'Woodthorpe') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S13' WHERE street_address IN ('Handsworth', 'Normanton Spring', 'Orgreave', 'Richmond', 'Woodhouse', 'Woodthorpe') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S13' WHERE street_address IN ('Handsworth', 'Normanton Spring', 'Orgreave', 'Richmond', 'Woodhouse', 'Woodthorpe') AND (postcode IS NULL OR postcode = '');

-- S14 areas
UPDATE unmatched_genealogy SET postcode = 'S14' WHERE street_address IN ('Gleadless Valley') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S14' WHERE street_address IN ('Gleadless Valley') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S14' WHERE street_address IN ('Gleadless Valley') AND (postcode IS NULL OR postcode = '');

-- S17 areas
UPDATE unmatched_genealogy SET postcode = 'S17' WHERE street_address IN ('Bradway', 'Dore', 'Totley') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S17' WHERE street_address IN ('Bradway', 'Dore', 'Totley') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S17' WHERE street_address IN ('Bradway', 'Dore', 'Totley') AND (postcode IS NULL OR postcode = '');

-- S20 areas
UPDATE unmatched_genealogy SET postcode = 'S20' WHERE street_address IN ('Beighton', 'Crystal Peaks', 'Halfway', 'Mosborough', 'Owlthorpe', 'Plumbley', 'Sothall', 'Waterthorpe', 'Westfield') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S20' WHERE street_address IN ('Beighton', 'Crystal Peaks', 'Halfway', 'Mosborough', 'Owlthorpe', 'Plumbley', 'Sothall', 'Waterthorpe', 'Westfield') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S20' WHERE street_address IN ('Beighton', 'Crystal Peaks', 'Halfway', 'Mosborough', 'Owlthorpe', 'Plumbley', 'Sothall', 'Waterthorpe', 'Westfield') AND (postcode IS NULL OR postcode = '');

-- S21 areas
UPDATE unmatched_genealogy SET postcode = 'S21' WHERE street_address IN ('Eckington', 'Killamarsh', 'Marsh Lane', 'Middle Handley', 'Renishaw', 'Spinkhill', 'Troway', 'West Handley') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S21' WHERE street_address IN ('Eckington', 'Killamarsh', 'Marsh Lane', 'Middle Handley', 'Renishaw', 'Spinkhill', 'Troway', 'West Handley') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S21' WHERE street_address IN ('Eckington', 'Killamarsh', 'Marsh Lane', 'Middle Handley', 'Renishaw', 'Spinkhill', 'Troway', 'West Handley') AND (postcode IS NULL OR postcode = '');

-- S25 areas
UPDATE unmatched_genealogy SET postcode = 'S25' WHERE street_address IN ('Anston', 'Brookhouse', 'Dinnington', 'Laughton en le Morthen', 'Slade Hooton') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S25' WHERE street_address IN ('Anston', 'Brookhouse', 'Dinnington', 'Laughton en le Morthen', 'Slade Hooton') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S25' WHERE street_address IN ('Anston', 'Brookhouse', 'Dinnington', 'Laughton en le Morthen', 'Slade Hooton') AND (postcode IS NULL OR postcode = '');

-- S26 areas
UPDATE unmatched_genealogy SET postcode = 'S26' WHERE street_address IN ('Aston', 'Aughton', 'Harthill', 'Kiveton Park', 'Swallownest', 'Todwick', 'Ulley', 'Wales', 'Waleswood', 'Woodall') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S26' WHERE street_address IN ('Aston', 'Aughton', 'Harthill', 'Kiveton Park', 'Swallownest', 'Todwick', 'Ulley', 'Wales', 'Waleswood', 'Woodall') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S26' WHERE street_address IN ('Aston', 'Aughton', 'Harthill', 'Kiveton Park', 'Swallownest', 'Todwick', 'Ulley', 'Wales', 'Waleswood', 'Woodall') AND (postcode IS NULL OR postcode = '');

-- S35 areas
UPDATE unmatched_genealogy SET postcode = 'S35' WHERE street_address IN ('Chapeltown', 'Crane Moor', 'Green Moor', 'Grenoside', 'Hermit Hill', 'High Green', 'Oughtibridge', 'Thurgoland', 'Wharncliffe Side', 'Wortley', 'Worrall') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S35' WHERE street_address IN ('Chapeltown', 'Crane Moor', 'Green Moor', 'Grenoside', 'Hermit Hill', 'High Green', 'Oughtibridge', 'Thurgoland', 'Wharncliffe Side', 'Wortley', 'Worrall') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S35' WHERE street_address IN ('Chapeltown', 'Crane Moor', 'Green Moor', 'Grenoside', 'Hermit Hill', 'High Green', 'Oughtibridge', 'Thurgoland', 'Wharncliffe Side', 'Wortley', 'Worrall') AND (postcode IS NULL OR postcode = '');

-- S36 areas
UPDATE unmatched_genealogy SET postcode = 'S36' WHERE street_address IN ('Bolsterstone', 'Carlecotes', 'Catshaw', 'Crow Edge', 'Deepcar', 'Dunford Bridge', 'Hoylandswaine', 'Ingbirchworth', 'Langsett', 'Midhopestones', 'Millhouse Green', 'Oxspring', 'Snowden Hill', 'Stocksbridge', 'Penistone', 'Thurlstone', 'Upper Midhope', 'Wigtwizzle') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S36' WHERE street_address IN ('Bolsterstone', 'Carlecotes', 'Catshaw', 'Crow Edge', 'Deepcar', 'Dunford Bridge', 'Hoylandswaine', 'Ingbirchworth', 'Langsett', 'Midhopestones', 'Millhouse Green', 'Oxspring', 'Snowden Hill', 'Stocksbridge', 'Penistone', 'Thurlstone', 'Upper Midhope', 'Wigtwizzle') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S36' WHERE street_address IN ('Bolsterstone', 'Carlecotes', 'Catshaw', 'Crow Edge', 'Deepcar', 'Dunford Bridge', 'Hoylandswaine', 'Ingbirchworth', 'Langsett', 'Midhopestones', 'Millhouse Green', 'Oxspring', 'Snowden Hill', 'Stocksbridge', 'Penistone', 'Thurlstone', 'Upper Midhope', 'Wigtwizzle') AND (postcode IS NULL OR postcode = '');

-- S2 areas (not S1 since those overlap with city centre)
UPDATE unmatched_genealogy SET postcode = 'S2' WHERE street_address IN ('Arbourthorne', 'Heeley', 'Highfield', 'Lowfield', 'Manor', 'Newfield Green', 'Norfolk Park', 'Park Hill', 'Wybourn') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S2' WHERE street_address IN ('Arbourthorne', 'Heeley', 'Highfield', 'Lowfield', 'Manor', 'Newfield Green', 'Norfolk Park', 'Park Hill', 'Wybourn') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S2' WHERE street_address IN ('Arbourthorne', 'Heeley', 'Highfield', 'Lowfield', 'Manor', 'Newfield Green', 'Norfolk Park', 'Park Hill', 'Wybourn') AND (postcode IS NULL OR postcode = '');

-- S3 areas (excluding city centre)
UPDATE unmatched_genealogy SET postcode = 'S3' WHERE street_address IN ('Neepsend', 'Netherthorpe') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S3' WHERE street_address IN ('Neepsend', 'Netherthorpe') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S3' WHERE street_address IN ('Neepsend', 'Netherthorpe') AND (postcode IS NULL OR postcode = '');

-- S4 areas (excluding duplicates from S3)
UPDATE unmatched_genealogy SET postcode = 'S4' WHERE street_address IN ('Grimesthorpe', 'Osgathorpe', 'Page Hall') AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S4' WHERE street_address IN ('Grimesthorpe', 'Osgathorpe', 'Page Hall') AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S4' WHERE street_address IN ('Grimesthorpe', 'Osgathorpe', 'Page Hall') AND (postcode IS NULL OR postcode = '');

COMMIT;

-- Show results
SELECT 'unmatched_genealogy postcodes updated:', COUNT(*) FROM unmatched_genealogy WHERE postcode IS NOT NULL AND postcode <> '';
SELECT 'sheffield_businesses postcodes updated:', COUNT(*) FROM sheffield_businesses WHERE postcode IS NOT NULL AND postcode <> '';
SELECT 'unmatched_ancestry postcodes updated:', COUNT(*) FROM unmatched_ancestry WHERE postcode IS NOT NULL AND postcode <> '';
