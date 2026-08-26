use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;

#[derive(Debug, serde::Deserialize)]
struct GenealogyRecord {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Spouse")]
    spouse: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Parish")]
    parish: String,
    #[serde(rename = "Area")]
    area: String,
    #[serde(rename = "Age")]
    age: String,
    #[serde(rename = "Born Approx")]
    born_approx: String,
    #[serde(rename = "Birth Place")]
    birth_place: String,
    #[serde(rename = "Relation")]
    relation: String,
    #[serde(rename = "Profession")]
    profession: String,
}

fn extract_street_name(address: &str) -> String {
    let trimmed = address.trim();

    // Try to extract leading number
    let mut chars = trimmed.chars();
    let mut number_str = String::new();

    while let Some(c) = chars.next() {
        if c.is_numeric() {
            number_str.push(c);
        } else if !number_str.is_empty() {
            break;
        }
    }

    // Extract street name (everything after the number)
    let street = if !number_str.is_empty() {
        trimmed[number_str.len()..].trim().to_lowercase()
    } else {
        trimmed.to_lowercase()
    };

    street
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let genealogy_dir = "D:/projects/Saturday at Three/genealogy";
    let output_file = "D:/projects/Saturday at Three/unique_streets.txt";

    println!("========================================");
    println!("EXTRACTING UNIQUE STREET ADDRESSES");
    println!("========================================\n");

    // Get all CSV files
    let mut csv_files: Vec<_> = fs::read_dir(genealogy_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "csv" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    csv_files.sort();

    println!("Found {} CSV files\n", csv_files.len());

    // Map of street name -> first example record
    let mut street_examples: HashMap<String, GenealogyRecord> = HashMap::new();
    let mut unique_streets: HashSet<String> = HashSet::new();

    let mut total_records = 0;
    let mut file_count = 0;
    let total_files = csv_files.len();

    for csv_path in csv_files {
        file_count += 1;
        let filename = csv_path.file_name().unwrap().to_string_lossy();

        if file_count % 100 == 0 {
            println!("Processing file {}/{}: {} (found {} unique streets so far)",
                     file_count, total_files, filename, unique_streets.len());
        }

        let mut reader = match ReaderBuilder::new()
            .has_headers(true)
            .from_path(&csv_path) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("ERROR: Failed to open file {}: {}", filename, e);
                    continue;
                }
            };

        for result in reader.deserialize::<GenealogyRecord>() {
            match result {
                Ok(record) => {
                    total_records += 1;

                    if !record.address.is_empty() {
                        let street = extract_street_name(&record.address);

                        if !street.is_empty() {
                            // Only add if we haven't seen this street before
                            if !unique_streets.contains(&street) {
                                unique_streets.insert(street.clone());
                                street_examples.insert(street, record);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse record in {}: {}", filename, e);
                }
            }
        }
    }

    println!("\n========================================");
    println!("EXTRACTION COMPLETE");
    println!("========================================");
    println!("Files processed: {}", file_count);
    println!("Total records read: {}", total_records);
    println!("Unique streets found: {}\n", unique_streets.len());

    // Sort streets alphabetically
    let mut sorted_streets: Vec<String> = unique_streets.into_iter().collect();
    sorted_streets.sort();

    // Write to file
    println!("Writing to file: {}\n", output_file);
    let mut file = fs::File::create(output_file)?;

    writeln!(file, "UNIQUE STREET ADDRESSES FROM GENEALOGY DATA")?;
    writeln!(file, "========================================\n")?;
    writeln!(file, "Total unique streets: {}\n", sorted_streets.len())?;
    writeln!(file, "Format: Street Name - Example Person (Full Address)\n")?;
    writeln!(file, "========================================\n")?;

    for (i, street) in sorted_streets.iter().enumerate() {
        if let Some(example) = street_examples.get(street) {
            writeln!(file, "{}. {} - Example: {} ({})",
                     i + 1,
                     street,
                     example.name,
                     example.address)?;
        }
    }

    println!("✓ Successfully wrote {} unique streets to file", sorted_streets.len());
    println!("Output file: {}", output_file);

    Ok(())
}
