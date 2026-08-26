use csv::ReaderBuilder;
use std::collections::HashSet;
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

fn get_first_two_letters(street: &str) -> Option<String> {
    // Remove any leading non-alphabetic characters
    let alphabetic_start: String = street
        .chars()
        .skip_while(|c| !c.is_alphabetic())
        .collect();

    if alphabetic_start.len() >= 2 {
        Some(alphabetic_start.chars().take(2).collect())
    } else if alphabetic_start.len() == 1 {
        Some(alphabetic_start.chars().take(1).collect())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let genealogy_dir = "D:/projects/Saturday at Three/genealogy";
    let output_file = "D:/projects/Saturday at Three/street_prefixes.txt";

    println!("========================================");
    println!("EXTRACTING STREET NAME PREFIXES (FIRST 2 LETTERS)");
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

    // Set to store unique two-letter prefixes
    let mut prefixes: HashSet<String> = HashSet::new();
    let mut streets_by_prefix: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    let mut total_records = 0;
    let mut file_count = 0;
    let total_files = csv_files.len();

    for csv_path in csv_files {
        file_count += 1;
        let filename = csv_path.file_name().unwrap().to_string_lossy();

        if file_count % 100 == 0 {
            println!("Processing file {}/{}: {} (found {} unique prefixes so far)",
                     file_count, total_files, filename, prefixes.len());
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
                            if let Some(prefix) = get_first_two_letters(&street) {
                                prefixes.insert(prefix.clone());

                                // Store example streets for each prefix
                                streets_by_prefix
                                    .entry(prefix)
                                    .or_insert_with(Vec::new)
                                    .push(street.clone());
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
    println!("Unique two-letter prefixes found: {}\n", prefixes.len());

    // Sort prefixes alphabetically
    let mut sorted_prefixes: Vec<String> = prefixes.into_iter().collect();
    sorted_prefixes.sort();

    // Write to file
    println!("Writing to file: {}\n", output_file);
    let mut file = fs::File::create(output_file)?;

    writeln!(file, "UNIQUE TWO-LETTER STREET PREFIXES FROM GENEALOGY DATA")?;
    writeln!(file, "========================================\n")?;
    writeln!(file, "Total unique prefixes: {}\n", sorted_prefixes.len())?;
    writeln!(file, "========================================\n")?;

    // Write all prefixes
    writeln!(file, "ALL PREFIXES (alphabetically):\n")?;
    for (i, prefix) in sorted_prefixes.iter().enumerate() {
        write!(file, "{:3}. {}", i + 1, prefix)?;

        // Add some example streets for this prefix
        if let Some(examples) = streets_by_prefix.get(prefix) {
            let sample: Vec<String> = examples.iter().take(3).cloned().collect();
            writeln!(file, " - Examples: {}", sample.join(", "))?;
        } else {
            writeln!(file)?;
        }
    }

    // Also print to console
    println!("\nALL {} UNIQUE PREFIXES:", sorted_prefixes.len());
    println!("========================================");
    for (i, prefix) in sorted_prefixes.iter().enumerate() {
        print!("{:3}. {}", i + 1, prefix);

        if let Some(examples) = streets_by_prefix.get(prefix) {
            let sample: Vec<String> = examples.iter().take(3).cloned().collect();
            println!(" - Examples: {}", sample.join(", "));
        } else {
            println!();
        }
    }

    println!("\n✓ Successfully wrote {} unique prefixes to file", sorted_prefixes.len());
    println!("Output file: {}", output_file);

    Ok(())
}
