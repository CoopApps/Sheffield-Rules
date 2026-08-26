-- Map ecclesiastical districts to Sheffield postcodes based on 1852 boundaries

-- St. Peter's District (City Centre) - S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. Peter%'
  AND ecclesiastical_parish LIKE '%Sheffield%';

-- Attercliffe and Darnall Districts - S9
UPDATE unmatched_genealogy
SET postcode = 'S9'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Attercliffe%' OR ecclesiastical_parish LIKE '%Darnall%');

-- Brightside District - S4
UPDATE unmatched_genealogy
SET postcode = 'S4'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Brightside%';

-- Wicker District (Trinity) - S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Wicker%' OR ecclesiastical_parish LIKE '%Trinity%');

-- Pitsmoor District - S4 (northern part)
UPDATE unmatched_genealogy
SET postcode = 'S4'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Pitsmoor%';

-- St. John's Church (Park area) - S2
UPDATE unmatched_genealogy
SET postcode = 'S2'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. John%';

-- Heeley District - S2 or S8
UPDATE unmatched_genealogy
SET postcode = 'S8'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Heeley%';

-- St. Paul's District (Norfolk street area) - S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. Paul%';

-- Carver street District - S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Carver%';

-- Eldon District (St. Jude) - S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Eldon%' OR ecclesiastical_parish LIKE '%St. Jude%');

-- St. George's District (West street area) - S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. George%';

-- Hollis croft District - S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Hollis%';

-- Netherthorpe District - S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Netherthorpe%';

-- Moorfields District - S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Moorfields%';

-- St. James's District - S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. James%';

-- St. Philip's District - S6
UPDATE unmatched_genealogy
SET postcode = 'S6'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. Philip%';

-- Crookes District (St. Thomas) - S10
UPDATE unmatched_genealogy
SET postcode = 'S10'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Crookes%' OR ecclesiastical_parish LIKE '%St. Thomas%');

-- Fulwood District - S10
UPDATE unmatched_genealogy
SET postcode = 'S10'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Fulwood%';

-- Ecclesall District (All Saints) - S11
UPDATE unmatched_genealogy
SET postcode = 'S11'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Ecclesall%' OR ecclesiastical_parish LIKE '%All Saints%');

-- St. Mary's District - S2
UPDATE unmatched_genealogy
SET postcode = 'S2'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. Mary%';

-- Broomhall District - S10
UPDATE unmatched_genealogy
SET postcode = 'S10'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Broomhall%';

-- Report results
SELECT 'Records with postcodes after ecclesiastical mapping:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NOT NULL AND postcode != '';

SELECT 'Records still without postcodes:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NULL OR postcode = '';
