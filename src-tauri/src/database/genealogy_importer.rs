/// Genealogy CSV Importer
///
/// Imports genealogy CSV files containing census data with addresses and professions

use super::genealogy_matcher::GenealogyRecord;
use std::path::Path;

/// Read a single genealogy CSV file
pub async fn read_genealogy_csv(
    file_path: &Path,
) -> Result<Vec<GenealogyRecord>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => {
                eprintln!("Warning: Failed to parse CSV row: {}", e);
                continue;
            }
        }
    }

    println!("Read {} records from {:?}", records.len(), file_path);
    Ok(records)
}

/// Read all genealogy CSV files from a directory
pub async fn read_all_genealogy_csvs(
    dir_path: &Path,
) -> Result<Vec<GenealogyRecord>, Box<dyn std::error::Error>> {
    let mut all_records = Vec::new();

    let entries = std::fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            match read_genealogy_csv(&path).await {
                Ok(mut records) => {
                    all_records.append(&mut records);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read {:?}: {}", path, e);
                }
            }
        }
    }

    println!("Total genealogy records loaded: {}", all_records.len());
    Ok(all_records)
}
