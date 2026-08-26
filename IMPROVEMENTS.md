# Database Editor Improvements

This document outlines the improvements made to the Sheffield Rules Database Editor and Sheffield1867.db system.

## ✅ Completed Improvements

### 1. Census CSV Structure Analysis
- **Status**: Complete
- **Details**: Analyzed 73 census CSV files (birth years 1772-1844)
- **Format**: Tab-separated values with 22 columns including:
  - AGE, Birth Date, Birth Place, CIVIL PARISH, COUNTRY, COUNTY/ISLAND
  - ECCLESIASTICAL PARISH, ED/INSTITUTION/VESSEL, ESTIMATED BIRTH YEAR
  - FOLIO, GENDER, HOUSEHOLD MEMBERS, HOUSEHOLD SCHEDULE NUMBER
  - NAME (2 columns), PAGE NUMBER, PIECE, REGISTRATION DISTRICT, RELATION
  - SUB-REGISTRATION DISTRICT, TOWN, WHERE BORN

### 2. Backup System
- **Status**: Complete
- **Files Created**:
  - `src-tauri/src/database/backup.rs` - Backup utility module
  - Added backup commands to `commands.rs`
  - Registered commands in `main.rs`
- **Features**:
  - `create_backup()` - Creates timestamped database backups
  - `list_backups()` - Lists all available backups
  - `restore_backup()` - Restores from backup
  - `cleanup_old_backups()` - Removes old backups, keeping N most recent
  - Backups stored in `./backups/` subdirectory with format: `Sheffield1867.db_YYYYMMDD_HHMMSS.backup`

### 3. UI Components
- **Status**: Complete
- **Files Created**:
  - `frontend/src/components/CsvImportButton.tsx` - File upload component
  - `frontend/src/components/ConfirmationDialog.tsx` - Modal confirmation dialog
  - `frontend/src/styles/confirmation-dialog.css` - Dialog styling

##

 🔄 In Progress

### 4. CSV Import Feature
- **Status**: In Progress
- **Existing**: `parseHorizontalTsvFormat()` function already handles census CSV format
- **TODO**:
  - Add CsvImportButton to Create Players tab
  - Wire up file upload to existing parser
  - Add success/error feedback

### 5. Integrate Backup with Editor
- **Status**: In Progress
- **TODO**:
  - Add backup button to Club Management tab
  - Auto-backup before:
    - Bulk player creation
    - Club deletion
    - Division changes
    - Player deletion
  - Show backup list with restore option

## 📋 Pending Improvements

### 6. Confirmation Dialogs
- Add ConfirmationDialog before:
  - Player deletion
  - Club deletion
  - Division reassignment
  - Bulk operations
  - Database restore

### 7. Input Validation
- Validate club names (no duplicates)
- Validate birth years (1800-1900 range)
- Validate postcodes (UK format: S1-S99 for Sheffield)
- Validate census field formats
- Validate attribute ranges (1-20 for most, 1-200 for abilities)
- Show validation errors inline

### 8. Fix Hardcoded Paths
- Replace `D:/projects/Saturday at Three/Sheffield1867.db` with:
  - Tauri app data directory
  - Or relative path from app root
  - Use `tauri::api::path::app_data_dir()` or similar

### 9. Batch Stat Assignment
- Add attribute presets:
  - Elite Forward (finishing 18, pace 17, etc.)
  - Good Midfielder (passing 15, stamina 16, etc.)
  - Average Defender (tackling 13, marking 12, etc.)
  - Poor Goalkeeper (handling 10, reflexes 9, etc.)
- Multi-select players
- Apply preset to selection
- Custom adjustment sliders (+/- 3 to all attributes)

### 10. Clean Up Debug Code
- Remove console.log statements from:
  - `DatabaseEditorScreen.tsx` lines 394, 396, 337, etc.
  - Other debug output in production code

### 11. Export Functionality
- Export club roster to CSV
- Export division to CSV
- Export all players to CSV
- Export database backup as downloadable file

### 12. Testing
- Test backup/restore functionality
- Test CSV import with actual census files
- Test validation rules
- Test confirmation dialogs
- Verify data integrity after operations

## 🎯 Implementation Order

1. **Phase 1: Safety Features** (Highest Priority)
   - ✅ Backup system (backend)
   - ⏳ Integrate backups into UI
   - ⏳ Add confirmation dialogs
   - ⏳ Add validation

2. **Phase 2: User Experience**
   - ⏳ CSV import button
   - ⏳ Batch stat assignment
   - ⏳ Clean up debug code

3. **Phase 3: Production Ready**
   - ⏳ Fix hardcoded paths
   - ⏳ Add export functionality
   - ⏳ Comprehensive testing

## 📝 Notes

- Existing `parseHorizontalTsvFormat()` function (lines 2325-2443 in DatabaseEditorScreen.tsx) already handles census CSV format perfectly
- Census CSV files are in `sheffield census/` directory (73 files for birth years 1772-1844)
- Database has comprehensive schema with census fields already defined
- All improvements designed to be non-breaking - existing functionality preserved

## 🚀 Next Steps

1. Add CSV import button to Create Players tab
2. Wire up auto-backup before destructive operations
3. Add confirmation dialogs for all destructive actions
4. Add input validation with error messages
5. Test thoroughly with real data
