-- Check what data types are in unmatched_genealogy
SELECT
    COUNT(*) as total,
    COUNT(CASE WHEN genealogy_source IS NULL OR genealogy_source = '' THEN 1 END) as census_imports,
    COUNT(CASE WHEN genealogy_source = 'genealogy' THEN 1 END) as genealogy_records,
    COUNT(CASE WHEN genealogy_source = 'geni' THEN 1 END) as geni_records,
    COUNT(CASE WHEN business_type IS NOT NULL THEN 1 END) as has_business_type
FROM unmatched_genealogy;

-- Show example of business records
SELECT 'Example business records in unmatched_genealogy:' as label;
SELECT name, business_type, street_address, genealogy_source
FROM unmatched_genealogy
WHERE business_type IS NOT NULL
LIMIT 5;
