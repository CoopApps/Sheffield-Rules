use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GameState {
    pub id: String,
    pub season: u16,
    #[serde(rename = "currentGameweek")]
    pub current_gameweek: u8,
    #[serde(rename = "currentDate")]
    pub current_date: String,
    #[serde(rename = "userClubId")]
    pub user_club_id: String,
    #[serde(rename = "gameMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    pub clubs: Vec<Club>,
    pub matches: Vec<Match>,
    pub players: Vec<Player>,
    pub standings: Vec<Standing>,
    #[serde(rename = "pendingEvents")]
    pub pending_events: Vec<GameEvent>,
    #[serde(rename = "processedEvents")]
    pub processed_events: Vec<GameEvent>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "loadedFromFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded_from_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub id: String,
    pub name: String,
    pub points: u16,
    pub played: u8,
    pub won: u8,
    pub drawn: u8,
    pub lost: u8,
    #[serde(rename = "goalsFor")]
    pub goals_for: u16,
    #[serde(rename = "goalsAgainst")]
    pub goals_against: u16,
    pub budget: u32,
    pub reputation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: String,
    pub gameweek: u8,
    #[serde(rename = "homeTeamId")]
    pub home_team_id: String,
    #[serde(rename = "awayTeamId")]
    pub away_team_id: String,
    #[serde(rename = "homeScore")]
    pub home_score: Option<u8>,
    #[serde(rename = "awayScore")]
    pub away_score: Option<u8>,
    pub date: String,
    pub played: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    #[serde(rename = "clubId")]
    pub club_id: String,
    pub position: String,
    pub age: u8,
    #[serde(rename = "overallRating")]
    pub overall_rating: f32,
    pub form: f32,
    pub fitness: f32,
    pub morale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standing {
    #[serde(rename = "clubId")]
    pub club_id: String,
    #[serde(rename = "clubName")]
    pub club_name: String,
    pub position: u16,
    pub points: u16,
    pub played: u8,
    pub won: u8,
    pub drawn: u8,
    pub lost: u8,
    #[serde(rename = "goalsFor")]
    pub goals_for: u16,
    #[serde(rename = "goalsAgainst")]
    pub goals_against: u16,
    #[serde(rename = "goalDifference")]
    pub goal_difference: i16,
    #[serde(rename = "rougesFor")]
    #[serde(default)]
    pub rouges_for: u16,
    #[serde(rename = "rougesAgainst")]
    #[serde(default)]
    pub rouges_against: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: String,
    #[serde(rename = "eventType")]
    pub event_type: EventType,
    pub date: String,
    #[serde(rename = "requiresUserAction")]
    pub requires_user_action: bool,
    pub processed: bool,
    pub result: Option<EventResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    Match {
        #[serde(rename = "matchId")]
        match_id: String,
        #[serde(rename = "isUserTeam")]
        is_user_team: bool,
    },
    MediaInquiry {
        question: String,
        options: Vec<String>,
    },
    PlayerNegotiation {
        #[serde(rename = "playerId")]
        player_id: String,
        #[serde(rename = "clubId")]
        club_id: String,
        #[serde(rename = "offerType")]
        offer_type: String,
    },
    HistoricalAnnouncement {
        title: String,
        description: String,
    },
    CupAnnouncement {
        #[serde(rename = "cupName")]
        cup_name: String,
        #[serde(rename = "competitionId")]
        competition_id: String,
        title: String,
        description: String,
        #[serde(rename = "eligibleDivisions")]
        eligible_divisions: Vec<String>,
    },
    CupDraw {
        #[serde(rename = "cupName")]
        cup_name: String,
        #[serde(rename = "competitionId")]
        competition_id: String,
        title: String,
        description: String,
        #[serde(rename = "drawBracket")]
        draw_bracket: Vec<serde_json::Value>,
    },
    CupDrawPending {
        #[serde(rename = "cupName")]
        cup_name: String,
        #[serde(rename = "competitionId")]
        competition_id: String,
        title: String,
        description: String,
    },
    TrialSession {
        #[serde(rename = "clubId")]
        club_id: String,
        title: String,
        description: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResult {
    #[serde(rename = "eventId")]
    pub event_id: String,
    pub summary: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayProcessingResult {
    #[serde(rename = "autoProcessed")]
    pub auto_processed: Vec<EventResult>,
    #[serde(rename = "requireUserAction")]
    pub require_user_action: Vec<GameEvent>,
    #[serde(rename = "allComplete")]
    pub all_complete: bool,
}

impl GameState {
    pub fn new(user_club_id: String) -> Self {
        let mut game = GameState {
            id: uuid::Uuid::new_v4().to_string(),
            season: 1888,
            current_gameweek: 1,
            current_date: "1888-04-17".to_string(),
            user_club_id,
            game_mode: None,
            clubs: Self::default_clubs(),
            matches: vec![],
            players: vec![],
            standings: Self::default_standings(),
            pending_events: vec![],
            processed_events: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            loaded_from_file: None,
        };

        // Load historical fixtures
        game.matches = Self::load_historical_fixtures(&game.user_club_id);
        // Schedule match events
        game.schedule_match_events();

        game
    }

    fn default_clubs() -> Vec<Club> {
        vec![
            Club {
                id: "accrington".to_string(),
                name: "Accrington FC".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5000,
                reputation: 50.0,
            },
            Club {
                id: "aston-villa".to_string(),
                name: "Aston Villa".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 6000,
                reputation: 55.0,
            },
            Club {
                id: "blackburn".to_string(),
                name: "Blackburn Rovers".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 6000,
                reputation: 55.0,
            },
            Club {
                id: "bolton".to_string(),
                name: "Bolton Wanderers".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5500,
                reputation: 52.0,
            },
            Club {
                id: "burnley".to_string(),
                name: "Burnley".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5200,
                reputation: 51.0,
            },
            Club {
                id: "derby".to_string(),
                name: "Derby County".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 6500,
                reputation: 58.0,
            },
            Club {
                id: "everton".to_string(),
                name: "Everton".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5800,
                reputation: 54.0,
            },
            Club {
                id: "notts-county".to_string(),
                name: "Notts County".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5000,
                reputation: 50.0,
            },
            Club {
                id: "preston".to_string(),
                name: "Preston North End".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 6800,
                reputation: 60.0,
            },
            Club {
                id: "stoke".to_string(),
                name: "Stoke".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5200,
                reputation: 51.0,
            },
            Club {
                id: "west-brom".to_string(),
                name: "West Bromwich Albion".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5600,
                reputation: 53.0,
            },
            Club {
                id: "wolves".to_string(),
                name: "Wolverhampton Wanderers".to_string(),
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                budget: 5400,
                reputation: 52.0,
            },
        ]
    }

    fn default_standings() -> Vec<Standing> {
        vec![
            Standing {
                club_id: "accrington".to_string(),
                club_name: "Accrington FC".to_string(),
                position: 1,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "aston-villa".to_string(),
                club_name: "Aston Villa".to_string(),
                position: 2,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "blackburn".to_string(),
                club_name: "Blackburn Rovers".to_string(),
                position: 3,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "bolton".to_string(),
                club_name: "Bolton Wanderers".to_string(),
                position: 4,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "burnley".to_string(),
                club_name: "Burnley".to_string(),
                position: 5,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "derby".to_string(),
                club_name: "Derby County".to_string(),
                position: 6,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "everton".to_string(),
                club_name: "Everton".to_string(),
                position: 7,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "notts-county".to_string(),
                club_name: "Notts County".to_string(),
                position: 8,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "preston".to_string(),
                club_name: "Preston North End".to_string(),
                position: 9,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "stoke".to_string(),
                club_name: "Stoke".to_string(),
                position: 10,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "west-brom".to_string(),
                club_name: "West Bromwich Albion".to_string(),
                position: 11,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
            Standing {
                club_id: "wolves".to_string(),
                club_name: "Wolverhampton Wanderers".to_string(),
                position: 12,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                rouges_for: 0,
                rouges_against: 0,
            },
        ]
    }

    fn load_historical_fixtures(user_club_id: &str) -> Vec<Match> {
        use std::fs::File;
        use std::io::BufReader;

        let csv_path = "MATCH_DATES_1888_89.csv";

        match File::open(csv_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut csv_reader = csv::Reader::from_reader(reader);
                let mut matches = vec![];

                for result in csv_reader.records() {
                    if let Ok(record) = result {
                        if let (Some(date), Some(gameweek), Some(home_team), Some(away_team)) = (
                            record.get(0),
                            record.get(1),
                            record.get(2),
                            record.get(4),
                        ) {
                            let home_team_id = Self::normalize_team_name(home_team);
                            let away_team_id = Self::normalize_team_name(away_team);
                            let gameweek_num = gameweek.parse::<u8>().unwrap_or(1);

                            let match_obj = Match {
                                id: uuid::Uuid::new_v4().to_string(),
                                gameweek: gameweek_num,
                                home_team_id,
                                away_team_id,
                                home_score: None,
                                away_score: None,
                                date: date.to_string(),
                                played: false,
                            };

                            matches.push(match_obj);
                        }
                    }
                }

                matches
            }
            Err(_) => {
                // If CSV not found, return empty vector
                // In production, should log this
                vec![]
            }
        }
    }

    fn normalize_team_name(name: &str) -> String {
        match name {
            "Preston North End" => "preston",
            "Aston Villa" => "aston-villa",
            "Wolverhampton Wanderers" => "wolves",
            "Blackburn Rovers" => "blackburn",
            "Bolton Wanderers" => "bolton",
            "West Bromwich Albion" => "west-brom",
            "Accrington" => "accrington",
            "Everton" => "everton",
            "Burnley" => "burnley",
            "Derby County" => "derby",
            "Notts County" => "notts-county",
            "Stoke" => "stoke",
            _ => return name.to_lowercase().replace(" ", "-"),
        }
        .to_string()
    }

    fn schedule_match_events(&mut self) {
        for match_obj in &self.matches {
            let is_user_team =
                match_obj.home_team_id == self.user_club_id || match_obj.away_team_id == self.user_club_id;

            self.pending_events.push(GameEvent {
                id: uuid::Uuid::new_v4().to_string(),
                event_type: EventType::Match {
                    match_id: match_obj.id.clone(),
                    is_user_team,
                },
                date: match_obj.date.clone(),
                requires_user_action: is_user_team,
                processed: false,
                result: None,
            });
        }

        // Sort by date
        self.pending_events.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn advance_day(&mut self) {
        if let Ok(date) = NaiveDate::parse_from_str(&self.current_date, "%Y-%m-%d") {
            if let Some(next_date) = date.succ_opt() {
                self.current_date = next_date.to_string();
                self.updated_at = Utc::now();
            }
        }
    }

    pub fn process_day_advance(&mut self) -> DayProcessingResult {
        // Advance to next day
        self.advance_day();

        // Find events for this date
        let today_events: Vec<usize> = self
            .pending_events
            .iter()
            .enumerate()
            .filter(|(_, e)| e.date == self.current_date && !e.processed)
            .map(|(i, _)| i)
            .collect();

        if today_events.is_empty() {
            return DayProcessingResult {
                auto_processed: vec![],
                require_user_action: vec![],
                all_complete: true,
            };
        }

        let mut auto_processed = vec![];
        let mut require_user_action = vec![];

        // Process events in reverse order to maintain indices
        for &idx in today_events.iter().rev() {
            if idx < self.pending_events.len() {
                let event = self.pending_events[idx].clone();

                if event.requires_user_action {
                    require_user_action.push(event);
                } else {
                    // Auto-process event
                    let result = self.execute_event(&event);

                    // Update the event in pending_events
                    if idx < self.pending_events.len() {
                        self.pending_events[idx].processed = true;
                        self.pending_events[idx].result = Some(result.clone());
                    }

                    auto_processed.push(result);
                }
            }
        }

        // Move processed events to history
        let processed: Vec<GameEvent> = self
            .pending_events
            .iter()
            .filter(|e| e.processed)
            .cloned()
            .collect();
        self.processed_events.extend(processed);
        self.pending_events.retain(|e| !e.processed);

        let all_complete = require_user_action.is_empty();
        DayProcessingResult {
            auto_processed,
            require_user_action,
            all_complete,
        }
    }

    fn execute_event(&mut self, event: &GameEvent) -> EventResult {
        match &event.event_type {
            EventType::Match {
                match_id,
                is_user_team: _,
            } => {
                // Take the match result. Normally the engine has already played
                // this match during advance_day (real Sheffield-Rules simulation,
                // pre-set scores, played = true) — keep that result. If the engine
                // couldn't reach it (no database context, no squads), fall back to
                // a deterministic roll seeded from the match id, so even the
                // fallback replays identically from a save.
                let match_data = if let Some(match_obj) = self.matches.iter_mut().find(|m| &m.id == match_id) {
                    if !match_obj.played {
                        let mut rng = fsim_core::Rng::seed(crate::fsim_bridge::seed_from(&match_obj.id));
                        match_obj.home_score = Some(rng.roll(5) as u8);
                        match_obj.away_score = Some(rng.roll(5) as u8);
                        match_obj.played = true;
                    }

                    // Clone match data before releasing the mutable borrow
                    Some(match_obj.clone())
                } else {
                    None
                };

                // Update standings based on result
                if let Some(match_obj) = match_data {
                    self.update_standings_from_match(&match_obj);

                    EventResult {
                        event_id: event.id.clone(),
                        summary: format!(
                            "{} {} - {} {}",
                            match_obj.home_team_id,
                            match_obj.home_score.unwrap(),
                            match_obj.away_score.unwrap(),
                            match_obj.away_team_id
                        ),
                        data: serde_json::json!({
                            "matchId": match_obj.id,
                            "homeTeam": match_obj.home_team_id,
                            "awayTeam": match_obj.away_team_id,
                            "homeScore": match_obj.home_score,
                            "awayScore": match_obj.away_score,
                        }),
                    }
                } else {
                    EventResult {
                        event_id: event.id.clone(),
                        summary: "Match not found".to_string(),
                        data: serde_json::json!({}),
                    }
                }
            }
            _ => EventResult {
                event_id: event.id.clone(),
                summary: "Event processed".to_string(),
                data: serde_json::json!({}),
            },
        }
    }

    fn update_standings_from_match(&mut self, match_obj: &Match) {
        // Update home team standing
        if let Some(standing) = self
            .standings
            .iter_mut()
            .find(|s| s.club_id == match_obj.home_team_id)
        {
            standing.played += 1;
            standing.goals_for += match_obj.home_score.unwrap_or(0) as u16;
            standing.goals_against += match_obj.away_score.unwrap_or(0) as u16;

            if match_obj.home_score > match_obj.away_score {
                standing.won += 1;
                standing.points += 2; // 2 points for win in 1888
            } else if match_obj.home_score == match_obj.away_score {
                standing.drawn += 1;
                standing.points += 1;
            } else {
                standing.lost += 1;
            }
            standing.goal_difference = standing.goals_for as i16 - standing.goals_against as i16;
        }

        // Update away team standing
        if let Some(standing) = self
            .standings
            .iter_mut()
            .find(|s| s.club_id == match_obj.away_team_id)
        {
            standing.played += 1;
            standing.goals_for += match_obj.away_score.unwrap_or(0) as u16;
            standing.goals_against += match_obj.home_score.unwrap_or(0) as u16;

            if match_obj.away_score > match_obj.home_score {
                standing.won += 1;
                standing.points += 2;
            } else if match_obj.home_score == match_obj.away_score {
                standing.drawn += 1;
                standing.points += 1;
            } else {
                standing.lost += 1;
            }
            standing.goal_difference = standing.goals_for as i16 - standing.goals_against as i16;
        }

        // Re-sort standings
        // If no matches have been played, sort alphabetically by club name
        // Otherwise sort by points (descending), then goal difference, then goals for
        if self.standings.iter().all(|s| s.played == 0) {
            self.standings.sort_by(|a, b| a.club_name.cmp(&b.club_name));
        } else {
            self.standings.sort_by(|a, b| {
                b.points
                    .cmp(&a.points)
                    .then_with(|| b.goal_difference.cmp(&a.goal_difference))
                    .then_with(|| b.goals_for.cmp(&a.goals_for))
            });
        }

        // Update positions
        for (i, standing) in self.standings.iter_mut().enumerate() {
            standing.position = (i + 1) as u16;
        }
    }
}

impl Match {
    pub fn result_string(&self) -> Option<String> {
        match (self.home_score, self.away_score) {
            (Some(h), Some(a)) => {
                if h > a {
                    Some(format!("{}-{} (Home Win)", h, a))
                } else if a > h {
                    Some(format!("{}-{} (Away Win)", h, a))
                } else {
                    Some(format!("{}-{} (Draw)", h, a))
                }
            }
            _ => None,
        }
    }
}

impl Club {
    pub fn goal_difference(&self) -> i16 {
        self.goals_for as i16 - self.goals_against as i16
    }
}
