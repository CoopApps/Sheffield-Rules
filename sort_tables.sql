-- Sort unmatched_ancestry by surname (A-Z)
-- SQLite doesn't have a "sort table" command, but we can recreate with ORDER BY

CREATE TABLE unmatched_ancestry_sorted AS
SELECT * FROM unmatched_ancestry
ORDER BY surname ASC, first_name ASC;

DROP TABLE unmatched_ancestry;

ALTER TABLE unmatched_ancestry_sorted RENAME TO unmatched_ancestry;

-- Sort unmatched_genealogy by address (street_address)
-- This groups people on the same street together

CREATE TABLE unmatched_genealogy_sorted AS
SELECT * FROM unmatched_genealogy
ORDER BY street_address ASC, surname ASC, first_name ASC;

DROP TABLE unmatched_genealogy;

ALTER TABLE unmatched_genealogy_sorted RENAME TO unmatched_genealogy;
