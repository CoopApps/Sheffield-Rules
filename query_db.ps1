# Load System.Data.SQLite assembly if available
Add-Type -Path "System.Data.SQLite.dll" -ErrorAction SilentlyContinue

# Try alternative: use ODBC
$connectionString = "Driver=SQLite3 ODBC Driver;Database=D:\projects\Saturday at Three\Sheffield1867.db"

try {
    $connection = New-Object System.Data.Odbc.OdbcConnection($connectionString)
    $connection.Open()
    
    $query = @"
SELECT id, name, surname, birth_year, ecclesiastical_parish,
       census_household_schedule, census_relation, census_gender
FROM sheffield_players
WHERE census_household_schedule IS NOT NULL
  AND census_household_schedule != ''
LIMIT 10
"@
    
    $command = New-Object System.Data.Odbc.OdbcCommand($query, $connection)
    $reader = $command.ExecuteReader()
    
    $count = 0
    while ($reader.Read()) {
        $count++
        Write-Host "`n$count. $($reader['name']) $($reader['surname']) (b. $($reader['birth_year']))"
        Write-Host "   ID: $($reader['id'])"
        Write-Host "   Parish: $($reader['ecclesiastical_parish'])"
        Write-Host "   Household: $($reader['census_household_schedule'])"
        Write-Host "   Relation: $($reader['census_relation'])"
        Write-Host "   Gender: $($reader['census_gender'])"
    }
    
    $reader.Close()
    $connection.Close()
    
    Write-Host "`nFound $count players with household data"
} catch {
    Write-Host "Error: $_"
}
