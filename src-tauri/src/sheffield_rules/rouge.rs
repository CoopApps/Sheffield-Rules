/// Sheffield Rules Rouge Scoring System (1862-1868)
///
/// The rouge was a unique scoring mechanic in Sheffield Rules where:
/// - A ball kicked between the uprights BELOW the crossbar = 1 rouge
/// - A ball kicked between the uprights ABOVE the crossbar (goal) = 2 points
/// - Rouge required the ball to be "touched down" past the goal line
///
/// This was effectively a predecessor to rugby's try system.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScoreType {
    Goal,
    Rouge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RougeScore {
    pub match_id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub home_goals: u8,
    pub home_rouges: u8,
    pub away_goals: u8,
    pub away_rouges: u8,
}

impl RougeScore {
    /// Create a new rouge score tracker
    pub fn new(match_id: String, home_team_id: String, away_team_id: String) -> Self {
        RougeScore {
            match_id,
            home_team_id,
            away_team_id,
            home_goals: 0,
            home_rouges: 0,
            away_goals: 0,
            away_rouges: 0,
        }
    }

    /// Record a score for the home team
    pub fn home_score(&mut self, score_type: ScoreType) {
        match score_type {
            ScoreType::Goal => self.home_goals += 1,
            ScoreType::Rouge => self.home_rouges += 1,
        }
    }

    /// Record a score for the away team
    pub fn away_score(&mut self, score_type: ScoreType) {
        match score_type {
            ScoreType::Goal => self.away_goals += 1,
            ScoreType::Rouge => self.away_rouges += 1,
        }
    }

    /// Total scores for display (goals + rouges, not for determining winner)
    pub fn home_points(&self) -> u16 {
        self.home_goals as u16 + self.home_rouges as u16
    }

    /// Total scores for display
    pub fn away_points(&self) -> u16 {
        self.away_goals as u16 + self.away_rouges as u16
    }

    /// Determine the match result per Oct 1867 Sheffield Rules:
    /// "A goal outweighs any number of rouges."
    /// Tier 1: goals; if equal, Tier 2: rouges; if still equal, draw.
    pub fn get_result(&self) -> MatchResult {
        if self.home_goals != self.away_goals {
            if self.home_goals > self.away_goals {
                MatchResult::HomeWin
            } else {
                MatchResult::AwayWin
            }
        } else if self.home_rouges != self.away_rouges {
            if self.home_rouges > self.away_rouges {
                MatchResult::HomeWin
            } else {
                MatchResult::AwayWin
            }
        } else {
            MatchResult::Draw
        }
    }

    /// Get a human-readable score string (e.g., "2-1, 1-0 r")
    pub fn display_score(&self) -> String {
        format!(
            "{}-{}, {}-{} r",
            self.home_goals, self.away_goals, self.home_rouges, self.away_rouges
        )
    }

    /// Get match summary
    pub fn summary(&self) -> String {
        let home_pts = self.home_points();
        let away_pts = self.away_points();
        let result = match self.get_result() {
            MatchResult::HomeWin => format!("{} wins", self.home_team_id),
            MatchResult::AwayWin => format!("{} wins", self.away_team_id),
            MatchResult::Draw => "Match drawn".to_string(),
        };

        format!(
            "{} {} vs {} {}: {} (pts: {} vs {})",
            self.home_team_id,
            self.display_score(),
            self.away_team_id,
            self.display_score(),
            result,
            home_pts,
            away_pts
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchResult {
    HomeWin,
    AwayWin,
    Draw,
}

/// Simulate a match with rouge scoring system
pub fn simulate_rouge_match() -> RougeScore {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut score = RougeScore::new(
        "test".to_string(),
        "home".to_string(),
        "away".to_string(),
    );

    // Simulate goals (higher probability)
    let home_goals = rng.gen_range(0..4);
    let away_goals = rng.gen_range(0..4);

    for _ in 0..home_goals {
        score.home_score(ScoreType::Goal);
    }

    for _ in 0..away_goals {
        score.away_score(ScoreType::Goal);
    }

    // Simulate rouges (lower probability)
    let home_rouges = if rng.gen_bool(0.4) {
        rng.gen_range(0..3)
    } else {
        0
    };
    let away_rouges = if rng.gen_bool(0.4) {
        rng.gen_range(0..3)
    } else {
        0
    };

    for _ in 0..home_rouges {
        score.home_score(ScoreType::Rouge);
    }

    for _ in 0..away_rouges {
        score.away_score(ScoreType::Rouge);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rouge_score_creation() {
        let score = RougeScore::new(
            "match1".to_string(),
            "sheffield-fc".to_string(),
            "hallam-fc".to_string(),
        );

        assert_eq!(score.home_goals, 0);
        assert_eq!(score.home_rouges, 0);
        assert_eq!(score.home_points(), 0);
    }

    #[test]
    fn test_rouge_scoring() {
        let mut score = RougeScore::new(
            "match1".to_string(),
            "sheffield-fc".to_string(),
            "hallam-fc".to_string(),
        );

        // Home team: 2 goals + 1 rouge = 3 total (display, not for winner determination)
        score.home_score(ScoreType::Goal);
        score.home_score(ScoreType::Goal);
        score.home_score(ScoreType::Rouge);

        assert_eq!(score.home_goals, 2);
        assert_eq!(score.home_rouges, 1);
        assert_eq!(score.home_points(), 3);  // goals + rouges for display
    }

    #[test]
    fn test_rouge_match_result() {
        let mut score = RougeScore::new(
            "match1".to_string(),
            "sheffield-fc".to_string(),
            "hallam-fc".to_string(),
        );

        // Home team: 1 goal (2 points)
        // Away team: 1 rouge (1 point)
        score.home_score(ScoreType::Goal);
        score.away_score(ScoreType::Rouge);

        assert_eq!(score.get_result(), MatchResult::HomeWin);
    }

    #[test]
    fn test_rouge_display_score() {
        let mut score = RougeScore::new(
            "match1".to_string(),
            "sheffield-fc".to_string(),
            "hallam-fc".to_string(),
        );

        score.home_score(ScoreType::Goal);
        score.home_score(ScoreType::Rouge);
        score.away_score(ScoreType::Goal);

        assert_eq!(score.display_score(), "1-1, 1-0 r");
    }
}
