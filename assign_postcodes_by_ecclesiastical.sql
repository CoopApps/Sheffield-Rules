-- Assign postcodes based on ecclesiastical districts
-- Using the most common postcode for each district based on existing data

-- St Mary - most common: S1, S2, S3, S4 (very mixed, default to S2)
UPDATE unmatched_genealogy
SET postcode = 'S2'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%St. Mary%' OR ecclesiastical_parish LIKE '%St Mary%';

-- St Philip - most common: S6, S3, S10
UPDATE unmatched_genealogy
SET postcode = 'S6'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%St. Philip%' OR ecclesiastical_parish LIKE '%St Philip%');

-- Wicker Trinity - most common: S3, S1, S9
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Wicker%' OR ecclesiastical_parish LIKE '%Trinity%');

-- All Saints (Ecclesall) - most common: S1, S4, S12, S13
UPDATE unmatched_genealogy
SET postcode = 'S11'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%All Saints%' OR ecclesiastical_parish LIKE '%Ecclesall%');

-- St John - most common: S1, S2, S3
UPDATE unmatched_genealogy
SET postcode = 'S2'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%St. John%' OR ecclesiastical_parish LIKE '%St John%');

-- Golcar/Gilcar - extremely mixed, default to S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Golcar%' OR ecclesiastical_parish LIKE '%Gilcar%');

-- Pitsmoor - most common: S3, S4
UPDATE unmatched_genealogy
SET postcode = 'S4'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Pitsmoor%';

-- Dyers Hill - most common: S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Dyers%' OR ecclesiastical_parish LIKE '%Dyer%');

-- Heeley - most common: S2
UPDATE unmatched_genealogy
SET postcode = 'S2'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Heeley%';

-- Christchurch - most common: S6, S10
UPDATE unmatched_genealogy
SET postcode = 'S10'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Christchurch%' OR ecclesiastical_parish LIKE '%Christ Church%';

-- Brightside - most common: S4, S3
UPDATE unmatched_genealogy
SET postcode = 'S4'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Brightside%';

-- St Paul - most common: S1, S3
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%St. Paul%' OR ecclesiastical_parish LIKE '%St Paul%');

-- Neepsend St Michael - most common: S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%Neepsend%' OR ecclesiastical_parish LIKE '%St. Michael%' OR ecclesiastical_parish LIKE '%St Michael%');

-- Broomhall - most common: S1, S10
UPDATE unmatched_genealogy
SET postcode = 'S10'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Broomhall%';

-- St Peter - most common: S1, S3
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%St. Peter%' OR ecclesiastical_parish LIKE '%St Peter%');

-- Eldon/Eldon Street - most common: S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Eldon%';

-- The Porter - mixed
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Porter%';

-- St George - most common: S10, S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%St. George%' OR ecclesiastical_parish LIKE '%St George%');

-- St Matthew - most common: S3
UPDATE unmatched_genealogy
SET postcode = 'S3'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%St. Matthew%' OR ecclesiastical_parish LIKE '%St Matthew%');

-- St Thomas/Crookes - most common: S10
UPDATE unmatched_genealogy
SET postcode = 'S10'
WHERE (postcode IS NULL OR postcode = '')
  AND (ecclesiastical_parish LIKE '%St. Thomas%' OR ecclesiastical_parish LIKE '%St Thomas%' OR ecclesiastical_parish LIKE '%Crookes%');

-- Fulwood - most common: S10
UPDATE unmatched_genealogy
SET postcode = 'S10'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Fulwood%';

-- Carver Street - most common: S1
UPDATE unmatched_genealogy
SET postcode = 'S1'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Carver%';

-- Sharrow - most common: S7
UPDATE unmatched_genealogy
SET postcode = 'S7'
WHERE (postcode IS NULL OR postcode = '')
  AND ecclesiastical_parish LIKE '%Sharrow%';

-- Report results
SELECT 'Records with postcodes after ecclesiastical assignment:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NOT NULL AND postcode != '';

SELECT 'Records still without postcodes:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NULL OR postcode = '';
