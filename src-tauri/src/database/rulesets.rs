use sqlx::SqlitePool;

/// Get the appropriate ruleset for a given year
/// Returns a simple JSON representation of the ruleset
pub fn get_ruleset_for_year(year: u32) -> String {
    // TODO: Integrate with actual sheffield_rules rulesets
    // For now, return a simple JSON with year indicator
    format!(
        r#"{{
    "year": {},
    "name": "{}",
    "goal_width": {},
    "goal_height": {},
    "rouge_active": {},
    "scoring_system": "{}"
}}"#,
        year,
        get_year_name(year),
        get_goal_width(year),
        get_goal_height(year),
        is_rouge_active(year),
        get_scoring_system(year)
    )
}

/// Save ruleset to database
pub async fn save_ruleset_to_db(
    pool: &SqlitePool,
    year: u32,
    season: u32,
) -> Result<(), sqlx::Error> {
    let ruleset_json = get_ruleset_for_year(year);

    sqlx::query(
        "INSERT OR REPLACE INTO sheffield_rules_history (season_year, rule_year, ruleset_json) VALUES (?, ?, ?)"
    )
    .bind(season as i32)
    .bind(year as i32)
    .bind(ruleset_json)
    .execute(pool)
    .await?;

    Ok(())
}

fn get_year_name(year: u32) -> &'static str {
    match year {
        1858 => "Founding Era",
        1859..=1861 => "Early Period",
        1862..=1868 => "Rouge Era",
        1869..=1876 => "Late Period",
        1877 => "Final Year",
        _ => "Unknown",
    }
}

fn get_goal_width(year: u32) -> u16 {
    match year {
        1858..=1861 => 12,  // 12 feet
        1862..=1875 => 12,  // 12 feet
        1876..=1877 => 24,  // 24 feet
        _ => 12,
    }
}

fn get_goal_height(year: u32) -> u16 {
    match year {
        1858..=1875 => 9,   // 9 feet
        1876..=1877 => 8,   // 8 feet
        _ => 9,
    }
}

fn is_rouge_active(year: u32) -> bool {
    // Rouge flags abolished October 1868; active only during 1862-1867
    year >= 1862 && year <= 1867
}

fn get_scoring_system(year: u32) -> &'static str {
    if is_rouge_active(year) {
        "goals_and_rouges"
    } else {
        "goals_only"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ruleset_for_year() {
        let ruleset = get_ruleset_for_year(1862);
        assert!(ruleset.contains("1862"));
        assert!(ruleset.contains("true")); // rouge_active
    }

    #[test]
    fn test_rouge_active_period() {
        assert!(is_rouge_active(1862));
        assert!(is_rouge_active(1867)); // last year rouge was active (abolished Oct 1868)
        assert!(!is_rouge_active(1868)); // rouge flags abolished in Oct 1868
        assert!(!is_rouge_active(1861));
        assert!(!is_rouge_active(1869));
    }

    #[test]
    fn test_goal_dimensions() {
        assert_eq!(get_goal_width(1870), 12);
        assert_eq!(get_goal_height(1870), 9);
        assert_eq!(get_goal_width(1876), 24);
        assert_eq!(get_goal_height(1876), 8);
    }
}
