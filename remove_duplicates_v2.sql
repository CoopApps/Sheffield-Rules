-- Remove exact duplicates from unmatched_genealogy
-- Keep the record with the lowest rowid for each duplicate group

DELETE FROM unmatched_genealogy
WHERE rowid NOT IN (
    SELECT MIN(rowid)
    FROM unmatched_genealogy
    WHERE name IS NOT NULL AND census_age IS NOT NULL AND street_address IS NOT NULL
    GROUP BY name, census_age, street_address
);

SELECT 'unmatched_genealogy: Deleted ' || changes() || ' duplicate records';

-- Verify no more duplicates in unmatched_genealogy
SELECT 'unmatched_genealogy remaining duplicate groups:', COUNT(*) FROM (
    SELECT name, census_age, street_address, COUNT(*) as cnt
    FROM unmatched_genealogy
    WHERE name IS NOT NULL AND census_age IS NOT NULL AND street_address IS NOT NULL
    GROUP BY name, census_age, street_address
    HAVING COUNT(*) > 1
);

-- Now do the same for unmatched_ancestry
DELETE FROM unmatched_ancestry
WHERE rowid NOT IN (
    SELECT MIN(rowid)
    FROM unmatched_ancestry
    WHERE name IS NOT NULL AND census_age IS NOT NULL AND street_address IS NOT NULL
    GROUP BY name, census_age, street_address
);

SELECT 'unmatched_ancestry: Deleted ' || changes() || ' duplicate records';

-- Verify no more duplicates in unmatched_ancestry
SELECT 'unmatched_ancestry remaining duplicate groups:', COUNT(*) FROM (
    SELECT name, census_age, street_address, COUNT(*) as cnt
    FROM unmatched_ancestry
    WHERE name IS NOT NULL AND census_age IS NOT NULL AND street_address IS NOT NULL
    GROUP BY name, census_age, street_address
    HAVING COUNT(*) > 1
);

-- Show final counts
SELECT 'unmatched_genealogy total records:', COUNT(*) FROM unmatched_genealogy;
SELECT 'unmatched_ancestry total records:', COUNT(*) FROM unmatched_ancestry;
