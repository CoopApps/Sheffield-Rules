use sqlx::Row;

// sheffield_footballers table schema:
// id INTEGER PRIMARY KEY AUTOINCREMENT
// person_id INTEGER NOT NULL (references sheffield_people.unique_id)

// sheffield_people table has full person data that we JOIN to get

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PlayerQueryResult {
    pub players: Vec<serde_json::Value>,
    pub total_count: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AgeStatistic {
    pub age: i32,
    pub birth_year: i32,
    pub count: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct NameStatistic {
    pub name: String,
    pub count: i32,
}

/// Get all players ordered by surname - JOINs with sheffield_people for full data
pub async fn get_all_players() -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let pool = super::get_pool().await?;

    let players = sqlx::query(
        "SELECT
            f.id,
            f.person_id,
            f.club_id,
            p.first_name,
            p.middle_name,
            p.surname,
            p.birth_year,
            p.profession,
            p.street_address,
            p.civil_parish,
            p.ecclesiastical_parish,
            p.registration_district,
            p.birth_town,
            p.birth_county,
            p.birth_country,
            p.where_born,
            p.census_age,
            p.census_relation,
            p.census_gender,
            p.census_ed,
            p.census_household_schedule,
            p.census_piece,
            p.census_folio,
            p.census_page,
            p.house_number,
            p.sub_area,
            p.street_name,
            p.postcode,
            p.latitude,
            p.longitude,
            p.occupation_expanded
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         ORDER BY p.surname ASC, p.first_name ASC"
    )
    .fetch_all(&pool)
    .await?;

    let result: Vec<serde_json::Value> = players.iter().map(|row| {
        let first_name: Option<String> = row.get(3);
        let middle_name: Option<String> = row.get(4);
        let surname: Option<String> = row.get(5);
        let name = match (&first_name, &middle_name, &surname) {
            (Some(f), Some(m), Some(s)) if !m.is_empty() => format!("{} {} {}", f, m, s),
            (Some(f), _, Some(s)) => format!("{} {}", f, s),
            (Some(f), Some(m), None) if !m.is_empty() => format!("{} {}", f, m),
            (Some(f), _, _) => f.clone(),
            (_, _, Some(s)) => s.clone(),
            _ => String::new(),
        };

        serde_json::json!({
            "id": row.get::<i64, _>(0),
            "person_id": row.get::<i64, _>(1),
            "club_id": row.get::<Option<String>, _>(2),
            "name": name,
            "first_name": first_name,
            "middle_name": middle_name,
            "surname": surname,
            "birth_year": row.get::<Option<i32>, _>(6),
            "profession": row.get::<Option<String>, _>(7),
            "street_address": row.get::<Option<String>, _>(8),
            "civil_parish": row.get::<Option<String>, _>(9),
            "ecclesiastical_parish": row.get::<Option<String>, _>(10),
            "registration_district": row.get::<Option<String>, _>(11),
            "birth_town": row.get::<Option<String>, _>(12),
            "birth_county": row.get::<Option<String>, _>(13),
            "birth_country": row.get::<Option<String>, _>(14),
            "where_born": row.get::<Option<String>, _>(15),
            "census_age": row.get::<Option<i32>, _>(16),
            "census_relation": row.get::<Option<String>, _>(17),
            "census_gender": row.get::<Option<String>, _>(18),
            "census_ed": row.get::<Option<String>, _>(19),
            "census_household_schedule": row.get::<Option<String>, _>(20),
            "census_piece": row.get::<Option<String>, _>(21),
            "census_folio": row.get::<Option<String>, _>(22),
            "census_page": row.get::<Option<String>, _>(23),
            "house_number": row.get::<Option<String>, _>(24),
            "sub_area": row.get::<Option<String>, _>(25),
            "street_name": row.get::<Option<String>, _>(26),
            "postcode": row.get::<Option<String>, _>(27),
            "latitude": row.get::<Option<f64>, _>(28),
            "longitude": row.get::<Option<f64>, _>(29),
            "occupation_expanded": row.get::<Option<String>, _>(30)
        })
    }).collect();

    Ok(result)
}

/// Get players with pagination and search - JOINs with sheffield_people for full data
pub async fn get_players_paginated(
    page: usize,
    page_size: usize,
    search: Option<String>,
    parish: Option<String>,
    birth_year: Option<i32>,
    _assignment_filter: Option<String>,
) -> Result<PlayerQueryResult, Box<dyn std::error::Error>> {
    let pool = super::get_pool().await?;
    let offset = (page * page_size) as i64;
    let limit = page_size as i64;

    let mut where_parts = vec!["1=1"];
    let has_search = search.as_ref().map_or(false, |s| !s.trim().is_empty());
    let has_parish = parish.as_ref().map_or(false, |s| !s.trim().is_empty());

    if has_search {
        where_parts.push("(p.first_name LIKE '%' || ? || '%' OR p.surname LIKE '%' || ? || '%' OR p.middle_name LIKE '%' || ? || '%')");
    }
    if birth_year.is_some() {
        where_parts.push("p.birth_year = ?");
    }
    if has_parish {
        where_parts.push("p.civil_parish LIKE '%' || ? || '%'");
    }

    let where_clause = where_parts.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*) FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE {}",
        where_clause
    );
    let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);

    if has_search {
        let search_val = search.as_ref().unwrap();
        count_query = count_query.bind(search_val).bind(search_val).bind(search_val);
    }
    if let Some(year) = birth_year {
        count_query = count_query.bind(year);
    }
    if has_parish {
        count_query = count_query.bind(parish.as_ref().unwrap());
    }

    let total_count = count_query.fetch_one(&pool).await?;

    let data_sql = format!(
        "SELECT
            f.id,
            f.person_id,
            f.club_id,
            p.first_name,
            p.middle_name,
            p.surname,
            p.birth_year,
            p.profession,
            p.street_address,
            p.civil_parish,
            p.ecclesiastical_parish,
            p.registration_district,
            p.birth_town,
            p.birth_county,
            p.birth_country,
            p.where_born,
            p.census_age,
            p.census_relation,
            p.census_gender,
            p.census_ed,
            p.census_household_schedule,
            p.census_piece,
            p.census_folio,
            p.census_page,
            p.house_number,
            p.sub_area,
            p.street_name,
            p.postcode,
            p.latitude,
            p.longitude,
            p.occupation_expanded
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE {}
         ORDER BY p.surname ASC, p.first_name ASC
         LIMIT ? OFFSET ?",
        where_clause
    );

    let mut data_query = sqlx::query(&data_sql);

    if has_search {
        let search_val = search.as_ref().unwrap();
        data_query = data_query.bind(search_val).bind(search_val).bind(search_val);
    }
    if let Some(year) = birth_year {
        data_query = data_query.bind(year);
    }
    if has_parish {
        data_query = data_query.bind(parish.as_ref().unwrap());
    }

    data_query = data_query.bind(limit).bind(offset);

    let players = data_query.fetch_all(&pool).await?;

    let result: Vec<serde_json::Value> = players.iter().map(|row| {
        let first_name: Option<String> = row.get(3);
        let middle_name: Option<String> = row.get(4);
        let surname: Option<String> = row.get(5);
        let name = match (&first_name, &middle_name, &surname) {
            (Some(f), Some(m), Some(s)) if !m.is_empty() => format!("{} {} {}", f, m, s),
            (Some(f), _, Some(s)) => format!("{} {}", f, s),
            (Some(f), Some(m), None) if !m.is_empty() => format!("{} {}", f, m),
            (Some(f), _, _) => f.clone(),
            (_, _, Some(s)) => s.clone(),
            _ => String::new(),
        };

        serde_json::json!({
            "id": row.get::<i64, _>(0),
            "person_id": row.get::<i64, _>(1),
            "club_id": row.get::<Option<String>, _>(2),
            "name": name,
            "first_name": first_name,
            "middle_name": middle_name,
            "surname": surname,
            "birth_year": row.get::<Option<i32>, _>(6),
            "profession": row.get::<Option<String>, _>(7),
            "street_address": row.get::<Option<String>, _>(8),
            "civil_parish": row.get::<Option<String>, _>(9),
            "ecclesiastical_parish": row.get::<Option<String>, _>(10),
            "registration_district": row.get::<Option<String>, _>(11),
            "birth_town": row.get::<Option<String>, _>(12),
            "birth_county": row.get::<Option<String>, _>(13),
            "birth_country": row.get::<Option<String>, _>(14),
            "where_born": row.get::<Option<String>, _>(15),
            "census_age": row.get::<Option<i32>, _>(16),
            "census_relation": row.get::<Option<String>, _>(17),
            "census_gender": row.get::<Option<String>, _>(18),
            "census_ed": row.get::<Option<String>, _>(19),
            "census_household_schedule": row.get::<Option<String>, _>(20),
            "census_piece": row.get::<Option<String>, _>(21),
            "census_folio": row.get::<Option<String>, _>(22),
            "census_page": row.get::<Option<String>, _>(23),
            "house_number": row.get::<Option<String>, _>(24),
            "sub_area": row.get::<Option<String>, _>(25),
            "street_name": row.get::<Option<String>, _>(26),
            "postcode": row.get::<Option<String>, _>(27),
            "latitude": row.get::<Option<f64>, _>(28),
            "longitude": row.get::<Option<f64>, _>(29),
            "occupation_expanded": row.get::<Option<String>, _>(30)
        })
    }).collect();

    Ok(PlayerQueryResult {
        players: result,
        total_count: total_count as usize,
        page,
        page_size,
    })
}

/// Get age statistics for all players - uses sheffield_people data
pub async fn get_age_statistics(year: i32) -> Result<Vec<AgeStatistic>, Box<dyn std::error::Error>> {
    let pool = super::get_pool().await?;

    let stats = sqlx::query_as::<_, (i32, i32, i32)>(
        "SELECT
            (? - p.birth_year) as age,
            p.birth_year,
            COUNT(*) as count
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.birth_year IS NOT NULL
         GROUP BY p.birth_year
         ORDER BY age ASC"
    )
    .bind(year)
    .fetch_all(&pool)
    .await?;

    Ok(stats.into_iter().map(|(age, birth_year, count)| AgeStatistic {
        age,
        birth_year,
        count,
    }).collect())
}

/// Get first name statistics - uses sheffield_people data
pub async fn get_first_name_statistics() -> Result<Vec<NameStatistic>, Box<dyn std::error::Error>> {
    let pool = super::get_pool().await?;

    let stats = sqlx::query_as::<_, (String, i32)>(
        "SELECT p.first_name as name, COUNT(*) as count
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.first_name IS NOT NULL AND p.first_name != ''
         GROUP BY p.first_name
         ORDER BY count DESC, name ASC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(stats.into_iter().map(|(name, count)| NameStatistic {
        name,
        count,
    }).collect())
}

/// Get surname statistics - uses sheffield_people data
pub async fn get_surname_statistics() -> Result<Vec<NameStatistic>, Box<dyn std::error::Error>> {
    let pool = super::get_pool().await?;

    let stats = sqlx::query_as::<_, (String, i32)>(
        "SELECT p.surname as name, COUNT(*) as count
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.surname IS NOT NULL AND p.surname != ''
         GROUP BY p.surname
         ORDER BY count DESC, name ASC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(stats.into_iter().map(|(name, count)| NameStatistic {
        name,
        count,
    }).collect())
}

/// Create database indexes for better performance
pub async fn create_player_indexes() -> Result<(), Box<dyn std::error::Error>> {
    let pool = super::get_pool_write().await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_footballers_person_id ON sheffield_footballers(person_id)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_people_unique_id ON sheffield_people(unique_id)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_people_surname ON sheffield_people(surname)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_people_first_name ON sheffield_people(first_name)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_people_birth_year ON sheffield_people(birth_year)")
        .execute(&pool)
        .await?;

    Ok(())
}

/// Get count of female footballers
pub async fn get_female_footballers_count() -> Result<i64, Box<dyn std::error::Error>> {
    let pool = super::get_pool().await?;

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.census_gender = 'F'"
    )
    .fetch_one(&pool)
    .await?;

    Ok(count)
}

/// Delete all female players from sheffield_footballers
pub async fn delete_female_footballers() -> Result<i64, Box<dyn std::error::Error>> {
    let pool = super::get_pool_write().await?;

    let result = sqlx::query(
        "DELETE FROM sheffield_footballers
         WHERE person_id IN (
            SELECT unique_id FROM sheffield_people WHERE census_gender = 'F'
         )"
    )
    .execute(&pool)
    .await?;

    Ok(result.rows_affected() as i64)
}
