/**
 * DATABASE EDITOR ENHANCEMENTS PATCH
 *
 * This file contains the code additions for DatabaseEditorScreen.tsx
 * Apply these changes to enhance the editor with CSV import and backup features
 */

// ============================================================================
// 1. ADD IMPORTS AT TOP OF FILE (after existing imports, around line 5)
// ============================================================================

import { CsvImportButton } from '../components/CsvImportButton';
import { ConfirmationDialog } from '../components/ConfirmationDialog';

// ============================================================================
// 2. ADD STATE VARIABLES (after existing state declarations, around line 290)
// ============================================================================

// Confirmation dialog state
const [confirmDialog, setConfirmDialog] = useState<{
  isOpen: boolean;
  title: string;
  message: string;
  onConfirm: () => void;
  danger?: boolean;
}>({
  isOpen: false,
  title: '',
  message: '',
  onConfirm: () => {},
  danger: false
});

// Backup state
const [backups, setBackups] = useState<any[]>([]);
const [showBackupManager, setShowBackupManager] = useState(false);

// ============================================================================
// 3. ADD BACKUP FUNCTIONS (after existing functions, around line 2800)
// ============================================================================

const loadBackups = async () => {
  try {
    const backupList = await invoke<any[]>('db_list_backups');
    setBackups(backupList);
  } catch (err) {
    console.error('Error loading backups:', err);
  }
};

const createBackup = async () => {
  try {
    setLoading(true);
    const backupPath = await invoke<string>('db_create_backup');
    setSuccess(`Backup created: ${backupPath}`);
    await loadBackups();
    setTimeout(() => setSuccess(''), 3000);
  } catch (err) {
    setError(`Failed to create backup: ${err}`);
  } finally {
    setLoading(false);
  }
};

const restoreBackup = async (backupPath: string) => {
  setConfirmDialog({
    isOpen: true,
    title: 'Restore Backup',
    message: `Are you sure you want to restore from this backup? The current database will be backed up first, but this operation cannot be undone.`,
    danger: true,
    onConfirm: async () => {
      try {
        setLoading(true);
        await invoke('db_restore_backup', { backupPath });
        setSuccess('Backup restored successfully! Please reload the page.');
        setTimeout(() => window.location.reload(), 2000);
      } catch (err) {
        setError(`Failed to restore backup: ${err}`);
      } finally {
        setLoading(false);
        setConfirmDialog(prev => ({ ...prev, isOpen: false }));
      }
    }
  });
};

const handleDeletePlayer = (playerId: string, playerName: string) => {
  setConfirmDialog({
    isOpen: true,
    title: 'Delete Player',
    message: `Are you sure you want to delete "${playerName}"? This action cannot be undone.`,
    danger: true,
    onConfirm: () => {
      deletePlayer(playerId);
      setConfirmDialog(prev => ({ ...prev, isOpen: false }));
    }
  });
};

const handleCsvImport = (content: string, filename: string) => {
  console.log(`Importing CSV file: ${filename}`);
  // Use existing parser - it already handles the census CSV format!
  handlePasteCensusData(content);
  setSuccess(`Imported data from ${filename}`);
  setTimeout(() => setSuccess(''), 3000);
};

const handleBulkCreatePlayers = async () => {
  // Create backup before bulk operation
  setConfirmDialog({
    isOpen: true,
    title: 'Create Players',
    message: `Create ${manualPlayers.length} player(s)? A backup will be created automatically before this operation.`,
    onConfirm: async () => {
      try {
        setLoading(true);
        setProgressMessage('Creating backup...');
        await invoke('db_create_backup');

        setProgressMessage('Creating players...');
        await handleCreatePlayers();

        setSuccess(`Created ${manualPlayers.length} players successfully!`);
        setTimeout(() => setSuccess(''), 3000);
      } catch (err) {
        setError(`Operation failed: ${err}`);
      } finally {
        setLoading(false);
        setProgressMessage('');
        setConfirmDialog(prev => ({ ...prev, isOpen: false }));
      }
    }
  });
};

// ============================================================================
// 4. ADD TO CLUB MANAGEMENT TAB (after migration button, around line 2980)
// ============================================================================

{/* Backup Management Section */}
<div className="backup-section" style={{ marginTop: '15px', padding: '10px', background: '#f0f8ff', borderRadius: '4px', border: '1px solid #3498db' }}>
  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
    <strong style={{ color: '#2c3e50' }}>📦 Database Backups</strong>
    <button
      onClick={() => {
        setShowBackupManager(!showBackupManager);
        if (!showBackupManager) loadBackups();
      }}
      style={{ padding: '4px 12px', fontSize: '12px', background: '#3498db', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
    >
      {showBackupManager ? 'Hide' : 'Show'} Backups
    </button>
  </div>

  <button
    onClick={createBackup}
    disabled={loading}
    style={{ width: '100%', padding: '8px', marginBottom: '10px', background: '#3498db', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
  >
    {loading ? 'Creating...' : '💾 Create Backup Now'}
  </button>

  {showBackupManager && (
    <div style={{ maxHeight: '200px', overflow: 'auto', background: 'white', padding: '8px', borderRadius: '4px', border: '1px solid #ddd' }}>
      <p style={{ fontSize: '12px', color: '#666', marginBottom: '8px' }}>
        {backups.length} backup(s) available
      </p>
      {backups.map((backup: any) => (
        <div key={backup.path} style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '6px', background: '#f9f9f9', marginBottom: '4px', borderRadius: '3px', fontSize: '12px' }}>
          <div>
            <div style={{ fontWeight: 'bold' }}>{backup.filename}</div>
            <div style={{ color: '#666', fontSize: '11px' }}>
              {new Date(backup.created_timestamp * 1000).toLocaleString()} • {(backup.size / 1024 / 1024).toFixed(2)} MB
            </div>
          </div>
          <button
            onClick={() => restoreBackup(backup.path)}
            style={{ padding: '4px 10px', background: '#2ecc71', color: 'white', border: 'none', borderRadius: '3px', cursor: 'pointer', fontSize: '11px' }}
          >
            Restore
          </button>
        </div>
      ))}
      {backups.length === 0 && (
        <p style={{ textAlign: 'center', color: '#999', fontSize: '12px', padding: '10px' }}>
          No backups yet. Click "Create Backup Now" to create one.
        </p>
      )}
    </div>
  )}

  <small style={{ display: 'block', marginTop: '5px', fontSize: '11px', color: '#666' }}>
    Backups are automatically created before bulk operations
  </small>
</div>

// ============================================================================
// 5. ADD TO CREATE PLAYERS TAB (after existing Census Data Paste Area, around line 3538)
// ============================================================================

{/* CSV File Import */}
<div className="form-group" style={{ marginTop: '15px', padding: '15px', background: '#f0fff0', borderRadius: '8px', border: '2px dashed #4CAF50' }}>
  <label style={{ fontWeight: 'bold', color: '#2c3e50', marginBottom: '8px', display: 'block' }}>
    📁 Import Census CSV Files
  </label>
  <p style={{ fontSize: '13px', color: '#666', marginBottom: '10px' }}>
    Import historical census data from CSV files in the "sheffield census" directory.
    Files should contain columns: NAME, AGE, BIRTH YEAR, ECCLESIASTICAL PARISH, etc.
  </p>
  <CsvImportButton
    onFileLoaded={handleCsvImport}
    accept=".csv,.txt,.tsv"
    className="import-csv-button"
    disabled={loading}
  />
  <p style={{ fontSize: '11px', color: '#999', marginTop: '8px' }}>
    Supported formats: Census CSV (tab-separated), regular CSV, TSV
  </p>
</div>

// ============================================================================
// 6. UPDATE PLAYER DATABASE DELETE BUTTON (around line 5638)
// ============================================================================

// REPLACE the existing delete button with:
<button
  onClick={() => handleDeletePlayer(player.id, player.name)}
  className="delete-player-btn"
>
  Delete
</button>

// ============================================================================
// 7. ADD CONFIRMATION DIALOG AT END OF RETURN STATEMENT (before closing </div>)
// ============================================================================

<ConfirmationDialog
  isOpen={confirmDialog.isOpen}
  title={confirmDialog.title}
  message={confirmDialog.message}
  onConfirm={confirmDialog.onConfirm}
  onCancel={() => setConfirmDialog(prev => ({ ...prev, isOpen: false }))}
  danger={confirmDialog.danger}
/>

// ============================================================================
// 8. REPLACE BULK CREATE BUTTON (around line 3509)
// ============================================================================

// REPLACE:
// <button onClick={handleCreatePlayers} ...>

// WITH:
<button
  onClick={handleBulkCreatePlayers}
  disabled={loading || manualPlayers.length === 0}
  className="save-players-button"
>
  {loading ? progressMessage || 'Creating...' : `Create ${manualPlayers.length} Player(s)`}
</button>

// ============================================================================
// INSTALLATION INSTRUCTIONS
// ============================================================================

/**
 * TO APPLY THESE CHANGES:
 *
 * 1. Open frontend/src/screens/DatabaseEditorScreen.tsx
 * 2. Add the imports at the top (section 1)
 * 3. Add the state variables with other useState declarations (section 2)
 * 4. Add the backup functions with other functions (section 3)
 * 5. Add the Backup Management UI to Club Management tab (section 4)
 * 6. Add the CSV Import button to Create Players tab (section 5)
 * 7. Update the delete button to use confirmation (section 6)
 * 8. Add the ConfirmationDialog component at the end (section 7)
 * 9. Replace the bulk create button (section 8)
 *
 * All changes are additive and non-breaking!
 */
