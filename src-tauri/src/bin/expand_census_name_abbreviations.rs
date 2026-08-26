use sqlx::sqlite::SqlitePool;
use regex::Regex;

fn get_name_expansions() -> Vec<(&'static str, &'static str)> {
    vec![
        // Common male abbreviations
        ("Wm", "William"),
        ("Wm.", "William"),
        ("Thos", "Thomas"),
        ("Thos.", "Thomas"),
        ("Chas", "Charles"),
        ("Chas.", "Charles"),
        ("Geo", "George"),
        ("Geo.", "George"),
        ("Jas", "James"),
        ("Jas.", "James"),
        ("Jno", "John"),
        ("Jno.", "John"),
        ("Robt", "Robert"),
        ("Robt.", "Robert"),
        ("Richd", "Richard"),
        ("Richd.", "Richard"),
        ("Edwd", "Edward"),
        ("Edwd.", "Edward"),
        ("Jos", "Joseph"),
        ("Jos.", "Joseph"),
        ("Saml", "Samuel"),
        ("Saml.", "Samuel"),
        ("Benj", "Benjamin"),
        ("Benj.", "Benjamin"),
        ("Danl", "Daniel"),
        ("Danl.", "Daniel"),
        ("Fred", "Frederick"),
        ("Fred.", "Frederick"),
        ("Fredk", "Frederick"),
        ("Fredk.", "Frederick"),
        ("Hy", "Henry"),
        ("Hy.", "Henry"),
        ("Michl", "Michael"),
        ("Michl.", "Michael"),
        ("Alexr", "Alexander"),
        ("Alexr.", "Alexander"),
        ("Andw", "Andrew"),
        ("Andw.", "Andrew"),
        ("Matt", "Matthew"),
        ("Matt.", "Matthew"),
        ("Steph", "Stephen"),
        ("Steph.", "Stephen"),

        // Common female abbreviations
        ("Elizth", "Elizabeth"),
        ("Elizth.", "Elizabeth"),
        ("Eliz", "Elizabeth"),
        ("Eliz.", "Elizabeth"),
        ("Margt", "Margaret"),
        ("Margt.", "Margaret"),
        ("Cath", "Catherine"),
        ("Cath.", "Catherine"),
        ("Cathe", "Catherine"),
        ("Cathe.", "Catherine"),
        ("Ann", "Anne"),
        ("Ann.", "Anne"),
        ("Rebca", "Rebecca"),
        ("Rebca.", "Rebecca"),
        ("Susah", "Susannah"),
        ("Susah.", "Susannah"),
    ]
}

fn expand_name(name: &str) -> String {
    let expansions = get_name_expansions();

    // Split name into parts
    let parts: Vec<&str> = name.split_whitespace().collect();

    if parts.is_empty() {
        return name.to_string();
    }

    let mut result_parts = Vec::new();

    for part in parts {
        let mut expanded = part.to_string();

        // Check if this part matches any abbreviation
        for (abbrev, full) in &expansions {
            // Case-insensitive match at word boundaries
            if part.eq_ignore_ascii_case(abbrev) {
                expanded = full.to_string();
                break;
            }
        }

        result_parts.push(expanded);
    }

    result_parts.join(" ")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("EXPANDING NAME ABBREVIATIONS IN CENSUS");
    println!("========================================\n");

    // Get all records with names
    let records: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, name FROM unmatched_sheffieldcensus WHERE name IS NOT NULL AND name != ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} records with names to check\n", records.len());

    let mut updated = 0;
    let mut unchanged = 0;
    let mut examples = Vec::new();

    for (i, (id, original_name)) in records.iter().enumerate() {
        if (i + 1) % 1000 == 0 {
            println!("Processed {}/{} records...", i + 1, records.len());
        }

        let expanded_name = expand_name(original_name);

        // Only update if changed
        if &expanded_name != original_name {
            sqlx::query("UPDATE unmatched_sheffieldcensus SET name = ? WHERE id = ?")
                .bind(&expanded_name)
                .bind(id)
                .execute(&pool)
                .await?;

            updated += 1;

            // Save first 20 examples
            if examples.len() < 20 {
                examples.push((original_name.clone(), expanded_name.clone()));
            }
        } else {
            unchanged += 1;
        }
    }

    println!("\n========================================");
    println!("EXPANSION COMPLETE");
    println!("========================================");
    println!("Total records processed: {}", records.len());
    println!("Records updated: {}", updated);
    println!("Records unchanged: {}", unchanged);

    if !examples.is_empty() {
        println!("\n20 Example transformations:");
        println!("{:=<60}", "");
        for (i, (original, expanded)) in examples.iter().enumerate() {
            println!("{}. {} → {}", i + 1, original, expanded);
        }
        println!("{:=<60}", "");
    }

    Ok(())
}
