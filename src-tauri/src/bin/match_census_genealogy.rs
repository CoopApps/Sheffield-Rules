use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct CensusRecord {
    id: String,
    name: String,
    civil_parish: Option<String>,
    household_members: Option<String>,
    birth_year: Option<i64>,
    relation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct GenealogyRecord {
    id: String,
    name: String,
    address: Option<String>,
    profession: Option<String>,
    spouse: Option<String>,
    relation: Option<String>,
    birth_year: Option<i64>,
    parish: Option<String>,
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(".", "")
        .replace(",", "")
        .trim()
        .to_string()
}

fn normalize_address(addr: &str) -> String {
    addr.to_lowercase()
        .replace("street", "st")
        .replace("road", "rd")
        .replace("lane", "ln")
        .replace(",", "")
        .replace(" ", "")
        .trim()
        .to_string()
}

fn calculate_match_score(census: &CensusRecord, gen: &GenealogyRecord) -> f32 {
    let mut score = 0.0;

    // Name match (most important - 50%)
    let c_name = normalize_name(&census.name);
    let g_name = normalize_name(&gen.name);
    if c_name == g_name {
        score += 0.5;
    } else if c_name.contains(&g_name) || g_name.contains(&c_name) {
        score += 0.25;
    }

    // Birth year match (40%)
    if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
        if c_year == g_year {
            score += 0.4;
        } else if (c_year - g_year).abs() <= 2 {
            score += 0.2; // Allow 2 year variance
        }
    }

    // Parish match (10%) - compare census civil_parish with gen parish
    if let (Some(c_parish), Some(g_parish)) = (&census.civil_parish, &gen.parish) {
        let c_norm = c_parish.to_lowercase();
        let g_norm = g_parish.to_lowercase();
        if c_norm == g_norm {
            score += 0.1;
        } else if c_norm.contains(&g_norm) || g_norm.contains(&c_norm) {
            score += 0.05;
        }
    }

    score
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Loading unmatched census records...");
    let census_records: Vec<CensusRecord> = sqlx::query_as(
        "SELECT id, name, civil_parish, household_members, birth_year, relation FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} census records", census_records.len());

    println!("Loading unmatched genealogy records...");
    let genealogy_records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT id, name, address, profession, spouse, relation, birth_year, parish FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

    println!("Finding matches (this may take a while)...");
    let mut matches_found = 0;
    let mut high_confidence_matches = Vec::new();

    // For each census record, find best matching genealogy record
    for (i, census) in census_records.iter().enumerate() {
        if i % 1000 == 0 {
            println!("Processed {}/{} census records...", i, census_records.len());
        }

        let mut best_match: Option<(&GenealogyRecord, f32)> = None;

        for gen in &genealogy_records {
            let score = calculate_match_score(census, gen);

            if score >= 0.6 { // Require at least 60% confidence
                if let Some((_, best_score)) = best_match {
                    if score > best_score {
                        best_match = Some((gen, score));
                    }
                } else {
                    best_match = Some((gen, score));
                }
            }
        }

        if let Some((gen, score)) = best_match {
            matches_found += 1;
            if score >= 0.7 { // High confidence
                high_confidence_matches.push((census, gen, score));
            }
        }
    }

    println!("\n========================================");
    println!("MATCHING RESULTS:");
    println!("========================================");
    println!("Total potential matches found: {}", matches_found);
    println!("High confidence matches (>70%): {}", high_confidence_matches.len());

    println!("\n\nTop 10 high confidence matches:");
    high_confidence_matches.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (i, (census, gen, score)) in high_confidence_matches.iter().take(10).enumerate() {
        println!("\n{}. Match Score: {:.1}%", i + 1, score * 100.0);
        println!("   Census: {} (born {}, parish: {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            census.civil_parish.as_deref().unwrap_or("no parish")
        );
        println!("   Genealogy: {} (born {}, {}, spouse: {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            gen.address.as_deref().unwrap_or("no address"),
            gen.spouse.as_deref().unwrap_or("none")
        );
    }

    println!("\n\nWould you like to apply these {} high confidence matches? (y/n)", high_confidence_matches.len());
    println!("(This is a DRY RUN - no changes will be made yet)");

    Ok(())
}
