@echo off
echo Populating Sheffield clubs 1857-1875...
type populate_clubs_1857_1875.sql | sqlite3 Sheffield1867.db
echo Done!
echo.
echo Verifying...
echo SELECT COUNT(*) as total_clubs FROM sheffield_clubs; | sqlite3 Sheffield1867.db
