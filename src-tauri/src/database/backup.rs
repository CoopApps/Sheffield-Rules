use std::path::{Path, PathBuf};
use std::fs;
use chrono::Utc;

/// Create a timestamped backup of a database file
pub fn create_backup(db_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let source = Path::new(db_path);

    if !source.exists() {
        return Err("Database file does not exist".into());
    }

    // Get parent directory for backups
    let parent = source.parent()
        .ok_or("Cannot determine parent directory")?;

    // Create backups subdirectory if it doesn't exist
    let backups_dir = parent.join("backups");
    fs::create_dir_all(&backups_dir)?;

    // Generate timestamped backup filename
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = source.file_name()
        .ok_or("Cannot determine filename")?;
    let backup_filename = format!("{}_{}.backup",
        filename.to_string_lossy(),
        timestamp
    );

    let backup_path = backups_dir.join(backup_filename);

    // Copy the database file
    fs::copy(source, &backup_path)?;

    Ok(backup_path.to_string_lossy().to_string())
}

/// List all backups for a database file
pub fn list_backups(db_path: &str) -> Result<Vec<BackupInfo>, Box<dyn std::error::Error>> {
    let source = Path::new(db_path);
    let parent = source.parent()
        .ok_or("Cannot determine parent directory")?;
    let backups_dir = parent.join("backups");

    if !backups_dir.exists() {
        return Ok(Vec::new());
    }

    let filename_prefix = source.file_name()
        .ok_or("Cannot determine filename")?
        .to_string_lossy()
        .to_string();

    let mut backups = Vec::new();

    for entry in fs::read_dir(&backups_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with(&filename_prefix) && name_str.ends_with(".backup") {
                    let metadata = fs::metadata(&path)?;
                    backups.push(BackupInfo {
                        path: path.to_string_lossy().to_string(),
                        filename: name_str.to_string(),
                        size: metadata.len(),
                        created: metadata.modified()?,
                    });
                }
            }
        }
    }

    // Sort by creation time, newest first
    backups.sort_by(|a, b| b.created.cmp(&a.created));

    Ok(backups)
}

/// Restore a backup file
pub fn restore_backup(backup_path: &str, target_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let backup = Path::new(backup_path);
    let target = Path::new(target_path);

    if !backup.exists() {
        return Err("Backup file does not exist".into());
    }

    // Create a backup of the current file before restoring
    if target.exists() {
        create_backup(target_path)?;
    }

    // Copy backup to target location
    fs::copy(backup, target)?;

    Ok(())
}

/// Delete old backups, keeping only the most recent N backups
pub fn cleanup_old_backups(db_path: &str, keep_count: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let backups = list_backups(db_path)?;

    if backups.len() <= keep_count {
        return Ok(0);
    }

    let mut deleted = 0;
    for backup in backups.iter().skip(keep_count) {
        fs::remove_file(&backup.path)?;
        deleted += 1;
    }

    Ok(deleted)
}

#[derive(Debug, Clone)]
pub struct BackupInfo {
    pub path: String,
    pub filename: String,
    pub size: u64,
    pub created: std::time::SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_create_and_list_backups() {
        let temp_dir = std::env::temp_dir();
        let test_db = temp_dir.join("test_backup.db");

        // Create a test database file
        let mut file = fs::File::create(&test_db).unwrap();
        file.write_all(b"test data").unwrap();

        // Create backup
        let backup_path = create_backup(test_db.to_str().unwrap()).unwrap();
        assert!(Path::new(&backup_path).exists());

        // List backups
        let backups = list_backups(test_db.to_str().unwrap()).unwrap();
        assert_eq!(backups.len(), 1);

        // Cleanup
        fs::remove_file(&test_db).ok();
        fs::remove_file(&backup_path).ok();
        fs::remove_dir(temp_dir.join("backups")).ok();
    }
}
