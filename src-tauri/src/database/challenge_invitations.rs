use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Duration, Datelike};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ChallengeInvitation {
    pub id: String,
    pub sender_club_id: String,
    pub recipient_club_id: String,
    pub sent_date: String,
    pub response_date: String,
    pub proposed_match_date: String,
    pub match_type: String,
    pub venue: String,
    pub stakes: String,
    pub tone: String,
    pub rules_type: String,
    pub match_duration: i32,
    pub status: String,
    pub response_text: Option<String>,
    pub acceptance_likelihood: Option<i32>,
    pub decline_reason: Option<String>,
    pub sent_news_id: Option<String>,
    pub response_news_id: Option<String>,
    pub scheduled_match_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct NewsItem {
    pub id: String,
    pub headline: String,
    pub article_type: String,
    pub publish_date: String,
    pub body_text: String,
    pub is_read: bool,
    pub is_important: bool,
    pub requires_action: bool,
    pub related_club_ids: Option<String>,
    pub related_player_ids: Option<String>,
    pub related_match_id: Option<String>,
    pub related_invitation_id: Option<String>,
    pub has_action_button: bool,
    pub action_button_text: Option<String>,
    pub action_type: Option<String>,
    pub action_data: Option<String>,
}

/// Send a challenge letter to another club
pub async fn send_challenge_letter(
    pool: &SqlitePool,
    sender_club_id: &str,
    recipient_club_id: &str,
    current_date: &str,
    proposed_match_date: &str,
    match_type: &str,
    venue: &str,
    stakes: &str,
    tone: &str,
    rules_type: &str,
    match_duration: i32,
) -> Result<ChallengeInvitation, sqlx::Error> {
    let invitation_id = uuid::Uuid::new_v4().to_string();

    // Calculate response date (3-4 days later - let's use 3 for now)
    let sent_date = NaiveDate::parse_from_str(current_date, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let response_date = sent_date + Duration::days(3);
    let response_date_str = response_date.format("%Y-%m-%d").to_string();

    // Validate that proposed match date is at least 7 days after response date
    let proposed_date = NaiveDate::parse_from_str(proposed_match_date, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let minimum_match_date = response_date + Duration::days(7);

    let final_match_date = if proposed_date < minimum_match_date {
        // If proposed date is too soon, automatically schedule for minimum date
        minimum_match_date.format("%Y-%m-%d").to_string()
    } else {
        proposed_match_date.to_string()
    };

    // Calculate acceptance likelihood (we'll implement proper logic later)
    let acceptance_likelihood = calculate_acceptance_likelihood(
        pool, sender_club_id, recipient_club_id, proposed_match_date, stakes
    ).await?;

    // Insert invitation
    sqlx::query(
        r#"
        INSERT INTO sheffield_challenge_invitations (
            id, sender_club_id, recipient_club_id, sent_date, response_date,
            proposed_match_date, match_type, venue, stakes, tone, rules_type,
            match_duration, status, acceptance_likelihood
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'sent', ?)
        "#
    )
    .bind(&invitation_id)
    .bind(sender_club_id)
    .bind(recipient_club_id)
    .bind(current_date)
    .bind(&response_date_str)
    .bind(&final_match_date)
    .bind(match_type)
    .bind(venue)
    .bind(stakes)
    .bind(tone)
    .bind(rules_type)
    .bind(match_duration)
    .bind(acceptance_likelihood)
    .execute(pool)
    .await?;

    // Create news item about sending the letter
    let news_id = create_challenge_sent_news(
        pool, &invitation_id, sender_club_id, recipient_club_id, current_date,
        &final_match_date, venue, stakes, rules_type, match_duration
    ).await?;

    // Update invitation with news ID
    sqlx::query(
        "UPDATE sheffield_challenge_invitations SET sent_news_id = ? WHERE id = ?"
    )
    .bind(&news_id)
    .bind(&invitation_id)
    .execute(pool)
    .await?;

    // Fetch and return the created invitation
    get_invitation_by_id(pool, &invitation_id).await
}

/// Calculate how likely a club is to accept based on various factors
async fn calculate_acceptance_likelihood(
    pool: &SqlitePool,
    sender_club_id: &str,
    recipient_club_id: &str,
    proposed_date: &str,
    stakes: &str,
) -> Result<i32, sqlx::Error> {
    let mut likelihood = 70; // Base acceptance rate

    // 1. Check if clubs are in similar divisions (prestige matching)
    let sender_division = get_club_division_level(pool, sender_club_id).await.unwrap_or(5);
    let recipient_division = get_club_division_level(pool, recipient_club_id).await.unwrap_or(5);

    let division_gap = (sender_division as i32 - recipient_division as i32).abs();
    match division_gap {
        0 => likelihood += 20,      // Same division - very likely
        1 => likelihood += 10,      // Adjacent divisions - likely
        2 => likelihood -= 5,       // 2 divisions apart - slight penalty
        3 => likelihood -= 15,      // 3 divisions apart - less likely
        _ => likelihood -= 30,      // 4+ divisions apart - very unlikely
    }

    // 2. Geographic distance (check if same city/region)
    let same_region = check_same_region(pool, sender_club_id, recipient_club_id).await.unwrap_or(false);
    if same_region {
        likelihood += 15; // Local clubs more likely to play each other
    } else {
        likelihood -= 10; // Distant clubs less likely (travel costs)
    }

    // 3. Stakes consideration
    match stakes {
        "honor" => likelihood += 5,      // Low pressure, easy to accept
        "small" => likelihood += 0,      // Small wager, neutral
        "medium" => likelihood -= 5,     // More pressure, slight penalty
        "trophy" => likelihood += 10,    // Prestige opportunity
        "silver-cup" => likelihood += 15, // High prestige
        "dinner" => likelihood += 5,     // Social aspect
        "charity" => likelihood += 10,   // Good cause
        _ => {}
    }

    // 4. Check for existing fixtures on proposed date
    let has_fixture = check_fixture_conflict(pool, recipient_club_id, proposed_date).await.unwrap_or(false);
    if has_fixture {
        likelihood -= 80; // Very unlikely to accept if already have a match
    }

    // 5. Time of year consideration (check if in season or off season)
    let in_season = is_in_season(proposed_date);
    if !in_season {
        likelihood += 15; // More likely to accept friendlies in off-season
    }

    // 6. Day of week (weekends more favorable)
    let is_weekend = is_weekend_date(proposed_date);
    if is_weekend {
        likelihood += 10;
    }

    // Clamp between 5 and 95 (always some chance either way)
    Ok(likelihood.max(5).min(95))
}

/// Get the division level for a club (1 = top division, 10 = lowest)
async fn get_club_division_level(pool: &SqlitePool, club_id: &str) -> Result<i32, sqlx::Error> {
    let result: Option<(i32,)> = sqlx::query_as(
        r#"
        SELECT d.level
        FROM sheffield_league_clubs lc
        JOIN sheffield_league_divisions d ON lc.division_id = d.id
        WHERE lc.club_id = ?
        "#
    )
    .bind(club_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|(level,)| level).unwrap_or(5)) // Default to mid-table if not found
}

/// Check if two clubs are in the same region
async fn check_same_region(pool: &SqlitePool, club1_id: &str, club2_id: &str) -> Result<bool, sqlx::Error> {
    let regions: Vec<(Option<String>,)> = sqlx::query_as(
        "SELECT region FROM sheffield_clubs WHERE id IN (?, ?)"
    )
    .bind(club1_id)
    .bind(club2_id)
    .fetch_all(pool)
    .await?;

    if regions.len() == 2 {
        let region1 = regions[0].0.as_ref();
        let region2 = regions[1].0.as_ref();
        Ok(region1.is_some() && region2.is_some() && region1 == region2)
    } else {
        Ok(false)
    }
}

/// Check if club already has a fixture on the proposed date
async fn check_fixture_conflict(pool: &SqlitePool, club_id: &str, date: &str) -> Result<bool, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM sheffield_matches
        WHERE (home_club_id = ? OR away_club_id = ?)
        AND match_date = ?
        "#
    )
    .bind(club_id)
    .bind(club_id)
    .bind(date)
    .fetch_one(pool)
    .await?;

    Ok(count.0 > 0)
}

/// Check if date is during the football season (September to April)
fn is_in_season(date_str: &str) -> bool {
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let month = date.month();
        // Season runs September (9) through April (4)
        month >= 9 || month <= 4
    } else {
        true // Default to in-season if can't parse
    }
}

/// Check if the date falls on a weekend (Saturday or Sunday)
fn is_weekend_date(date_str: &str) -> bool {
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let weekday = date.weekday();
        matches!(weekday, chrono::Weekday::Sat | chrono::Weekday::Sun)
    } else {
        false
    }
}

/// Create a news item when a challenge is sent
async fn create_challenge_sent_news(
    pool: &SqlitePool,
    invitation_id: &str,
    sender_club_id: &str,
    recipient_club_id: &str,
    publish_date: &str,
    proposed_date: &str,
    venue: &str,
    stakes: &str,
    rules_type: &str,
    match_duration: i32,
) -> Result<String, sqlx::Error> {
    let news_id = uuid::Uuid::new_v4().to_string();

    // Get club names
    let sender_name: (String,) = sqlx::query_as(
        "SELECT name FROM sheffield_clubs WHERE id = ?"
    )
    .bind(sender_club_id)
    .fetch_one(pool)
    .await?;

    let recipient_name: (String,) = sqlx::query_as(
        "SELECT name FROM sheffield_clubs WHERE id = ?"
    )
    .bind(recipient_club_id)
    .fetch_one(pool)
    .await?;

    let venue_desc = match venue {
        "home" => "at our ground",
        "away" => &format!("at {}'s ground", recipient_name.0),
        _ => "at a neutral ground",
    };

    let stakes_desc = get_stakes_description(stakes);

    let headline = "Challenge Letter Dispatched";
    let body = format!(
        "A letter of challenge was sent by post this morning to the Secretary of {}, proposing a match to be played on {} {}.\n\nThe terms proposed include play under {} Rules for {} minutes, {}.\n\nOur Secretary has expressed confidence that the challenge shall be well-received, and we eagerly await their response, which should arrive within the coming days.",
        recipient_name.0,
        proposed_date,
        venue_desc,
        rules_type,
        match_duration,
        stakes_desc
    );

    sqlx::query(
        r#"
        INSERT INTO sheffield_news_items (
            id, headline, article_type, publish_date, body_text,
            is_read, is_important, requires_action,
            related_club_ids, related_invitation_id,
            has_action_button, action_button_text, action_type, action_data
        ) VALUES (?, ?, 'challenge_sent', ?, ?, 0, 0, 0, ?, ?, 0, NULL, NULL, NULL)
        "#
    )
    .bind(&news_id)
    .bind(headline)
    .bind(publish_date)
    .bind(&body)
    .bind(&format!("[\"{}\",\"{}\"]", sender_club_id, recipient_club_id))
    .bind(invitation_id)
    .execute(pool)
    .await?;

    Ok(news_id)
}

fn get_stakes_description(stakes: &str) -> String {
    match stakes {
        "honor" => "for the honour and glory of our respective clubs".to_string(),
        "small" => "with a modest wager of Five Pounds to add interest to the proceedings".to_string(),
        "medium" => "with a wager of Ten Pounds to heighten the competitive spirit".to_string(),
        "trophy" => "with a handsome trophy to be awarded to the victorious side".to_string(),
        "silver-cup" => "with a fine Silver Cup to be held by the victors for the coming year".to_string(),
        "dinner" => "with the losing side to provide a handsome dinner for the victors".to_string(),
        "charity" => "with all gate receipts to be donated to charitable causes in Sheffield".to_string(),
        _ => "for the honour and glory of our respective clubs".to_string(),
    }
}

/// Get an invitation by ID
pub async fn get_invitation_by_id(
    pool: &SqlitePool,
    invitation_id: &str,
) -> Result<ChallengeInvitation, sqlx::Error> {
    sqlx::query_as::<_, ChallengeInvitation>(
        r#"
        SELECT
            id, sender_club_id, recipient_club_id, sent_date, response_date,
            proposed_match_date, match_type, venue, stakes, tone, rules_type,
            match_duration, status, response_text, acceptance_likelihood,
            decline_reason, sent_news_id, response_news_id, scheduled_match_id
        FROM sheffield_challenge_invitations
        WHERE id = ?
        "#
    )
    .bind(invitation_id)
    .fetch_one(pool)
    .await
}

/// Get all news items for a specific date (to show in event feed)
pub async fn get_news_for_date(
    pool: &SqlitePool,
    date: &str,
) -> Result<Vec<NewsItem>, sqlx::Error> {
    sqlx::query_as::<_, NewsItem>(
        r#"
        SELECT
            id, headline, article_type, publish_date, body_text,
            is_read, is_important, requires_action,
            related_club_ids, related_player_ids, related_match_id,
            related_invitation_id, has_action_button,
            action_button_text, action_type, action_data
        FROM sheffield_news_items
        WHERE publish_date = ?
        ORDER BY id DESC
        "#
    )
    .bind(date)
    .fetch_all(pool)
    .await
}

/// Get the news the club has actually received — everything published on or before
/// `up_to` (the current game date). Tomorrow's post has not arrived yet: without
/// this bound, future-dated items leak into the feed the moment they are written.
pub async fn get_all_news(
    pool: &SqlitePool,
    up_to: &str,
) -> Result<Vec<NewsItem>, sqlx::Error> {
    sqlx::query_as::<_, NewsItem>(
        r#"
        SELECT
            id, headline, article_type, publish_date, body_text,
            is_read, is_important, requires_action,
            related_club_ids, related_player_ids, related_match_id,
            related_invitation_id, has_action_button,
            action_button_text, action_type, action_data
        FROM sheffield_news_items
        WHERE publish_date <= ?
        ORDER BY publish_date DESC, id DESC
        "#
    )
    .bind(up_to)
    .fetch_all(pool)
    .await
}

/// Check for pending invitation responses and process them
pub async fn check_pending_responses(
    pool: &SqlitePool,
    current_date: &str,
) -> Result<Vec<ChallengeInvitation>, sqlx::Error> {
    println!("[INVITATIONS] Checking for pending responses on date: {}", current_date);

    // Find invitations where response_date <= current_date and status is 'sent'
    let pending: Vec<ChallengeInvitation> = sqlx::query_as::<_, ChallengeInvitation>(
        r#"
        SELECT
            id, sender_club_id, recipient_club_id, sent_date, response_date,
            proposed_match_date, match_type, venue, stakes, tone, rules_type,
            match_duration, status, response_text, acceptance_likelihood,
            decline_reason, sent_news_id, response_news_id, scheduled_match_id
        FROM sheffield_challenge_invitations
        WHERE response_date <= ? AND status = 'sent'
        "#
    )
    .bind(current_date)
    .fetch_all(pool)
    .await?;

    println!("[INVITATIONS] Found {} pending invitations to process", pending.len());

    // Process each pending response
    for invitation in &pending {
        println!("[INVITATIONS] Processing invitation {} (sent: {}, response due: {})",
            invitation.id, invitation.sent_date, invitation.response_date);
        process_invitation_response(pool, &invitation, current_date).await?;
    }

    Ok(pending)
}

/// Process a single invitation response
async fn process_invitation_response(
    pool: &SqlitePool,
    invitation: &ChallengeInvitation,
    current_date: &str,
) -> Result<(), sqlx::Error> {
    // Determine if accepted or declined based on acceptance_likelihood.
    // Deterministic: seeded from the invitation id, so a replayed save gets the
    // same answer (and no i32::MIN.abs() overflow panic).
    let acceptance_likelihood = invitation.acceptance_likelihood.unwrap_or(50);
    let mut roll_rng = fsim_core::Rng::seed(crate::fsim_bridge::seed_from(&invitation.id));
    let random_value = roll_rng.roll(100) as i32;
    let accepted = random_value < acceptance_likelihood;

    // Fetch recipient club name
    let recipient_name: (String,) = sqlx::query_as(
        "SELECT name FROM sheffield_clubs WHERE id = ?"
    )
    .bind(&invitation.recipient_club_id)
    .fetch_one(pool)
    .await?;

    // Determine decline reason if declined
    let decline_reason = if !accepted {
        determine_decline_reason(pool, &invitation).await?
    } else {
        None
    };

    let new_status = if accepted { "accepted" } else { "declined" };
    let response_text = generate_response_text(invitation, accepted, decline_reason.as_deref(), &recipient_name.0);

    // Update invitation status
    sqlx::query(
        "UPDATE sheffield_challenge_invitations SET status = ?, response_text = ?, decline_reason = ? WHERE id = ?"
    )
    .bind(new_status)
    .bind(&response_text)
    .bind(&decline_reason)
    .bind(&invitation.id)
    .execute(pool)
    .await?;

    // Create news item about receiving the response
    create_response_received_news(pool, invitation, current_date, accepted, &response_text).await?;

    // If accepted, schedule the match
    if accepted {
        schedule_friendly_match(pool, invitation).await?;
    }

    Ok(())
}

/// Determine the reason for declining based on various factors
async fn determine_decline_reason(
    pool: &SqlitePool,
    invitation: &ChallengeInvitation,
) -> Result<Option<String>, sqlx::Error> {
    // Check for fixture conflict first
    let has_conflict = check_fixture_conflict(pool, &invitation.recipient_club_id, &invitation.proposed_match_date).await?;
    if has_conflict {
        return Ok(Some("schedule_conflict".to_string()));
    }

    // Check prestige mismatch
    let sender_level = get_club_division_level(pool, &invitation.sender_club_id).await.unwrap_or(5);
    let recipient_level = get_club_division_level(pool, &invitation.recipient_club_id).await.unwrap_or(5);
    let division_gap = (sender_level as i32 - recipient_level as i32).abs();

    if division_gap >= 3 {
        return Ok(Some("prestige_mismatch".to_string()));
    }

    // Check if too soon (less than a week from current date)
    let sent_date = chrono::NaiveDate::parse_from_str(&invitation.sent_date, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let proposed_date = chrono::NaiveDate::parse_from_str(&invitation.proposed_match_date, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let days_notice = (proposed_date - sent_date).num_days();

    if days_notice < 7 {
        return Ok(Some("too_soon".to_string()));
    }

    // Default generic decline
    Ok(Some("general".to_string()))
}

/// Generate Victorian-era response text
fn generate_response_text(invitation: &ChallengeInvitation, accepted: bool, decline_reason: Option<&str>, recipient_club_name: &str) -> String {
    if accepted {
        format!(
            "Dear Sir,\n\nThe Secretary of {} acknowledges with great pleasure your kind letter of the {}.\n\nWe are delighted to accept your proposal for a match of football between our respective clubs. The date of {} suits our fixtures admirably, and we look forward with keen anticipation to what promises to be a most sporting contest.\n\nYour terms regarding the venue and stakes are most agreeable to us, and we shall be honoured to meet your gentlemen on the field of play under {} Rules.\n\nWe remain confident that the match shall be contested in the finest spirit of sportsmanship.\n\nYour obedient servant,\nSecretary",
            recipient_club_name,
            invitation.sent_date,
            invitation.proposed_match_date,
            invitation.rules_type
        )
    } else {
        let reason_text = match decline_reason {
            Some("schedule_conflict") => {
                format!(
                    "It is with sincere regret that we must inform you that we have already engaged to meet another club on that date, and our Committee feels it would be improper to break such an engagement.\n\nMight we suggest an alternative date at your convenience? We would be most happy to arrange a match at a mutually suitable time."
                )
            },
            Some("prestige_mismatch") => {
                format!(
                    "While we are most grateful for your invitation, our Committee feels that at present our club's commitments to matches with clubs of similar standing must take precedence in our fixture list.\n\nWe wish your club every success in the coming season and trust you shall find suitable opponents for your gentlemen to test their mettle against."
                )
            },
            Some("too_soon") => {
                format!(
                    "While we are most grateful for your invitation, we must respectfully decline as several of our principal players are presently indisposed, and we fear we could not field our strongest eleven at such short notice.\n\nWe would be honoured to arrange a match at a later date, when we might better hope to provide your gentlemen with worthy opposition."
                )
            },
            _ => {
                format!(
                    "It is with sincere regret that we must inform you that we are unable to accept your sporting challenge at this time due to existing commitments.\n\nWe remain, however, admirers of your club and hope that a future opportunity for a match between our respective elevens may arise."
                )
            }
        };

        format!(
            "Dear Sir,\n\nWe acknowledge with thanks your kind invitation to a match on {}.\n\n{}\n\nWe remain, Sir,\nYour obedient servants",
            invitation.proposed_match_date,
            reason_text
        )
    }
}

/// Create news item when a response is received
async fn create_response_received_news(
    pool: &SqlitePool,
    invitation: &ChallengeInvitation,
    publish_date: &str,
    accepted: bool,
    response_text: &str,
) -> Result<String, sqlx::Error> {
    let news_id = uuid::Uuid::new_v4().to_string();

    println!("[INVITATIONS] Creating response news (accepted: {}) for invitation {}", accepted, invitation.id);

    let recipient_name: (String,) = sqlx::query_as(
        "SELECT name FROM sheffield_clubs WHERE id = ?"
    )
    .bind(&invitation.recipient_club_id)
    .fetch_one(pool)
    .await?;

    let headline = format!("Letter Received from {}", recipient_name.0);

    let body = if accepted {
        format!(
            "The secretary has received a letter from {}.\n\nClick 'Read the Letter' below to view the full correspondence.",
            recipient_name.0
        )
    } else {
        format!(
            "The secretary has received a letter from {}.\n\nClick 'Read the Letter' below to view the full correspondence.",
            recipient_name.0
        )
    };

    // Create action button to read the full letter
    let action_data = serde_json::json!({
        "invitation_id": invitation.id,
        "response_text": response_text
    }).to_string();

    sqlx::query(
        r#"
        INSERT INTO sheffield_news_items (
            id, headline, article_type, publish_date, body_text,
            is_read, is_important, requires_action,
            related_club_ids, related_invitation_id,
            has_action_button, action_button_text, action_type, action_data
        ) VALUES (?, ?, ?, ?, ?, 0, 1, 0, ?, ?, 1, 'Read the Letter', 'read_letter', ?)
        "#
    )
    .bind(&news_id)
    .bind(&headline)
    .bind(if accepted { "challenge_accepted" } else { "challenge_declined" })
    .bind(publish_date)
    .bind(&body)
    .bind(&format!("[\"{}\",\"{}\"]", invitation.sender_club_id, invitation.recipient_club_id))
    .bind(&invitation.id)
    .bind(&action_data)
    .execute(pool)
    .await?;

    // Update invitation with response news ID
    sqlx::query(
        "UPDATE sheffield_challenge_invitations SET response_news_id = ? WHERE id = ?"
    )
    .bind(&news_id)
    .bind(&invitation.id)
    .execute(pool)
    .await?;

    println!("[INVITATIONS] Created response news with ID: {}, headline: {}", news_id, headline);

    Ok(news_id)
}

/// Schedule a friendly match when invitation is accepted
async fn schedule_friendly_match(
    pool: &SqlitePool,
    invitation: &ChallengeInvitation,
) -> Result<(), sqlx::Error> {
    // Generate unique match ID
    let match_id = uuid::Uuid::new_v4().to_string();

    // Determine home and away based on venue
    let (home_club_id, away_club_id) = match invitation.venue.as_str() {
        "home" => (invitation.sender_club_id.clone(), invitation.recipient_club_id.clone()),
        "away" => (invitation.recipient_club_id.clone(), invitation.sender_club_id.clone()),
        "neutral" => {
            // For neutral venue, sender is still considered "home" for record purposes
            (invitation.sender_club_id.clone(), invitation.recipient_club_id.clone())
        },
        _ => (invitation.sender_club_id.clone(), invitation.recipient_club_id.clone()),
    };

    // Extract year from proposed date to determine rule year
    let rule_year = if let Ok(date) = chrono::NaiveDate::parse_from_str(&invitation.proposed_match_date, "%Y-%m-%d") {
        date.year()
    } else {
        1867 // Default to 1867 if parsing fails
    };

    // Insert match into sheffield_matches table
    sqlx::query(
        r#"
        INSERT INTO sheffield_matches (
            id, gameweek, season, rule_year, home_club_id, away_club_id,
            home_score, away_score, home_rouges, away_rouges,
            played, match_date, attendance, weather, pitch_condition
        ) VALUES (?, 0, ?, ?, ?, ?, 0, 0, 0, 0, 0, ?, NULL, NULL, NULL)
        "#
    )
    .bind(&match_id)
    .bind(rule_year)
    .bind(rule_year)
    .bind(&home_club_id)
    .bind(&away_club_id)
    .bind(&invitation.proposed_match_date)
    .execute(pool)
    .await?;

    // Update invitation with scheduled match ID
    sqlx::query(
        "UPDATE sheffield_challenge_invitations SET scheduled_match_id = ? WHERE id = ?"
    )
    .bind(&match_id)
    .bind(&invitation.id)
    .execute(pool)
    .await?;

    // DON'T create match confirmed news automatically - it will be created when user reads the acceptance letter
    // create_match_scheduled_news(pool, invitation, &match_id).await?;

    Ok(())
}

/// Create news item when a match is scheduled
async fn create_match_scheduled_news(
    pool: &SqlitePool,
    invitation: &ChallengeInvitation,
    match_id: &str,
) -> Result<(), sqlx::Error> {
    let news_id = uuid::Uuid::new_v4().to_string();

    // Get club names
    let sender_name: (String,) = sqlx::query_as(
        "SELECT name FROM sheffield_clubs WHERE id = ?"
    )
    .bind(&invitation.sender_club_id)
    .fetch_one(pool)
    .await?;

    let recipient_name: (String,) = sqlx::query_as(
        "SELECT name FROM sheffield_clubs WHERE id = ?"
    )
    .bind(&invitation.recipient_club_id)
    .fetch_one(pool)
    .await?;

    let venue_desc = match invitation.venue.as_str() {
        "home" => format!("at {}'s ground", sender_name.0),
        "away" => format!("at {}'s ground", recipient_name.0),
        "neutral" => "at a neutral ground".to_string(),
        _ => "venue to be determined".to_string(),
    };

    let headline = "MATCH CONFIRMED";
    let body = format!(
        "The match between {} and {} has been officially confirmed for {}.\n\nThe match will be played {} under {} Rules for {} minutes.\n\nBoth clubs have begun preparations, and spectators are anticipated to attend in good numbers.",
        sender_name.0,
        recipient_name.0,
        invitation.proposed_match_date,
        venue_desc,
        invitation.rules_type,
        invitation.match_duration
    );

    sqlx::query(
        r#"
        INSERT INTO sheffield_news_items (
            id, headline, article_type, publish_date, body_text,
            is_read, is_important, requires_action,
            related_club_ids, related_match_id, related_invitation_id,
            has_action_button, action_button_text, action_type, action_data
        ) VALUES (?, ?, 'match_scheduled', ?, ?, 0, 0, 0, ?, ?, ?, 0, NULL, NULL, NULL)
        "#
    )
    .bind(&news_id)
    .bind(headline)
    .bind(&invitation.proposed_match_date)
    .bind(&body)
    .bind(&format!("[\"{}\",\"{}\"]", invitation.sender_club_id, invitation.recipient_club_id))
    .bind(match_id)
    .bind(&invitation.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Public function to create match confirmed news for an accepted invitation
/// Called from the frontend when user reads the acceptance letter
pub async fn create_match_confirmed_news_for_invitation(
    pool: &SqlitePool,
    invitation_id: &str,
) -> Result<(), sqlx::Error> {
    // Fetch the invitation
    let invitation: ChallengeInvitation = sqlx::query_as(
        "SELECT * FROM sheffield_challenge_invitations WHERE id = ?"
    )
    .bind(invitation_id)
    .fetch_one(pool)
    .await?;

    // Only create news if the invitation was accepted
    if invitation.status != "accepted" {
        return Ok(());
    }

    // Get the match ID from the invitation (clone to avoid partial move)
    let match_id = invitation.scheduled_match_id.clone().ok_or_else(|| {
        sqlx::Error::RowNotFound
    })?;

    // Create the match confirmed news
    create_match_scheduled_news(pool, &invitation, &match_id).await?;

    Ok(())
}

/// Load upcoming friendly matches from the database
/// Returns all unplayed friendly matches (gameweek = 0) that should be visible in the game state
pub async fn load_friendly_matches(
    pool: &SqlitePool,
) -> Result<Vec<crate::game::Match>, sqlx::Error> {
    use sqlx::Row;

    let rows = sqlx::query(
        r#"
        SELECT id, gameweek, home_club_id, away_club_id, home_score, away_score, match_date, played
        FROM sheffield_matches
        WHERE gameweek = 0 AND played = 0
        ORDER BY match_date ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    let matches = rows.iter().map(|row| {
        crate::game::Match {
            id: row.get("id"),
            gameweek: row.get::<i32, _>("gameweek") as u8,
            home_team_id: row.get("home_club_id"),
            away_team_id: row.get("away_club_id"),
            home_score: if row.get::<i32, _>("played") == 1 {
                Some(row.get::<i32, _>("home_score") as u8)
            } else {
                None
            },
            away_score: if row.get::<i32, _>("played") == 1 {
                Some(row.get::<i32, _>("away_score") as u8)
            } else {
                None
            },
            date: row.get("match_date"),
            played: row.get::<i32, _>("played") == 1,
        }
    }).collect();

    Ok(matches)
}
