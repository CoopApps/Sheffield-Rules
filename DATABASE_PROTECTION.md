# Sheffield1867.db Protection Guide

## Current Status

### Protected Files
- `Sheffield1867.db` - **READ-ONLY** - Master database (cannot be modified or deleted)
- `Sheffield1867_backup_20260305_123217.db` - Backup created March 5, 2026

## Protection Details

The main database has been set to read-only using Windows file attributes:
```bash
attrib +R Sheffield1867.db
```

This prevents:
- Accidental deletion
- Accidental modification
- Scripts from overwriting the file

## To Make Changes to the Database

If you need to modify the database structure or data:

1. **Create a working copy:**
   ```bash
   cp Sheffield1867.db Sheffield1867_working.db
   ```

2. **Make your changes to the working copy**

3. **If you want to replace the master:**
   ```bash
   # Remove read-only attribute
   attrib -R Sheffield1867.db

   # Replace with working copy
   cp Sheffield1867_working.db Sheffield1867.db

   # Set read-only again
   attrib +R Sheffield1867.db
   ```

## To Restore from Backup

```bash
# Remove read-only from master
attrib -R Sheffield1867.db

# Restore from backup
cp Sheffield1867_backup_20260305_123217.db Sheffield1867.db

# Set read-only again
attrib +R Sheffield1867.db
```

## Creating New Backups

```bash
cp Sheffield1867.db "Sheffield1867_backup_$(date +%Y%m%d_%H%M%S).db"
```

## Current Database Contents

### Tables
- `sheffield_people` - 156,743 people (main table)
- `sheffield_footballers` - 22,226 males aged 14-40 (excluding clergy, large employers, police)
- `sheffield_clubs` - 372 clubs (186 main + 186 reserves)
- `sheffield_employers` - 557 employers (125 large, 432 small)
- `sheffield_publicans` - 350 publicans
- `sheffield_tradesmen` - 30,686 skilled workers
- `sheffield_clergy` - 83 clergy members
- `sheffield_medical` - 387 medical professionals
- `sheffield_teachers` - 296 teachers
- `sheffield_police` - 241 police officers
- `sheffield_military` - 794 military personnel
- `sheffield_irish` - 5,021 Irish immigrants
- `sheffield_scottish` - 827 Scottish immigrants
- `sheffield_welsh` - 234 Welsh immigrants
- `sheffield_german` - 191 German immigrants
- `sheffield_lodgers` - 17,026 lodgers/boarders
- `sheffield_family_groups` - 39,269 family groups (surname + address)
- `sheffield_family_members` - Family member details
- Other supporting tables

### Size
- Main database: 102 MB
- Contains comprehensive 1867 Sheffield census and genealogy data
