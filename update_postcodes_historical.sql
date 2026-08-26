-- Update postcodes based on historical street name mappings
BEGIN TRANSACTION;

-- Charlotte Street → Mappin Street : S1 4DT
UPDATE unmatched_genealogy SET postcode = 'S1 4DT' WHERE street_address LIKE '%Charlotte Street%' AND (postcode IS NULL OR postcode = '');
UPDATE sheffield_businesses SET postcode = 'S1 4DT' WHERE street_address LIKE '%Charlotte Street%' AND (postcode IS NULL OR postcode = '');
UPDATE unmatched_ancestry SET postcode = 'S1 4DT' WHERE street_address LIKE '%Charlotte Street%' AND (postcode IS NULL OR postcode = '');

COMMIT;

-- Show results
SELECT 'unmatched_genealogy postcodes:', COUNT(*) FROM unmatched_genealogy WHERE postcode IS NOT NULL AND postcode <> '';
SELECT 'sheffield_businesses postcodes:', COUNT(*) FROM sheffield_businesses WHERE postcode IS NOT NULL AND postcode <> '';
SELECT 'unmatched_ancestry postcodes:', COUNT(*) FROM unmatched_ancestry WHERE postcode IS NOT NULL AND postcode <> '';
