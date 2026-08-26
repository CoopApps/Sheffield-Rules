use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInput {
    pub name: String,
    pub position: Option<String>, // If None, will be randomized
    pub birth_year: Option<i32>,  // If None, will be randomized (age 18-30)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPlayer {
    pub id: String,
    pub name: String,
    pub club_id: String,
    pub position: String,
    pub birth_year: i32,
    pub nationality: String,
    pub height_cm: i32,
    // Physical
    pub pace: i32,
    pub strength: i32,
    pub stamina: i32,
    pub balance: i32,
    pub jumping: i32,
    pub agility: i32,
    // Technical
    pub passing: i32,
    pub dribbling: i32,
    pub first_touch: i32,
    pub heading: i32,
    pub long_passing: i32,
    pub crossing: i32,
    pub long_shots: i32,
    pub tackling: i32,
    pub handling: i32,
    pub reflexes: i32,
    // Mental
    pub courage: i32,
    pub concentration: i32,
    pub decision_making: i32,
    pub leadership: i32,
    pub aggression: i32,
    pub anticipation: i32,
    pub determination: i32,
    pub flair: i32,
    pub influence: i32,
    // Positioning
    pub awareness: i32,
    pub marking: i32,
    pub positioning: i32,
    pub work_rate: i32,
    pub off_the_ball: i32,
    pub teamwork: i32,
    // Specialization
    pub finishing: i32,
    pub penalties: i32,
    pub set_pieces: i32,
    // Hidden
    pub consistency: i32,
    pub dirtiness: i32,
    pub versatility: i32,
    // Calculated
    pub overall_rating: i32,
    // Status
    pub morale: i32,
    pub form: i32,
    pub fitness: i32,
    pub is_real_player: bool,
}

/// Generate random stats for a player based on their position and era
pub fn generate_player(
    input: &PlayerInput,
    club_id: &str,
    year: i32,
    has_goalkeeper: bool,
) -> GeneratedPlayer {
    let mut rng = rand::thread_rng();

    // Determine position (if not specified, random based on era)
    let position = if let Some(pos) = &input.position {
        pos.clone()
    } else {
        determine_random_position(year, has_goalkeeper, &mut rng)
    };

    // Birth year: use provided value or randomize age 18-30 in the current year
    let birth_year = input.birth_year.unwrap_or_else(|| year - rng.gen_range(18..=30));

    // Height in cm (different by position and era - Victorian players were shorter)
    let height_cm = generate_height(&position, year, &mut rng);

    // Generate base stats with position-specific weights
    let stats = generate_position_stats(&position, year, &mut rng);

    GeneratedPlayer {
        id: Uuid::new_v4().to_string(),
        name: input.name.clone(),
        club_id: club_id.to_string(),
        position: position.clone(),
        birth_year,
        nationality: "English".to_string(), // Sheffield clubs were predominantly English
        height_cm,
        // Physical
        pace: stats.pace,
        strength: stats.strength,
        stamina: stats.stamina,
        balance: stats.balance,
        jumping: stats.jumping,
        agility: stats.agility,
        // Technical
        passing: stats.passing,
        dribbling: stats.dribbling,
        first_touch: stats.first_touch,
        heading: stats.heading,
        long_passing: stats.long_passing,
        crossing: stats.crossing,
        long_shots: stats.long_shots,
        tackling: stats.tackling,
        handling: stats.handling,
        reflexes: stats.reflexes,
        // Mental
        courage: stats.courage,
        concentration: stats.concentration,
        decision_making: stats.decision_making,
        leadership: stats.leadership,
        aggression: stats.aggression,
        anticipation: stats.anticipation,
        determination: stats.determination,
        flair: stats.flair,
        influence: stats.influence,
        // Positioning
        awareness: stats.awareness,
        marking: stats.marking,
        positioning: stats.positioning,
        work_rate: stats.work_rate,
        off_the_ball: stats.off_the_ball,
        teamwork: stats.teamwork,
        // Specialization
        finishing: stats.finishing,
        penalties: stats.penalties,
        set_pieces: stats.set_pieces,
        // Hidden
        consistency: rng.gen_range(5..=15),
        dirtiness: rng.gen_range(1..=10),
        versatility: rng.gen_range(5..=15),
        // Calculated
        overall_rating: calculate_overall_rating(&stats, &position),
        // Status
        morale: 10,
        form: 0,
        fitness: 100,
        is_real_player: false,
    }
}

fn determine_random_position(year: i32, has_goalkeeper: bool, rng: &mut impl Rng) -> String {
    // Pre-1870: No specific positions beyond goalkeeper (if enabled)
    // 1870+: Goalkeeper, cover goals (CB), half backs (FB), midfielders, forward
    // 1875+: Goalkeeper, 2 cover goals, 2 half backs, 5 midfielders, 1 forward (2-2-5-1)

    if year >= 1875 {
        // 2-2-5-1 formation era
        let positions = vec![
            ("GK", if has_goalkeeper { 5 } else { 0 }),
            ("CB", 15),  // Cover goals
            ("FB", 15),  // Half backs
            ("MID", 50), // Midfielders
            ("FWD", 15), // Forward
        ];
        weighted_random_position(&positions, rng)
    } else if year >= 1870 {
        // Early positional play
        let positions = vec![
            ("GK", if has_goalkeeper { 10 } else { 0 }),
            ("CB", 20),
            ("MID", 50),
            ("FWD", 20),
        ];
        weighted_random_position(&positions, rng)
    } else {
        // Pre-1870: mostly just outfield players
        if has_goalkeeper && rng.gen_bool(0.1) {
            "GK".to_string()
        } else {
            // Everyone else was essentially a midfielder/forward
            if rng.gen_bool(0.5) {
                "MID".to_string()
            } else {
                "FWD".to_string()
            }
        }
    }
}

fn weighted_random_position(positions: &[(&str, u32)], rng: &mut impl Rng) -> String {
    let total_weight: u32 = positions.iter().map(|(_, w)| w).sum();
    if total_weight == 0 {
        return "MID".to_string(); // Fallback
    }

    let mut roll = rng.gen_range(0..total_weight);
    for (pos, weight) in positions {
        if roll < *weight {
            return pos.to_string();
        }
        roll -= weight;
    }
    "MID".to_string() // Fallback
}

fn generate_height(position: &str, year: i32, rng: &mut impl Rng) -> i32 {
    // Victorian era players were shorter on average
    // Average height increased over time
    let base_height = if year < 1870 {
        170 // ~5'7" average
    } else {
        172 // ~5'8" average
    };

    match position {
        "GK" => base_height + rng.gen_range(-5..=10), // Slightly taller
        "CB" => base_height + rng.gen_range(-3..=8),  // Taller for heading
        "FB" => base_height + rng.gen_range(-5..=5),  // Average
        "MID" => base_height + rng.gen_range(-8..=5), // Slightly shorter, more agile
        "FWD" => base_height + rng.gen_range(-5..=8), // Varied
        "WG" => base_height + rng.gen_range(-10..=3), // Shorter, quicker
        _ => base_height,
    }
}

struct PlayerStats {
    // Physical
    pace: i32,
    strength: i32,
    stamina: i32,
    balance: i32,
    jumping: i32,
    agility: i32,
    // Technical
    passing: i32,
    dribbling: i32,
    first_touch: i32,
    heading: i32,
    long_passing: i32,
    crossing: i32,
    long_shots: i32,
    tackling: i32,
    handling: i32,
    reflexes: i32,
    // Mental
    courage: i32,
    concentration: i32,
    decision_making: i32,
    leadership: i32,
    aggression: i32,
    anticipation: i32,
    determination: i32,
    flair: i32,
    influence: i32,
    // Positioning
    awareness: i32,
    marking: i32,
    positioning: i32,
    work_rate: i32,
    off_the_ball: i32,
    teamwork: i32,
    // Specialization
    finishing: i32,
    penalties: i32,
    set_pieces: i32,
}

// Helper functions for stat generation
fn gen_low(rng: &mut impl Rng) -> i32 {
    rng.gen_range(3..=8)
}

fn gen_avg(rng: &mut impl Rng, bonus: i32) -> i32 {
    rng.gen_range(6..=14) + bonus
}

fn gen_high(rng: &mut impl Rng) -> i32 {
    rng.gen_range(12..=18)
}

fn generate_position_stats(position: &str, _year: i32, rng: &mut impl Rng) -> PlayerStats {
    // Base stats: 1-20 scale, average player is 8-12

    match position {
        "GK" => PlayerStats {
            // Physical - goalkeepers don't need pace but need strength and jumping
            pace: gen_low(rng),
            strength: gen_avg(rng, 2),
            stamina: gen_avg(rng, 0),
            balance: gen_avg(rng, 1),
            jumping: gen_high(rng),
            agility: gen_avg(rng, 2),
            // Technical - focus on handling and reflexes
            passing: gen_avg(rng, -2),
            dribbling: gen_low(rng),
            first_touch: gen_avg(rng, -1),
            heading: gen_avg(rng, 0),
            long_passing: gen_avg(rng, -1),
            crossing: gen_low(rng),
            long_shots: gen_low(rng),
            tackling: gen_low(rng),
            handling: gen_high(rng),
            reflexes: gen_high(rng),
            // Mental
            courage: gen_high(rng),
            concentration: gen_high(rng),
            decision_making: gen_avg(rng, 2),
            leadership: gen_avg(rng, 1),
            aggression: gen_avg(rng, -1),
            anticipation: gen_high(rng),
            determination: gen_avg(rng, 1),
            flair: gen_low(rng),
            influence: gen_avg(rng, 0),
            // Positioning
            awareness: gen_avg(rng, 2),
            marking: gen_avg(rng, -2),
            positioning: gen_high(rng),
            work_rate: gen_avg(rng, 0),
            off_the_ball: gen_low(rng),
            teamwork: gen_avg(rng, 0),
            // Specialization
            finishing: gen_low(rng),
            penalties: gen_avg(rng, 2),
            set_pieces: gen_low(rng),
        },
        "CB" => PlayerStats {
            // Cover goals (centre backs) - physical, defensive
            pace: gen_avg(rng, -1),
            strength: gen_high(rng),
            stamina: gen_avg(rng, 1),
            balance: gen_avg(rng, 0),
            jumping: gen_high(rng),
            agility: gen_avg(rng, -1),
            // Technical
            passing: gen_avg(rng, -1),
            dribbling: gen_avg(rng, -2),
            first_touch: gen_avg(rng, -1),
            heading: gen_high(rng),
            long_passing: gen_avg(rng, 0),
            crossing: gen_low(rng),
            long_shots: gen_avg(rng, -2),
            tackling: gen_high(rng),
            handling: gen_low(rng),
            reflexes: gen_low(rng),
            // Mental
            courage: gen_high(rng),
            concentration: gen_avg(rng, 2),
            decision_making: gen_avg(rng, 1),
            leadership: gen_avg(rng, 2),
            aggression: gen_avg(rng, 2),
            anticipation: gen_avg(rng, 2),
            determination: gen_avg(rng, 2),
            flair: gen_low(rng),
            influence: gen_avg(rng, 1),
            // Positioning
            awareness: gen_avg(rng, 2),
            marking: gen_high(rng),
            positioning: gen_high(rng),
            work_rate: gen_avg(rng, 1),
            off_the_ball: gen_avg(rng, -2),
            teamwork: gen_avg(rng, 1),
            // Specialization
            finishing: gen_low(rng),
            penalties: gen_avg(rng, -1),
            set_pieces: gen_avg(rng, 1),
        },
        "FB" => PlayerStats {
            // Half backs (fullbacks) - balanced defensive players
            pace: gen_avg(rng, 1),
            strength: gen_avg(rng, 1),
            stamina: gen_high(rng),
            balance: gen_avg(rng, 1),
            jumping: gen_avg(rng, 1),
            agility: gen_avg(rng, 1),
            // Technical
            passing: gen_avg(rng, 0),
            dribbling: gen_avg(rng, 0),
            first_touch: gen_avg(rng, 0),
            heading: gen_avg(rng, 1),
            long_passing: gen_avg(rng, 1),
            crossing: gen_avg(rng, 1),
            long_shots: gen_avg(rng, -1),
            tackling: gen_avg(rng, 2),
            handling: gen_low(rng),
            reflexes: gen_low(rng),
            // Mental
            courage: gen_avg(rng, 1),
            concentration: gen_avg(rng, 1),
            decision_making: gen_avg(rng, 1),
            leadership: gen_avg(rng, 0),
            aggression: gen_avg(rng, 1),
            anticipation: gen_avg(rng, 1),
            determination: gen_avg(rng, 1),
            flair: gen_avg(rng, -1),
            influence: gen_avg(rng, 0),
            // Positioning
            awareness: gen_avg(rng, 1),
            marking: gen_avg(rng, 2),
            positioning: gen_avg(rng, 2),
            work_rate: gen_high(rng),
            off_the_ball: gen_avg(rng, -1),
            teamwork: gen_avg(rng, 2),
            // Specialization
            finishing: gen_avg(rng, -2),
            penalties: gen_avg(rng, -1),
            set_pieces: gen_avg(rng, 0),
        },
        "MID" => PlayerStats {
            // Midfielders - balanced, good technical skills
            pace: gen_avg(rng, 0),
            strength: gen_avg(rng, 0),
            stamina: gen_high(rng),
            balance: gen_avg(rng, 1),
            jumping: gen_avg(rng, 0),
            agility: gen_avg(rng, 1),
            // Technical
            passing: gen_high(rng),
            dribbling: gen_avg(rng, 2),
            first_touch: gen_avg(rng, 2),
            heading: gen_avg(rng, 0),
            long_passing: gen_avg(rng, 2),
            crossing: gen_avg(rng, 1),
            long_shots: gen_avg(rng, 1),
            tackling: gen_avg(rng, 0),
            handling: gen_low(rng),
            reflexes: gen_low(rng),
            // Mental
            courage: gen_avg(rng, 0),
            concentration: gen_avg(rng, 1),
            decision_making: gen_avg(rng, 2),
            leadership: gen_avg(rng, 0),
            aggression: gen_avg(rng, 0),
            anticipation: gen_avg(rng, 1),
            determination: gen_avg(rng, 1),
            flair: gen_avg(rng, 1),
            influence: gen_avg(rng, 1),
            // Positioning
            awareness: gen_avg(rng, 2),
            marking: gen_avg(rng, 0),
            positioning: gen_avg(rng, 1),
            work_rate: gen_avg(rng, 2),
            off_the_ball: gen_avg(rng, 1),
            teamwork: gen_high(rng),
            // Specialization
            finishing: gen_avg(rng, 0),
            penalties: gen_avg(rng, 0),
            set_pieces: gen_avg(rng, 1),
        },
        "FWD" => PlayerStats {
            // Forwards - pace, finishing, attacking
            pace: gen_high(rng),
            strength: gen_avg(rng, 0),
            stamina: gen_avg(rng, 1),
            balance: gen_avg(rng, 1),
            jumping: gen_avg(rng, 1),
            agility: gen_avg(rng, 2),
            // Technical
            passing: gen_avg(rng, 0),
            dribbling: gen_avg(rng, 2),
            first_touch: gen_avg(rng, 2),
            heading: gen_avg(rng, 1),
            long_passing: gen_avg(rng, -1),
            crossing: gen_avg(rng, 0),
            long_shots: gen_avg(rng, 2),
            tackling: gen_avg(rng, -2),
            handling: gen_low(rng),
            reflexes: gen_low(rng),
            // Mental
            courage: gen_avg(rng, 1),
            concentration: gen_avg(rng, 0),
            decision_making: gen_avg(rng, 1),
            leadership: gen_avg(rng, -1),
            aggression: gen_avg(rng, 0),
            anticipation: gen_avg(rng, 2),
            determination: gen_avg(rng, 1),
            flair: gen_high(rng),
            influence: gen_avg(rng, 0),
            // Positioning
            awareness: gen_avg(rng, 1),
            marking: gen_avg(rng, -2),
            positioning: gen_avg(rng, 2),
            work_rate: gen_avg(rng, 0),
            off_the_ball: gen_high(rng),
            teamwork: gen_avg(rng, 0),
            // Specialization
            finishing: gen_high(rng),
            penalties: gen_avg(rng, 2),
            set_pieces: gen_avg(rng, 1),
        },
        "WG" => PlayerStats {
            // Wingers (sides) - pace, crossing, dribbling
            pace: gen_high(rng),
            strength: gen_avg(rng, -1),
            stamina: gen_high(rng),
            balance: gen_avg(rng, 2),
            jumping: gen_avg(rng, -1),
            agility: gen_high(rng),
            // Technical
            passing: gen_avg(rng, 1),
            dribbling: gen_high(rng),
            first_touch: gen_avg(rng, 2),
            heading: gen_avg(rng, -1),
            long_passing: gen_avg(rng, 0),
            crossing: gen_high(rng),
            long_shots: gen_avg(rng, 0),
            tackling: gen_avg(rng, -2),
            handling: gen_low(rng),
            reflexes: gen_low(rng),
            // Mental
            courage: gen_avg(rng, 0),
            concentration: gen_avg(rng, 0),
            decision_making: gen_avg(rng, 0),
            leadership: gen_avg(rng, -2),
            aggression: gen_avg(rng, -1),
            anticipation: gen_avg(rng, 1),
            determination: gen_avg(rng, 0),
            flair: gen_high(rng),
            influence: gen_avg(rng, -1),
            // Positioning
            awareness: gen_avg(rng, 0),
            marking: gen_avg(rng, -2),
            positioning: gen_avg(rng, 1),
            work_rate: gen_avg(rng, 1),
            off_the_ball: gen_avg(rng, 2),
            teamwork: gen_avg(rng, 0),
            // Specialization
            finishing: gen_avg(rng, 1),
            penalties: gen_avg(rng, -1),
            set_pieces: gen_high(rng),
        },
        _ => {
            // Default: balanced player
            PlayerStats {
                pace: gen_avg(rng, 0),
                strength: gen_avg(rng, 0),
                stamina: gen_avg(rng, 0),
                balance: gen_avg(rng, 0),
                jumping: gen_avg(rng, 0),
                agility: gen_avg(rng, 0),
                passing: gen_avg(rng, 0),
                dribbling: gen_avg(rng, 0),
                first_touch: gen_avg(rng, 0),
                heading: gen_avg(rng, 0),
                long_passing: gen_avg(rng, 0),
                crossing: gen_avg(rng, 0),
                long_shots: gen_avg(rng, 0),
                tackling: gen_avg(rng, 0),
                handling: gen_low(rng),
                reflexes: gen_low(rng),
                courage: gen_avg(rng, 0),
                concentration: gen_avg(rng, 0),
                decision_making: gen_avg(rng, 0),
                leadership: gen_avg(rng, 0),
                aggression: gen_avg(rng, 0),
                anticipation: gen_avg(rng, 0),
                determination: gen_avg(rng, 0),
                flair: gen_avg(rng, 0),
                influence: gen_avg(rng, 0),
                awareness: gen_avg(rng, 0),
                marking: gen_avg(rng, 0),
                positioning: gen_avg(rng, 0),
                work_rate: gen_avg(rng, 0),
                off_the_ball: gen_avg(rng, 0),
                teamwork: gen_avg(rng, 0),
                finishing: gen_avg(rng, 0),
                penalties: gen_avg(rng, 0),
                set_pieces: gen_avg(rng, 0),
            }
        }
    }
}

fn calculate_overall_rating(stats: &PlayerStats, position: &str) -> i32 {
    // Weight different stats based on position
    let total = match position {
        "GK" => {
            stats.handling * 3 + stats.reflexes * 3 + stats.positioning * 2 +
            stats.concentration * 2 + stats.anticipation * 2 + stats.agility * 1 +
            stats.jumping * 1 + stats.courage * 1
        },
        "CB" => {
            stats.tackling * 3 + stats.marking * 3 + stats.heading * 2 +
            stats.positioning * 2 + stats.strength * 2 + stats.courage * 2 +
            stats.anticipation * 1 + stats.concentration * 1
        },
        "FB" => {
            stats.tackling * 2 + stats.marking * 2 + stats.stamina * 2 +
            stats.pace * 2 + stats.positioning * 2 + stats.work_rate * 2 +
            stats.crossing * 1 + stats.passing * 1
        },
        "MID" => {
            stats.passing * 3 + stats.first_touch * 2 + stats.decision_making * 2 +
            stats.stamina * 2 + stats.work_rate * 2 + stats.teamwork * 2 +
            stats.dribbling * 1 + stats.awareness * 1
        },
        "FWD" => {
            stats.finishing * 3 + stats.pace * 2 + stats.dribbling * 2 +
            stats.off_the_ball * 2 + stats.anticipation * 2 + stats.first_touch * 2 +
            stats.agility * 1 + stats.flair * 1
        },
        "WG" => {
            stats.pace * 3 + stats.dribbling * 3 + stats.crossing * 2 +
            stats.agility * 2 + stats.stamina * 2 + stats.first_touch * 1 +
            stats.flair * 1 + stats.balance * 1
        },
        _ => {
            // Generic calculation
            (stats.pace + stats.strength + stats.stamina + stats.passing +
             stats.dribbling + stats.tackling + stats.finishing) / 7
        }
    };

    // Normalize to 1-20 scale
    let weights_sum = match position {
        "GK" | "CB" | "FB" | "MID" | "FWD" | "WG" => 15,
        _ => 7,
    };

    (total / weights_sum).clamp(1, 20)
}
