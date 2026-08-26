use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;

    println!("Fixing reserve team names to proper title case...\n");

    // Get all clubs with names that are all uppercase (likely reserve teams)
    let clubs = sqlx::query_as::<_, (String, String)>(
        "SELECT id, name FROM sheffield_clubs
         WHERE name = UPPER(name)
         AND name != LOWER(name)
         AND (name LIKE '%RESERVE%' OR name LIKE '%JUNIOR%' OR name LIKE '%SECOND%' OR name LIKE '% B%')"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} clubs with all-caps names\n", clubs.len());

    let abbreviations = vec!["FC", "XI", "B"];
    let preserve_words = vec!["Reserves", "Reserve", "Juniors", "Junior", "Second"];
    let mut updated = 0;

    for (id, name) in clubs {
        // Remove " (Reserve)" or " (RESERVE)" suffix if it exists
        let clean_name = name
            .replace(" (RESERVE)", "")
            .replace(" (Reserve)", "");

        // Convert to proper title case with abbreviations and special words preserved
        let name_parts: Vec<String> = clean_name
            .split_whitespace()
            .map(|word| {
                // Check for parentheses
                let has_open_paren = word.starts_with('(');
                let has_close_paren = word.ends_with(')');
                let mut clean_word = word.to_string();

                if has_open_paren {
                    clean_word = clean_word[1..].to_string();
                }
                if has_close_paren {
                    clean_word = clean_word[..clean_word.len()-1].to_string();
                }

                let word_lower = clean_word.to_lowercase();
                let mut result = String::new();

                // Keep abbreviations in uppercase
                if abbreviations.contains(&clean_word.to_uppercase().as_str()) {
                    result = clean_word.to_uppercase();
                }
                // Preserve specific words with their proper capitalization
                else if let Some(preserved) = preserve_words.iter().find(|w| w.to_lowercase() == word_lower) {
                    result = preserved.to_string();
                }
                // Capitalize first letter of each word
                else {
                    let mut chars = clean_word.chars();
                    result = match chars.next() {
                        None => String::new(),
                        Some(first) => {
                            first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                        }
                    };
                }

                // Add back parentheses
                if has_open_paren {
                    result = format!("({}", result);
                }
                if has_close_paren {
                    result = format!("{})", result);
                }

                result
            })
            .collect();

        let new_name = name_parts.join(" ");

        // Update the database
        sqlx::query("UPDATE sheffield_clubs SET name = ? WHERE id = ?")
            .bind(&new_name)
            .bind(&id)
            .execute(&pool)
            .await?;

        println!("✓ {} → {}", name, new_name);
        updated += 1;
    }

    println!("\n✓ Successfully updated {} reserve team names to title case", updated);

    Ok(())
}
