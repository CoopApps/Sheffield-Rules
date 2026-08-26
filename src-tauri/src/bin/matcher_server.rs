use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct GenealogyRecord {
    id: String,
    name: String,
    address: Option<String>,
    profession: Option<String>,
    age: Option<i64>,
    is_matched: bool,
}

#[derive(Debug, Serialize)]
struct BusinessRecord {
    id: String,
    name: String,
    address: Option<String>,
    occupation: Option<String>,
    year: i64,
    is_matched: bool,
}

#[derive(Debug, Serialize)]
struct DataResponse {
    genealogy: Vec<GenealogyRecord>,
    businesses: Vec<BusinessRecord>,
    stats: DataStats,
}

#[derive(Debug, Serialize)]
struct DataStats {
    total_genealogy: i64,
    total_businesses: i64,
    current_offset: i64,
    page_size: i64,
}

#[derive(Debug, Deserialize)]
struct DataQuery {
    offset: Option<i64>,
    limit: Option<i64>,
    gen_name: Option<String>,
    gen_profession: Option<String>,
    gen_address: Option<String>,
    bus_name: Option<String>,
    bus_occupation: Option<String>,
    bus_address: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApprovalRequest {
    gen_id: String,  // Changed from i64 to String
    business_id: String,
    business_name: String,
    business_occupation: String,
    business_address: String,
    business_year: i64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RecordId {
    Int(i64),
    String(String),
}

#[derive(Debug, Deserialize)]
struct DeleteRequest {
    record_type: String,  // "genealogy" or "business"
    record_id: RecordId,
    name: Option<String>,      // For genealogy records with empty IDs
    address: Option<String>,   // For genealogy records with empty IDs
}

#[derive(Debug, Deserialize)]
struct UpdateGenealogyRequest {
    id: String,
    old_name: Option<String>,      // Original name before edit (for finding record)
    old_address: Option<String>,   // Original address before edit
    name: String,
    address: String,
    profession: String,
}

#[derive(Debug, Deserialize)]
struct UpdateBusinessRequest {
    id: String,
    surname: String,
    forename: String,
    occupation: String,
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    // Clone pool for routes
    let pool_clone = pool.clone();

    // GET /data - Load paginated genealogy and business records
    let data_route = warp::path("data")
        .and(warp::get())
        .and(warp::query::<DataQuery>())
        .and(warp::any().map(move || pool_clone.clone()))
        .and_then(handle_get_data);

    // POST /approve - Approve a match
    let pool_clone2 = pool.clone();
    let approve_route = warp::path("approve")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool_clone2.clone()))
        .and_then(handle_approve);

    // POST /delete - Delete a record
    let pool_clone3 = pool.clone();
    let delete_route = warp::path("delete")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool_clone3.clone()))
        .and_then(handle_delete);

    // POST /update_genealogy - Update a genealogy record
    let pool_clone4 = pool.clone();
    let update_genealogy_route = warp::path("update_genealogy")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool_clone4.clone()))
        .and_then(handle_update_genealogy);

    // POST /update_business - Update a business record
    let pool_clone5 = pool.clone();
    let update_business_route = warp::path("update_business")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool_clone5.clone()))
        .and_then(handle_update_business);

    // POST /unmatch - Unmatch a genealogy record
    let pool_clone6 = pool.clone();
    let unmatch_route = warp::path("unmatch")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool_clone6.clone()))
        .and_then(handle_unmatch);

    // POST /unmatch_by_business - Unmatch by business name and year
    let pool_clone7 = pool.clone();
    let unmatch_by_business_route = warp::path("unmatch_by_business")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || pool_clone7.clone()))
        .and_then(handle_unmatch_by_business);

    // GET /name_matches - Get all genealogy and business records with matching names
    let pool_clone8 = pool.clone();
    let name_matches_route = warp::path("name_matches")
        .and(warp::get())
        .and(warp::any().map(move || pool_clone8.clone()))
        .and_then(handle_name_matches);

    // POST /remove_duplicates - Remove exact duplicate genealogy records
    let pool_clone9 = pool.clone();
    let remove_duplicates_route = warp::path("remove_duplicates")
        .and(warp::post())
        .and(warp::any().map(move || pool_clone9.clone()))
        .and_then(handle_remove_duplicates);

    // POST /normalize_addresses - Normalize genealogy addresses to match business format
    let pool_clone10 = pool.clone();
    let normalize_addresses_route = warp::path("normalize_addresses")
        .and(warp::post())
        .and(warp::any().map(move || pool_clone10.clone()))
        .and_then(handle_normalize_addresses);

    // CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type"]);

    let routes = data_route
        .or(approve_route)
        .or(delete_route)
        .or(update_genealogy_route)
        .or(update_business_route)
        .or(unmatch_route)
        .or(unmatch_by_business_route)
        .or(name_matches_route)
        .or(remove_duplicates_route)
        .or(normalize_addresses_route)
        .with(cors);

    println!("========================================");
    println!("MATCHER SERVER STARTING");
    println!("========================================");
    println!("Server running at http://localhost:3030");
    println!("Open matcher-app/index.html in your browser");
    println!("========================================\n");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

async fn handle_get_data(
    query: DataQuery,
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(100).min(150000); // Max 150000 to allow loading all genealogy records (~124k)

    println!("Loading data: offset={}, limit={}", offset, limit);

    // Build genealogy query with filters
    let mut gen_where = vec!["address IS NOT NULL AND address != ''",
                             "name IS NOT NULL AND name != ''"];
    let mut gen_params: Vec<String> = Vec::new();

    if let Some(name) = &query.gen_name {
        gen_where.push("name LIKE ?");
        gen_params.push(format!("%{}%", name));
    }
    if let Some(profession) = &query.gen_profession {
        gen_where.push("profession LIKE ?");
        gen_params.push(format!("%{}%", profession));
    }
    if let Some(address) = &query.gen_address {
        gen_where.push("address LIKE ?");
        gen_params.push(format!("%{}%", address));
    }

    let gen_query = format!(
        "SELECT id, name, address, profession, age, business_surname FROM unmatched_genealogy
         WHERE {} ORDER BY name LIMIT ? OFFSET ?",
        gen_where.join(" AND ")
    );

    // Execute genealogy query with dynamic parameters
    let mut gen_sql = sqlx::query(&gen_query);
    for param in &gen_params {
        gen_sql = gen_sql.bind(param);
    }
    gen_sql = gen_sql.bind(limit).bind(offset);

    let rows = gen_sql.fetch_all(&pool).await.map_err(|e| {
        eprintln!("Error loading genealogy: {}", e);
        warp::reject::reject()
    })?;

    let genealogy: Vec<GenealogyRecord> = rows.iter().map(|row| {
        let business_surname: Option<String> = row.get("business_surname");
        GenealogyRecord {
            id: row.get("id"),
            name: row.get("name"),
            address: row.get("address"),
            profession: row.get("profession"),
            age: row.get("age"),
            is_matched: business_surname.is_some() && !business_surname.as_ref().unwrap().is_empty(),
        }
    }).collect();

    // Build business query with filters
    let mut bus_where = vec!["address IS NOT NULL AND address != ''",
                             "surname IS NOT NULL AND forename IS NOT NULL"];
    let mut bus_params: Vec<String> = Vec::new();

    if let Some(name) = &query.bus_name {
        bus_where.push("(surname LIKE ? OR forename LIKE ?)");
        bus_params.push(format!("%{}%", name));
        bus_params.push(format!("%{}%", name));
    }
    if let Some(occupation) = &query.bus_occupation {
        bus_where.push("occupation LIKE ?");
        bus_params.push(format!("%{}%", occupation));
    }
    if let Some(address) = &query.bus_address {
        bus_where.push("address LIKE ?");
        bus_params.push(format!("%{}%", address));
    }

    let bus_query = format!(
        "SELECT id, surname, forename, occupation, address, year
         FROM sheffield_businesses
         WHERE {} ORDER BY surname, forename LIMIT ? OFFSET ?",
        bus_where.join(" AND ")
    );

    // Execute business query with dynamic parameters
    let mut bus_sql = sqlx::query_as::<_, (String, String, String, String, String, i64)>(&bus_query);
    for param in &bus_params {
        bus_sql = bus_sql.bind(param);
    }
    bus_sql = bus_sql.bind(limit).bind(offset);
    let bus_rows = bus_sql.fetch_all(&pool).await.map_err(|e| {
        eprintln!("Error loading businesses: {}", e);
        warp::reject::reject()
    })?;

    // Get list of matched business IDs from genealogy table
    // Include address to distinguish between people with same name
    let matched_business_ids: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT business_forename || ' ' || business_surname || '-' || business_address || '-' || business_year
         FROM unmatched_genealogy
         WHERE business_surname IS NOT NULL AND business_surname != ''"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let businesses: Vec<BusinessRecord> = bus_rows.into_iter()
        .map(|(id, surname, forename, occupation, address, year)| {
            let business_key = format!("{} {}-{}-{}", forename, surname, address, year);
            BusinessRecord {
                id: id.clone(),
                name: format!("{} {}", forename, surname),
                address: Some(address),
                occupation: Some(occupation),
                year,
                is_matched: matched_business_ids.contains(&business_key),
            }
        }).collect();

    // Get total counts
    let total_gen: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address IS NOT NULL AND name IS NOT NULL"
    ).fetch_one(&pool).await.unwrap_or((0,));

    let total_bus: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses
         WHERE address IS NOT NULL AND surname IS NOT NULL"
    ).fetch_one(&pool).await.unwrap_or((0,));

    println!("Loaded {} genealogy records and {} businesses (of {} total genealogy, {} total businesses)",
             genealogy.len(), businesses.len(), total_gen.0, total_bus.0);

    Ok(warp::reply::json(&DataResponse {
        genealogy,
        businesses,
        stats: DataStats {
            total_genealogy: total_gen.0,
            total_businesses: total_bus.0,
            current_offset: offset,
            page_size: limit,
        },
    }))
}

async fn handle_approve(req: ApprovalRequest, pool: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Approving match: {} -> {}", req.gen_id, req.business_name);

    // Extract surname and forename from business name
    let name_parts: Vec<&str> = req.business_name.split_whitespace().collect();
    let surname = if !name_parts.is_empty() {
        name_parts[name_parts.len() - 1]
    } else {
        ""
    };
    let forename = if name_parts.len() > 1 {
        name_parts[..name_parts.len() - 1].join(" ")
    } else {
        String::new()
    };

    sqlx::query(
        "UPDATE unmatched_genealogy
         SET business_surname = ?,
             business_forename = ?,
             business_occupation = ?,
             business_address = ?,
             business_year = ?,
             business_source = ?
         WHERE id = ?"
    )
    .bind(surname)
    .bind(&forename)
    .bind(&req.business_occupation)
    .bind(&req.business_address)
    .bind(req.business_year.to_string())
    .bind("White's 1871")
    .bind(req.gen_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error updating database: {}", e);
        warp::reject::reject()
    })?;

    Ok(warp::reply::json(&serde_json::json!({"success": true})))
}

async fn handle_delete(
    req: DeleteRequest,
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = match req.record_type.as_str() {
        "genealogy" => {
            // If ID is empty, use name and address to identify the record
            if let (Some(name), Some(address)) = (&req.name, &req.address) {
                println!("Deleting genealogy record by name/address: {} at {}", name, address);
                // SQLite doesn't support LIMIT in DELETE, so we use rowid subquery
                sqlx::query(
                    "DELETE FROM unmatched_genealogy
                     WHERE rowid = (
                         SELECT rowid FROM unmatched_genealogy
                         WHERE name = ? AND address = ?
                         LIMIT 1
                     )"
                )
                    .bind(name)
                    .bind(address)
                    .execute(&pool)
                    .await
            } else {
                let id_str = match &req.record_id {
                    RecordId::String(s) => s.clone(),
                    RecordId::Int(i) => i.to_string(),
                };
                println!("Deleting genealogy record by ID: {}", id_str);
                sqlx::query("DELETE FROM unmatched_genealogy WHERE id = ?")
                    .bind(&id_str)
                    .execute(&pool)
                    .await
            }
        }
        "business" => {
            let id_str = match &req.record_id {
                RecordId::String(s) => s.clone(),
                RecordId::Int(i) => i.to_string(),
            };
            println!("Deleting business record: {}", id_str);
            sqlx::query("DELETE FROM sheffield_businesses WHERE id = ?")
                .bind(&id_str)
                .execute(&pool)
                .await
        }
        _ => return Err(warp::reject::reject())
    };

    match result {
        Ok(rows) if rows.rows_affected() > 0 => {
            println!("Successfully deleted record");
            Ok(warp::reply::json(&serde_json::json!({"success": true})))
        }
        Ok(_) => {
            println!("Record not found");
            Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": "Record not found"
            })))
        }
        Err(e) => {
            eprintln!("Delete error: {}", e);
            Err(warp::reject::reject())
        }
    }
}

async fn handle_update_genealogy(
    req: UpdateGenealogyRequest,
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Updating genealogy record");

    let result = if let (Some(old_name), Some(old_address)) = (&req.old_name, &req.old_address) {
        // Use old name/address to find the record
        println!("Updating by name/address: {} at {}", old_name, old_address);
        sqlx::query(
            "UPDATE unmatched_genealogy
             SET name = ?, address = ?, profession = ?
             WHERE rowid = (
                 SELECT rowid FROM unmatched_genealogy
                 WHERE name = ? AND address = ?
                 LIMIT 1
             )"
        )
        .bind(&req.name)
        .bind(&req.address)
        .bind(&req.profession)
        .bind(old_name)
        .bind(old_address)
        .execute(&pool)
        .await
    } else {
        // Use ID if available
        println!("Updating by ID: {}", req.id);
        sqlx::query(
            "UPDATE unmatched_genealogy
             SET name = ?, address = ?, profession = ?
             WHERE id = ?"
        )
        .bind(&req.name)
        .bind(&req.address)
        .bind(&req.profession)
        .bind(&req.id)
        .execute(&pool)
        .await
    };

    match result {
        Ok(rows) if rows.rows_affected() > 0 => {
            println!("Successfully updated genealogy record");
            Ok(warp::reply::json(&serde_json::json!({"success": true})))
        }
        Ok(_) => {
            println!("Genealogy record not found");
            Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": "Record not found"
            })))
        }
        Err(e) => {
            eprintln!("Update error: {}", e);
            Err(warp::reject::reject())
        }
    }
}

async fn handle_update_business(
    req: UpdateBusinessRequest,
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Updating business record: {}", req.id);

    let result = sqlx::query(
        "UPDATE sheffield_businesses
         SET surname = ?, forename = ?, occupation = ?, address = ?
         WHERE id = ?"
    )
    .bind(&req.surname)
    .bind(&req.forename)
    .bind(&req.occupation)
    .bind(&req.address)
    .bind(&req.id)
    .execute(&pool)
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => {
            println!("Successfully updated business record");
            Ok(warp::reply::json(&serde_json::json!({"success": true})))
        }
        Ok(_) => {
            println!("Business record not found");
            Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": "Record not found"
            })))
        }
        Err(e) => {
            eprintln!("Update error: {}", e);
            Err(warp::reject::reject())
        }
    }
}

#[derive(Debug, Deserialize)]
struct UnmatchRequest {
    gen_id: String,
}

async fn handle_unmatch(
    req: UnmatchRequest,
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Unmatching genealogy record: {}", req.gen_id);

    let result = sqlx::query(
        "UPDATE unmatched_genealogy
         SET business_surname = NULL,
             business_forename = NULL,
             business_occupation = NULL,
             business_address = NULL,
             business_year = NULL,
             business_source = NULL
         WHERE id = ?"
    )
    .bind(&req.gen_id)
    .execute(&pool)
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => {
            println!("Successfully unmatched record");
            Ok(warp::reply::json(&serde_json::json!({"success": true})))
        }
        Ok(_) => {
            println!("Record not found");
            Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": "Record not found"
            })))
        }
        Err(e) => {
            eprintln!("Unmatch error: {}", e);
            Err(warp::reject::reject())
        }
    }
}

#[derive(Debug, Deserialize)]
struct UnmatchByBusinessRequest {
    business_name: String,
    business_year: i64,
}

async fn handle_unmatch_by_business(
    req: UnmatchByBusinessRequest,
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Unmatching by business: {} ({})", req.business_name, req.business_year);

    // Parse business name into forename and surname
    let name_parts: Vec<&str> = req.business_name.split_whitespace().collect();
    let surname = if !name_parts.is_empty() {
        name_parts[name_parts.len() - 1]
    } else {
        ""
    };
    let forename = if name_parts.len() > 1 {
        name_parts[..name_parts.len() - 1].join(" ")
    } else {
        String::new()
    };

    let result = sqlx::query(
        "UPDATE unmatched_genealogy
         SET business_surname = NULL,
             business_forename = NULL,
             business_occupation = NULL,
             business_address = NULL,
             business_year = NULL,
             business_source = NULL
         WHERE business_surname = ?
           AND business_forename = ?
           AND business_year = ?"
    )
    .bind(surname)
    .bind(&forename)
    .bind(req.business_year.to_string())
    .execute(&pool)
    .await;

    match result {
        Ok(rows) if rows.rows_affected() > 0 => {
            println!("Successfully unmatched {} record(s)", rows.rows_affected());
            Ok(warp::reply::json(&serde_json::json!({"success": true})))
        }
        Ok(_) => {
            println!("No matching record found");
            Ok(warp::reply::json(&serde_json::json!({
                "success": false,
                "error": "Record not found"
            })))
        }
        Err(e) => {
            eprintln!("Unmatch error: {}", e);
            Err(warp::reject::reject())
        }
    }
}


async fn handle_name_matches(pool: SqlitePool) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Finding all name matches...");

    // Simple name matching - just lowercase comparison
    let gen_query = "
        SELECT DISTINCT g.id, g.name, g.address, g.profession, g.age, g.business_surname
        FROM unmatched_genealogy g
        INNER JOIN sheffield_businesses b
        ON LOWER(TRIM(g.name)) = LOWER(TRIM(b.forename || ' ' || b.surname))
        WHERE g.address IS NOT NULL AND g.address != ''
          AND g.name IS NOT NULL AND g.name != ''
        LIMIT 5000
    ";

    println!("Executing genealogy query...");
    let rows = sqlx::query_as::<_, (String, String, Option<String>, Option<String>, Option<i64>, Option<String>)>(gen_query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("ERROR finding name matches: {}", e);
            eprintln!("Query was: {}", gen_query);
            warp::reject::reject()
        })?;

    println!("Query returned {} genealogy rows", rows.len());

    let genealogy: Vec<GenealogyRecord> = rows.into_iter().map(|(id, name, address, profession, age, business_surname)| {
        GenealogyRecord {
            id,
            name,
            address,
            profession,
            age,
            is_matched: business_surname.is_some() && !business_surname.as_ref().unwrap().is_empty(),
        }
    }).collect();

    let bus_query = "
        SELECT DISTINCT b.id, b.surname, b.forename, b.occupation, b.address, b.year
        FROM sheffield_businesses b
        INNER JOIN unmatched_genealogy g
        ON LOWER(TRIM(b.forename || ' ' || b.surname)) = LOWER(TRIM(g.name))
        WHERE b.address IS NOT NULL AND b.address != ''
          AND b.surname IS NOT NULL AND b.forename IS NOT NULL
        LIMIT 5000
    ";

    let bus_rows = sqlx::query_as::<_, (String, String, String, String, String, i64)>(bus_query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error finding business matches: {}", e);
            warp::reject::reject()
        })?;

    println!("Query returned {} business rows", bus_rows.len());

    let matched_business_ids: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT business_forename || ' ' || business_surname || '-' || business_address || '-' || business_year
         FROM unmatched_genealogy
         WHERE business_surname IS NOT NULL AND business_surname != ''"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let businesses: Vec<BusinessRecord> = bus_rows.into_iter()
        .map(|(id, surname, forename, occupation, address, year)| {
            let business_key = format!("{} {}-{}-{}", forename, surname, address, year);
            BusinessRecord {
                id,
                name: format!("{} {}", forename, surname),
                address: Some(address),
                occupation: Some(occupation),
                year,
                is_matched: matched_business_ids.contains(&business_key),
            }
        }).collect();

    let gen_count = genealogy.len() as i64;
    let bus_count = businesses.len() as i64;

    println!("Found {} genealogy matches and {} business matches", gen_count, bus_count);

    Ok(warp::reply::json(&DataResponse {
        genealogy,
        businesses,
        stats: DataStats {
            total_genealogy: gen_count,
            total_businesses: bus_count,
            current_offset: 0,
            page_size: 5000,
        },
    }))
}

async fn handle_remove_duplicates(
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("\n========================================");
    println!("REMOVING DUPLICATE GENEALOGY RECORDS");
    println!("========================================");

    // Get count before deletion
    let count_before: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error getting count before deletion: {}", e);
        warp::reject::reject()
    })?;

    println!("Records before deletion: {}", count_before.0);

    // Delete duplicates - keep the record with the minimum rowid for each (name, address) pair
    let delete_result = sqlx::query(
        "DELETE FROM unmatched_genealogy
         WHERE rowid NOT IN (
             SELECT MIN(rowid)
             FROM unmatched_genealogy
             GROUP BY name, address
         )"
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error deleting duplicates: {}", e);
        warp::reject::reject()
    })?;

    let deleted_count = delete_result.rows_affected();
    println!("Deleted {} duplicate records", deleted_count);

    // VACUUM the database to reclaim space and update statistics
    sqlx::query("VACUUM")
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error running VACUUM: {}", e);
            warp::reject::reject()
        })?;

    println!("VACUUM completed");

    // Get count after deletion
    let count_after: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error getting count after deletion: {}", e);
        warp::reject::reject()
    })?;

    println!("Records after deletion: {}", count_after.0);
    println!("========================================\n");

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "deleted": deleted_count,
        "before": count_before.0,
        "after": count_after.0
    })))
}

fn normalize_address(address: &str) -> String {
    use regex::Regex;

    let mut normalized = address.to_string();

    // Replace comma with space
    normalized = normalized.replace(',', " ");

    // Handle "back" prefix (e.g., "backhouse" -> "Back house")
    let back_re = Regex::new(r"\bback([a-z])").unwrap();
    normalized = back_re.replace_all(&normalized, "Back $1").to_string();

    // Handle other directional prefixes
    let new_re = Regex::new(r"\bnew([A-Z])").unwrap();
    normalized = new_re.replace_all(&normalized, "New $1").to_string();

    let old_re = Regex::new(r"\bold([A-Z])").unwrap();
    normalized = old_re.replace_all(&normalized, "Old $1").to_string();

    let west_re = Regex::new(r"\bwest([A-Z])").unwrap();
    normalized = west_re.replace_all(&normalized, "West $1").to_string();

    let east_re = Regex::new(r"\beast([A-Z])").unwrap();
    normalized = east_re.replace_all(&normalized, "East $1").to_string();

    let north_re = Regex::new(r"\bnorth([A-Z])").unwrap();
    normalized = north_re.replace_all(&normalized, "North $1").to_string();

    let south_re = Regex::new(r"\bsouth([A-Z])").unwrap();
    normalized = south_re.replace_all(&normalized, "South $1").to_string();

    // Add space before capital letters that follow lowercase letters
    let camel_re = Regex::new(r"([a-z])([A-Z])").unwrap();
    normalized = camel_re.replace_all(&normalized, "$1 $2").to_string();

    // Collapse multiple spaces
    let space_re = Regex::new(r"\s+").unwrap();
    normalized = space_re.replace_all(&normalized, " ").to_string();

    normalized.trim().to_string()
}

async fn handle_normalize_addresses(
    pool: SqlitePool
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("\n========================================");
    println!("NORMALIZING GENEALOGY ADDRESSES");
    println!("========================================");

    // Get all addresses
    let rows: Vec<(i64, String)> = sqlx::query_as(
        "SELECT rowid, address FROM unmatched_genealogy
         WHERE address IS NOT NULL AND address != ''"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching addresses: {}", e);
        warp::reject::reject()
    })?;

    let total_count = rows.len();
    println!("Found {} addresses to process", total_count);

    let mut updated = 0;
    let mut unchanged = 0;
    let mut samples_shown = 0;

    println!("\nSample transformations:");

    for (rowid, address) in rows {
        let normalized = normalize_address(&address);

        if normalized != address {
            sqlx::query("UPDATE unmatched_genealogy SET address = ? WHERE rowid = ?")
                .bind(&normalized)
                .bind(rowid)
                .execute(&pool)
                .await
                .map_err(|e| {
                    eprintln!("Error updating address: {}", e);
                    warp::reject::reject()
                })?;

            updated += 1;

            // Show first 10 examples
            if samples_shown < 10 {
                println!("  \"{}\" -> \"{}\"", address, normalized);
                samples_shown += 1;
            }

            if updated % 10000 == 0 {
                println!("\nUpdated {} addresses...", updated);
            }
        } else {
            unchanged += 1;
        }
    }

    println!("\n========================================");
    println!("COMPLETE");
    println!("========================================");
    println!("Updated: {}", updated);
    println!("Unchanged: {}", unchanged);
    println!("Total: {}", total_count);
    println!("========================================\n");

    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "updated": updated,
        "unchanged": unchanged,
        "total": total_count
    })))
}
